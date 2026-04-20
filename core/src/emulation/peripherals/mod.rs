use std::cell::Cell;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::emulation::mem::OpenBus;

#[enum_dispatch(PeripheralDevice)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Peripheral {
    DefaultController,
}

#[enum_dispatch]
pub trait PeripheralDevice: Debug + Eq + PartialEq + Hash + Clone {
    fn read(&mut self, open_bus: &OpenBus) -> u8;
    fn read_debug(&self, open_bus: &OpenBus) -> u8;
    fn handle_strobe_data(&mut self, data: u8);
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct HashCell<T: Debug + Eq + PartialEq + Hash + Clone + Copy>(Cell<T>);

impl<T: Debug + Eq + PartialEq + Hash + Clone + Copy> Hash for HashCell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) { self.0.get().hash(state); }
}

impl<T: Debug + Eq + PartialEq + Hash + Clone + Copy> HashCell<T> {
    pub fn set(&self, val: T) { self.0.set(val) }

    pub fn get(&self) -> T { self.0.get() }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DefaultController {
    input: u8,
    shift: HashCell<u8>,
    strobe: bool,
}

impl PeripheralDevice for DefaultController {
    #[inline]
    fn read(&mut self, open_bus: &OpenBus) -> u8 {
        if self.strobe {
            self.shift.set(self.input)
        }

        self.poll(open_bus.read())
    }

    #[inline]
    fn read_debug(&self, open_bus: &OpenBus) -> u8 {
        if self.strobe {
            self.shift.set(self.input)
        }

        self.poll(open_bus.read())
    }

    #[inline]
    fn handle_strobe_data(&mut self, data: u8) {
        self.strobe = (data & 1) == 1;
        if self.strobe {
            self.shift.set(self.input)
        }
    }
}

impl DefaultController {
    #[inline]
    fn poll(&self, open_bus: u8) -> u8 {
        let res = (open_bus & !0b111) | (self.shift.get() & 1);
        self.shift.set((self.shift.get() >> 1) | 0x80);
        res
    }
}

pub enum PeripheralWriteResult {
    Handled,
    StrobeTriggered,
}
