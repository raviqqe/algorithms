use std::io::{stdin, Read};

pub fn run() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    s.split('\n');
}
