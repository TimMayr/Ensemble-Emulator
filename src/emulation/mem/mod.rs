use crate::emulation::mem::mirror_memory::MirrorMemory;
use crate::emulation::mem::palette_ram::PaletteRam;
use crate::emulation::mem::ppu_registers::PpuRegisters;
use std::fmt::{Debug, Formatter};

pub mod memory_map;
pub mod mirror_memory;
pub mod palette_ram;
pub mod ppu_registers;

pub enum Memory {
    Ram(Ram),
    Rom(Rom),
    MirrorMemory(MirrorMemory),
    PaletteRam(PaletteRam),
    PpuRegisters(PpuRegisters),
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Memory::Ram(ram) => ram.fmt(f),
            Memory::Rom(rom) => rom.fmt(f),
            Memory::MirrorMemory(mirror_memory) => mirror_memory.fmt(f),
            Memory::PaletteRam(palette_ram) => palette_ram.fmt(f),
            Memory::PpuRegisters(ppu_registers) => ppu_registers.fmt(f),
        }
    }
}

impl MemoryDevice for Memory {
    fn read(&self, addr: u16) -> u8 {
        match self {
            Memory::Ram(ram) => ram.read(addr),
            Memory::Rom(rom) => rom.read(addr),
            Memory::MirrorMemory(mirror_memory) => mirror_memory.read(addr),
            Memory::PaletteRam(palette_ram) => palette_ram.read(addr),
            Memory::PpuRegisters(ppu_registers) => ppu_registers.read(addr),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match self {
            Memory::Ram(ram) => ram.write(addr, data),
            Memory::Rom(rom) => rom.write(addr, data),
            Memory::MirrorMemory(mirror_memory) => mirror_memory.write(addr, data),
            Memory::PaletteRam(palette_ram) => palette_ram.write(addr, data),
            Memory::PpuRegisters(ppu_registers) => ppu_registers.write(addr, data),
        }
    }

    fn init(&mut self, addr: u16, data: u8) {
        match self {
            Memory::Ram(ram) => ram.init(addr, data),
            Memory::Rom(rom) => rom.init(addr, data),
            Memory::MirrorMemory(mirror_memory) => mirror_memory.init(addr, data),
            Memory::PaletteRam(palette_ram) => palette_ram.init(addr, data),
            Memory::PpuRegisters(ppu_registers) => ppu_registers.init(addr, data),
        }
    }

    fn load(&mut self, data: Box<[u8]>) {
        match self {
            Memory::Ram(ram) => ram.load(data),
            Memory::Rom(rom) => rom.load(data),
            Memory::MirrorMemory(mirror_memory) => mirror_memory.load(data),
            Memory::PaletteRam(palette_ram) => palette_ram.load(data),
            Memory::PpuRegisters(ppu_registers) => ppu_registers.load(data),
        }
    }

    fn snapshot(&self, addr: u16) -> u8 {
        match self {
            Memory::Ram(ram) => ram.snapshot(addr),
            Memory::Rom(rom) => rom.snapshot(addr),
            Memory::MirrorMemory(mirror_memory) => mirror_memory.snapshot(addr),
            Memory::PaletteRam(palette_ram) => palette_ram.snapshot(addr),
            Memory::PpuRegisters(ppu_registers) => ppu_registers.snapshot(addr),
        }
    }
}

pub trait MemoryDevice: Debug {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn init(&mut self, addr: u16, data: u8);
    fn load(&mut self, data: Box<[u8]>);

    fn snapshot(&self, addr: u16) -> u8 {
        self.read(addr)
    }
}

#[derive(Debug, Clone)]
pub struct Ram {
    memory: Box<[u8]>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size].into_boxed_slice(),
        }
    }
}

impl MemoryDevice for Ram {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize % self.memory.len()]
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize % self.memory.len()] = data;
    }

    #[inline(always)]
    fn init(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize % self.memory.len()] = data;
    }

    fn load(&mut self, data: Box<[u8]>) {
        self.memory = data
    }
}

#[derive(Debug, Clone)]
pub struct Rom {
    memory: Box<[u8]>,
}

impl Rom {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size].into_boxed_slice(),
        }
    }
}

impl MemoryDevice for Rom {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize % self.memory.len()]
    }

    #[inline(always)]
    fn write(&mut self, _: u16, _: u8) {}

    #[inline(always)]
    fn init(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize % self.memory.len()] = data;
    }

    fn load(&mut self, data: Box<[u8]>) {
        self.memory = data
    }
}
