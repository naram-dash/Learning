use rand::Rng;
use std::collections::HashMap;
use std::collections::*;
// use std::{cmp::Ordering, io};
use std::io::{self, Write};

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1, 101);
}
