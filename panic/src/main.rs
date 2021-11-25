use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::fs;

//    let f = File::open("hello.txt").expect("Failed to open hello.txt");
//    let f = File::open("hello.txt").unwrap();
fn main() {
    let _f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            }) 
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    let result = read_username_from_file();
    println!("{:?}",result);
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
