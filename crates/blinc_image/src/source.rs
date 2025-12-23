//! Image source types

use std::path::PathBuf;

/// Source of an image
#[derive(Debug, Clone)]
pub enum ImageSource {
    /// Load from a file path
    File(PathBuf),

    /// Load from a URL (requires "network" feature)
    Url(String),

    /// Load from base64-encoded data
    /// Can optionally include data URI prefix (e.g., "data:image/png;base64,...")
    Base64(String),

    /// Load from raw bytes with format hint
    Bytes {
        data: Vec<u8>,
        format: Option<ImageFormat>,
    },
}

impl ImageSource {
    /// Create a file source
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Self::File(path.into())
    }

    /// Create a URL source
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// Create a base64 source
    pub fn base64(data: impl Into<String>) -> Self {
        Self::Base64(data.into())
    }

    /// Create a bytes source
    pub fn bytes(data: Vec<u8>) -> Self {
        Self::Bytes { data, format: None }
    }

    /// Create a bytes source with format hint
    pub fn bytes_with_format(data: Vec<u8>, format: ImageFormat) -> Self {
        Self::Bytes {
            data,
            format: Some(format),
        }
    }

    /// Parse a resource URI string into an ImageSource
    ///
    /// Supported formats:
    /// - `file:///path/to/image.png` - File path
    /// - `http://...` or `https://...` - URL
    /// - `data:image/png;base64,...` - Data URI with base64
    /// - `/path/to/image.png` - Treated as file path
    pub fn from_uri(uri: &str) -> Self {
        if uri.starts_with("data:") {
            // Data URI
            Self::Base64(uri.to_string())
        } else if uri.starts_with("http://") || uri.starts_with("https://") {
            // URL
            Self::Url(uri.to_string())
        } else if uri.starts_with("file://") {
            // File URI
            let path = uri.strip_prefix("file://").unwrap_or(uri);
            Self::File(PathBuf::from(path))
        } else {
            // Assume file path
            Self::File(PathBuf::from(uri))
        }
    }
}

impl From<&str> for ImageSource {
    fn from(s: &str) -> Self {
        Self::from_uri(s)
    }
}

impl From<String> for ImageSource {
    fn from(s: String) -> Self {
        Self::from_uri(&s)
    }
}

impl From<PathBuf> for ImageSource {
    fn from(path: PathBuf) -> Self {
        Self::File(path)
    }
}

impl From<&std::path::Path> for ImageSource {
    fn from(path: &std::path::Path) -> Self {
        Self::File(path.to_path_buf())
    }
}

/// Image format hint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
    Bmp,
}

impl ImageFormat {
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "gif" => Some(Self::Gif),
            "webp" => Some(Self::WebP),
            "bmp" => Some(Self::Bmp),
            _ => None,
        }
    }

    /// Detect format from MIME type
    pub fn from_mime(mime: &str) -> Option<Self> {
        match mime {
            "image/png" => Some(Self::Png),
            "image/jpeg" | "image/jpg" => Some(Self::Jpeg),
            "image/gif" => Some(Self::Gif),
            "image/webp" => Some(Self::WebP),
            "image/bmp" => Some(Self::Bmp),
            _ => None,
        }
    }
}
