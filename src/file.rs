use colored::*;

use std::io::{BufReader, BufRead, Write, Read};
use std::fs::{File, metadata};

pub fn read_file(filename: String) -> Vec<String> {
    let file = match File::open(filename.clone()) {
        Ok(fl) => fl,
        Err(why) => {
            eprintln!("{}: couldn't open {} cause {}", "ERROR".bright_red(), filename.italic().bold(), why);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        data.push(line);
    }
    data
}

pub fn read_file_bin(filename: String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");
    buffer
}

pub fn write_file_bin(filename: String, data: Vec<u8>) {
    let mut file = match File::create(filename.clone()) {
        Ok(f) => f,
        Err(why) => {
            eprintln!("{}: couldn't create file {} cause {}", "ERROR".bright_red(), filename.italic().bold(), why);
            std::process::exit(1);
        }
    };
    file.write_all(&data);
}
