use redis::Value;
use t_redis::Connection;

lazy_static! {
    /// 静态变量，存储 redis 连接实例
    static ref REDIS_CONN: Option<Connection> = {
        let conn_info = get_conn();
        if conn_info.is_ok() {
            return conn_info.unwrap();
        } else {
            // 打印一些错误信息 todo
            return None;
        }
    };
}

/// 获取 redis 连接实例
fn get_conn() -> t_redis::RedisResult<Option<Connection>> {
    let client = t_redis::Client::open("redis://127.0.0.1/")?;
    let conn = client.get_connection()?;
    Ok(Some(conn))
}

/// 获取 redis db
fn get_redis_db(conn: &mut Connection) -> anyhow::Result<Vec<u8>> {
    let mut dbs: Vec<u8> = vec![];
    let res = t_redis::cmd("config").arg("get").arg("databases").query::<Value>(conn)?;
    if let t_redis::Value::Bulk(db_info) = res {
        if db_info.len() == 2 {
            let db_num_val = &db_info[1];
            match db_num_val {
                t_redis::Value::Status(ref s1) =>{
                    let db_num = s1[..].parse::<u8>().unwrap();
                    for i in 0..db_num {
                        dbs.push(i);
                    }
                },
                _=>{}
            }
        }
    }

    return Ok(dbs);
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
    use std::borrow::BorrowMut;

    use super::*;

    #[test]
    fn test_get_redis_db() {
        let conn_res = get_conn();
        let conn_op = conn_res.unwrap();
        conn_op.map(|mut conn| {
            let res = get_redis_db(&mut conn);
            assert!(res.is_ok());
        });
    }
}
