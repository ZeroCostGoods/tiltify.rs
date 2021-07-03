mod campaign;
pub mod client;
mod paginator;
mod user;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;
