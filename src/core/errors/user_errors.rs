use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user_not_found_error")]
    NotFound,
    #[error("unexpected_error")]
    Unexpected,
}
