use crate::nes::CPU_CYCLES_PER_FRAME;
use crate::savestate::PpuState;

#[derive(Debug, Clone)]
pub struct PpuStub {
    pub cycle_counter: u64,
    pub status: u8,
    pub mask_register: u8,
    pub ctrl: u8,
    pub nmi_requested: bool,
}

impl Default for PpuStub {
    fn default() -> Self {
        Self::new()
    }
}

impl PpuStub {
    pub fn new() -> Self {
        Self {
            status: 0x00,
            cycle_counter: 0,
            ctrl: 0,
            mask_register: 0,
            nmi_requested: false,
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
                    self.nmi_requested = true;
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

    pub fn get_ppu_status_debug(&self) -> u8 {
        self.status
    }

    pub fn get_ppu_ctrl(&self) -> u8 {
        self.status
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        self.ctrl = value;
    }

    pub fn get_mask_register(&self) -> u8 {
        self.mask_register
    }

    pub fn set_mask_register(&mut self, value: u8) {
        self.mask_register = value;
    }

    pub fn poll_nmi(&mut self) -> bool {
        if self.nmi_requested {
            self.nmi_requested = false;
            return true;
        }

        false
    }
}

impl From<PpuState> for PpuStub {
    fn from(state: PpuState) -> Self {
        Self {
            cycle_counter: state.cycle_counter,
            status: state.status,
            ctrl: state.ctrl,
            mask_register: state.mask_register,
            nmi_requested: state.nmi_requested,
        }
    }
}
