//! redis command

use anyhow::Ok;

use crate::error;
use crate::redis;

pub enum CmdType {
    Get(String),
    Set(String, String),
    // scan 0 MATCH *key1* COUNT 10000
    Scan(i32, String, u32),
    HGet(String, Vec<String>),
    HGetAll(String),
    HSet(String),
}

pub struct Command {
    pub cmd_type: CmdType,
}

impl Command {
    pub fn new(param_arr: &Vec<String>) -> anyhow::Result<Self> {
        if param_arr.len() < 2 {
            return Err(error::ClientError::InputParamInvalid.into());
        }
        let cmd_type = match &param_arr[0][..] {
            "get" => CmdType::Get(param_arr[1].to_string()),
            "scan" => {
                let param_len = param_arr.len();
                match param_len {
                    4 => {
                        // scan 0 match key1
                        let cursor = param_arr[1].parse::<i32>()?;
                        let count = 10000; // 默认查询 1w 条
                        CmdType::Scan(cursor, param_arr[3].to_string(), count)
                    }
                    6 => {
                        // scan 0 match key1 count 1000
                        let cursor = param_arr[1].parse::<i32>()?;
                        let count = param_arr[5].parse::<u32>()?;
                        CmdType::Scan(cursor, param_arr[3].to_string(), count)
                    }
                    _ => {
                        return Err(error::ClientError::InputParamInvalid.into());
                    }
                }
            }
            "set" => {
                if param_arr.len() != 3 {
                    return Err(error::ClientError::InputParamInvalid.into());
                }
                CmdType::Set(param_arr[1].to_string(), param_arr[2].to_string())
            }
            _ => {
                eprintln!("Unsupport cmd...");
                return Err(error::ClientError::UnsupportErr.into());
            }
        };
        Ok(Command { cmd_type })
    }

    pub fn apply(&self) -> anyhow::Result<t_redis::Value> {
        let res = match &self.cmd_type {
            CmdType::Get(key) => self.get(key)?,
            CmdType::Set(key, value) => self.set(key, value)?,
            CmdType::Scan(cursor, match_str, count) => self.scan(*cursor, match_str, *count)?,
            _ => {
                eprintln!("Unknown cmd...");
                t_redis::Value::Nil
            }
        };
        Ok(res)
    }

    fn get(&self, key: &str) -> anyhow::Result<t_redis::Value> {
        redis::get_conn_ins().lock().unwrap().as_mut().map_or(
            Ok(t_redis::Value::Nil),
            |conn| -> anyhow::Result<t_redis::Value> { redis::get(conn, key) },
        )
    }

    fn set(&self, key: &str, value: &str) -> anyhow::Result<t_redis::Value> {
        redis::get_conn_ins()
            .lock()
            .unwrap()
            .as_mut()
            .map_or(Ok(t_redis::Value::Nil), |conn| {
                let res = redis::set(conn, key, value, 3600 * 24);
                assert!(res.is_ok());
                Ok(t_redis::Value::Nil)
            })
    }

    fn scan(&self, cursor: i32, match_str: &str, count: u32) -> anyhow::Result<t_redis::Value> {
        redis::get_conn_ins().lock().unwrap().as_mut().map_or(
            Ok(t_redis::Value::Nil),
            |conn| -> anyhow::Result<t_redis::Value> {
                redis::scan(conn, cursor, match_str, count);
                return Ok(t_redis::Value::Nil);
            },
        )
    }
}
