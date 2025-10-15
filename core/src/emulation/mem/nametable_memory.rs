use crate::emulation::mem::{MemoryDevice, Ram};
use crate::emulation::ppu::{NAMETABLE_SIZE, VRAM_SIZE};

#[derive(Debug, Clone, Copy)]
pub enum NametableMirroring {
    Vertical,
    Horizontal,
    SingleScreenLower,
    SingleScreenUpper,
    FourScreen,
}

#[derive(Debug, Clone)]
pub struct NametableMemory {
    vram: Ram,
    mirroring: NametableMirroring,
}

impl NametableMemory {
    pub fn new(mirroring: NametableMirroring) -> Self {
        Self {
            vram: Ram::new(VRAM_SIZE), // 2KB of VRAM
            mirroring,
        }
    }

    /// Translate PPU address (0x2000–0x3EFF) to physical VRAM offset
    #[inline(always)]
    fn mirror_addr(&self, addr: u16) -> u16 {
        let table = addr / NAMETABLE_SIZE;
        let offset = addr % NAMETABLE_SIZE;

        match self.mirroring {
            NametableMirroring::Vertical => match table {
                0 | 2 => 0x000 + offset,
                1 | 3 => 0x400 + offset,
                _ => unreachable!(),
            },
            NametableMirroring::Horizontal => match table {
                0 | 1 => 0x000 + offset,
                2 | 3 => 0x400 + offset,
                _ => unreachable!(),
            },
            NametableMirroring::SingleScreenLower => 0x000 + offset,
            NametableMirroring::SingleScreenUpper => 0x400 + offset,
            NametableMirroring::FourScreen => addr,
        }
    }
}

impl MemoryDevice for NametableMemory {
    #[inline(always)]
    fn read(&self, addr: u16, open_bus: u8) -> u8 {
        self.vram.read(self.mirror_addr(addr), open_bus)
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, data: u8) { self.vram.write(self.mirror_addr(addr), data) }

    #[inline(always)]
    fn init(&mut self, addr: u16, data: u8) { self.vram.init(self.mirror_addr(addr), data) }

    fn load(&mut self, data: Box<[u8]>) { self.vram.load(data) }
}
