use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buffer = Vec::new();
    let file_name = args().nth(1).expect("no input");

    File::open(&file_name)
        .unwrap()
        .read_to_end(&mut buffer)
        .expect("error while reading");

    println!("File loaded: {}", file_name);
}
