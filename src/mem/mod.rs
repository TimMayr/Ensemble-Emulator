use std::fmt::Debug;

pub mod memory_map;
pub mod mirror_memory;
pub mod ppu_registers;

pub trait Memory: Debug {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn init(&mut self, addr: u16, data: u8);
    fn load(&mut self, data: Box<[u8]>);

    fn snapshot(&self, addr: u16) -> u8 {
        self.read(addr)
    }
}

#[derive(Debug, Clone)]
pub struct Ram {
    memory: Box<[u8]>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size].into_boxed_slice(),
        }
    }
}

impl Memory for Ram {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize % self.memory.len()]
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize % self.memory.len()] = value;
    }

    #[inline(always)]
    fn init(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize % self.memory.len()] = data;
    }

    fn load(&mut self, data: Box<[u8]>) {
        self.memory = data
    }
}

#[derive(Debug, Clone)]
pub struct Rom {
    memory: Box<[u8]>,
}

impl Rom {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size].into_boxed_slice(),
        }
    }
}

impl Memory for Rom {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize % self.memory.len()]
    }

    #[inline(always)]
    fn write(&mut self, _: u16, _: u8) {}

    #[inline(always)]
    fn init(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize % self.memory.len()] = data;
    }

    fn load(&mut self, data: Box<[u8]>) {
        self.memory = data
    }
}
