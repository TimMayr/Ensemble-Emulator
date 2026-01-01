use std::cell::RefCell;
use std::ops::{Add, BitAnd, Shr};

use crate::emulation::mem::MemoryDevice;

#[derive(Debug, Clone)]
pub struct ApuRegisters {
    status: RefCell<u8>,
    input_a: u8,
    input_b: u8,
    read_counter: RefCell<u8>,
}

impl Default for ApuRegisters {
    fn default() -> Self { Self::new() }
}

impl ApuRegisters {
    pub fn new() -> Self {
        Self {
            status: RefCell::new(0xFF),
            input_a: 0b1110_0000,
            input_b: 0b1110_0000,
            read_counter: 0.into(),
        }
    }
}

impl MemoryDevice for ApuRegisters {
    #[inline(always)]
    fn read(&self, addr: u16, open_bus: u8) -> u8 {
        match addr {
            0x00..=0x14 => open_bus,
            0x15 => {
                let status = self.status.borrow_mut();
                let new = status.bitand(0b1011111);
                drop(status);

                self.status.replace(new);
                new
            }
            0x16 => {
                let mut counter = self.read_counter.borrow().add(0);
                let res = open_bus & self.input_a.shr(counter);

                counter += 1;

                if counter > 8 {
                    counter = 0;
                }

                self.read_counter.replace(counter);

                res
            }
            0x17 => {
                let mut counter = self.read_counter.borrow().add(0);
                let res = open_bus & self.input_b.shr(counter);

                counter += 1;

                if counter > 8 {
                    counter = 0;
                }

                self.read_counter.replace(counter);

                res
            }
            _ => 0,
        }
    }

    #[inline(always)]
    fn write(&mut self, _: u16, _: u8) {}

    fn init(&mut self, addr: u16, val: u8) {
        match addr {
            0x16 => self.input_a = val,
            0x17 => self.input_b = val,
            _ => {}
        }
    }

    fn load(&mut self, _: Box<[u8]>) {}

    fn is_internal(&self) -> bool { true }

    fn snapshot(&self, addr: u16, open_bus: u8) -> u8 {
        match addr {
            0x00..=0x14 => open_bus,
            0x15 => self.status.clone().take(),
            _ => 0,
        }
    }
}
