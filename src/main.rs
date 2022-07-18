#![allow(non_snake_case)]

mod cli;
use cli::CliSet;

mod cpu;
use cpu::Cpu;

mod mem;
use mem::Mem;

mod file;
use file::read_file_bin;

mod screen;
use screen::Screen;

mod keyboard;
use keyboard::Keyboard;

use colored::*;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

fn main() {
    let cli_settings = CliSet::get_cli_settings();

    let mut term = match Termios::from_fd(0) {
        Ok(t) => t,
        Err(e) => in_error(format!(
            "failed to get term settings of {} because {}",
            "STDIN".italic(),
            e
        )),
    };

    term.c_lflag &= !(ECHO | ICANON);
    match tcsetattr(0, TCSANOW, &term) {
        Ok(_) => (),
        Err(e) => in_error(format!(
            "failed to setup screen for emulating ZCS-8 beacuse {}",
            e
        )),
    }

    let mut rom = Mem::new(usize::pow(2, 15));
    // TODO: make a normal address decoder
    let mut ram = Mem::new(usize::pow(2, 16));
    let mut cpu = Cpu::new();
    let mut screen = Screen::new();
    let mut keyboard = Keyboard::new();

    rom.load(read_file_bin(cli_settings.get_rom_fname()));

    cpu.reset();
    
    loop {
        let hlt = cpu.execute(&rom, &mut ram, cli_settings.get_manuality());
        if cli_settings.get_manuality() {
            cpu.dump();
            ram.dump();
            let mut input = String::new();
            let _string = std::io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("Failed to read line");
            let bytes = input.bytes().nth(0).expect("no byte read");
            if bytes == 113 || bytes == 81 {
                break;
            }
        }
        keyboard.update(&mut ram);
        screen.update(&ram);
        if hlt {
            break;
        }
    }
    // if !manual { cpu.dump(); ram.dump(); }
    // print!("\x1B[2J");
    // std::io::stdout().flush().unwrap();
    // print!("\x1B[H");
    // std::io::stdout().flush().unwrap();
    // for ad in 0x80FF..0x81FF {
    //     print!("{:>0w$X} ", ram.read(ad), w=2);
    // }

    term.c_lflag |= ECHO | ICANON;
    tcsetattr(0, TCSANOW, &term).unwrap();

    println!();
}

pub fn in_error<T>(err: T) -> !
where
    T: std::fmt::Display,
{
    eprintln!("{}: {}", "ERROR".bright_red(), err);
    std::process::exit(1);
}
