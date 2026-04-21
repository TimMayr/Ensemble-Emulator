use std::fmt::Debug;
use std::hash::Hash;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::emulation::mem::OpenBus;

#[enum_dispatch(PeripheralDevice)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Peripheral {
    DefaultController,
}

#[enum_dispatch]
pub trait PeripheralDevice: Debug + Eq + PartialEq + Hash + Clone + Serialize {
    fn read(&mut self, open_bus: &OpenBus) -> u8;
    fn read_debug(&self, open_bus: &OpenBus) -> u8;
    fn handle_strobe_data(&mut self, data: u8);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DefaultController {
    input: u8,
    shift: u8,
    strobe: bool,
}

impl PeripheralDevice for DefaultController {
    #[inline]
    fn read(&mut self, open_bus: &OpenBus) -> u8 {
        if self.strobe {
            self.shift = self.input
        }

        self.poll(open_bus.read())
    }

    #[inline]
    fn read_debug(&self, open_bus: &OpenBus) -> u8 {
        let mut shift = self.shift;

        if self.strobe {
            shift = self.input;
        }

        self.poll_with_shift(shift, open_bus.read())
    }

    #[inline]
    fn handle_strobe_data(&mut self, data: u8) {
        self.strobe = (data & 1) == 1;
        if self.strobe {
            self.shift = self.input
        }
    }
}

impl DefaultController {
    #[inline]
    fn poll(&mut self, open_bus: u8) -> u8 {
        let res = (open_bus & !0b111) | (self.shift & 1);
        self.shift = (self.shift >> 1) | 0x80;
        res
    }

    #[inline]
    fn poll_with_shift(&self, shift: u8, open_bus: u8) -> u8 { (open_bus & !0b111) | (shift & 1) }
}
