//! Rendering engine integration for hot reload updates
//!
//! This module bridges the hot reload system with the actual rendering engine (junita_gpu).
//! Integration points are marked for connection with the real rendering backend.

use crate::hot_reload::{WidgetDiff, WidgetNode};
use anyhow::Result;
use tracing::{info, debug};

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
}

impl RenderingAdapter {
    pub fn new() -> Self {
        Self {
            scene_nodes: Default::default(),
            root_id: None,
        }
    }

    /// Apply a diff to the scene graph
    pub fn apply_diff(&mut self, diff: &WidgetDiff) -> Result<()> {
        match diff {
            WidgetDiff::Updated {
                id,
                changed_props,
            } => {
                self.update_widget_properties(id.0, changed_props)?;
                info!("Updated widget {:?} properties", id);
            }
            WidgetDiff::Added {
                id,
                widget,
                parent_id,
            } => {
                self.add_widget(id.0, &widget.widget_type, parent_id.map(|p| p.0))?;
                info!("Added widget {:?} to parent {:?}", id, parent_id);
            }
            WidgetDiff::Removed { id } => {
                self.remove_widget(id.0)?;
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
        self.request_frame()?;

        Ok(())
    }

    /// Update widget properties in the scene graph
    fn update_widget_properties(
        &mut self,
        id: u32,
        changed_props: &std::collections::HashMap<String, String>,
    ) -> Result<()> {
        if let Some(node) = self.scene_nodes.get_mut(&id) {
            for (key, value) in changed_props {
                node.properties.insert(key.clone(), value.clone());
                debug!("Updated {}={}", key, value);
            }

            // TODO: When junita_gpu is integrated:
            // - Call render backend to update widget properties
            // - Example: gpu_device.update_widget_properties(id, changed_props)?;
            // - This triggers a property update in the wgpu render pipeline

            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Widget {} not found in scene graph",
                id
            ))
        }
    }

    /// Add a new widget to the scene graph
    fn add_widget(
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

        // TODO: When junita_gpu is integrated:
        // - Create widget instance in GPU
        // - Example: gpu_device.create_widget(id, widget_type)?;
        // - Add to render batch

        debug!("Created scene node {} (type: {})", id, widget_type);
        Ok(())
    }

    /// Remove a widget from the scene graph
    fn remove_widget(&mut self, id: u32) -> Result<()> {
        if self.scene_nodes.remove(&id).is_some() {
            // Remove from parent's children
            for node in self.scene_nodes.values_mut() {
                node.children.retain(|&child_id| child_id != id);
            }

            // TODO: When junita_gpu is integrated:
            // - Destroy widget instance in GPU
            // - Example: gpu_device.destroy_widget(id)?;
            // - Remove from render batch

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

            // TODO: When junita_gpu is integrated:
            // - Update render order in GPU
            // - Example: gpu_device.set_child_order(parent_id, new_order)?;

            debug!("Reordered children of widget {}", parent_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Parent widget {} not found",
                parent_id
            ))
        }
    }

    /// Request a frame render (triggers the render loop)
    fn request_frame(&self) -> Result<()> {
        // TODO: When junita_gpu is integrated:
        // - Signal the render thread to redraw
        // - Example: frame_request_channel.send(FrameRequest::Render)?;
        // - This causes the GPU to re-render with updated state

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
    fn test_add_widget() {
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
