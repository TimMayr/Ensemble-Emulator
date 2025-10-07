use std::cell::Cell;
use std::ops::RangeInclusive;

use crate::emulation::emu::{HEIGHT, WIDTH};
use crate::emulation::mem::memory_map::MemoryMap;
use crate::emulation::mem::mirror_memory::MirrorMemory;
use crate::emulation::mem::palette_ram::PaletteRam;
use crate::emulation::mem::{Memory, MemoryDevice, Ram};
use crate::emulation::rom::{RomFile, RomFileConvertible};
use crate::emulation::savestate::PpuState;
use crate::util;

#[derive(Debug)]
pub struct Ppu {
    pub dot_counter: u128,
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
    pub bg_next_tile_id: u8,
    pub bg_next_tile_attribute: u8,
    pub bg_next_tile: u16,
    pub bg_shifter_pattern: u16,
    pub bg_shifter_attribute: u16,
    pub fine_x_scroll: u8,
    pub even_frame: bool,
    pub reset_signal: bool,
    pub pixel_buffer: [u32; (WIDTH * HEIGHT) as usize],
    pub master_cycle: u128,
    pub vbl_clear_scheduled: Option<u128>,
    pub scanline: u16,
    pub dot: u16,
}

impl Default for Ppu {
    fn default() -> Self { Self::new() }
}

const NES_PALETTE: [u32; 64] = [
    0x545454FF, 0x001E74F, 0x081090FF, 0x300088FF, 0x440064FF, 0x5C0030FF, 0x540400FF, 0x3C1800FF,
    0x202A00FF, 0x083A00FF, 0x004000F, 0x003C00FF, 0x00323CFF, 0x000000FF, 0x000000FF, 0x000000FF,
    0x989698FF, 0x084CC4FF, 0x3032ECFF, 0x5C1EE4F, 0x8814B0FF, 0xA01464FF, 0x982220FF, 0x783C00FF,
    0x545A00FF, 0x287200FF, 0x087C00FF, 0x007628FF, 0x006678F, 0x000000FF, 0x000000FF, 0x000000FF,
    0xECEEECFF, 0x4C9AECFF, 0x787CECFF, 0xB062ECFF, 0xE454ECFF, 0xEC58B4F, 0xEC6A64FF, 0xD48820FF,
    0xA0AA00FF, 0x74C400FF, 0x4CD020FF, 0x38CC6CFF, 0x38B4CCFF, 0x3C3C3CFF, 0x000000F, 0x000000FF,
    0xECEEECFF, 0xA8CCECFF, 0xBCBCECFF, 0xD4B2ECFF, 0xECAEECFF, 0xECAED4FF, 0xECB4B0FF, 0xE4C490F,
    0xCCD278FF, 0xB4DE78FF, 0xA8E290FF, 0x98E2B4FF, 0xA0D6E4FF, 0xA0A2A0FF, 0x000000FF, 0x000000FF,
];

const VBLANK_NMI_BIT: u8 = 0x80;
const VRAM_ADDR_INC_BIT: u8 = 0x4;
const UPPER_BYTE: u16 = 0xFF00;
const LOWER_BYTE: u16 = 0x00FF;
const BIT_14: u16 = 0x2000;
const BACKGROUND_RENDER_BIT: u8 = 0x8;
const SPRITE_RENDER_BIT: u8 = 0x10;
const VRAM_ADDR_COARSE_X_SCROLL_MASK: u16 = 0x1F;
const VRAM_ADDR_COARSE_Y_SCROLL_MASK: u16 = 0x3E0;
const VRAM_ADDR_FINE_Y_SCROLL_MASK: u16 = 0x7000;
const VRAM_ADDR_NAMETABLE_X_BIT: u16 = 0x400;
const VRAM_ADDR_NAMETABLE_Y_BIT: u16 = 0x800;
const FINE_Y_SCROLL_WIDTH: u8 = 0x7;
const COARSE_SCROLL_WIDTH: u8 = 0x1F;
pub const DOTS_PER_FRAME: u64 = 89342;
const PATTERN_TABLE_SIZE: usize = 0x1000;
const PALETTE_RAM_START_ADDRESS: u16 = 0x3F00;
const PALETTE_RAM_END_INDEX: u16 = 0x3FFF;
const PALETTE_RAM_SIZE: u16 = 0x20;
const PALETTE_SIZE: u16 = 0x4;
const VRAM_SIZE: usize = 2048;

impl Ppu {
    pub fn new() -> Self {
        Self {
            dot_counter: 0,
            ctrl_register: 0,
            mask_register: 0,
            status_register: 0,
            oam_addr_register: 0,
            oam_data_register: 0,
            ppu_x_scroll_register: 0,
            ppu_y_scroll_register: 0,
            vram_addr_register: 0,
            ppu_data_register: 0,
            ppu_data_buffer: 0,
            oam_dma_register: 0,
            bg_next_tile_id: 0,
            bg_next_tile_attribute: 0,
            bg_next_tile: 0,
            bg_shifter_pattern: 0,
            bg_shifter_attribute: 0,
            fine_x_scroll: 0,
            nmi_requested: Cell::new(false),
            memory: Box::new(Self::get_default_memory_map()),
            oam: Box::new(Self::get_default_oam()),
            write_latch: false,
            t_register: 0,
            even_frame: true,
            reset_signal: true,
            pixel_buffer: [0u32; (WIDTH * HEIGHT) as usize],
            master_cycle: 0,
            vbl_clear_scheduled: None,
            scanline: 0,
            dot: 0,
        }
    }

    fn get_default_memory_map() -> MemoryMap {
        let mut mem = MemoryMap::default();

        mem.add_memory(
            PALETTE_RAM_START_ADDRESS..=PALETTE_RAM_END_INDEX,
            Memory::MirrorMemory(MirrorMemory::new(
                Box::new(Memory::PaletteRam(PaletteRam::default())),
                PALETTE_RAM_SIZE - 1,
            )),
        );
        mem
    }

    fn get_default_oam() -> Ram { Ram::new(0xFF) }

    pub fn step(&mut self, master_cycle: u128) {
        self.master_cycle = master_cycle;

        if self.reset_signal {
            self.ctrl_register = 0;
            self.mask_register = 0;
            self.write_latch = false;
            self.ppu_y_scroll_register = 0;
            self.ppu_x_scroll_register = 0;
            self.ppu_data_buffer = 0;
            self.t_register = 0;
        }

        let mut frame_dot = self.dot_counter % DOTS_PER_FRAME as u128;

        if frame_dot == 0 {
            self.even_frame = !self.even_frame;
        }

        if frame_dot == 261 * 113 && !self.even_frame && self.is_rendering() {
            self.dot_counter += 1;
            frame_dot = self.dot_counter % DOTS_PER_FRAME as u128;
        }

        self.scanline = (frame_dot / 341) as u16;
        self.dot = (frame_dot % 341) as u16;

        if self.scanline == 241 && self.dot == 1 {
            // Just entered VBlank
            self.status_register |= VBLANK_NMI_BIT;
        }

        if let Some(vbl_clear_cycle) = self.vbl_clear_scheduled {
            if vbl_clear_cycle >= self.master_cycle {
                self.clear_vbl_bit();
            }

            if vbl_clear_cycle <= self.master_cycle {
                self.vbl_clear_scheduled = None;
            }
        }

        if self.scanline == 261 && self.dot == 1 {
            self.status_register &= !VBLANK_NMI_BIT;
            self.reset_signal = false;
        }

        if self.status_register & self.ctrl_register & VBLANK_NMI_BIT != 0 {
            self.nmi_requested.set(true);
        } else {
            self.nmi_requested.set(false);
        }

        self.dot_counter += 1;
    }

    pub fn is_rendering(&self) -> bool {
        self.is_background_rendering() || self.is_sprite_rendering()
    }

    pub fn get_vram_addr_step(&self) -> u8 {
        if self.ctrl_register & VRAM_ADDR_INC_BIT == 0 {
            1
        } else {
            32
        }
    }

    pub fn clear_vbl_bit(&mut self) { self.status_register &= !VBLANK_NMI_BIT; }

    pub fn get_ppu_status(&mut self) -> u8 {
        let result = self.status_register;
        self.nmi_requested.set(false);

        self.vbl_clear_scheduled = Some(self.master_cycle + 7);

        self.write_latch = false;
        result
    }

    pub fn get_ppu_ctrl(&self) -> u8 { self.ctrl_register }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        if !self.reset_signal {
            self.ctrl_register = value;

            self.t_register = (self.t_register & 0xF3FF) | (((value as u16) | 0x03) << 10)
        }
    }

    pub fn get_mask_register(&self) -> u8 { self.mask_register }

    pub fn set_mask_register(&mut self, value: u8) {
        if !self.reset_signal {
            self.mask_register = value;
        }
    }

    pub fn set_oam_addr_register(&mut self, value: u8) { self.oam_addr_register = value }

    pub fn get_oam_at_addr(&self) -> u8 { self.oam.read(self.oam_addr_register as u16, 0) }

    pub fn get_vram_at_addr(&mut self) -> u8 {
        let ret = self.ppu_data_buffer;
        self.ppu_data_buffer = self.memory.mem_read(self.vram_addr_register);
        self.vram_addr_register += self.get_vram_addr_step() as u16;
        ret
    }

    pub fn write_oam(&mut self, data: u8) {
        self.oam.write(self.oam_addr_register as u16, data);
        self.oam_addr_register = self.oam_addr_register.wrapping_add(1);
    }

    pub fn write_vram(&mut self, data: u8) {
        self.memory.mem_write(self.vram_addr_register, data);
        self.vram_addr_register = self
            .vram_addr_register
            .wrapping_add(self.get_vram_addr_step() as u16);
    }

    pub fn write_ppu_scroll(&mut self, data: u8) {
        if !self.reset_signal {
            if !self.write_latch {
                self.ppu_x_scroll_register = data
            } else {
                self.ppu_y_scroll_register = data
            }

            self.write_latch = !self.write_latch;
        }
    }

    pub fn write_vram_addr(&mut self, data: u8) {
        if !self.reset_signal {
            if !self.write_latch {
                self.vram_addr_register =
                    ((data as u16) << 8) | (self.vram_addr_register & LOWER_BYTE);
                self.t_register &= !BIT_14
            } else {
                self.vram_addr_register = (self.vram_addr_register & UPPER_BYTE) | data as u16;
            }

            self.write_latch = !self.write_latch;
        }
    }

    pub fn poll_nmi(&self) -> bool { self.nmi_requested.get() }

    pub fn clear_nmi_requested(&self) { self.nmi_requested.set(false) }

    pub fn is_background_rendering(&self) -> bool {
        self.mask_register & BACKGROUND_RENDER_BIT != 0
    }

    pub fn is_sprite_rendering(&self) -> bool { self.mask_register & SPRITE_RENDER_BIT != 0 }

    pub fn get_coarse_x_scroll(&self) -> u8 {
        (self.vram_addr_register & VRAM_ADDR_COARSE_X_SCROLL_MASK) as u8
    }

    pub fn set_coarse_x_scroll(&mut self, val: u8) {
        self.vram_addr_register = util::set_packed(
            &self.vram_addr_register,
            &val,
            &VRAM_ADDR_COARSE_X_SCROLL_MASK,
            &COARSE_SCROLL_WIDTH,
        );
    }

    pub fn get_coarse_y_scroll(&self) -> u8 {
        (self.vram_addr_register & VRAM_ADDR_COARSE_Y_SCROLL_MASK) as u8
    }

    pub fn set_coarse_y_scroll(&mut self, val: u8) {
        self.vram_addr_register = util::set_packed(
            &self.vram_addr_register,
            &val,
            &VRAM_ADDR_COARSE_Y_SCROLL_MASK,
            &COARSE_SCROLL_WIDTH,
        );
    }

    pub fn get_fine_x_scroll(&self) -> u8 { self.fine_x_scroll }

    pub fn get_fine_y_scroll(&self) -> u8 {
        (self.vram_addr_register & VRAM_ADDR_FINE_Y_SCROLL_MASK) as u8
    }

    pub fn set_fine_y_scroll(&mut self, val: u8) {
        self.vram_addr_register = util::set_packed(
            &self.vram_addr_register,
            &val,
            &VRAM_ADDR_FINE_Y_SCROLL_MASK,
            &FINE_Y_SCROLL_WIDTH,
        );
    }

    pub fn get_nametable_x(&self) -> u8 {
        (self.vram_addr_register & VRAM_ADDR_NAMETABLE_X_BIT) as u8
    }

    pub fn get_nametable_y(&self) -> u8 {
        (self.vram_addr_register & VRAM_ADDR_NAMETABLE_Y_BIT) as u8
    }

    pub fn increment_scroll_x(&mut self) {
        if self.is_rendering() {
            if self.get_coarse_x_scroll() == 31 {
                self.vram_addr_register &= !VRAM_ADDR_COARSE_X_SCROLL_MASK;
                self.vram_addr_register ^= VRAM_ADDR_NAMETABLE_X_BIT;
            } else {
                self.set_coarse_x_scroll(self.get_coarse_x_scroll() + 1)
            }
        }
    }

    pub fn increment_scroll_y(&mut self) {
        if self.is_rendering() {
            if self.get_fine_y_scroll() < 7 {
                self.set_fine_y_scroll(self.get_fine_y_scroll() + 1)
            } else {
                self.set_fine_y_scroll(0);

                if self.get_coarse_y_scroll() == 29 {
                    self.set_coarse_y_scroll(0);
                    self.vram_addr_register ^= VRAM_ADDR_NAMETABLE_Y_BIT;
                } else if self.get_coarse_y_scroll() == 31 {
                    self.set_coarse_y_scroll(0);
                } else {
                    self.set_coarse_y_scroll(self.get_coarse_y_scroll() + 1)
                }
            }
        }
    }

    pub fn get_pattern_table_0(&mut self, buffer: &mut [u8; PATTERN_TABLE_SIZE]) {
        for (i, byte) in buffer.iter_mut().enumerate().take(PATTERN_TABLE_SIZE) {
            *byte = self.mem_read(i as u16);
        }
    }

    pub fn get_pattern_table_1(&mut self, buffer: &mut [u8; PATTERN_TABLE_SIZE]) {
        for i in PATTERN_TABLE_SIZE..PATTERN_TABLE_SIZE * 2 - 1 {
            buffer[i - PATTERN_TABLE_SIZE] = self.mem_read(i as u16);
        }
    }

    pub fn mem_read(&mut self, addr: u16) -> u8 { self.memory.mem_read(addr) }

    pub fn load_rom<T: RomFileConvertible>(&mut self, rom_get: &T) {
        let rom_file = rom_get.as_rom_file();
        let chr_rom = rom_file.get_chr_rom();

        if let Some(chr_rom) = chr_rom {
            self.memory.add_memory(0x0000..=0x1FFF, chr_rom);
        }

        self.memory.add_memory(
            0x2000..=0x3EFF,
            Memory::MirrorMemory(MirrorMemory::new(
                Box::new(Memory::Ram(Ram::new(VRAM_SIZE))),
                (VRAM_SIZE - 1) as u16,
            )),
        )
    }

    pub fn reset(&mut self) { self.reset_signal = true; }

    pub fn get_color_for_bits(&mut self, color_bits: &u8) -> u32 {
        let mut current_palette: [u8; 4] = [1; 4];

        for (i, item) in current_palette.iter_mut().enumerate() {
            let _mem = self.get_memory_debug(Some(PALETTE_RAM_START_ADDRESS..=0x3F1F));

            *item = self.mem_read(
                PALETTE_RAM_START_ADDRESS
                    + (self.get_selected_palette() as u16 * PALETTE_SIZE)
                    + i as u16,
            )
        }

        let palette_index = current_palette[*color_bits as usize] as usize;

        // let palette_index = match color_bits {
        //     0b00 => 0x0F,
        //     0b11 => 0x26,
        //     0b01 => 0x2A,
        //     0b10 => 0x21,
        //     _ => 0x0F,
        // };

        NES_PALETTE[palette_index]
    }

    pub fn get_selected_palette(&self) -> u8 { 2 }

    fn decode_pattern_table(table: &[u8; 0x1000]) -> [u8; 0x4000] {
        let mut buffer = [0u8; 256 * 8 * 8]; // 256 tiles * 64 pixels
        for (tile_index, tile_bytes) in table.chunks_exact(16).enumerate() {
            let tile_bytes: &[u8; 16] = tile_bytes.try_into().unwrap();
            let flat_tile = Self::decode_tile_flat(tile_bytes);
            let start = tile_index * 64;
            buffer[start..start + 64].copy_from_slice(&flat_tile);
        }
        buffer
    }

    fn decode_tile_flat(tile_bytes: &[u8; 16]) -> [u8; 64] {
        let mut flat_tile = [0u8; 64];

        for y in 0..8 {
            let plane0 = tile_bytes[y];
            let plane1 = tile_bytes[y + 8];

            for x in 0..8 {
                let bit0 = (plane0 >> (7 - x)) & 1;
                let bit1 = (plane1 >> (7 - x)) & 1;
                flat_tile[y * 8 + x] = (bit1 << 1) | bit0;
            }
        }

        flat_tile
    }

    pub fn get_pixel_buffer(&self) -> &[u32; (WIDTH * HEIGHT) as usize] { &self.pixel_buffer }

    pub fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<u8> {
        self.memory.get_memory_debug(range)
    }

    pub fn frame(&mut self) {
        let mut table = [0u8; PATTERN_TABLE_SIZE];
        self.get_pattern_table_0(&mut table);
        let pattern0 = Self::decode_pattern_table(&table);

        let mut table = [0u8; PATTERN_TABLE_SIZE];
        self.get_pattern_table_1(&mut table);
        let pattern1 = Self::decode_pattern_table(&table);

        for (table_index, table) in [pattern0, pattern1].iter().enumerate() {
            let offset_x = if table_index == 0 { 0 } else { 18 * 8 };

            for tile_index in 0..256 {
                let tile_start = tile_index * 64;
                let tile_x = (tile_index % 16) * 8 + offset_x;
                let tile_y = (tile_index / 16) * 8;

                for y in 0..8 {
                    for x in 0..8 {
                        let pixel_index = tile_start + y * 8 + x;
                        let color_index = table[pixel_index]; // 0..3
                        let color = self.get_color_for_bits(&color_index);

                        self.pixel_buffer[(tile_y + y) * WIDTH as usize + (tile_x + x)] = color;
                    }
                }
            }
        }
    }
}

impl Ppu {
    pub fn from(state: &PpuState, rom: &RomFile) -> Self {
        let mut ppu = Self {
            dot_counter: state.cycle_counter,
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
            bg_next_tile_id: state.bg_next_tile_id,
            bg_next_tile_attribute: state.bg_next_tile_attribute,
            bg_next_tile: state.bg_next_tile,
            bg_shifter_pattern: state.bg_shifter_pattern,
            bg_shifter_attribute: state.bg_shifter_attribute,
            fine_x_scroll: state.fine_x_scroll,
            even_frame: state.even_frame,
            reset_signal: state.reset_signal,
            pixel_buffer: state.pixel_buffer.clone().try_into().unwrap(),
            master_cycle: state.master_cycle,
            vbl_clear_scheduled: None,
            scanline: state.scanline,
            dot: state.dot,
        };

        ppu.load_rom(rom);

        ppu
    }
}
