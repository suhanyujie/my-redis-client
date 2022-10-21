extern crate redis as t_redis;
#[macro_use]
extern crate lazy_static;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use t_redis::Commands;

mod cmd;
mod error;
mod redis;

fn main() {
    let rl_res = Editor::<()>::new();
    if let Ok(mut rl) = rl_res {
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => match &line[..] {
                    "quit" => break,
                    "redis" => {
                        redis::get_conn_ins().lock().unwrap().as_mut().map(|conn| {
                            let res = redis::get_redis_db(conn);
                            dbg!(&res);
                            assert!(res.is_ok());
                        });
                    }
                    _ => {
                        let input_str = line[..].trim();
                        let parser = redis::RedisCmdParser::new(input_str);
                        if let Ok(cmd) = parser.map_cmd() {
                            match cmd.cmd_type {
                                cmd::CmdType::Get(key) => {
                                    // todo
                                    println!("get param: {}", key);
                                }
                                cmd::CmdType::Set(key, value) => {
                                    // todo
                                    println!("set param: {}-{}", key, value);
                                }
                                _ => {
                                    eprintln!("Unknown cmd...");
                                }
                            }
                        } else {
                            eprintln!("Unknown cmd...");
                        }
                    }
                },
                Err(_) => {
                    println!("Unsupport input. You can type 'quit' to exit. ");
                }
            }
        }
    }
}
