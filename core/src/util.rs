use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

use crate::emulation::cpu::UPPER_BYTE;

pub fn write_at_offset(path: &str, value: u8, offset: u16) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;

    // Seek to 0xFFFC (65532 bytes)
    file.seek(SeekFrom::Start(offset as u64))?;

    // Write the byte
    file.write_all(&[value])?;

    Ok(())
}

pub fn write_to_file(path: &str, data: Vec<u8>) -> Result<(), String> {
    let file = File::create(path);

    match file {
        Ok(mut file) => {
            let res = file.write_all(&data);
            match res {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error writing to file: {}\n\t{}", path, e)),
            }
        }
        Err(e) => Err(format!("Error creating file: {}\n\t{}", path, e)),
    }
}

#[inline(always)]
pub fn crosses_page_boundary_u8(base: u16, offset: u8) -> bool {
    (base & UPPER_BYTE) != ((base + offset as u16) & UPPER_BYTE)
}

#[inline(always)]
pub fn crosses_page_boundary_i8(base: u16, offset: i8) -> bool {
    let target = base.wrapping_add(offset as i16 as u16);
    (base & UPPER_BYTE) != (target & UPPER_BYTE)
}

#[inline(always)]
pub fn add_to_low_byte(val: u16, add: u8) -> u16 {
    let high = val & 0xFF00; // preserve high byte
    let low = ((val & 0x00FF) as u8).wrapping_add(add); // add with wrapping
    high | low as u16
}
