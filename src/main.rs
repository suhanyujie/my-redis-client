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
                    "quit" | "exit" => break,
                    "redis" => {
                        redis::get_conn_ins().lock().unwrap().as_mut().map(|conn| {
                            let res = redis::get_redis_db(conn);
                            dbg!(&res);
                            assert!(res.is_ok());
                        });
                    }
                    _ => {
                        let input_str = line[..].trim();
                        if input_str.len() < 1 {
                            continue;
                        }
                        let parser = redis::RedisCmdParser::new(input_str);
                        if let Ok(cmd) = parser.map_cmd() {
                            if let Ok(res_value) = cmd.apply() {
                                match res_value {
                                    t_redis::Value::Nil => {
                                        println!("[Nil] Nil");
                                    }
                                    t_redis::Value::Int(v) => {
                                        println!("[Int] {}", v);
                                    }
                                    t_redis::Value::Data(v) => {
                                        println!("[Data] {}", String::from_utf8_lossy(&v));
                                    }
                                    t_redis::Value::Bulk(v) => {
                                        println!("[Bulk] {:?}", v);
                                    }
                                    t_redis::Value::Status(v) => {
                                        println!("[Status] {}", v);
                                    }
                                    t_redis::Value::Okay => {
                                        println!("[Okay] Okay");
                                    }
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
