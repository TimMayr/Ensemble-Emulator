use crate::emulation::mem::{Memory, MemoryDevice};
use std::ops::RangeInclusive;

const MEMORY_SIZE: u16 = 0xFFFF;

#[derive(Clone, Copy, Debug)]
struct RegionEntry {
    device: usize, // index into self.regions
    offset: u16,   // offset into that device
}

#[derive(Debug)]
pub struct MemoryMap {
    pub regions: Vec<Memory>,
    lookup: Box<[Option<RegionEntry>]>,
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
            lookup: vec![None; MEMORY_SIZE as usize + 1].into_boxed_slice(),
        }
    }

    pub fn add_memory(&mut self, address_space: RangeInclusive<u16>, memory: Memory) {
        let device_index = self.regions.len();

        let start = *address_space.start();
        for addr in address_space {
            let offset = addr - start;
            self.lookup[addr as usize] = Some(RegionEntry {
                device: device_index,
                offset,
            });
        }

        self.regions.push(memory)
    }

    #[inline]
    pub fn mem_read(&self, addr: u16) -> u8 {
        if let Some(entry) = self.lookup[addr as usize] {
            self.regions[entry.device].read(entry.offset)
        } else {
            0
        }
    }

    #[inline]
    pub fn mem_read_debug(&self, addr: u16) -> u8 {
        if let Some(entry) = self.lookup[addr as usize] {
            self.regions[entry.device].snapshot(entry.offset)
        } else {
            0
        }
    }

    #[inline]
    pub fn mem_write(&mut self, addr: u16, data: u8) {
        if let Some(entry) = self.lookup[addr as usize] {
            self.regions[entry.device].write(entry.offset, data)
        }
    }

    #[inline]
    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let least_significant_bits = self.mem_read(addr) as u16;
        let highest_significant_bits = self.mem_read(addr + 1) as u16;

        (highest_significant_bits << 8) | (least_significant_bits)
    }

    #[inline]
    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let least_significant_bits = (data & 0x00FF) as u8;
        let highest_significant_bits = (data >> 8) as u8;
        self.mem_write(addr, least_significant_bits);
        self.mem_write(addr + 1, highest_significant_bits)
    }

    #[inline]
    fn mem_init(&mut self, addr: u16, data: u8) {
        if let Some(entry) = self.lookup[addr as usize] {
            self.regions[entry.device].init(entry.offset, data)
        }
    }

    pub fn get_memory_debug(&self, range: RangeInclusive<u16>) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();
        for addr in range {
            vec.push(self.mem_read_debug(addr));
        }
        vec
    }

    pub fn load(&mut self, data: &[u8]) {
        for (addr, byte) in data.iter().enumerate() {
            self.mem_init(addr as u16, *byte);
        }
    }
}
