
pub enum ClientError {

}

// pub type Result<T> = std::result::Result<T, ClientError>;
pub type Result<T> = std::result::Result<T, anyhow::Error>;
