use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashStorageError {
    #[error("Resource not found")]
    ResourceNotFound,
}
