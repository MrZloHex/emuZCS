use colored::*;

use super::in_error;

use std::fs::{metadata, File};
use std::io::{BufRead, BufReader, Read, Write};

#[allow(dead_code)]
pub fn read_file(filename: String) -> Vec<String> {
    let file = match File::open(filename.clone()) {
        Ok(fl) => fl,
        Err(why) => {
            in_error(format!(
                "couldn't open {} cause {}",
                filename.italic().bold(),
                why
            ));
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
    let mut f = match File::open(&filename) {
        Ok(f)  => f,
        Err(e) => in_error(format!(
            "failed to open file {} because {}",
            filename.bold().italic(),
            e
        ))
    };

    let metadata = match metadata(&filename) {
        Ok(m)  => m,
        Err(e) => in_error(format!(
            "failed to read metadata on file {} beacuase {}" ,
            filename.bold().italic(),
            e
        ))
    };
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    if let Err(e) = f.read_exact(&mut buffer) {
        in_error(format!(
            "failed to read file {}, probably buffer overflow -> `{}`",
            filename,
            e
        ));
    }
    buffer
}

#[allow(dead_code)]
pub fn write_file_bin(filename: String, data: Vec<u8>) {
    let mut file = match File::create(filename.clone()) {
        Ok(f) => f,
        Err(why) => {
            in_error(format!(
                "couldn't create file {} cause {}",
                filename.italic().bold(),
                why
            ));
        }
    };
    if let Err(e) = file.write_all(&data) {
        in_error(format!(
            "failed to write data to file {} because {}",
            filename.italic().bold(),
            e
        ));
    }
}
