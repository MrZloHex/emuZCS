use std::collections::VecDeque;

use std::io::Read;
use termion::input::TermRead;
use termion::{async_stdin, AsyncReader, event::Key};

use crate::mem::Mem;

const NEXT_BYTE: u8 = 0b0000_0001;
const ENABLE:    u8 = 0b0000_0010;
const CLEAR:     u8 = 0b0000_0100;

pub struct Keyboard {
    buf: VecDeque<char>,
    char_now: char,
    astdin: AsyncReader
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            buf: VecDeque::new(),
            char_now: 0 as char,
            astdin: async_stdin()
        }
    }

    pub fn update(&mut self, ram: &mut Mem) {
        let ctrl = ram.read(0xF000);
        if ctrl & ENABLE == ENABLE {
            for key in self.astdin.by_ref().keys() {
                // println!("You have hit: {:?}", key.as_ref().unwrap());
                match key.unwrap() {
                    Key::Char(c) => self.buf.push_back(c),
                    Key::Ctrl(c) => {
                        if c == 'h' {
                            self.buf.push_back(8 as char);
                        }
                    },
                    _ => panic!("WRITE KERNEL IN HASKELL {:?}", "WEQ")
                }
            }
            if ctrl & CLEAR == CLEAR {
                self.buf.clear();
            }
            if ctrl & NEXT_BYTE == NEXT_BYTE {
                if let Some(ch) = self.buf.pop_front() {
                    self.char_now = ch;
                    ram.write(0xF001, ch as u8 | 0b1000_0000);

                } else {
                    self.char_now = 0 as char;
                    ram.write(0xF001, 0);
                }
            }
        }
    }
}
