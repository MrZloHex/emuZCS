use std::io::{self, Write};

use crate::mem::Mem;

pub struct Screen {
    screen: [[char; 40]; 25],
    x_pos: u8,
    y_pos: u8,
    ch: char
}

impl Screen {
    pub fn new() -> Screen {
        print!("\x1B[2J");
        io::stdout().flush().unwrap();
        print!("\x1B[H");
        io::stdout().flush().unwrap();
        let scr = Screen {
            screen: [[' '; 40]; 25],
            x_pos: 0,
            y_pos: 0,
            ch: ' '
        };
        scr.print();
        print!("\x1B[H");
        io::stdout().flush().unwrap();
        
        scr
        
    }

    fn put(&mut self) {
        self.screen[self.x_pos as usize][self.y_pos as usize] = self.ch;
        print!("\x1B[{};{}H{}", self.y_pos+1, self.x_pos+1, self.ch);
        io::stdout().flush().unwrap();
    }

    fn print(&self) {
        for line in self.screen {
            for ch in line {
                print!("{}", ch);
            }
            println!();
        }
    }

    pub fn update(&mut self, ram: &Mem) {
        self.x_pos = ram.read(0xF002);
        self.y_pos = ram.read(0xF003);
        let ch = ram.read(0xF004);
        self.ch = (ch & 0b0111_1111) as char;
        if (ch & 0b1000_0000) == 0b1000_0000 {
            self.put();
        }
    }
}
