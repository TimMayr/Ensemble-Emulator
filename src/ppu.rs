use crate::nes::CPU_CYCLES_PER_FRAME;

#[derive(Debug, Clone)]
pub struct PpuStub {
    status: u8,
    cycle_counter: u64,
    mask_register: u8,
    ctrl: u8,
    nmi_requested: bool,
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

        if self.cycle_counter % CPU_CYCLES_PER_FRAME as u64 == 0 {
            dbg!()
        }

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
