use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found.")]
    NotFound,
    #[error("Unauthorized user.")]
    Unauthorized,
    #[error("An unexpected error has occurred.")]
    Unexpected,
}
