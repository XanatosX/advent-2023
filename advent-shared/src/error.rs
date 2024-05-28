use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error
{
    #[error("Generic Error: {0}")]
    Generic(String),
}