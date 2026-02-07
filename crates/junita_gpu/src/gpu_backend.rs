//! GpuBackend implementation for junita_core hot reload integration
//!
//! This module provides the concrete implementation of the GpuBackend trait
//! from junita_core, bridging the hot reload system with the GPU renderer.
//!
//! # Design
//!
//! The GpuBackend implementation tracks the widget scene graph and coordinates
//! with the rendering pipeline. This design keeps the widget abstraction layer
//! separate from the low-level GPU rendering.
//!
//! While junita_gpu operates at the primitive level (triangles, SDFs, text glyphs),
//! this backend maintains the high-level widget hierarchy for state management
//! and debugging.
//!
//! # Architecture
//!
//! ```text
//! Hot Reload Manager
//!     ↓ (widget diffs)
//! WidgetBackend (GpuBackend trait impl)
//!     ↓ (tracks scene graph)
//! RenderingPipeline
//!     ↓ (full frame re-render)
//! GpuRenderer
//! ```

use std::collections::HashMap;
use anyhow::{Result, anyhow};
use tracing::{debug, info};

/// Simple widget backend that tracks scene graph state
///
/// This backend is designed to be thread-safe and can be wrapped in Arc<Mutex<>>
/// for use with the hot reload system. It tracks widget creation, updates, and
/// destruction without holding references to GPU resources.
pub struct WidgetBackend {
    /// Map of widget ID -> widget info
    widgets: HashMap<u32, WidgetInfo>,
    
    /// Root widget ID (top-level widget)
    root_id: Option<u32>,
    
    /// Frame dirty flag (set when updates require re-render)
    frame_dirty: bool,
}

/// Information about a widget in the scene
#[derive(Debug, Clone)]
struct WidgetInfo {
    /// Unique widget identifier
    id: u32,
    
    /// Widget type name (e.g., "button", "container", "text")
    widget_type: String,
    
    /// Current properties (key-value pairs)
    properties: HashMap<String, String>,
    
    /// Child widget IDs in order
    children: Vec<u32>,
    
    /// Parent widget ID (if any)
    parent_id: Option<u32>,
}

impl WidgetBackend {
    /// Create a new widget backend
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            root_id: None,
            frame_dirty: false,
        }
    }

    /// Get widget registry statistics
    pub fn stats(&self) -> WidgetRegistryStats {
        WidgetRegistryStats {
            total_widgets: self.widgets.len(),
            root_id: self.root_id,
            frame_dirty: self.frame_dirty,
        }
    }

    /// Check if the widget exists in the registry
    fn has_widget(&self, id: u32) -> bool {
        self.widgets.contains_key(&id)
    }

    /// Get a widget by ID
    fn get_widget(&self, id: u32) -> Option<&WidgetInfo> {
        self.widgets.get(&id)
    }

    /// Get a mutable widget by ID
    fn get_widget_mut(&mut self, id: u32) -> Option<&mut WidgetInfo> {
        self.widgets.get_mut(&id)
    }

    /// Mark the frame as needing re-render
    fn mark_frame_dirty(&mut self) {
        self.frame_dirty = true;
        debug!("Frame marked as dirty - will re-render on next cycle");
    }

    /// Clear the frame dirty flag after rendering
    pub fn clear_frame_dirty(&mut self) {
        self.frame_dirty = false;
    }

    /// Check if frame needs re-rendering
    pub fn is_frame_dirty(&self) -> bool {
        self.frame_dirty
    }

    /// Get all widgets in the registry (for debugging)
    pub fn all_widgets(&self) -> Vec<WidgetInfo> {
        self.widgets
            .values()
            .cloned()
            .collect()
    }

    /// Validate the scene graph structure
    pub fn validate_hierarchy(&self) -> Result<()> {
        // Check that all parent references point to existing widgets
        for (id, widget) in &self.widgets {
            if let Some(parent_id) = widget.parent_id {
                if !self.widgets.contains_key(&parent_id) {
                    return Err(anyhow!(
                        "Widget {} has non-existent parent {}",
                        id,
                        parent_id
                    ));
                }
            }

            // Check that all children exist
            for child_id in &widget.children {
                if !self.widgets.contains_key(child_id) {
                    return Err(anyhow!(
                        "Widget {} has non-existent child {}",
                        id,
                        child_id
                    ));
                }
            }
        }

        Ok(())
    }
}

impl Default for WidgetBackend {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the widget registry
#[derive(Debug, Clone)]
pub struct WidgetRegistryStats {
    /// Total number of widgets currently registered
    pub total_widgets: usize,
    
    /// Root widget ID (if any)
    pub root_id: Option<u32>,
    
    /// Whether the frame needs re-rendering
    pub frame_dirty: bool,
}

// ============================================================================
// GpuBackend Trait Implementation
// ============================================================================

impl junita_core::rendering::GpuBackend for WidgetBackend {
    /// Create a new widget in the GPU backend
    fn create_widget(&mut self, id: u32, widget_type: &str) -> Result<()> {
        if self.has_widget(id) {
            return Err(anyhow!(
                "Widget {} already exists in registry",
                id
            ));
        }

        let widget = WidgetInfo {
            id,
            widget_type: widget_type.to_string(),
            properties: HashMap::new(),
            children: Vec::new(),
            parent_id: None,
        };

        self.widgets.insert(id, widget);
        self.mark_frame_dirty();

        info!(
            "Created widget {} of type '{}' (total widgets: {})",
            id,
            widget_type,
            self.widgets.len()
        );

        Ok(())
    }

    /// Update widget properties in the GPU backend
    fn update_widget_properties(
        &mut self,
        id: u32,
        props: &HashMap<String, String>,
    ) -> Result<()> {
        let widget = self.get_widget_mut(id)
            .ok_or_else(|| anyhow!("Widget {} not found", id))?;

        for (key, value) in props {
            widget.properties.insert(key.clone(), value.clone());
            debug!("Updated widget {} property: {}={}", id, key, value);
        }

        self.mark_frame_dirty();

        info!(
            "Updated widget {} with {} properties",
            id,
            props.len()
        );

        Ok(())
    }

    /// Delete a widget from the GPU backend
    fn destroy_widget(&mut self, id: u32) -> Result<()> {
        // Remove from widget registry
        if self.widgets.remove(&id).is_none() {
            return Err(anyhow!("Widget {} not found", id));
        }

        // Remove from parent's children list
        for widget in self.widgets.values_mut() {
            widget.children.retain(|&child_id| child_id != id);
        }

        // Update root ID if we removed the root
        if self.root_id == Some(id) {
            self.root_id = None;
        }

        self.mark_frame_dirty();

        info!(
            "Destroyed widget {} (remaining widgets: {})",
            id,
            self.widgets.len()
        );

        Ok(())
    }

    /// Request frame re-render
    fn request_frame(&self) -> Result<()> {
        // In the full implementation, this would:
        // 1. Accumulate all pending widget updates into a batch
        // 2. Trigger layout recalculation
        // 3. Generate PrimitiveBatch for GPU rendering
        // 4. Call renderer.render() with the batch
        //
        // For now, this is a no-op - the frame dirty flag signals that
        // a re-render is needed at the next rendering cycle.

        debug!("Frame render requested");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test widget creation and registration
    #[test]
    fn test_create_widget() -> Result<()> {
        let mut backend = WidgetBackend::new();
        backend.create_widget(1, "button")?;
        
        assert_eq!(backend.stats().total_widgets, 1);
        assert!(backend.is_frame_dirty());
        Ok(())
    }

    /// Test widget property updates
    #[test]
    fn test_update_properties() -> Result<()> {
        let mut backend = WidgetBackend::new();
        backend.create_widget(1, "button")?;
        
        let mut props = HashMap::new();
        props.insert("color".to_string(), "blue".to_string());
        props.insert("size".to_string(), "large".to_string());
        
        backend.clear_frame_dirty();
        backend.update_widget_properties(1, &props)?;
        
        assert!(backend.is_frame_dirty());
        assert_eq!(backend.stats().total_widgets, 1);
        Ok(())
    }

    /// Test widget destruction
    #[test]
    fn test_destroy_widget() -> Result<()> {
        let mut backend = WidgetBackend::new();
        backend.create_widget(1, "button")?;
        backend.clear_frame_dirty();
        
        backend.destroy_widget(1)?;
        
        assert_eq!(backend.stats().total_widgets, 0);
        assert!(backend.is_frame_dirty());
        Ok(())
    }

    /// Test error on duplicate creation
    #[test]
    fn test_duplicate_creation_error() {
        let mut backend = WidgetBackend::new();
        backend.create_widget(1, "button").unwrap();
        
        let result = backend.create_widget(1, "button");
        assert!(result.is_err());
    }

    /// Test error on non-existent destruction
    #[test]
    fn test_nonexistent_destruction_error() {
        let backend = WidgetBackend::new();
        
        let result = backend.destroy_widget(999);
        assert!(result.is_err());
    }
}

