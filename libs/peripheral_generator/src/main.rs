#![feature(conservative_impl_trait)]
#![cfg_attr(test, feature(test))]
#![allow(dead_code)]

#[macro_use] extern crate lazy_static;
extern crate svd_parser;
extern crate itertools;
extern crate xmltree;
extern crate utils;
extern crate regex;

mod peripheral_utils;
mod generator;
mod read_file;

use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use generator::*;

fn main() {
    let file = read_file::from_args();
    println!("Parsing XML");
    let device = {
        let mut device = svd_parser::parse(&file);
        peripheral_utils::resolve_derives(&mut device);
        device
    };

    println!("Setting up generators");
    let mut generator_output = GeneratorOutput::default();

    generator_output.add(String::from("lib.rs"), LibGenerator::new(device.clone()));

    for peripheral in &device.peripherals {
        let name = peripheral_utils::get_name(&device, peripheral);
        generator_output.add(name, get_generator_for_peripheral(peripheral));
    }

    println!("Running generators");
    let output = generator_output.generate();

    println!("Writing files");
    if Path::new("peripheral").exists() {
        fs::remove_dir_all("peripheral").expect("Code to be able to delete dir");
    }
    
    for (filename, content) in output {
        let path = Path::new("peripheral").join(&filename);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Path to be created");
        }
        let mut handle = File::create(path).expect("File to be created");
        handle.write_all(content.as_bytes()).expect("File to be written");
    }
}
