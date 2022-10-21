use redis::{Commands, Value};
use std::sync::Mutex;
use t_redis::Connection;

use crate::cmd;

lazy_static! {
    /// 静态变量，存储 redis 连接实例
    pub static ref REDIS_CONN: Mutex<Option<Connection>> = {
        let conn_info = get_conn();
        if conn_info.is_ok() {
            return Mutex::new(conn_info.unwrap());
        } else {
            // 打印一些错误信息 todo
            return Mutex::new(None);
        }
    };
}

pub struct RedisConfig {
    /// 连接地址
    address: String,
    /// 端口
    port: String,
    /// 密码
    password: String,
    /// 数据库
    db: String,
}

impl RedisConfig {
    fn new() -> Self {
        RedisConfig {
            address: "".to_string(),
            port: "".to_string(),
            password: "".to_string(),
            db: "".to_string(),
        }
    }

    fn set_address(&mut self, host: &str) {
        self.address = host.to_string();
    }

    fn set_port(&mut self, param: String) {
        self.port = param;
    }

    fn set_db(&mut self, param: String) {
        self.db = param;
    }

    fn set_password(&mut self, param: &str) {
        self.password = param.to_string();
    }
}

/// 使用 redis 连接执行一些命令
fn with_connection<F, T>(func: F) -> anyhow::Result<T>
where
    F: FnOnce(&mut Connection) -> anyhow::Result<T>,
{
    let client = t_redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_connection()?;
    return func(&mut conn);
}

/// 获取 redis 连接实例
fn get_conn() -> t_redis::RedisResult<Option<Connection>> {
    let client = t_redis::Client::open("redis://127.0.0.1/")?;
    let conn = client.get_connection()?;
    Ok(Some(conn))
}

pub fn get_conn_ins<'a>() -> &'a Mutex<Option<Connection>> {
    &REDIS_CONN
}

/// 获取 redis db
pub fn get_redis_db(conn: &mut Connection) -> anyhow::Result<Vec<u8>> {
    let mut dbs: Vec<u8> = vec![];
    let res = t_redis::cmd("config")
        .arg("get")
        .arg("databases")
        .query::<Value>(conn)?;
    if let t_redis::Value::Bulk(db_info) = res {
        if db_info.len() == 2 {
            let db_num_val = &db_info[1];
            match db_num_val {
                t_redis::Value::Data(ref s1) => match std::str::from_utf8(s1) {
                    Ok(str1) => {
                        let db_num = str1.parse::<u8>().unwrap();
                        for i in 0..db_num {
                            dbs.push(i);
                        }
                    }
                    Err(_) => {}
                },
                _ => {}
            }
        }
    }

    return Ok(dbs);
}

/// 选择使用的 redis 数据库
fn use_db(conn: &mut Connection, db_num: u8) -> anyhow::Result<()> {
    let mut dbs: Vec<u8> = vec![];
    t_redis::cmd("select").arg(db_num).query::<Value>(conn)?;
    return Ok(());
}

fn show_help() {
    todo!()
}

/// 设定键值对
pub fn set(conn: &mut Connection, k: &str, v: &str, expr_s: usize) -> anyhow::Result<()> {
    if expr_s > 0 {
        conn.set_ex(k, v, expr_s)?;
    } else {
        conn.set(k, v)?;
    }
    return Ok(());
}

/// 获取值
pub fn get(conn: &mut Connection, k: &str) -> anyhow::Result<t_redis::Value> {
    let res = conn.get(k)?;
    return Ok(res);
}

/// 获取 redis 客户端状态
fn get_redis_status() {
    todo!()
}

/// 启动客户端。主要是建立连接
fn serve() {
    todo!()
}

/// 用户输入 redis 的参数解析
pub struct RedisCmdParser<'a> {
    raw: &'a str,
    param_arr: Vec<String>,
}

impl<'a> RedisCmdParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let param_arr = input
            .split_whitespace()
            .filter(|s| s.len() > 0)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        RedisCmdParser {
            raw: input,
            param_arr: param_arr,
        }
    }

    /// 通过输入的 cmd，执行对应的函数
    pub fn map_cmd(&self) -> anyhow::Result<cmd::Command> {
        cmd::Command::new(&self.param_arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_redis_db() {
        let conn_res = get_conn();
        let conn_op = conn_res.unwrap();
        conn_op.map(|mut conn| {
            let res = get_redis_db(&mut conn);
            dbg!(&res);
            assert!(res.is_ok());
        });
    }

    #[test]
    fn test_get_redis_db_1() {
        // let mut conn_guard = REDIS_CONN.lock().unwrap();
        // conn_guard.as_mut().map(|conn| {
        //     let res = get_redis_db(conn);
        //     dbg!(&res);
        //     assert!(res.is_ok());
        // });

        get_conn_ins().lock().unwrap().as_mut().map(|conn| {
            let res = get_redis_db(conn);
            dbg!(&res);
            assert!(res.is_ok());
        });
    }

    #[test]
    fn test_use_db() {
        let conn_res = get_conn();
        let conn_op = conn_res.unwrap();
        conn_op.map(|mut conn| {
            let res = use_db(&mut conn, 1);
            dbg!(&res);
            assert!(res.is_ok());
        });
    }

    #[test]
    fn test_set() {
        let conn_res = get_conn();
        let conn_op = conn_res.unwrap();
        conn_op.map(|mut conn| {
            let res = set(&mut conn, "test1", "su-val1", 15);
            assert!(res.is_ok());
        });
    }

    #[test]
    fn test_get() {
        let conn_res = get_conn();
        let conn_op = conn_res.unwrap();
        conn_op.map(|mut conn| {
            let res = get(&mut conn, "test1");
            dbg!(&res);
            assert!(res.is_ok());
        });
    }
}
