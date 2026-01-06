use std::collections::VecDeque;

use rkyv::rancor::BoxedError;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::emulation::cpu::{Cpu, MicroOp};
use crate::emulation::messages::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::emulation::ppu::Ppu;
use crate::emulation::rom::RomFile;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone)]
#[rkyv(serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator,
                        __S::Error: rkyv::rancor::Source))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
pub struct CpuState {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub processor_status: u8,
    pub memory: Vec<u8>,
    pub lo: u8,
    pub hi: u8,
    pub current_op: MicroOp,
    pub op_queue: VecDeque<MicroOp>,
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
            lo: cpu.lo,
            hi: cpu.hi,
            current_op: cpu.current_op,
            op_queue: cpu.op_queue.to_vec_deque(),
            current_opcode,
            temp: cpu.temp,
            ane_constant: cpu.ane_constant,
            is_halted: cpu.is_halted,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone)]
#[rkyv(serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator,
                        __S::Error: rkyv::rancor::Source))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
pub struct PpuState {
    pub cycle_counter: u128,
    pub vbl_reset_counter: u8,
    pub status_register: u8,
    pub ctrl_register: u8,
    pub mask_register: u8,
    pub nmi_requested: bool,
    pub memory: Vec<u8>,
    pub ppu_addr_register: u16,
    pub oam_addr_register: u8,
    pub write_latch: bool,
    pub ppu_data_buffer: u8,
    pub t_register: u16,
    pub bg_next_tile_attribute: u8,
    pub fine_x_scroll: u8,
    pub even_frame: bool,
    pub reset_signal: bool,
    pub pixel_buffer: Vec<u32>,
    pub dot: u16,
    pub scanline: u16,
    pub bg_next_tile_id: u8,
}

impl From<&Ppu> for PpuState {
    fn from(ppu: &Ppu) -> Self {
        Self {
            cycle_counter: ppu.dot_counter,
            vbl_reset_counter: ppu.vbl_reset_counter.get(),
            status_register: ppu.status_register.get(),
            ctrl_register: ppu.ctrl_register,
            mask_register: ppu.mask_register,
            nmi_requested: ppu.nmi_requested.get(),
            memory: ppu.memory.get_memory_debug(Some(0x0..=0x3FFF)),
            ppu_addr_register: ppu.v_register,
            oam_addr_register: ppu.oam_addr_register,
            write_latch: ppu.write_latch.get(),
            ppu_data_buffer: ppu.ppu_data_buffer,
            t_register: ppu.t_register,
            bg_next_tile_id: ppu.bg_next_tile_id,
            bg_next_tile_attribute: ppu.bg_next_tile_attribute,
            fine_x_scroll: ppu.fine_x_scroll,
            even_frame: ppu.even_frame,
            reset_signal: ppu.reset_signal,
            pixel_buffer: vec![0u32; TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT],
            dot: ppu.dot,
            scanline: ppu.scanline,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone)]
#[rkyv(serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator,
                        __S::Error: rkyv::rancor::Source))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
pub struct SaveState {
    pub cpu: CpuState,
    pub ppu: PpuState,
    pub rom_file: RomFile,
    pub version: u16,
    pub total_cycles: u128,
    pub cycle: u8,
}

pub fn save_state(state: SaveState, path: &str) {
    let bytes = rkyv::to_bytes::<BoxedError>(&state).expect("Failed to serialize SaveState");
    std::fs::write(path, &bytes).expect("Failed to write save file");
}

pub fn load_state(path: &str) -> SaveState {
    let encoded = std::fs::read(path).expect("Failed to read save file");
    rkyv::from_bytes::<SaveState, BoxedError>(&encoded).expect("Failed to deserialize SaveState")
}
