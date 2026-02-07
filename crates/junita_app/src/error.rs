//! Error types for junita_app

use thiserror::Error;

/// Errors that can occur in the Junita application
#[derive(Error, Debug)]
pub enum JunitaError {
    /// Failed to initialize GPU
    #[error("GPU initialization failed: {0}")]
    GpuInit(String),

    /// Failed to create renderer
    #[error("Renderer creation failed: {0}")]
    RendererCreate(String),

    /// Failed to render
    #[error("Rendering failed: {0}")]
    Render(String),

    /// Failed to load font
    #[error("Font loading failed: {0}")]
    FontLoad(String),

    /// Failed to parse SVG
    #[error("SVG parsing failed: {0}")]
    SvgParse(String),

    /// Platform error (windowing, input, etc.)
    #[error("Platform error: {0}")]
    Platform(String),

    /// Platform unsupported (running on wrong OS)
    #[error("Platform unsupported: {0}")]
    PlatformUnsupported(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for JunitaError {
    fn from(err: anyhow::Error) -> Self {
        JunitaError::Other(err.to_string())
    }
}

/// Result type for junita_app operations
pub type Result<T> = std::result::Result<T, JunitaError>;
