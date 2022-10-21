//! redis command

use anyhow::Ok;

use crate::error;
use crate::redis;

pub enum CmdType {
    Get(String),
    Set(String, String),
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
                let res = redis::set(conn, key, value, 10);
                assert!(res.is_ok());
                Ok(t_redis::Value::Nil)
            })
    }
}
