use crate::mem::memory_map::MemoryMap;
use crate::mem::mirror_memory::MirrorMemory;
use crate::mem::{Memory, Ram};
use crate::nes::CPU_CYCLES_PER_FRAME;
use crate::rom::{RomFile, RomFileConvertible};
use crate::savestate::PpuState;
use std::cell::Cell;

#[derive(Debug)]
pub struct Ppu {
    pub cycle_counter: u64,
    pub ctrl_register: u8,
    pub mask_register: u8,
    pub status_register: u8,
    pub oam_addr_register: u8,
    pub oam_data_register: u8,
    pub ppu_x_scroll_register: u8,
    pub ppu_y_scroll_register: u8,
    pub vram_addr_register: u16,
    pub ppu_data_register: u8,
    pub ppu_data_buffer: u8,
    pub oam_dma_register: u8,
    pub nmi_requested: Cell<bool>,
    pub memory: Box<MemoryMap>,
    pub oam: Box<Ram>,
    pub write_latch: bool,
    pub t_register: u16,
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

const VBLANK_NMI_BIT: u8 = 0x80;
const VRAM_ADDR_INC_BIT: u8 = 0x4;

impl Ppu {
    pub fn new() -> Self {
        Self {
            cycle_counter: 0,
            ctrl_register: 0,
            mask_register: 0,
            status_register: 0x00,
            oam_addr_register: 0,
            oam_data_register: 0,
            ppu_x_scroll_register: 0,
            ppu_y_scroll_register: 0,
            vram_addr_register: 0,
            ppu_data_register: 0,
            ppu_data_buffer: 0,
            oam_dma_register: 0,
            nmi_requested: Cell::new(false),
            memory: Box::new(Self::get_default_memory_map()),
            oam: Box::new(Self::get_default_oam()),
            write_latch: false,
            t_register: 0,
        }
    }

    fn get_default_memory_map() -> MemoryMap {
        let mut mem = MemoryMap::default();

        mem.add_memory(
            0x3F00..=0x3FFF,
            Box::new(MirrorMemory::new(Box::new(Ram::new(0x20)), 0x20)),
        );
        mem
    }

    fn get_default_oam() -> Ram {
        Ram::new(0xFF)
    }

    pub fn step(&mut self) {
        self.cycle_counter += 1;

        let frame_cycle = self.cycle_counter % CPU_CYCLES_PER_FRAME as u64;

        if frame_cycle >= 241 * 113 {
            if (self.status_register & VBLANK_NMI_BIT) == 0 {
                // Just entered VBlank
                self.status_register |= VBLANK_NMI_BIT;
                if self.ctrl_register & VBLANK_NMI_BIT != 0 {
                    self.nmi_requested.set(true);
                }
            }
        } else {
            self.status_register &= !0x80; // clear vblank bit
        }
    }

    pub fn get_vram_addr_step(&self) -> u8 {
        if self.ctrl_register & VRAM_ADDR_INC_BIT == 0 {
            1
        } else {
            32
        }
    }

    pub fn get_ppu_status(&mut self) -> u8 {
        let result = self.status_register;
        self.status_register &= !0x80;
        self.write_latch = false;
        result
    }

    pub fn get_ppu_ctrl(&self) -> u8 {
        self.ctrl_register
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        self.ctrl_register = value;
    }

    pub fn get_mask_register(&self) -> u8 {
        self.mask_register
    }

    pub fn set_mask_register(&mut self, value: u8) {
        self.mask_register = value;
    }

    pub fn set_oam_addr_register(&mut self, value: u8) {
        self.oam_addr_register = value
    }

    pub fn get_oam_at_addr(&self) -> u8 {
        self.oam.read(self.oam_addr_register as u16)
    }

    pub fn get_vram_at_addr(&mut self) -> u8 {
        let ret = self.ppu_data_buffer;
        self.ppu_data_buffer = self.memory.mem_read(self.vram_addr_register);
        self.vram_addr_register += self.get_vram_addr_step() as u16;
        ret
    }

    pub fn write_oam(&mut self, data: u8) {
        self.oam.write(self.oam_addr_register as u16, data);
        self.oam_addr_register += 1;
    }

    pub fn write_vram(&mut self, data: u8) {
        self.memory.mem_write(self.vram_addr_register, data);
        self.vram_addr_register += self.get_vram_addr_step() as u16;
    }

    pub fn write_ppu_scroll(&mut self, data: u8) {
        if !self.write_latch {
            self.ppu_x_scroll_register = data
        } else {
            self.ppu_y_scroll_register = data
        }

        self.write_latch = !self.write_latch;
    }

    pub fn write_vram_addr(&mut self, data: u8) {
        if !self.write_latch {
            self.vram_addr_register = ((data as u16) << 8) | (self.vram_addr_register & 0xFF);
            self.t_register &= 0xBFFF
        } else {
            self.vram_addr_register = (self.vram_addr_register & 0xFF00) | data as u16;
        }

        self.write_latch = !self.write_latch;
    }

    pub fn poll_nmi(&self) -> bool {
        if self.nmi_requested.get() {
            self.nmi_requested.set(false);
            return true;
        }

        false
    }

    pub fn load_rom<T: RomFileConvertible>(&mut self, rom_get: &T) {
        let rom_file = rom_get.as_rom_file();
        let chr_rom = rom_file.get_chr_rom();

        self.memory.add_memory(0x0000..=0x1FFF, chr_rom);
        self.memory.add_memory(
            0x2000..=0x3EFF,
            Box::new(MirrorMemory::new(Box::new(Ram::new(2 * 1024)), 2 * 1024)),
        )
    }
}

impl Ppu {
    pub fn from(state: &PpuState, rom: &RomFile) -> Self {
        let mut ppu = Self {
            cycle_counter: state.cycle_counter,
            ctrl_register: state.ctrl_register,
            mask_register: state.mask_register,
            status_register: state.status_register,
            oam_addr_register: state.oam_addr_register,
            oam_data_register: state.oam_data_register,
            ppu_x_scroll_register: state.ppu_x_scroll_register,
            ppu_y_scroll_register: state.ppu_y_scroll_register,
            vram_addr_register: state.ppu_addr_register,
            ppu_data_register: state.ppu_data_register,
            ppu_data_buffer: state.ppu_data_buffer,
            nmi_requested: Cell::new(state.nmi_requested),
            memory: Box::new(Self::get_default_memory_map()),
            oam: Box::new(Self::get_default_oam()),
            oam_dma_register: state.oam_dma_register,
            write_latch: state.write_latch,
            t_register: state.t_register,
        };

        ppu.load_rom(rom);

        ppu
    }
}
