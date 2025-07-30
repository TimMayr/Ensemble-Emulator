use crate::cpu::AddressingMode::Immediate;
use crate::opcode;
use crate::opcode::{OpCode, OPCODES_MAP};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const MEMORY_SIZE: u16 = 0xFFFF;
const STACK_START: u8 = 0xFF;

#[derive(Debug, Eq, PartialEq)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MathematicalOperation {
    Add,
    Sub,
}

#[derive(Debug, Copy, Clone)]
pub struct Cpu {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub processor_status: u8,
    pub memory: [u8; MEMORY_SIZE as usize],
}

impl Default for Cpu {
    fn default() -> Self {
        let memory: [u8; MEMORY_SIZE as usize] = [0; MEMORY_SIZE as usize];
        Self {
            program_counter: 0,
            processor_status: 0b00100000,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            memory,
            stack_pointer: STACK_START,
        }
    }
}

#[allow(dead_code)]
impl Cpu {
    pub fn new() -> Self {
        OPCODES_MAP.get_or_init(opcode::init);
        Self::default()
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let least_significant_bits = self.mem_read(addr) as u16;
        let highest_significant_bits = self.mem_read(addr + 1) as u16;

        (highest_significant_bits << 8) | (least_significant_bits)
    }

    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let least_significant_bits = (data & 0x00FF) as u8;
        let highest_significant_bits = (data >> 8) as u8;
        self.mem_write(addr, least_significant_bits);
        self.mem_write(addr + 1, highest_significant_bits)
    }

    fn set_zero_flag(&mut self) {
        self.processor_status |= 0b0000_0010;
    }

    fn clear_zero_flag(&mut self) {
        self.processor_status &= 0b1111_1101;
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.set_zero_flag()
        } else {
            self.clear_zero_flag()
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            self.processor_status |= 0b1000_0000;
        } else {
            self.processor_status &= 0b0111_1111;
        }
    }

    fn update_negative_and_zero_flags(&mut self, result: u8) {
        self.update_negative_flag(result);
        self.update_zero_flag(result);
    }

    fn set_carry_flag(&mut self) {
        self.processor_status |= 0b0000_0001;
    }

    fn clear_carry_flag(&mut self) {
        self.processor_status &= 0b1111_1110;
    }

    fn set_overflow_flag(&mut self) {
        self.processor_status |= 0b0100_0000;
    }

    fn clear_overflow_flag(&mut self) {
        self.processor_status &= 0b1011_1111;
    }

    pub fn get_zero_flag(self) -> bool {
        (self.processor_status & 0b0000_0010) == 0b0000_0010
    }

    pub fn get_negative_flag(self) -> bool {
        (self.processor_status & 0b1000_0000) == 0b1000_0000
    }

    fn get_carry_flag(&self) -> bool {
        (self.processor_status & 0b0000_0001) == 0b0000_0001
    }

    pub fn get_overflow_flag(&self) -> bool {
        (self.processor_status & 0b0100_0000) == 0b0100_0000
    }

    fn update_carry_and_overflow_flags(&mut self, result: Option<u8>, op: MathematicalOperation) {
        match result {
            Some(_e) => {
                self.clear_overflow_flag();
                if op == MathematicalOperation::Sub {
                    self.set_carry_flag();
                } else {
                    self.clear_carry_flag();
                }
            }
            None => {
                self.set_overflow_flag();
                if op == MathematicalOperation::Sub {
                    self.clear_carry_flag();
                } else {
                    self.set_carry_flag();
                }
            }
        }
    }

    fn get_operand_address(&self, addressing_mode: &AddressingMode) -> u16 {
        match addressing_mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.read_next_byte() as u16,
            AddressingMode::ZeroPageX => {
                let pos = self.read_next_byte();
                pos.wrapping_add(self.x_register) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos = self.read_next_byte();
                pos.wrapping_add(self.y_register) as u16
            }
            AddressingMode::Relative => self.program_counter,
            AddressingMode::Absolute => self.read_next_two_bytes(),
            AddressingMode::AbsoluteX => {
                let pos = self.read_next_two_bytes();
                pos + self.x_register as u16
            }
            AddressingMode::AbsoluteY => {
                let pos = self.read_next_two_bytes();
                pos + self.y_register as u16
            }
            AddressingMode::IndirectX => {
                let base = self.read_next_byte();
                let lookup_addr = base.wrapping_add(self.x_register);
                self.get_indirect_lookup(lookup_addr as u16)
            }
            AddressingMode::IndirectY => {
                let lookup_addr = self.read_next_byte();
                let addr = self.get_indirect_lookup(lookup_addr as u16);
                addr.wrapping_add(self.y_register as u16)
            }
            _ => panic!("Invalid addressing mode"),
        }
    }

    fn read_next_byte(&self) -> u8 {
        self.mem_read(self.program_counter)
    }

    fn read_next_two_bytes(&self) -> u16 {
        self.mem_read_u16(self.program_counter)
    }

    fn get_indirect_lookup(&self, addr: u16) -> u16 {
        let lsb = self.mem_read(addr);
        let hsb = self.mem_read(addr.wrapping_add(1));

        (hsb as u16) << 8 | (lsb as u16)
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_val = self.mem_read(target);
        self.accumulator = target_val;
        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_val = self.mem_read(target);
        self.x_register = target_val;
        self.update_negative_and_zero_flags(self.x_register);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_val = self.mem_read(target);
        self.y_register = target_val;
        self.update_negative_and_zero_flags(self.y_register);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        self.mem_write(target, self.accumulator);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        self.mem_write(target, self.x_register);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        self.mem_write(target, self.y_register);
    }

    fn tax(&mut self) {
        self.x_register = self.accumulator;
        self.update_negative_and_zero_flags(self.x_register);
    }

    fn tay(&mut self) {
        self.y_register = self.accumulator;
        self.update_negative_and_zero_flags(self.y_register);
    }

    fn txa(&mut self) {
        self.accumulator = self.x_register;
        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn tya(&mut self) {
        self.accumulator = self.y_register;
        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn pha(&mut self) {
        self.mem_write(0x0100u16 + self.stack_pointer as u16, self.accumulator);
        self.stack_pointer -= 1;
    }

    fn php(&mut self) {
        self.mem_write(0x0100u16 + self.stack_pointer as u16, self.processor_status);
        self.stack_pointer -= 1;
    }

    fn pla(&mut self) {
        self.stack_pointer += 1;
        self.accumulator = self.mem_read(0x0100u16 + self.stack_pointer as u16);
        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn plp(&mut self) {
        self.stack_pointer += 1;
        self.processor_status = self.mem_read(0x0100u16 + self.stack_pointer as u16);
    }

    fn tsx(&mut self) {
        self.x_register = self.stack_pointer;
        self.update_negative_and_zero_flags(self.x_register)
    }

    fn txs(&mut self) {
        self.stack_pointer = self.x_register;
    }

    fn and(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_val = self.mem_read(target);
        self.accumulator &= target_val;
        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_val = self.mem_read(target);
        self.accumulator ^= target_val;
        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_val = self.mem_read(target);
        self.accumulator |= target_val;
        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_val = self.mem_read(target);
        let res = self.accumulator & target_val;
        self.update_zero_flag(res);
        self.processor_status |= target_val & 0b11000000
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_value = self.mem_read(target);
        let (mod_value, _) = target_value.overflowing_add(1);
        self.mem_write(target, mod_value);
        self.update_negative_and_zero_flags(mod_value);
    }

    fn inx(&mut self) {
        let (mod_value, _) = self.x_register.overflowing_add(1);
        self.x_register = mod_value;
        self.update_negative_and_zero_flags(self.x_register);
    }

    fn iny(&mut self) {
        let (mod_value, _) = self.y_register.overflowing_add(1);
        self.y_register = mod_value;
        self.update_negative_and_zero_flags(self.y_register);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_value = self.mem_read(target);
        let (mod_value, _) = target_value.overflowing_sub(1);
        self.mem_write(target, mod_value);
        self.update_negative_and_zero_flags(mod_value);
    }

    fn dex(&mut self) {
        let (mod_value, _) = self.x_register.overflowing_sub(1);
        self.x_register = mod_value;
        self.update_negative_and_zero_flags(self.x_register);
    }

    fn dey(&mut self) {
        let (mod_value, _) = self.y_register.overflowing_sub(1);
        self.y_register = mod_value;
        self.update_negative_and_zero_flags(self.y_register);
    }

    pub fn init(&mut self) {
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        let mut cycles = 0u16;
        loop {
            let current_cycles = self.step();

            if current_cycles == 0xFF {
                return;
            }

            cycles += current_cycles as u16;

            if cycles > 29780 {
                cycles = 0;
                sleep(Duration::from_nanos(16_666_666))
            }
        }
    }

    pub fn step(&mut self) -> u8 {
        let opcode = self.mem_read(self.program_counter);
        let prnt = &OpCode::new(0xFF, "PRT", 1, 0, Immediate);
        let op = OPCODES_MAP.get().unwrap().get(&opcode).unwrap_or(&prnt);
        self.program_counter += 1u16;

        match op.opcode {
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&op.addressing_mode),
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&op.addressing_mode),
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&op.addressing_mode),
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&op.addressing_mode),
            0x86 | 0x96 | 0x8E => self.stx(&op.addressing_mode),
            0x84 | 0x94 | 0x8C => self.sty(&op.addressing_mode),
            0xAA => self.tax(),
            0xA8 => self.tay(),
            0x8A => self.txa(),
            0x98 => self.tya(),
            0x48 => self.pha(),
            0x08 => self.php(),
            0x68 => self.pla(),
            0x28 => self.plp(),
            0xBA => self.tsx(),
            0x9A => self.txs(),
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&op.addressing_mode),
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(&op.addressing_mode),
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(&op.addressing_mode),
            0x24 | 0x2C => self.bit(&op.addressing_mode),
            0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&op.addressing_mode),
            0xE8 => self.inx(),
            0xC8 => self.iny(),
            0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&op.addressing_mode),
            0xCA => self.dex(),
            0x88 => self.dey(),
            0xFF => println!("{}", self.accumulator),
            _ => {
                println!("No instruction at address 0x{:x}", self.program_counter - 1);
                return 0xFF;
            }
        }

        self.program_counter += (op.bytes - 1) as u16;
        op.cycles
    }

    pub fn load(&mut self, path: String) {
        let path = Path::new(&path);
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("Couldn't read file {}: {}", path.display(), e),
        };

        let mut read: Vec<u8> = Vec::new();
        file.read_to_end(&mut read).expect("Couldn't read file");
        let len = read.len().min(MEMORY_SIZE as usize);
        self.memory[..len].copy_from_slice(&read[..len])
    }
}
