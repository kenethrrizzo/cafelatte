use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserErrors {
    #[error("unknown error")]
    Unknown,
}
