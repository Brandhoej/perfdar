use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Encountered a runtime error: {message:}")]
    RuntimeError { message: String },
    #[error("Encountered a type checking error: {message:}")]
    TypeCheckingError { message: String },
}
