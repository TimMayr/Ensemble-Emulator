use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::rom::RomFile;
use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
pub struct CpuState {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub processor_status: u8,
}

impl From<&Cpu> for CpuState {
    fn from(cpu: &Cpu) -> Self {
        Self {
            program_counter: cpu.program_counter,
            stack_pointer: cpu.stack_pointer,
            accumulator: cpu.accumulator,
            x_register: cpu.x_register,
            y_register: cpu.y_register,
            processor_status: cpu.processor_status,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
pub struct PpuState {
    pub cycle_counter: u64,
    pub status: u8,
    pub ctrl: u8,
    pub mask_register: u8,
    pub nmi_requested: bool,
}

impl From<&Ppu> for PpuState {
    fn from(ppu: &Ppu) -> Self {
        Self {
            cycle_counter: ppu.cycle_counter,
            status: ppu.status,
            ctrl: ppu.ctrl,
            mask_register: ppu.mask,
            nmi_requested: ppu.nmi_requested.get(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
pub struct SaveState {
    pub cpu: CpuState,
    pub ppu: PpuState,
    pub memory: Vec<u8>, // PRG RAM + Work RAM
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
