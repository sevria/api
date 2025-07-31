#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("already present")]
    AlreadyPresent,
    #[error("internal error")]
    Internal,
    #[error("{0}")]
    InvalidArgument(String),
    #[error("not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("unauthenticated")]
    Unauthenticated,
}
