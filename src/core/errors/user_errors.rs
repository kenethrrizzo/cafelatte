use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user not found")]
    NotFound,
    #[error("unexpected error")]
    Unexpected,
}
