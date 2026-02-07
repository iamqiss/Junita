//! Rendering engine integration for hot reload updates
//!
//! This module bridges the hot reload system with the actual rendering engine (junita_gpu).
//! Integration points are marked for connection with the real rendering backend.
//!
//! # GPU Integration
//!
//! The RenderingAdapter manages the widget scene graph and applies hot reload diffs.
//! When a diff is applied, it updates the scene nodes and signals the GPU renderer
//! to re-render the affected regions.
//!
//! # Architecture
//!
//! ```text
//! Hot Reload Manager
//!     ↓ (tree diffs)
//! RenderingAdapter
//!     ↓ (update scene nodes)
//! junita_gpu::GpuRenderer
//!     ↓ (render pipeline)
//! Frame on Screen
//! ```

use crate::hot_reload::{WidgetDiff, WidgetNode};
use anyhow::Result;
use tracing::{info, debug};
use std::sync::Arc;
use tokio::sync::Mutex;

/// GPU Backend Interface (trait for testing and platform independence)
pub trait GpuBackend: Send + Sync {
    /// Update widget properties in the GPU
    fn update_widget_properties(
        &mut self,
        id: u32,
        props: &std::collections::HashMap<String, String>,
    ) -> Result<()>;

    /// Create a new widget in the GPU
    fn create_widget(&mut self, id: u32, widget_type: &str) -> Result<()>;

    /// Delete a widget from the GPU
    fn destroy_widget(&mut self, id: u32) -> Result<()>;

    /// Request frame re-render
    fn request_frame(&self) -> Result<()>;
}

/// Mock GPU backend for testing
struct MockGpuBackend;

impl GpuBackend for MockGpuBackend {
    fn update_widget_properties(
        &mut self,
        _id: u32,
        _props: &std::collections::HashMap<String, String>,
    ) -> Result<()> {
        Ok(())
    }

    fn create_widget(&mut self, _id: u32, _widget_type: &str) -> Result<()> {
        Ok(())
    }

    fn destroy_widget(&mut self, _id: u32) -> Result<()> {
        Ok(())
    }

    fn request_frame(&self) -> Result<()> {
        Ok(())
    }
}

/// Scene graph node representing a rendered widget
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id: u32,
    pub widget_type: String,
    pub properties: std::collections::HashMap<String, String>,
    pub children: Vec<u32>,
}

/// Rendering engine adapter for hot reload
pub struct RenderingAdapter {
    scene_nodes: std::collections::HashMap<u32, SceneNode>,
    root_id: Option<u32>,
    gpu_backend: Arc<Mutex<Box<dyn GpuBackend>>>,
}

impl RenderingAdapter {
    pub fn new() -> Self {
        Self {
            scene_nodes: Default::default(),
            root_id: None,
            gpu_backend: Arc::new(Mutex::new(Box::new(MockGpuBackend))),
        }
    }

    /// Create with a custom GPU backend (for integration with junita_gpu)
    pub fn with_gpu_backend(
        gpu_backend: Arc<Mutex<Box<dyn GpuBackend>>>,
    ) -> Self {
        Self {
            scene_nodes: Default::default(),
            root_id: None,
            gpu_backend,
        }
    }

    /// Apply a diff to the scene graph (async for GPU integration)
    pub async fn apply_diff(&mut self, diff: &WidgetDiff) -> Result<()> {
        match diff {
            WidgetDiff::Updated {
                id,
                changed_props,
            } => {
                self.update_widget_properties_async(id.0, changed_props).await?;
                info!("Updated widget {:?} properties", id);
            }
            WidgetDiff::Added {
                id,
                widget,
                parent_id,
            } => {
                self.add_widget_async(id.0, &widget.widget_type, parent_id.map(|p| p.0)).await?;
                info!("Added widget {:?} to parent {:?}", id, parent_id);
            }
            WidgetDiff::Removed { id } => {
                self.remove_widget_async(id.0).await?;
                info!("Removed widget {:?}", id);
            }
            WidgetDiff::Reordered {
                parent_id,
                new_order,
            } => {
                let order: Vec<u32> = new_order.iter().map(|id| id.0).collect();
                self.reorder_children(parent_id.0, &order)?;
                info!("Reordered children of widget {:?}", parent_id);
            }
        }

        // Mark frame dirty to trigger render
        self.request_frame().await?;

        Ok(())
    }

    /// Update widget properties in the scene graph (async with GPU backend)
    async fn update_widget_properties_async(
        &mut self,
        id: u32,
        changed_props: &std::collections::HashMap<String, String>,
    ) -> Result<()> {
        if let Some(node) = self.scene_nodes.get_mut(&id) {
            for (key, value) in changed_props {
                node.properties.insert(key.clone(), value.clone());
                debug!("Updated {}={}", key, value);
            }

            // GPU Backend Integration: Update properties in render pipeline
            let mut backend = self.gpu_backend.lock().await;
            backend.update_widget_properties(id, changed_props)?;

            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Widget {} not found in scene graph",
                id
            ))
        }
    }

    /// Add a new widget to the scene graph (async with GPU backend)
    async fn add_widget_async(
        &mut self,
        id: u32,
        widget_type: &str,
        parent_id: Option<u32>,
    ) -> Result<()> {
        let node = SceneNode {
            id,
            widget_type: widget_type.to_string(),
            properties: Default::default(),
            children: Vec::new(),
        };

        self.scene_nodes.insert(id, node);

        if let Some(parent) = parent_id {
            if let Some(parent_node) = self.scene_nodes.get_mut(&parent) {
                parent_node.children.push(id);
            }
        } else {
            self.root_id = Some(id);
        }

        // GPU Backend Integration: Create widget in render pipeline
        let mut backend = self.gpu_backend.lock().await;
        backend.create_widget(id, widget_type)?;

        debug!("Created scene node {} (type: {})", id, widget_type);
        Ok(())
    }

    /// Remove a widget from the scene graph (async with GPU backend)
    async fn remove_widget_async(&mut self, id: u32) -> Result<()> {
        if self.scene_nodes.remove(&id).is_some() {
            // Remove from parent's children
            for node in self.scene_nodes.values_mut() {
                node.children.retain(|&child_id| child_id != id);
            }

            // GPU Backend Integration: Destroy widget in render pipeline
            let mut backend = self.gpu_backend.lock().await;
            backend.destroy_widget(id)?;

            debug!("Removed scene node {}", id);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Widget {} not found in scene graph",
                id
            ))
        }
    }

    /// Reorder children of a widget
    fn reorder_children(
        &mut self,
        parent_id: u32,
        new_order: &[u32],
    ) -> Result<()> {
        if let Some(parent) = self.scene_nodes.get_mut(&parent_id) {
            parent.children = new_order.to_vec();
            debug!("Reordered children of widget {}", parent_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Parent widget {} not found",
                parent_id
            ))
        }
    }

    /// Request frame re-render (async)
    async fn request_frame(&self) -> Result<()> {
        let backend = self.gpu_backend.lock().await;
        backend.request_frame()?;
        info!("Frame render requested");
        Ok(())
    }

    /// Get a scene node
    pub fn get_node(&self, id: u32) -> Option<&SceneNode> {
        self.scene_nodes.get(&id)
    }

    /// Get mutable scene node
    pub fn get_node_mut(&mut self, id: u32) -> Option<&mut SceneNode> {
        self.scene_nodes.get_mut(&id)
    }

    /// Clear all nodes
    pub fn clear(&mut self) {
        self.scene_nodes.clear();
        self.root_id = None;
    }

    /// Get scene statistics
    pub fn stats(&self) -> SceneStats {
        SceneStats {
            total_nodes: self.scene_nodes.len(),
            root_id: self.root_id,
        }
    }

    // Synchronous wrappers for backwards compatibility
    pub fn add_widget(
        &mut self,
        id: u32,
        widget_type: &str,
        parent_id: Option<u32>,
    ) -> Result<()> {
        let node = SceneNode {
            id,
            widget_type: widget_type.to_string(),
            properties: Default::default(),
            children: Vec::new(),
        };

        self.scene_nodes.insert(id, node);

        if let Some(parent) = parent_id {
            if let Some(parent_node) = self.scene_nodes.get_mut(&parent) {
                parent_node.children.push(id);
            }
        } else {
            self.root_id = Some(id);
        }

        debug!("Created scene node {} (type: {})", id, widget_type);
        Ok(())
    }

    pub fn update_widget_properties(
        &mut self,
        id: u32,
        changed_props: &std::collections::HashMap<String, String>,
    ) -> Result<()> {
        if let Some(node) = self.scene_nodes.get_mut(&id) {
            for (key, value) in changed_props {
                node.properties.insert(key.clone(), value.clone());
                debug!("Updated {}={}", key, value);
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Widget {} not found in scene graph",
                id
            ))
        }
    }

    pub fn remove_widget(&mut self, id: u32) -> Result<()> {
        if self.scene_nodes.remove(&id).is_some() {
            for node in self.scene_nodes.values_mut() {
                node.children.retain(|&child_id| child_id != id);
            }
            debug!("Removed scene node {}", id);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Widget {} not found in scene graph",
                id
            ))
        }
    }
}

impl Default for RenderingAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the scene graph
#[derive(Debug, Clone)]
pub struct SceneStats {
    pub total_nodes: usize,
    pub root_id: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_widget() {
        let mut adapter = RenderingAdapter::new();
        adapter
            .add_widget(1, "Box", None)
            .expect("Failed to add widget");
        assert!(adapter.get_node(1).is_some());
    }

    #[test]
    fn test_update_properties() {
        let mut adapter = RenderingAdapter::new();
        adapter
            .add_widget(1, "Text", None)
            .expect("Failed to add widget");

        let mut props = std::collections::HashMap::new();
        props.insert("text".to_string(), "Hello".to_string());
        adapter
            .update_widget_properties(1, &props)
            .expect("Failed to update");

        let node = adapter.get_node(1).unwrap();
        assert_eq!(node.properties.get("text"), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_remove_widget() {
        let mut adapter = RenderingAdapter::new();
        adapter
            .add_widget(1, "Box", None)
            .expect("Failed to add widget");
        adapter.remove_widget(1).expect("Failed to remove");
        assert!(adapter.get_node(1).is_none());
    }

    #[test]
    fn test_hierarchy() {
        let mut adapter = RenderingAdapter::new();
        adapter
            .add_widget(1, "Box", None)
            .expect("Failed to add root");
        adapter
            .add_widget(2, "Text", Some(1))
            .expect("Failed to add child");

        let parent = adapter.get_node(1).unwrap();
        assert!(parent.children.contains(&2));
    }
}
