use std::{fs::File, io::Read};

pub fn load_input(name: &str) -> String {
    let mut s: String = String::new();
    File::open(format!("inputs/{name}.txt")).unwrap().read_to_string(&mut s).unwrap();
    s
}