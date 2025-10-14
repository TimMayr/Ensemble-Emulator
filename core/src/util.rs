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
    //    0b0000_0000_0001_1111 << 5 == 0b0000_0011_1110_0000
    // 4. 0b1010_1000_0000_1010 | 0b0000_0011_1110_0000 == 0b1010_1011_1110_1010
    (packed & !mask) | (((val & val_mask) as u16) << (mask.bit_width() - mask.count_ones()))
}

pub fn set_packed_u16(packed: &u16, val: &u16, mask: &u16, val_mask: &u16) -> u16 {
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
    //    0b0000_0000_0001_1111 << 5 == 0b0000_0011_1110_0000
    // 4. 0b1010_1000_0000_1010 | 0b0000_0011_1110_0000 == 0b1010_1011_1110_1010
    (packed & !mask) | ((val & val_mask) << (mask.bit_width() - mask.count_ones()))
}

pub fn crosses_page_boundary_u8(base: u16, offset: u8) -> bool {
    (base & UPPER_BYTE) != ((base + offset as u16) & UPPER_BYTE)
}

pub fn crosses_page_boundary_i8(base: u16, offset: i8) -> bool {
    let target = base.wrapping_add(offset as i16 as u16);
    (base & UPPER_BYTE) != (target & UPPER_BYTE)
}

pub fn add_to_low_byte(val: u16, add: u8) -> u16 {
    let high = val & 0xFF00; // preserve high byte
    let low = ((val & 0x00FF) as u8).wrapping_add(add); // add with wrapping
    high | low as u16
}
