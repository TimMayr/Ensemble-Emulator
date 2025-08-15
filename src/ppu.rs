use crate::nes::CPU_CYCLES_PER_FRAME;
use crate::savestate::PpuState;
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Ppu {
    pub cycle_counter: u64,
    pub status: u8,
    pub mask: u8,
    pub ctrl: u8,
    pub nmi_requested: Cell<bool>,
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            status: 0x00,
            cycle_counter: 0,
            ctrl: 0,
            mask: 0,
            nmi_requested: Cell::new(false),
        }
    }

    pub fn step(&mut self) {
        self.cycle_counter += 1;

        let frame_cycle = self.cycle_counter % CPU_CYCLES_PER_FRAME as u64;

        if frame_cycle >= 241 * 113 {
            if (self.status & 0x80) == 0 {
                // Just entered VBlank
                self.status |= 0x80;
                if self.ctrl & 0x80 != 0 {
                    self.nmi_requested.set(true);
                }
            }
        } else {
            self.status &= !0x80; // clear vblank bit
        }
    }

    pub fn get_ppu_status(&mut self) -> u8 {
        let result = self.status;
        self.status &= !0x80;
        result
    }

    pub fn get_ppu_ctrl(&self) -> u8 {
        self.ctrl
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        self.ctrl = value;
    }

    pub fn get_mask_register(&self) -> u8 {
        self.mask
    }

    pub fn set_mask_register(&mut self, value: u8) {
        self.mask = value;
    }

    pub fn poll_nmi(&self) -> bool {
        if self.nmi_requested.get() {
            self.nmi_requested.set(false);
            return true;
        }

        false
    }
}

impl From<PpuState> for Ppu {
    fn from(state: PpuState) -> Self {
        Self {
            cycle_counter: state.cycle_counter,
            status: state.status,
            ctrl: state.ctrl,
            mask: state.mask_register,
            nmi_requested: Cell::new(state.nmi_requested),
        }
    }
}
