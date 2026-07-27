use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DedupError {
    #[error("Disk read failed: {source}")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("The requested configuration item '{0}' was not found.")]
    FilesNotFound(String),
}
