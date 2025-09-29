use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

use crate::emulation::cpu::{
    CARRY_BIT, DECIMAL_BIT, IRQ_BIT, NEGATIVE_BIT, OVERFLOW_BIT, UNUSED_BIT, ZERO_BIT,
};
use crate::emulation::nes::Nes;

pub struct TraceLog {
    log: String,
}
impl TraceLog {
    pub fn new() -> Self { Self { log: String::from("") } }

    pub fn trace(&mut self, nes: &Nes) {
        let cpu = &nes.cpu;
        let ppu = nes.ppu.borrow();
        self.log += format!(
            "{:04X}  {} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:3},{:3} CYC:{}\n",
            cpu.program_counter - 1,
            cpu.current_opcode.name,
            cpu.accumulator,
            cpu.x_register,
            cpu.y_register,
            cpu.processor_status | UNUSED_BIT,
            cpu.stack_pointer,
            ppu.dot_counter / 341u128,
            ppu.dot_counter % 341u128,
            cpu.master_cycle / 12 - 1
        )
        .as_str();
    }

    #[allow(dead_code)]
    fn status_as_string(status: u8) -> String {
        let mut str = String::new();

        if status & NEGATIVE_BIT != 0 {
            str += "N"
        } else {
            str += "n"
        }

        if status & OVERFLOW_BIT != 0 {
            str += "V"
        } else {
            str += "v"
        }

        str += "--";

        if status & DECIMAL_BIT != 0 {
            str += "D"
        } else {
            str += "d"
        }

        if status & IRQ_BIT != 0 {
            str += "I"
        } else {
            str += "i"
        }

        if status & ZERO_BIT != 0 {
            str += "Z"
        } else {
            str += "z"
        }

        if status & CARRY_BIT != 0 {
            str += "C"
        } else {
            str += "c"
        }

        str
    }

    pub fn flush(&mut self) {
        let mut file = OpenOptions::new().write(true)
                                         .create(true)
                                         .truncate(true)
                                         .open("./trace-log.txt")
                                         .expect("Error saving log");

        unsafe {
            file.write_all(self.log.as_mut_vec().as_slice())
                .expect("error");
        }
    }
}

pub fn write_at_offset(path: &str, value: u8, offset: u16) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true)
                                     .create(true)
                                     .truncate(false)
                                     .open(path)?;

    // Seek to 0xFFFC (65532 bytes)
    file.seek(SeekFrom::Start(offset as u64))?;

    // Write the byte
    file.write_all(&[value])?;

    Ok(())
}

pub fn write_to_file(path: &str, data: Vec<u8>) {
    let mut file = File::create(path).expect("Error creating file");
    file.write_all(&data).expect("Error writing to file")
}

pub fn set_packed(packed: &u16, val: &u8, mask: &u16, val_mask: &u8) -> u16 {
    // 1. Flip masked bits off
    // 2. Only take masked bits of value
    // 3. Shift them right by the difference between the bits and the ones
    // 4. Or together
    // Example:
    // packed:  0b1010_1010_1010_1010
    // val:               0b0101_1111
    // mask:    0b0000_0011_1110_0000
    // val_mask:          0b0001_1111
    // 1. packed & !mask == 0b1010_1000_0000_1010
    // 2. (val & val_mask) as u16 = 0b0000_0000_0001_1111
    // 3. mask.bit_with() == 10; mask.count_ones() == 5; 10 - 5 == 5;
    //      0b0000_0000_0001_1111 << 5 == 0b0000_0011_1110_0000
    // 4. 0b1010_1000_0000_1010 | 0b0000_0011_1110_0000 == 0b1010_1011_1110_1010
    (packed & !mask) | (((val & val_mask) as u16) << (mask.bit_width() - mask.count_ones()))
}
