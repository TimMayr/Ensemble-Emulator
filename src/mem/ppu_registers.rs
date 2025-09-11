use crate::mem::Memory;
use crate::ppu::Ppu;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PpuRegisters {
    ppu: Rc<RefCell<Ppu>>,
}

impl PpuRegisters {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Self {
        Self { ppu }
    }
}

impl Memory for PpuRegisters {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x2 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.get_ppu_status()
            }
            0x4 => {
                let ppu = self.ppu.borrow();
                ppu.get_oam_at_addr()
            }
            0x7 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.get_vram_at_addr()
            }
            _ => 0,
        }
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.set_ppu_ctrl(data);
            }
            0x1 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.set_mask_register(data);
            }
            0x3 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.set_oam_addr_register(data);
            }
            0x4 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.write_oam(data);
            }
            0x5 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.write_ppu_scroll(data);
            }
            0x6 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.write_vram_addr(data);
            }
            0x7 => {
                let mut ppu = self.ppu.borrow_mut();
                ppu.write_vram(data);
            }
            _ => (),
        }
    }

    #[inline(always)]
    fn init(&mut self, _: u16, _: u8) {}

    fn load(&mut self, _: Box<[u8]>) {}

    fn snapshot(&self, addr: u16) -> u8 {
        match addr {
            0x0 => self.ppu.borrow().ctrl_register,
            0x1 => self.ppu.borrow().mask_register,
            0x2 => self.ppu.borrow().status_register,
            _ => 0,
        }
    }
}
