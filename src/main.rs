extern crate redis as t_redis;
#[macro_use]
extern crate lazy_static;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use t_redis::Commands;

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
                        let res = parser.map_cmd();
                        println!("input param: {:?}", res);
                    }
                },
                Err(_) => {
                    println!("Unsupport input. You can type 'quit' to exit. ");
                }
            }
        }
    }
}
