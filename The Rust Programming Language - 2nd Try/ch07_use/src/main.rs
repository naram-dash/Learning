use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut map = HashMap::new();

    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    todo!()
}

fn function2() -> io::Result<()> {
    todo!()
}


use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    todo!()
}

fn function4() -> IoResult<()> {
    todo!()
}