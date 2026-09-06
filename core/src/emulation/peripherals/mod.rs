use std::fmt::Debug;
use std::hash::Hash;

use crate::emulation::rom::ExpansionDevice;

#[enum_delegate::implement(PeripheralDevice)]
pub enum Peripheral {
    StandardController(StandardController),
}

impl Default for Peripheral {
    fn default() -> Self { Peripheral::StandardController(StandardController::default()) }
}

#[enum_delegate::register]
pub trait PeripheralDevice {
    fn set_refresh_func(&mut self, func: Box<dyn Fn(Peripheral) -> Peripheral>);
    fn refresh(self) -> Self;
    fn read(self) -> (u8, Peripheral);
    fn read_debug(&self) -> u8;
    fn handle_strobe_data(self, data: u8) -> Self;
}

impl From<ExpansionDevice> for Peripheral {
    fn from(value: ExpansionDevice) -> Self {
        #[allow(clippy::panic)]
        match value {
            ExpansionDevice::StandardController => {
                Peripheral::StandardController(StandardController::default())
            }
            ExpansionDevice::Unknown(id) => {
                panic!("Peripheral with id \"{id}\" is not known")
            }
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Default)]
pub struct StandardController {
    pub shift: u8,
    pub strobe: bool,
    pub refresh_func: Option<Box<dyn Fn(Peripheral) -> Peripheral>>,
}

impl Clone for StandardController {
    fn clone(&self) -> Self {
        Self {
            shift: self.shift,
            strobe: self.strobe,
            refresh_func: None,
        }
    }
}

impl PeripheralDevice for StandardController {
    fn set_refresh_func(&mut self, func: Box<dyn Fn(Peripheral) -> Peripheral>) {
        self.refresh_func = Some(func);
    }

    fn refresh(mut self) -> Self {
        let refresh_func = self.refresh_func.take();

        if let Some(refresh_func) = refresh_func {
            let mut r = match refresh_func(Peripheral::StandardController(self)) {
                Peripheral::StandardController(c) => c,
                #[allow(unreachable_patterns)]
                _ => {
                    unreachable!()
                }
            };

            r.set_refresh_func(refresh_func);
            r
        } else {
            self.refresh_func = refresh_func;
            self
        }
    }

    #[inline(always)]
    fn read(mut self) -> (u8, Peripheral) {
        if self.strobe {
            self = self.refresh();
        }

        (self.poll(), Peripheral::StandardController(self))
    }

    #[inline(always)]
    fn read_debug(&self) -> u8 {
        let mut cloned = (*self).clone();

        if cloned.strobe {
            cloned = cloned.refresh();
        }

        cloned.read().0
    }

    #[inline]
    fn handle_strobe_data(mut self, data: u8) -> Self {
        self.strobe = (data & 1) == 1;
        if self.strobe {
            self = self.refresh();
        }

        self
    }
}

impl StandardController {
    #[inline(always)]
    fn poll(&mut self) -> u8 {
        let res = self.shift & 1;
        self.shift = (self.shift >> 1) | 0x80;
        res
    }

    #[must_use]
    pub fn new(shift: u8, strobe: bool) -> Self {
        StandardController {
            shift,
            strobe,
            refresh_func: None,
        }
    }

    pub fn reload(mut self, input: StandardControllerState) -> Self {
        self.shift = u8::from(input.a)
            | u8::from(input.b) << 1
            | u8::from(input.select) << 2
            | u8::from(input.start) << 3
            | u8::from(input.up) << 4
            | u8::from(input.down) << 5
            | u8::from(input.left) << 6
            | u8::from(input.right) << 7;

        self
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct StandardControllerState {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
