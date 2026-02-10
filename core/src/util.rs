use crate::emulation::cpu::UPPER_BYTE;
use crate::emulation::mem::{Memory, MemoryDevice};
use crate::emulation::savestate::SaveState;

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

pub trait Hashable {
    fn hash(&self) -> u64;
}

pub trait ToBytes {
    fn to_bytes(&self, format: Option<String>) -> Vec<u8>;
}

impl ToBytes for SaveState {
    fn to_bytes(&self, format: Option<String>) -> Vec<u8> {
        if let Some(format) = format {
            match format.as_str() {
                "json" => serde_json::to_vec_pretty(self).expect("Failed to serialize SaveState"),
                _ => bincode::serde::encode_to_vec(self, bincode::config::standard())
                    .expect("Failed to serialize SaveState"),
            }
        } else {
            bincode::serde::encode_to_vec(self, bincode::config::standard())
                .expect("Failed to serialize SaveState")
        }
    }
}

impl Hashable for Memory {
    fn hash(&self) -> u64 { compute_hash(&self.snapshot_all()) }
}

impl Hashable for Vec<u8> {
    fn hash(&self) -> u64 { compute_hash(self) }
}

/// Compute a fast hash of the given data for change detection.
/// Uses FNV-1a algorithm which is fast and has good distribution.
#[inline]
pub fn compute_hash(data: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xCBF29CE484222325;
    const FNV_PRIME: u64 = 0x100000001B3;

    let mut hash = FNV_OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
