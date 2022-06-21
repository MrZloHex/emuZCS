use clap::{load_yaml, App};

mod cpu;
mod mem;
mod file;
use file::read_file_bin;

mod screen;
use screen::Screen;

mod keyboard;
use keyboard::Keyboard;

use termios::{tcsetattr, Termios, TCSANOW, ECHO, ICANON, ISIG};


fn main() {
    let mut term = Termios::from_fd(0).unwrap();
    term.c_lflag &= !(ECHO | ICANON);
    tcsetattr(0, TCSANOW, &term).unwrap();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let input_fname = matches.value_of("rom").unwrap().to_string();
        let verbosity = matches.is_present("verbose");
        let manual = matches.is_present("manual");

        let mut rom = mem::Mem::new(usize::pow(2, 15));
        // TODO: make a normal address decoder
        let mut ram = mem::Mem::new(usize::pow(2, 16));
        let mut cpu = cpu::Cpu::new();

        let mut screen = Screen::new();
        let mut keyboard = Keyboard::new();

        rom.load(read_file_bin(input_fname));
        
        cpu.reset();


        loop {
            let hlt = cpu.execute(&rom, &mut ram, manual);
            if manual {
                cpu.dump();
                ram.dump();
                let mut input = String::new();
                let _string = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
                let bytes = input.bytes().nth(0).expect("no byte read");
                if bytes == 113 || bytes == 81 { break; }
            }
            keyboard.update(&mut ram);
            screen.update(&ram);

            if hlt { break; }
        }   
        // if !manual { cpu.dump(); ram.dump(); } 
        // for ad in 0x4F..0x54 {
        //     println!("{:>0w$X} ", rom.read(ad), w=2);
        // }
    }

    term.c_lflag |= ECHO | ICANON;
    tcsetattr(0, TCSANOW, &term).unwrap();



}
