mod formats;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::emulation::mem::{Memory, MemoryDevice, Ram, Rom};
use crate::emulation::rom::formats::archaic_ines::ArchaicInes;
use crate::emulation::rom::formats::ines::Ines;
use crate::emulation::rom::formats::ines_07::Ines07;
use crate::emulation::rom::formats::ines2::Ines2;

#[derive(Debug, Clone)]
pub enum ParseError {
    SizeBiggerThanFile,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::SizeBiggerThanFile => {
                write!(f,
                       "Rom sizes specified in header are larger than total rom size")
            }
        }
    }
}

impl Error for ParseError {}

pub trait RomParser: Debug {
    fn parse(&self, rom: &[u8]) -> Result<RomFile, ParseError>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct RomFile {
    pub prg_memory: PrgMemory,
    pub chr_memory: ChrMemory,
    pub mapper_number: u16,
    pub default_expansion_device: u8,
    pub misc_rom_count: u8,
    pub extended_console_type: Option<u8>,
    pub vs_system_hardware_type: Option<u8>,
    pub vs_system_ppu_type: Option<u8>,
    pub cpu_ppu_timing: u8,
    pub console_type: u8,
    pub hardwired_nametable_layout: bool,
    pub is_battery_backed: bool,
    pub trainer_present: bool,
    pub alternative_nametables: bool,
    pub submapper_number: u8,
    #[serde(skip)]
    data: Vec<u8>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PrgMemory {
    pub prg_rom_size: u32,
    pub prg_ram_size: u32,
    pub prg_nvram_size: u32,
}

impl PrgMemory {
    fn new(prg_rom_size: u32, prg_ram_size: u32, prg_nvram_size: u32) -> PrgMemory {
        Self { prg_rom_size,
               prg_nvram_size,
               prg_ram_size }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ChrMemory {
    pub chr_rom_size: u32,
    pub chr_ram_size: u32,
    pub chr_nvram_size: u32,
}

impl ChrMemory {
    fn new(chr_rom_size: u32, chr_ram_size: u32, chr_nvram_size: u32) -> ChrMemory {
        Self { chr_rom_size,
               chr_nvram_size,
               chr_ram_size }
    }
}

impl RomFile {
    fn range_all_zeros(arr: &[u8], start: usize, end: usize) -> bool {
        if start > end || end > arr.len() {
            return false;
        }
        arr[start..end].iter().all(|&x| x == 0)
    }

    fn get_rom_type(rom: &[u8]) -> Box<dyn RomParser> {
        if rom.starts_with(&[0x4E, 0x45, 0x53, 0x1A]) {
            let prg_rom_size_lsb = rom[4] as u16;
            let prg_rom_size_msb = (rom[9] & 0xF) as u16;

            let prg_rom_size = Ines2::get_prg_rom_size(prg_rom_size_lsb, prg_rom_size_msb);

            let chr_rom_size_lsb = rom[5] as u16;
            let chr_rom_size_msb = (rom[9] & 0xF0) as u16;

            let chr_rom_size = Ines2::get_chr_rom_size(chr_rom_size_lsb, chr_rom_size_msb);

            if rom[7] & 0b00001100 == 8
               && (prg_rom_size as usize + chr_rom_size as usize) < rom.len()
            {
                return Box::new(Ines2);
            }

            if rom[7] & 0b00001100 == 4 {
                return Box::new(ArchaicInes);
            }

            if rom[7] & 0b00001100 == 0 && Self::range_all_zeros(rom, 12, 16) {
                return Box::new(Ines);
            }

            return Box::new(Ines07);
        }

        panic!("Romtype not yet implemented")
    }

    pub fn load(path: &String) -> RomFile {
        let path = Path::new(&path);
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("Couldn't read file {}: {}", path.display(), e),
        };

        let mut rom: Vec<u8> = Vec::new();
        file.read_to_end(&mut rom).expect("Couldn't read file");

        let rom_type = RomFile::get_rom_type(&rom);
        let mut rom_file = rom_type.parse(&rom).expect("Error loading Rom");
        rom_file.data = rom;
        rom_file
    }

    pub fn get_prg_rom(&self) -> Memory {
        let mut rom = Rom::new(self.prg_memory.prg_rom_size as usize);

        let mut start = 16usize;

        if self.trainer_present {
            start += 512;
        }

        rom.load(
            self.data[start..start + self.prg_memory.prg_rom_size as usize]
                .to_vec()
                .into_boxed_slice(),
        );
        Memory::Rom(rom)
    }

    pub fn get_chr_rom(&self) -> Option<Memory> {
        if self.chr_memory.chr_rom_size == 0 {
            return None;
        }

        let mut rom = Rom::new(self.chr_memory.chr_rom_size as usize);

        let mut start = 16usize;

        if self.trainer_present {
            start += 512;
        }

        rom.load(self.data[start + self.prg_memory.prg_rom_size as usize
                           ..start
                             + self.prg_memory.prg_rom_size as usize
                             + self.chr_memory.chr_rom_size as usize]
                                                                     .to_vec()
                                                                     .into_boxed_slice());
        Some(Memory::Rom(rom))
    }

    pub fn get_prg_ram(&self) -> Memory {
        let mut ram = Ram::new(self.prg_memory.prg_ram_size as usize);

        let mut start = 16usize;

        if self.trainer_present {
            start += 512;
        }

        ram.load(
            self.data[start..start + self.prg_memory.prg_rom_size as usize]
                .to_vec()
                .into_boxed_slice(),
        );
        Memory::Ram(ram)
    }
}

impl From<&String> for RomFile {
    fn from(path: &String) -> Self { RomFile::load(path) }
}

impl From<&RomFile> for RomFile {
    fn from(rom: &RomFile) -> Self { rom.clone() }
}

pub struct RomBuilder {
    prg_rom_size: u32,
    chr_rom_size: u32,
    mapper_number: u16,
    default_expansion_device: u8,
    misc_rom_count: u8,
    extended_console_type: Option<u8>,
    vs_system_hardware_type: Option<u8>,
    vs_system_ppu_type: Option<u8>,
    cpu_ppu_timing: u8,
    chr_nvram_size: u32,
    chr_ram_size: u32,
    prg_nvram_size: u32,
    prg_ram_size: u32,
    console_type: u8,
    hardwired_nametable_layout: bool,
    is_battery_backed: bool,
    trainer_present: bool,
    alternative_nametables: bool,
    submapper_number: u8,
}

impl Default for RomBuilder {
    fn default() -> Self {
        Self { prg_rom_size: 0,
               chr_rom_size: 0,
               mapper_number: 0,
               default_expansion_device: 0,
               misc_rom_count: 0,
               extended_console_type: None,
               vs_system_hardware_type: None,
               vs_system_ppu_type: None,
               cpu_ppu_timing: 0,
               chr_nvram_size: 0,
               chr_ram_size: 0,
               prg_nvram_size: 0,
               prg_ram_size: 8 * 1024,
               console_type: 0,
               hardwired_nametable_layout: false,
               is_battery_backed: false,
               trainer_present: false,
               alternative_nametables: false,
               submapper_number: 0 }
    }
}

impl RomBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn prg_rom_size(mut self, size: u32) -> Self {
        self.prg_rom_size = size;
        self
    }

    pub fn chr_rom_size(mut self, size: u32) -> Self {
        self.chr_rom_size = size;
        self
    }

    pub fn mapper_number(mut self, number: u16) -> Self {
        self.mapper_number = number;
        self
    }

    pub fn default_expansion_device(mut self, device: u8) -> Self {
        self.default_expansion_device = device;
        self
    }

    pub fn misc_rom_count(mut self, count: u8) -> Self {
        self.misc_rom_count = count;
        self
    }

    pub fn extended_console_type(mut self, console_type: Option<u8>) -> Self {
        self.extended_console_type = console_type;
        self
    }

    pub fn vs_system_hardware_type(mut self, hardware_type: Option<u8>) -> Self {
        self.vs_system_hardware_type = hardware_type;
        self
    }

    pub fn vs_system_ppu_type(mut self, ppu_type: Option<u8>) -> Self {
        self.vs_system_ppu_type = ppu_type;
        self
    }

    pub fn cpu_ppu_timing(mut self, timing: u8) -> Self {
        self.cpu_ppu_timing = timing;
        self
    }

    pub fn chr_nvram_size(mut self, size: u32) -> Self {
        self.chr_nvram_size = size;
        self
    }

    pub fn chr_ram_size(mut self, size: u32) -> Self {
        self.chr_ram_size = size;
        self
    }

    pub fn prg_nvram_size(mut self, size: u32) -> Self {
        self.prg_nvram_size = size;
        self
    }

    pub fn prg_ram_size(mut self, size: u32) -> Self {
        self.prg_ram_size = size;
        self
    }

    pub fn console_type(mut self, console_type: u8) -> Self {
        self.console_type = console_type;
        self
    }

    pub fn hardwired_nametable_layout(mut self, value: bool) -> Self {
        self.hardwired_nametable_layout = value;
        self
    }

    pub fn battery_backed(mut self, value: bool) -> Self {
        self.is_battery_backed = value;
        self
    }

    pub fn trainer_present(mut self, value: bool) -> Self {
        self.trainer_present = value;
        self
    }

    pub fn alternative_nametables(mut self, value: bool) -> Self {
        self.alternative_nametables = value;
        self
    }

    pub fn submapper_number(mut self, number: u8) -> Self {
        self.submapper_number = number;
        self
    }

    pub fn build(self) -> RomFile {
        RomFile { prg_memory: PrgMemory::new(self.prg_rom_size,
                                             self.prg_ram_size,
                                             self.prg_nvram_size),
                  chr_memory: ChrMemory::new(self.chr_rom_size,
                                             self.chr_ram_size,
                                             self.chr_nvram_size),
                  mapper_number: self.mapper_number,
                  default_expansion_device: self.default_expansion_device,
                  misc_rom_count: self.misc_rom_count,
                  extended_console_type: self.extended_console_type,
                  vs_system_hardware_type: self.vs_system_hardware_type,
                  vs_system_ppu_type: self.vs_system_ppu_type,
                  cpu_ppu_timing: self.cpu_ppu_timing,
                  console_type: self.console_type,
                  hardwired_nametable_layout: self.hardwired_nametable_layout,
                  is_battery_backed: self.is_battery_backed,
                  trainer_present: self.trainer_present,
                  alternative_nametables: self.alternative_nametables,
                  submapper_number: self.submapper_number,
                  data: Vec::new() }
    }
}

pub trait RomFileConvertible {
    fn as_rom_file(&self) -> RomFile;
}

impl RomFileConvertible for String {
    fn as_rom_file(&self) -> RomFile { RomFile::from(self) }
}

impl RomFileConvertible for RomFile {
    fn as_rom_file(&self) -> RomFile { RomFile::from(self) }
}
