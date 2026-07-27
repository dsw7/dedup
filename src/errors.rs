use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DedupError {
    #[error("An I/O error occurred: {source}")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("{0}")]
    FilesNotFound(String),
}
