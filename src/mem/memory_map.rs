use crate::mem::Memory;
use std::ops::RangeInclusive;

const MEMORY_SIZE: u16 = 0xFFFF;

pub struct MemoryMap {
    pub regions: Vec<Box<dyn Memory>>,
    lookup: Box<[usize]>,
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            lookup: vec![0; MEMORY_SIZE as usize + 1].into_boxed_slice(),
        }
    }

    pub fn add_memory(&mut self, address_space: RangeInclusive<u16>, memory: Box<dyn Memory>) {
        let device_index = self.regions.len() + 1;

        for addr in address_space.clone() {
            self.lookup[addr as usize] = device_index;
        }

        self.regions.push(memory)
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        let device = self.lookup[addr as usize];

        if device == 0 {
            return 0x00u8;
        }

        self.regions[device - 1].read(addr)
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        let device = self.lookup[addr as usize];

        if device == 0 {
            return;
        }

        self.regions[device - 1].mem_write(addr, data)
    }

    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let least_significant_bits = self.mem_read(addr) as u16;
        let highest_significant_bits = self.mem_read(addr + 1) as u16;

        (highest_significant_bits << 8) | (least_significant_bits)
    }

    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let least_significant_bits = (data & 0x00FF) as u8;
        let highest_significant_bits = (data >> 8) as u8;
        self.mem_write(addr, least_significant_bits);
        self.mem_write(addr + 1, highest_significant_bits)
    }

    #[cfg(debug_assertions)]
    pub fn get_memory(&self, range: RangeInclusive<u16>) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();
        for addr in range {
            vec.push(self.mem_read(addr));
        }
        vec
    }
}
