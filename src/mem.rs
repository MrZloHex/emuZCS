use super::in_error;

pub struct Mem {
    capacity: usize,
    data: Vec<u8>,
}

impl Mem {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity],
            capacity
        }
    }

    pub fn load(&mut self, data: Vec<u8>) {
        if self.capacity < data.len() {
            in_error("attemp to write to memory with overflow");
        }
        for (index, byte) in data.iter().enumerate() {
            self.data.insert(index, *byte);
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        if address >= self.capacity {
            in_error(format!(
                "tried to read at unreal address - {:X}", address
            ));
        }
        self.data[address].clone()
    }

    pub fn write(&mut self, address: usize, data: u8) {
        if address >= self.capacity {
            in_error(format!(
                "tried to write at unreal address - {:X}", address
            ));
        }
        self.data[address] = data;
    }

    pub fn dump(&self) {
        for i in 0xF000..0xF006 {
            print!("{:>0w$X} ", self.data[i], w = 2);
        }
        println!();
        for i in 0x8000..0x8003 {
            print!("{:>0w$X} ", self.data[i], w = 2);
        }
        println!();
    }
}
