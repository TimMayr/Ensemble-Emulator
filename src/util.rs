use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

pub fn write_at_offset(path: &str, value: u8, offset: u16) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

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
