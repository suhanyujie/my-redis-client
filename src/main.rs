extern crate redis as t_redis;

use t_redis::Commands;

mod redis;
mod error;

fn main() {
    let res = hello_redis();
    println!("res: {:?}", res);
}

fn hello_redis() -> t_redis::RedisResult<isize> {
    // connect to redis
    let client = t_redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}


