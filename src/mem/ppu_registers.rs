use crate::mem::Memory;
use crate::ppu::PpuStub;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PpuRegisters {
    ppu: Rc<RefCell<PpuStub>>,
}

impl PpuRegisters {
    pub fn new(ppu: Rc<RefCell<PpuStub>>) -> Self {
        Self { ppu }
    }
}

impl Memory for PpuRegisters {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0 => self.ppu.borrow().get_ppu_ctrl(),
            0x1 => self.ppu.borrow().get_mask_register(),
            0x2 => self.ppu.borrow_mut().get_ppu_status(),
            _ => 0,
        }
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0 => self.ppu.borrow_mut().set_ppu_ctrl(data),
            0x1 => self.ppu.borrow_mut().set_ppu_ctrl(data),
            _ => (),
        }
    }

    fn load(&mut self, _: Box<[u8]>) {}

    #[cfg(debug_assertions)]
    fn read_debug(&self, addr: u16) -> u8 {
        match addr {
            0x0 => self.ppu.borrow().get_ppu_ctrl(),
            0x1 => self.ppu.borrow().get_mask_register(),
            0x2 => self.ppu.borrow().get_ppu_status_debug(),
            _ => 0,
        }
    }
}
