/// 获取 redis db
fn get_redis_db() -> crate::error::Result<Vec<u8>> {
    let mut dbs: Vec<u8> = vec![];
    Ok(dbs)
}

/// 选择使用的 redis 数据库
fn use_db() {
    todo!()
}

fn show_help() {
    todo!()
}

/// 设定键值对
fn set() {
    todo!()
}

/// 获取值
fn get() {
    todo!()
}

/// 获取 redis 客户端状态
fn get_redis_status() {
    todo!()
}

/// 启动客户端。主要是建立连接
fn serve() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_redis_db() {
        let res = get_redis_db();
        assert!(res.is_ok());
    }
}
