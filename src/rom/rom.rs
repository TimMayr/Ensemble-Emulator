use crate::rom::formats::archaic_ines::ArchaicInes;
use crate::rom::formats::ines::Ines;
use crate::rom::formats::ines2::Ines2;
use crate::rom::formats::ines_07::Ines07;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum ParseError {
    SizeBiggerThanFile,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::SizeBiggerThanFile => {
                write!(
                    f,
                    "Rom sizes specified in header are larger than total rom size"
                )
            }
        }
    }
}

impl Error for ParseError {}

pub trait RomParser: Debug {
    fn parse(&self, rom: &[u8]) -> Result<Rom, ParseError>;
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Rom {
    prg_rom_size: u32,
    chr_rom_size: u32,
    mapper_number: u16,
    default_expansion_device: u8,
    no_misc_roms: u8,
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
    battery_and_stuff_present: bool,
    trainer_present: bool,
    alternative_nametables: bool,
    submapper_number: u8,
}

impl Rom {
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

    pub fn load(path: String) -> Rom {
        let path = Path::new(&path);
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("Couldn't read file {}: {}", path.display(), e),
        };

        let mut rom: Vec<u8> = Vec::new();
        file.read_to_end(&mut rom).expect("Couldn't read file");

        let rom_type = Rom::get_rom_type(&rom);
        println!("Rom Type: {rom_type:?}");
        rom_type.parse(&rom).expect("Error loading Rom")
    }

    pub fn new(
        prg_rom_size: u32,
        chr_rom_size: u32,
        mapper_number: u16,
        submapper_number: u8,
        alternative_nametables: bool,
        trainer_present: bool,
        battery_and_stuff_present: bool,
        hardwired_nametable_layout: bool,
        console_type: u8,
        prg_ram_size: u32,
        prg_nvram_size: u32,
        chr_ram_size: u32,
        chr_nvram_size: u32,
        cpu_ppu_timing: u8,
        vs_system_ppu_type: Option<u8>,
        vs_system_hardware_type: Option<u8>,
        extended_console_type: Option<u8>,
        no_misc_roms: u8,
        default_expansion_device: u8,
    ) -> Rom {
        Self {
            prg_rom_size,
            chr_rom_size,
            mapper_number,
            submapper_number,
            alternative_nametables,
            trainer_present,
            battery_and_stuff_present,
            hardwired_nametable_layout,
            console_type,
            prg_ram_size,
            prg_nvram_size,
            chr_ram_size,
            chr_nvram_size,
            cpu_ppu_timing,
            vs_system_ppu_type,
            vs_system_hardware_type,
            extended_console_type,
            no_misc_roms,
            default_expansion_device,
        }
    }
}
