use std::io::Read;
use std::fs::File;
use std::env;

fn get_filename_from_args() -> String {
    let mut args = env::args();
    if let Some(_) = args.next() {
        if let Some(file) = args.next(){
            return file;
        } 
    }
    String::from("src/STM32F103xx.svd")
}

pub fn from_args() -> String {
    let file_name = get_filename_from_args();
    println!("Reading file {:?}", file_name);
    let mut handle = File::open(&file_name).unwrap();
    let mut str = String::new();
    handle.read_to_string(&mut str).unwrap();
    if str.chars().next() == Some('\u{feff}') {
        println!("Removing first character {:?}", str.remove(0));
    }
    str
}
