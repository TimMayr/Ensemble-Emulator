use crate::mem::Memory;

#[derive(Debug)]
pub struct MirrorMemory {
    base: Box<dyn Memory>,
    mirror_mask: u16,
}

impl MirrorMemory {
    pub fn new(base: Box<dyn Memory>, mirror_mask: u16) -> Self {
        Self { base, mirror_mask }
    }
}

impl Memory for MirrorMemory {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        self.base.read(addr & self.mirror_mask)
    }

    #[inline(always)]
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.base.mem_write(addr & self.mirror_mask, data)
    }

    fn load(&mut self, data: Box<[u8]>) {
        self.base.load(data)
    }
}
