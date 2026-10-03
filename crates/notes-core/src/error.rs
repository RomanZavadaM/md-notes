use std::path::{Path, PathBuf};

/// Errors returned by `notes-core`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid path: {0}")]
    InvalidPath(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("already exists: {0}")]
    AlreadyExists(String),
    #[error("invalid vault config: {0}")]
    Config(String),
    #[error("invalid sync state: {0}")]
    Sync(String),
    #[error("Git sync error: {0}")]
    Git(String),
    #[error("index error: {0}")]
    Index(#[from] rusqlite::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Builds a `map_err` adapter that attaches the path to an I/O error.
pub(crate) fn io_err(path: &Path) -> impl FnOnce(std::io::Error) -> Error {
    let path = path.to_path_buf();
    move |source| Error::Io { path, source }
}

// Errors cross the Tauri bridge as plain messages.
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
