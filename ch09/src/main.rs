use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
};

fn main() {
    let mut file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    file.write("hello".as_bytes()).unwrap();
}
