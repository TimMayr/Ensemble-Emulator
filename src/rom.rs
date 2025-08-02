use crate::rom::RomType::INES2;
use std::fs::File;
use std::io::Read;
use std::path::Path;

enum RomType {
    INES2,
    INES,
}

pub struct Rom {
    rom_type: RomType,
    prg_rom_size: u32,
    chr_rom_size: u32,
}

impl Rom {
    fn get_rom_type(rom: &Vec<u8>) -> RomType {
        if rom.starts_with(&[0x4E, 0x45, 0x53, 0x1A]) {
            if rom[7] & 0b00001000 != 0 && rom[7] & 0b00000100 == 0 {
                return INES2;
            }
        }

        panic!("Romtype not yet implemented")
    }
}

impl Rom {
    pub fn load(path: String) {
        let path = Path::new(&path);
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("Couldn't read file {}: {}", path.display(), e),
        };

        let mut rom: Vec<u8> = Vec::new();
        file.read_to_end(&mut rom).expect("Couldn't read file");
        // let len = read.len().min(crate::cpu::MEMORY_SIZE as usize);
        // self.memory[..len].copy_from_slice(&read[..len])

        let rom_type = Rom::get_rom_type(&rom);
        let prg_rom_size_lsb = rom[4];
        let chr_rom_size_lsb = rom[5];

        let mapper_number = rom[6] >> 4 | (rom[7] & 0x15) | (rom[8] & 0x15) << 8;

        let alternative_nametables = rom[6] & 0b00001000 != 0;
        let trainer_present = rom[6] & 0b00000100 != 0;
        let battery_and_stuff_present = rom[6] & 0b00000010 != 0;
        let hard_wired_nametable_layout = rom[6] & 0b0000001 != 0;

        let console_type = rom[7] & 0b00000011;
    }
}
