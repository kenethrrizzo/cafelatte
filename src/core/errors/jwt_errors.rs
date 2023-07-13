use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Bearer not present.")]
    BearerNotPresent,
    #[error("An unexpected error has occurred")]
    Unexpected,
}
