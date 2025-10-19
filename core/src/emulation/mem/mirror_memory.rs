use crate::emulation::mem::{Memory, MemoryDevice};

#[derive(Debug, Clone)]
pub struct MirrorMemory {
    base: Box<Memory>,
    mirror_mask: u16,
}

impl MirrorMemory {
    #[inline(always)]
    pub fn new(base: Box<Memory>, mirror_mask: u16) -> Self {
        Self {
            base,
            mirror_mask,
        }
    }
}

impl MemoryDevice for MirrorMemory {
    #[inline(always)]
    fn read(&self, addr: u16, open_bus: u8) -> u8 {
        self.base.read(addr & self.mirror_mask, open_bus)
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, data: u8) { self.base.write(addr & self.mirror_mask, data) }

    #[inline(always)]
    fn init(&mut self, addr: u16, data: u8) { self.base.init(addr & self.mirror_mask, data) }

    #[inline(always)]
    fn load(&mut self, data: Box<[u8]>) { self.base.load(data) }

    #[inline(always)]
    fn snapshot(&self, addr: u16, open_bus: u8) -> u8 {
        self.base.snapshot(addr & self.mirror_mask, open_bus)
    }
}
