//! redis command

use crate::error;

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
            "set" => CmdType::Set(param_arr[1].to_string(), param_arr[2].to_string()),
            _ => {
                eprintln!("Unsupport cmd...");
                return Err(error::ClientError::UnsupportErr.into());
            }
        };
        Ok(Command { cmd_type })
    }
}
