/// crate 级别的错误类型
pub enum ClientError {

}

// pub type Result<T> = std::result::Result<T, ClientError>;

/// 基于 anyhow::Error 错误类型的 Result 类型别名
pub type Result<T> = std::result::Result<T, anyhow::Error>;
