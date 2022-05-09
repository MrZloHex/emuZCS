pub struct Mem {
    capacity: usize,
    data: Vec<u8>
}

impl Mem {
    pub fn new(capacity: usize) -> Mem {
        let mut mem = Mem {
            capacity,
            data: Vec::new()
        };
        for _ in 0..mem.capacity {
            mem.data.push(0);
        }
        mem
    }

    pub fn load(&mut self, data: Vec<u8>) {
        for i in 0..data.len() {
            self.data[i] = data[i].clone();
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        if address >= self.capacity {
            eprintln!("ERROR: tried to read at unreal address");
            std::process::exit(1);
        }
        self.data[address].clone()
    }

    pub fn write(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }

    pub fn dump(&self) {
        for i in 0xF000..0xF006 {
            print!("{:>0w$X} ", self.data[i], w=2);
        }
        println!();
        for i in 0x8300..0x830F {
            print!("{:>0w$X} ", self.data[i], w=2);
        }
        println!();
    }
}

