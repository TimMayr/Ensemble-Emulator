use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::emulation::mem::MemoryDevice;
use crate::emulation::ppu::Ppu;

#[derive(Debug, Clone)]
pub struct PpuRegisters {
    ppu: Rc<RefCell<Ppu>>,
    data_bus: Cell<u8>,
}

impl PpuRegisters {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            ppu,
            data_bus: 0.into(),
        }
    }
}

impl MemoryDevice for PpuRegisters {
    #[inline(always)]
    fn read(&self, addr: u16, _: u8) -> u8 {
        match addr {
            0x2 => {
                let mut ppu = self.ppu.borrow_mut();
                let val = ppu.get_ppu_status() | (self.data_bus.get() & 0b0001_1111);
                self.data_bus.set(val);
            }
            0x4 => {
                let ppu = self.ppu.borrow();
                self.data_bus.set(ppu.get_oam_at_addr());
            }
            0x7 => {
                let mut ppu = self.ppu.borrow_mut();
                self.data_bus.set(ppu.get_vram_at_addr());
            }
            _ => {}
        };

        self.data_bus.get()
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, data: u8) {
        self.data_bus.set(data);
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

    fn init(&mut self, _: u16, _: u8) {}

    fn load(&mut self, _: Box<[u8]>) {}

    fn snapshot(&self, addr: u16, _: u8) -> u8 {
        match addr {
            0x2 => {
                let ppu = self.ppu.borrow();
                ppu.status_register
            }
            0x4 => {
                let ppu = self.ppu.borrow();
                ppu.get_oam_at_addr()
            }
            0x7 => {
                let ppu = self.ppu.borrow();
                ppu.ppu_data_buffer
            }
            _ => 0,
        }
    }
}
