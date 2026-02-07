//! Junita Embedding SDK
//!
//! Integrate Junita UI into Rust applications.

#[cfg(feature = "junita_core")]
pub use junita_core;

#[cfg(feature = "junita_animation")]
pub use junita_animation;

#[cfg(feature = "junita_layout")]
pub use junita_layout;

#[cfg(feature = "junita_gpu")]
pub use junita_gpu;

#[cfg(feature = "junita_paint")]
pub use junita_paint;

// #[cfg(feature = "junita_cn")]
// pub use junita_cn;

/// Initialize the Junita runtime
pub fn init() -> anyhow::Result<()> {
    // TODO: Initialize Zyntax runtime with Junita grammar
    Ok(())
}
