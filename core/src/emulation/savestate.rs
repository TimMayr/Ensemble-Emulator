use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};

use crate::emulation::cpu::{Cpu, MicroOp};
use crate::emulation::emu::{HEIGHT, WIDTH};
use crate::emulation::ppu::Ppu;
use crate::emulation::rom::RomFile;

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
pub struct CpuState {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub processor_status: u8,
    pub memory: Vec<u8>,
    pub master_cycle: u128,
    pub lo: u8,
    pub hi: u8,
    pub current_op: MicroOp,
    pub op_queue: Vec<MicroOp>,
    pub current_opcode: Option<u8>,
    pub temp: u8,
    pub ane_constant: u8,
    pub is_halted: bool,
}

impl From<&Cpu> for CpuState {
    fn from(cpu: &Cpu) -> Self {
        let mut current_opcode = None;

        if let Some(op) = cpu.current_opcode {
            current_opcode = Some(op.opcode);
        }

        Self {
            program_counter: cpu.program_counter,
            stack_pointer: cpu.stack_pointer,
            accumulator: cpu.accumulator,
            x_register: cpu.x_register,
            y_register: cpu.y_register,
            processor_status: cpu.processor_status,
            memory: cpu.memory.get_memory_debug(Some(0x0..=0xFFFF)),
            master_cycle: cpu.master_cycle,
            lo: cpu.lo,
            hi: cpu.hi,
            current_op: cpu.current_op,
            op_queue: cpu.op_queue.clone(),
            current_opcode,
            temp: cpu.temp,
            ane_constant: cpu.ane_constant,
            is_halted: cpu.is_halted,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
pub struct PpuState {
    pub cycle_counter: u128,
    pub master_cycle: u128,
    pub status_register: u8,
    pub ctrl_register: u8,
    pub mask_register: u8,
    pub nmi_requested: bool,
    pub memory: Vec<u8>,
    pub oam_data_register: u8,
    pub ppu_x_scroll_register: u8,
    pub ppu_y_scroll_register: u8,
    pub ppu_addr_register: u16,
    pub ppu_data_register: u8,
    pub oam_addr_register: u8,
    pub write_latch: bool,
    pub oam_dma_register: u8,
    pub ppu_data_buffer: u8,
    pub t_register: u16,
    pub bg_next_tile_id: u8,
    pub bg_next_tile_attribute: u8,
    pub bg_next_tile: u16,
    pub bg_shifter_pattern: u16,
    pub bg_shifter_attribute: u16,
    pub fine_x_scroll: u8,
    pub even_frame: bool,
    pub reset_signal: bool,
    pub pixel_buffer: Vec<u32>,
    pub dot: u16,
    pub scanline: u16,
}

impl From<&Ppu> for PpuState {
    fn from(ppu: &Ppu) -> Self {
        Self {
            cycle_counter: ppu.dot_counter,
            master_cycle: ppu.master_cycle,
            status_register: ppu.status_register,
            ctrl_register: ppu.ctrl_register,
            mask_register: ppu.mask_register,
            nmi_requested: ppu.nmi_requested.get(),
            memory: ppu.memory.get_memory_debug(Some(0x0..=0x3FFF)),
            oam_data_register: ppu.oam_data_register,
            ppu_x_scroll_register: ppu.ppu_x_scroll_register,
            ppu_y_scroll_register: ppu.ppu_y_scroll_register,
            ppu_addr_register: ppu.vram_addr_register,
            ppu_data_register: ppu.ppu_data_register,
            oam_addr_register: ppu.oam_addr_register,
            write_latch: ppu.write_latch,
            oam_dma_register: ppu.oam_dma_register,
            ppu_data_buffer: ppu.ppu_data_buffer,
            t_register: ppu.t_register,
            bg_next_tile_id: ppu.bg_next_tile_id,
            bg_next_tile_attribute: ppu.bg_next_tile_attribute,
            bg_next_tile: ppu.bg_next_tile,
            bg_shifter_pattern: ppu.bg_shifter_pattern,
            bg_shifter_attribute: ppu.bg_shifter_attribute,
            fine_x_scroll: ppu.fine_x_scroll,
            even_frame: ppu.even_frame,
            reset_signal: ppu.reset_signal,
            pixel_buffer: vec![0u32; (WIDTH * HEIGHT) as usize],
            dot: ppu.dot,
            scanline: ppu.scanline,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
pub struct SaveState {
    pub cpu: CpuState,
    pub ppu: PpuState,
    pub cycles: u128,
    pub rom_file: RomFile,
    pub version: u16,
}

pub fn save_state(state: SaveState, path: &str) {
    let serialized =
        bincode::encode_to_vec(state, config::standard()).expect("Failed to serialize SaveState");
    std::fs::write(path, &serialized).expect("Failed to write save file");
}

pub fn load_state(path: &str) -> SaveState {
    let encoded = std::fs::read(path).expect("Failed to read save file");

    let (decoded, _): (SaveState, _) = bincode::decode_from_slice(&encoded, config::standard())
        .expect("Failed to deserialize SaveState");
    decoded
}
