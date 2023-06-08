use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    // let mut file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    // file.write("hello".as_bytes()).unwrap();
    let username = read_username_from_file()?;
    println!("{username}");
    Ok(())
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
