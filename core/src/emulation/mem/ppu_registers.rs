use std::cell::RefCell;
use std::rc::Rc;

use crate::emulation::mem::MemoryDevice;
use crate::emulation::ppu::Ppu;

#[derive(Debug, Clone)]
pub struct PpuRegisters {
    ppu: Rc<RefCell<Ppu>>,
}

impl PpuRegisters {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            ppu,
        }
    }
}

impl MemoryDevice for PpuRegisters {
    #[inline(always)]
    fn read(&self, addr: u16, _: u8) -> u8 {
        let mut ppu = self.ppu.borrow_mut();
        match addr {
            0x2 => {
                let val = ppu.get_ppu_status();
                ppu.open_bus.set_masked(val, 0b1110_0000);
            }
            0x4 => {
                let val = ppu.get_oam_at_addr();
                ppu.open_bus.set_masked(val, 0xFF);
            }
            0x7 => {
                let val = ppu.get_vram_at_addr();

                match ppu.vram_addr_register {
                    0x3F00..=0x3FFF => {
                        ppu.open_bus.set_masked(val, 0b0011_1111);
                    }
                    _ => {
                        ppu.open_bus.set_masked(val, 0xFF);
                    }
                }
            }
            _ => {}
        };

        ppu.open_bus.read()
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, data: u8) {
        let mut ppu = self.ppu.borrow_mut();
        ppu.open_bus.set_masked(data, 0xFF);
        match addr {
            0x0 => {
                ppu.set_ppu_ctrl(data);
            }
            0x1 => {
                ppu.set_mask_register(data);
            }
            0x3 => {
                ppu.set_oam_addr_register(data);
            }
            0x4 => {
                ppu.write_oam(data);
            }
            0x5 => {
                ppu.write_ppu_scroll(data);
            }
            0x6 => {
                ppu.write_vram_addr(data);
            }
            0x7 => {
                ppu.write_vram(data);
            }
            _ => (),
        }
    }

    fn init(&mut self, _: u16, _: u8) {}

    fn load(&mut self, _: Box<[u8]>) {}

    fn snapshot(&self, addr: u16, _: u8) -> u8 {
        match addr {
            0x0 => {
                let ppu = self.ppu.borrow();
                ppu.get_ppu_ctrl()
            }
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
