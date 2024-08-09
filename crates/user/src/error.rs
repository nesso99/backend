use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("unknown error")]
    Unknown,
}
