use crate::emulation::cpu::{Cpu, INTERNAL_RAM_SIZE};
use crate::emulation::mem::{Memory, MemoryDevice, OpenBus, Ram};
use crate::emulation::mem::mirror_memory::MirrorMemory;
use crate::emulation::ppu::{Ppu, OPEN_BUS_DECAY_DELAY, PALETTE_RAM_END_ADDRESS, PALETTE_RAM_START_ADDRESS, VRAM_SIZE};

pub struct Board {
    cpu: Cpu,
    ppu: Ppu,
    cpu_ram: Box<Ram>,
    cartridge: Box<[u8]>,
    cpu_open_bus: OpenBus,
    ppu_open_bus: OpenBus,
}

#[derive(Debug, Clone, Copy)]
pub enum BusMaster {
    Cpu,
    Ppu,
}

#[derive(Debug, Clone, Copy)]
pub struct BusRead {
    pub master: BusMaster,
    pub addr: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct BusWrite {
    pub master: BusMaster,
    pub addr: u16,
    pub data: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct ReadResult {
    pub data: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum Signal {
    NmiRise,
    NmiFall,
    IrqAssert,
    IrqClear,
    DmaStart { page: u8 },
}

#[derive(Debug, Default, Clone)]
pub struct SignalLatch {
    pub pending: Vec<Signal>,
}

impl SignalLatch {
    pub fn push(&mut self, signal: Signal) { self.pending.push(signal) }

    pub fn drain(&mut self) -> impl Iterator<Item=Signal> { self.pending }
}

pub trait CpuBus {
    fn read(&mut self, addr: u16) -> ReadResult;
    fn write(&mut self, addr: u16, data: u8);
}

pub trait PpuBus {
    fn read(&mut self, addr: u16) -> ReadResult;
    fn write(&mut self, addr: u16, data: u8);
}

impl CpuBus for Board {
    fn read(&mut self, addr: u16) -> ReadResult {
        match addr {
            0x0..=0x1FFF => ReadResult {
                data: self.cpu_ram.read(addr, 0),
            },
            0x2000..=0x3FFF => ReadResult {
                data: self.read_ppu_reg(addr, 0),
            },
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0..=0x1FFF => {
                self.cpu_ram.write(addr, data);
            }
            0x2000..=0x3FFF => {
                self.write_ppu_reg(addr, data);
            }
        }
    }
}

impl PpuBus for Board {
    fn read(&mut self, addr: u16) -> ReadResult {

    }

    fn write(&mut self, addr: u16, data: u8) {
        
    }
}

impl Board {
    pub fn init(&mut self) {
        self.cpu_ram = Box::new(Ram::new(INTERNAL_RAM_SIZE));
        self.ppu_open_bus = OpenBus::new(OPEN_BUS_DECAY_DELAY)
    }

    #[inline]
    fn read_ppu_reg(&mut self, addr: u16, _: u8) -> u8 {
        match addr {
            0x2 => {
                let val = self.ppu.get_ppu_status();
                self.ppu_open_bus.set_masked(val, 0b1110_0000);
            }
            0x4 => {
                let val = self.ppu.get_oam_at_addr();
                self.ppu_open_bus.set_masked(val, 0xFF);
            }
            0x7 => {
                let val = self.ppu.get_vram_at_addr();

                match self.ppu.v_register {
                    PALETTE_RAM_START_ADDRESS..=PALETTE_RAM_END_ADDRESS => {
                        self.ppu_open_bus.set_masked(val, 0b0011_1111);
                    }
                    _ => {
                        self.ppu_open_bus.set_masked(val, 0xFF);
                    }
                }
            }
            _ => {}
        };

        self.ppu_open_bus.read()
    }

    #[inline]
    fn write_ppu_reg(&mut self, addr: u16, data: u8) {
        self.ppu_open_bus.set_masked(data, 0xFF);
        match addr {
            0x0 => {
                self.ppu.set_ppu_ctrl(data);
            }
            0x1 => {
                self.ppu.set_mask_register(data);
            }
            0x3 => {
                self.ppu.set_oam_addr_register(data);
            }
            0x4 => {
                self.ppu.write_oam(data);
            }
            0x5 => {
                self.ppu.write_ppu_scroll(data);
            }
            0x6 => {
                self.ppu.write_vram_addr(data);
            }
            0x7 => {
                self.ppu.write_vram(data);
            }
            _ => (),
        };
    }
}
