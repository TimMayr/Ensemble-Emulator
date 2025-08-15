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
    fn write(&mut self, addr: u16, data: u8) {
        self.base.write(addr & self.mirror_mask, data)
    }

    #[inline(always)]
    fn init(&mut self, addr: u16, data: u8) {
        self.base.init(addr & self.mirror_mask, data)
    }

    fn load(&mut self, data: Box<[u8]>) {
        self.base.load(data)
    }

    fn snapshot(&self, addr: u16) -> u8 {
        self.base.snapshot(addr & self.mirror_mask)
    }
}
