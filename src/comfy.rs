// standard library
mod collections;
mod env;
mod fs;
mod http;
mod io;
mod json;
mod math;
mod thread;
mod time;

use lazy_static::lazy_static;

use hashbrown::HashMap;

lazy_static! {
    static ref HTTP_FUNCTIONS: HashMap<&'static str, &'static str> = {
        let mut hash = HashMap::new();
        hash.insert("test", "test");
        hash
    };
}

pub fn print(expr: &str) {
    println!("{expr}")
}