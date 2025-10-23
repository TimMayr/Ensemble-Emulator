use std::sync::{Arc, Mutex};

use crate::emulation::mem::MemoryDevice;
use crate::emulation::ppu::Ppu;

#[derive(Debug, Clone)]
pub struct PpuRegisters {
    ppu: Arc<Mutex<Ppu>>,
}

impl PpuRegisters {
    pub fn new(ppu: Arc<Mutex<Ppu>>) -> Self {
        Self {
            ppu,
        }
    }
}

impl MemoryDevice for PpuRegisters {
    #[inline(always)]
    fn read(&self, addr: u16, _: u8) -> u8 {
        match addr {
            0x2 => {
                let ppu = self.ppu.lock().unwrap();
                let val = ppu.get_ppu_status();

                let mut bus = ppu.open_bus.get();
                bus.set_masked(val, 0b1110_0000);
                ppu.open_bus.set(bus);
                bus.read()
            }
            0x4 => {
                let mut ppu = self.ppu.lock().unwrap();
                let val = ppu.get_oam_at_addr();
                let mut bus = ppu.open_bus.get();
                bus.set_masked(val, 0xFF);
                ppu.open_bus.set(bus);
                bus.read()
            }
            0x7 => {
                let mut ppu = self.ppu.lock().unwrap();
                let val = ppu.get_vram_at_addr();

                let mut bus = ppu.open_bus.get();
                match ppu.v_register {
                    0x3F00..=0x3FFF => {
                        bus.set_masked(val, 0b0011_1111);
                    }
                    _ => {
                        bus.set_masked(val, 0xFF);
                    }
                }
                ppu.open_bus.set(bus);
                bus.read()
            }
            _ => self.ppu.lock().unwrap().open_bus.get().read(),
        }
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, data: u8) {
        let mut ppu = self.ppu.lock().unwrap();
        let mut bus = ppu.open_bus.get();
        bus.set_masked(data, 0xFF);
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
        };

        ppu.open_bus.set(bus);
    }

    fn init(&mut self, _: u16, _: u8) {}

    fn load(&mut self, _: Box<[u8]>) {}

    fn snapshot(&self, addr: u16, _: u8) -> u8 {
        match addr {
            0x0 => {
                let ppu = self.ppu.lock().unwrap();
                ppu.get_ppu_ctrl()
            }
            0x2 => {
                let ppu = self.ppu.lock().unwrap();
                ppu.status_register.get()
            }
            0x4 => {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.get_oam_at_addr()
            }
            0x7 => {
                let ppu = self.ppu.lock().unwrap();
                ppu.ppu_data_buffer
            }
            _ => 0,
        }
    }
}
