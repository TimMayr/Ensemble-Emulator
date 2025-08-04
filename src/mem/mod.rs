pub mod memory_map;
pub mod mirror_memory;

pub trait Memory {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let least_significant_bits = self.mem_read(addr) as u16;
        let highest_significant_bits = self.mem_read(addr + 1) as u16;

        (highest_significant_bits << 8) | (least_significant_bits)
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let least_significant_bits = (data & 0x00FF) as u8;
        let highest_significant_bits = (data >> 8) as u8;
        self.mem_write(addr, least_significant_bits);
        self.mem_write(addr + 1, highest_significant_bits)
    }

    fn load(&mut self, data: Box<[u8]>);
}

pub fn _mem_write(addr: u16, data: u8, memory: &mut [u8]) {
    memory[addr as usize] = data;
}

pub fn _mem_read(addr: u16, memory: &mut &[u8]) -> u8 {
    memory[addr as usize]
}

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
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize % self.memory.len()]
    }

    #[inline(always)]
    fn mem_write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize % self.memory.len()] = value;
    }

    fn load(&mut self, data: Box<[u8]>) {
        self.memory = data
    }
}

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
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize % self.memory.len()]
    }

    #[inline(always)]
    fn mem_write(&mut self, _: u16, _: u8) {}

    fn load(&mut self, data: Box<[u8]>) {
        self.memory = data
    }
}
