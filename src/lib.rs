pub mod campaign;
pub mod client;
mod paginator;
pub mod user;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;
