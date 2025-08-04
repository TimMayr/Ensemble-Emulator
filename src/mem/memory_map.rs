use crate::mem::Memory;
use std::ops::RangeInclusive;

const MEMORY_SIZE: u16 = 0xFFFF;

pub struct MemoryMap {
    pub regions: Vec<Box<dyn Memory>>,
    lookup: [usize; MEMORY_SIZE as usize + 1],
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
            lookup: [0; MEMORY_SIZE as usize + 1],
        }
    }

    pub fn add_memory(&mut self, address_space: RangeInclusive<u16>, memory: Box<dyn Memory>) {
        let device_index = self.regions.len();

        for addr in address_space.clone() {
            self.lookup[addr as usize] = device_index;
        }

        self.regions.push(memory)
    }
}

impl Memory for MemoryMap {
    #[inline(always)]
    fn mem_read(&self, addr: u16) -> u8 {
        let device = self.lookup[addr as usize];
        self.regions[device].mem_read(addr)
    }

    #[inline(always)]
    fn mem_write(&mut self, addr: u16, data: u8) {
        let device = self.lookup[addr as usize];
        self.regions[device].mem_write(addr, data)
    }

    fn load(&mut self, _: Box<[u8]>) {}
}
