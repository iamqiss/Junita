//! Compile-time unique component identifiers
//!
//! Provides macros and utilities for generating unique component IDs
//! that can be used with stateful systems without depending on blinc_app.
//!
//! # Usage
//!
//! ```ignore
//! use blinc_cn::component_id;
//!
//! // Generate a unique ID for this call site
//! let id = component_id!();
//!
//! // Generate a unique ID with a suffix
//! let id = component_id!("my_component");
//! ```
//!
//! # Function Scope Awareness
//!
//! The macros include `module_path!()` which captures the full module path
//! including function names when called inside functions, ensuring uniqueness
//! across different scopes.
//!
//! ```ignore
//! mod foo {
//!     fn render_a() {
//!         let id = component_id!(); // "crate::foo::render_a:file.rs:10:5"
//!     }
//!     fn render_b() {
//!         let id = component_id!(); // "crate::foo::render_b:file.rs:14:5"
//!     }
//! }
//! ```

/// Generate a unique compile-time component ID based on module path, file, line, and column
///
/// This macro creates a unique string key that is stable across rebuilds
/// as long as the source location doesn't change. It includes the full module path
/// (including function names when called inside functions) for additional uniqueness.
///
/// # Example
///
/// ```ignore
/// use blinc_cn::component_id;
///
/// // Basic usage - ID based on call site with module path
/// let id = component_id!();
///
/// // With suffix for multiple IDs at same location
/// let id1 = component_id!("button_1");
/// let id2 = component_id!("button_2");
/// ```
#[macro_export]
macro_rules! component_id {
    () => {
        concat!(module_path!(), ":", file!(), ":", line!(), ":", column!())
    };
    ($suffix:expr) => {
        concat!(module_path!(), ":", file!(), ":", line!(), ":", column!(), ":", $suffix)
    };
}

/// Generate a unique component ID from a type name
///
/// Uses `module_path!()` and `stringify!()` to create a unique key
/// based on the fully qualified type name.
///
/// # Example
///
/// ```ignore
/// use blinc_cn::component_type_id;
///
/// struct MyButton;
///
/// // Generates: "my_module::MyButton"
/// let id = component_type_id!(MyButton);
/// ```
#[macro_export]
macro_rules! component_type_id {
    ($type:ty) => {
        concat!(module_path!(), "::", stringify!($type))
    };
}

/// A trait for components that provide their own unique ID
///
/// Implement this for components that need stable identity across rebuilds.
pub trait ComponentId {
    /// The unique identifier for this component type
    const ID: &'static str;
}

/// Macro to implement ComponentId for a type
///
/// # Example
///
/// ```ignore
/// use blinc_cn::{impl_component_id, ComponentId};
///
/// struct MyCard;
/// impl_component_id!(MyCard);
///
/// // Now MyCard::ID is available
/// assert!(MyCard::ID.contains("MyCard"));
/// ```
#[macro_export]
macro_rules! impl_component_id {
    ($type:ty) => {
        impl $crate::component_id::ComponentId for $type {
            const ID: &'static str = concat!(module_path!(), "::", stringify!($type));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_id_macro() {
        let id1 = component_id!();
        let id2 = component_id!();

        // Same line = same ID
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_component_id_with_suffix() {
        let id1 = component_id!("a");
        let id2 = component_id!("b");

        // Different suffixes = different IDs
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_component_type_id() {
        struct TestComponent;
        let id = component_type_id!(TestComponent);

        assert!(id.contains("TestComponent"));
    }

    struct TestWidget;
    impl_component_id!(TestWidget);

    #[test]
    fn test_impl_component_id() {
        assert!(TestWidget::ID.contains("TestWidget"));
    }
}
