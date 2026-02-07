//! Hot reload runtime client
//!
//! Handles state preservation, incremental updates, and client-side diff application.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::fmt;
use tracing::debug;

/// Unique identifier for a widget instance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WidgetId(pub u32);

impl fmt::Display for WidgetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "w{}", self.0)
    }
}

/// Snapshot of widget state for preservation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub signals: HashMap<String, Vec<u8>>,
    pub derived_values: HashMap<String, Vec<u8>>,
    pub dynamic_state: HashMap<String, Vec<u8>>,
    pub timestamp: u64,
}

impl StateSnapshot {
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
            derived_values: HashMap::new(),
            dynamic_state: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

/// Widget tree node for diffing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetNode {
    pub id: WidgetId,
    pub widget_type: String,
    pub props: HashMap<String, String>,
    pub children: Vec<WidgetNode>,
    pub state_hash: u64,
}

/// Represents changes between two widget trees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetDiff {
    /// Widget properties changed
    Updated {
        id: WidgetId,
        changed_props: HashMap<String, String>,
    },
    /// New widget added
    Added {
        id: WidgetId,
        widget: WidgetNode,
        parent_id: Option<WidgetId>,
    },
    /// Widget removed
    Removed {
        id: WidgetId,
    },
    /// Children reordered
    Reordered {
        parent_id: WidgetId,
        new_order: Vec<WidgetId>,
    },
}

/// Hot reload state manager
pub struct HotReloadManager {
    /// Current widget tree
    widget_tree: Arc<RwLock<Option<WidgetNode>>>,
    /// Preserved state snapshots
    state_snapshots: Arc<Mutex<Vec<StateSnapshot>>>,
    /// Pending updates waiting application
    pending_diffs: Arc<Mutex<Vec<WidgetDiff>>>,
    /// Whether hot reload is active
    enabled: bool,
}

impl HotReloadManager {
    pub fn new(enabled: bool) -> Self {
        Self {
            widget_tree: Arc::new(RwLock::new(None)),
            state_snapshots: Arc::new(Mutex::new(Vec::new())),
            pending_diffs: Arc::new(Mutex::new(Vec::new())),
            enabled,
        }
    }

    /// Save current state before recompilation
    pub fn save_state(&self) -> StateSnapshot {
        let snapshot = StateSnapshot::new();
        self.state_snapshots.lock().unwrap().push(snapshot.clone());
        snapshot
    }

    /// Restore state after successful recompilation
    pub fn restore_state(&self) -> Option<StateSnapshot> {
        self.state_snapshots.lock().unwrap().pop()
    }

    /// Clear all saved state
    pub fn clear_snapshots(&self) {
        self.state_snapshots.lock().unwrap().clear();
    }

    /// Store current widget tree
    pub fn set_widget_tree(&self, tree: WidgetNode) {
        *self.widget_tree.write().unwrap() = Some(tree);
    }

    /// Get current widget tree
    pub fn get_widget_tree(&self) -> Option<WidgetNode> {
        self.widget_tree.read().unwrap().clone()
    }

    /// Compute diff between old and new widget trees
    pub fn compute_diff(&self, new_tree: &WidgetNode) -> Vec<WidgetDiff> {
        let old_tree = self.widget_tree.read().unwrap();

        if let Some(old) = old_tree.as_ref() {
            Self::tree_diff(old, new_tree, None)
        } else {
            vec![WidgetDiff::Added {
                id: new_tree.id,
                widget: new_tree.clone(),
                parent_id: None,
            }]
        }
    }

    /// Recursively diff two widget trees
    fn tree_diff(old: &WidgetNode, new: &WidgetNode, parent_id: Option<WidgetId>) -> Vec<WidgetDiff> {
        let mut diffs = Vec::new();

        if old.id != new.id {
            diffs.push(WidgetDiff::Removed { id: old.id });
            diffs.push(WidgetDiff::Added {
                id: new.id,
                widget: new.clone(),
                parent_id,
            });
            return diffs;
        }

        // Check for property changes
        let mut changed_props = HashMap::new();
        for (key, new_val) in &new.props {
            if old.props.get(key) != Some(new_val) {
                changed_props.insert(key.clone(), new_val.clone());
            }
        }

        if !changed_props.is_empty() {
            diffs.push(WidgetDiff::Updated {
                id: old.id,
                changed_props,
            });
        }

        // Check for removed properties
        for key in old.props.keys() {
            if !new.props.contains_key(key) {
                diffs.push(WidgetDiff::Updated {
                    id: old.id,
                    changed_props: {
                        let mut m = HashMap::new();
                        m.insert(key.clone(), String::new()); // Empty means remove
                        m
                    },
                });
            }
        }

        // Recursively diff children
        let old_children = &old.children;
        let new_children = &new.children;

        // Match children by type and recompile matching trees
        let mut matched = vec![false; new_children.len()];

        for (_i, old_child) in old_children.iter().enumerate() {
            let mut found = false;
            for (j, new_child) in new_children.iter().enumerate() {
                if !matched[j] && old_child.widget_type == new_child.widget_type {
                    matched[j] = true;
                    diffs.extend(Self::tree_diff(old_child, new_child, Some(old.id)));
                    found = true;
                    break;
                }
            }
            if !found {
                diffs.push(WidgetDiff::Removed { id: old_child.id });
            }
        }

        // Add new children
        for (i, new_child) in new_children.iter().enumerate() {
            if !matched[i] {
                diffs.push(WidgetDiff::Added {
                    id: new_child.id,
                    widget: new_child.clone(),
                    parent_id: Some(new.id),
                });
            }
        }

        // Check for reordering
        let old_order: Vec<_> = old_children.iter().map(|c| c.id).collect();
        let new_order: Vec<_> = new_children.iter().map(|c| c.id).collect();
        if old_order != new_order {
            diffs.push(WidgetDiff::Reordered {
                parent_id: old.id,
                new_order,
            });
        }

        diffs
    }

    /// Queue widget diffs for application
    pub fn queue_diffs(&self, diffs: Vec<WidgetDiff>) {
        self.pending_diffs.lock().unwrap().extend(diffs);
    }

    /// Get and clear pending diffs
    pub fn take_pending_diffs(&self) -> Vec<WidgetDiff> {
        self.pending_diffs.lock().unwrap().drain(..).collect()
    }

    /// Check if hot reload is active
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Update applier for hot reload diffs
pub struct UpdateApplier {
    manager: Arc<HotReloadManager>,
}

impl UpdateApplier {
    pub fn new(manager: Arc<HotReloadManager>) -> Self {
        Self { manager }
    }

    /// Apply pending diffs to the running application
    pub async fn apply_updates(&self) -> anyhow::Result<()> {
        let diffs = self.manager.take_pending_diffs();

        if diffs.is_empty() {
            return Ok(());
        }

        // Apply diffs in order
        for diff in diffs {
            self.apply_diff(diff).await?;
        }

        Ok(())
    }

    async fn apply_diff(&self, diff: WidgetDiff) -> anyhow::Result<()> {
        match diff {
            WidgetDiff::Updated {
                id,
                changed_props,
            } => {
                // Update widget properties (requires integration with rendering engine)
                debug!("Updating widget {:?} with {:?}", id, changed_props);
            }
            WidgetDiff::Added {
                id,
                widget,
                parent_id,
            } => {
                debug!("Adding widget {:?} (parent: {:?})", id, parent_id);
                // Create new widget and add to tree
            }
            WidgetDiff::Removed { id } => {
                debug!("Removing widget {:?}", id);
                // Remove widget from tree
            }
            WidgetDiff::Reordered {
                parent_id,
                new_order,
            } => {
                debug!("Reordering children of {:?}: {:?}", parent_id, new_order);
                // Reorder children
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_snapshot() {
        let snap = StateSnapshot::new();
        assert_eq!(snap.signals.len(), 0);
        assert_eq!(snap.derived_values.len(), 0);
    }

    #[test]
    fn test_widget_diff_simple() {
        let manager = HotReloadManager::new(true);

        let old = WidgetNode {
            id: WidgetId(1),
            widget_type: "div".to_string(),
            props: {
                let mut m = HashMap::new();
                m.insert("color".to_string(), "red".to_string());
                m
            },
            children: vec![],
            state_hash: 0,
        };

        let new = WidgetNode {
            id: WidgetId(1),
            widget_type: "div".to_string(),
            props: {
                let mut m = HashMap::new();
                m.insert("color".to_string(), "blue".to_string());
                m
            },
            children: vec![],
            state_hash: 0,
        };

        manager.set_widget_tree(old);
        let diffs = manager.compute_diff(&new);

        assert!(!diffs.is_empty());
    }
}
