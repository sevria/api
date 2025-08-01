#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Already present")]
    AlreadyPresent,
    #[error("Internal error")]
    Internal,
    #[error("{0}")]
    InvalidArgument(String),
    #[error("Not found")]
    NotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Unauthenticated")]
    Unauthenticated,
}
