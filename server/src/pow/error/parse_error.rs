use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Message is malformed")]
    MessageMalformed,

    #[error("Resource is invalid")]
    ResourceInvalid,
}
