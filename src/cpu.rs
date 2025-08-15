use crate::mem::Ram;
use crate::mem::memory_map::MemoryMap;
use crate::mem::mirror_memory::MirrorMemory;
use crate::opcode;
use crate::opcode::{OPCODES_MAP, OpCode};
use crate::ppu::Ppu;
use crate::rom::{RomFile, RomFileConvertible};
use crate::savestate::CpuState;
use std::cell::RefCell;
#[cfg(debug_assertions)]
use std::ops::RangeInclusive;
use std::rc::Rc;

const INTERNAL_RAM_SIZE: u16 = 0x800;
const STACK_START: u8 = 0xFF;

const STACK_START_ADDRESS: u16 = 0x0100;

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

pub struct Cpu {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub processor_status: u8,
    pub memory: Box<MemoryMap>,
    pub ppu: Option<Rc<RefCell<Ppu>>>,
    additional_cycles: u8,
}

impl Default for Cpu {
    fn default() -> Self {
        let mut mem = MemoryMap::default();
        mem.add_memory(
            0x0..=0x1FFF,
            Box::new(MirrorMemory::new(
                Box::new(Ram::new(INTERNAL_RAM_SIZE as usize)),
                0x07FF,
            )),
        );

        mem.add_memory(0x4000..=0x4017, Box::new(Ram::new(0x18)));
        mem.add_memory(0x4018..=0x401F, Box::new(Ram::new(0x8)));

        Self {
            program_counter: 0,
            processor_status: 0b00000000,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            memory: Box::new(mem),
            stack_pointer: STACK_START,
            additional_cycles: 0,
            ppu: None,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        OPCODES_MAP.get_or_init(opcode::init);
        Self::default()
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory.mem_read(addr)
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory.mem_write(addr, data);
    }

    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        self.memory.mem_read_u16(addr)
    }

    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        self.memory.mem_write_u16(addr, data);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.mem_read(STACK_START_ADDRESS + self.stack_pointer as u16)
    }

    pub fn stack_push(&mut self, data: u8) {
        self.mem_write(STACK_START_ADDRESS + self.stack_pointer as u16, data);
        self.stack_pointer -= 1;
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        self.stack_pointer += 2;
        self.mem_read_u16(STACK_START_ADDRESS + self.stack_pointer as u16 - 1)
    }

    pub fn stack_push_u16(&mut self, data: u16) {
        self.mem_write_u16(STACK_START_ADDRESS + self.stack_pointer as u16 - 1, data);
        self.stack_pointer -= 2;
    }

    fn set_zero_flag(&mut self) {
        self.processor_status |= 0b0000_0010;
    }

    fn clear_zero_flag(&mut self) {
        self.processor_status &= 0b1111_1101;
    }

    fn clear_negative_flag(&mut self) {
        self.processor_status &= 0b01111111
    }

    fn set_negative_flag(&mut self) {
        self.processor_status |= 0b10000000
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

    fn set_interrupt_disable(&mut self) {
        self.processor_status |= 0b00000100;
    }

    fn set_decimal_flag(&mut self) {
        self.processor_status |= 0b00001000;
    }

    fn clear_interrupt_disable(&mut self) {
        self.processor_status &= 0b1011_1011;
    }

    fn clear_decimal_flag(&mut self) {
        self.processor_status &= 0b1111_0111;
    }

    pub fn get_zero_flag(&mut self) -> bool {
        (self.processor_status & 0b0000_0010) == 0b0000_0010
    }

    pub fn get_negative_flag(&mut self) -> bool {
        (self.processor_status & 0b1000_0000) == 0b1000_0000
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.processor_status & 0b0000_0001) == 0b0000_0001
    }

    pub fn get_overflow_flag(&self) -> bool {
        (self.processor_status & 0b0100_0000) == 0b0100_0000
    }

    fn get_operand_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
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
                let base = self.read_next_two_bytes();
                let pos = base + self.x_register as u16;

                if Cpu::crosses_page_boundary_u8(base, self.x_register) {
                    self.additional_cycles += 1;
                }

                pos
            }
            AddressingMode::AbsoluteY => {
                let base = self.read_next_two_bytes();
                let pos = base + self.y_register as u16;

                if Cpu::crosses_page_boundary_u8(base, self.y_register) {
                    self.additional_cycles += 1;
                }

                pos
            }
            AddressingMode::Indirect => {
                let base = self.read_next_two_bytes();
                self.mem_read_u16(base)
            }
            AddressingMode::IndirectX => {
                let base = self.read_next_byte();
                let lookup_addr = base.wrapping_add(self.x_register);
                self.mem_read_u16(lookup_addr as u16)
            }
            AddressingMode::IndirectY => {
                let lookup_addr = self.read_next_byte();
                let addr = self.mem_read_u16(lookup_addr as u16);

                if Self::crosses_page_boundary_u8(addr, self.y_register) {
                    self.additional_cycles += 1;
                }

                addr.wrapping_add(self.y_register as u16)
            }
            _ => panic!("Invalid addressing mode"),
        }
    }

    fn crosses_page_boundary_u8(base: u16, offset: u8) -> bool {
        (base & 0xFF00) != ((base + offset as u16) & 0xFF00)
    }

    fn crosses_page_boundary_i8(base: u16, offset: i8) -> bool {
        let target = base.wrapping_add(offset as i16 as u16);
        (base & 0xFF00) != (target & 0xFF00)
    }

    fn read_next_byte(&self) -> u8 {
        self.mem_read(self.program_counter)
    }

    fn read_next_two_bytes(&self) -> u16 {
        self.mem_read_u16(self.program_counter)
    }

    fn shift_left(&mut self, data: u8) -> u8 {
        let res = data << 1;

        if data & 0b10000000 != 0 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_negative_and_zero_flags(res);
        res
    }

    fn shift_right(&mut self, data: u8) -> u8 {
        let res = data >> 1;

        if data & 0b00000001 != 0 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_negative_and_zero_flags(res);
        res
    }

    fn rotate_left(&mut self, data: u8) -> u8 {
        let mut res = data << 1;

        res |= self.processor_status & 0b00000001;

        if data & 0b10000000 != 0 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_negative_and_zero_flags(res);
        res
    }

    fn rotate_right(&mut self, data: u8) -> u8 {
        let mut res = data >> 1;

        if self.get_carry_flag() {
            res |= 0b10000000;
        }

        if data & 0b00000001 != 0 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_negative_and_zero_flags(res);
        res
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
        self.stack_push(self.accumulator);
    }

    fn php(&mut self) {
        self.stack_push(self.processor_status | 0b00110000);
    }

    fn pla(&mut self) {
        self.accumulator = self.stack_pop();
        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn plp(&mut self) {
        self.processor_status = self.stack_pop() & 0b11001111;
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

    fn asl(&mut self, mode: &AddressingMode) {
        if mode != &AddressingMode::Accumulator {
            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target);
            let res = self.shift_left(target_value);
            self.mem_write(target, res);
        } else {
            self.accumulator = self.shift_left(self.accumulator);
        }
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        if mode != &AddressingMode::Accumulator {
            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target);
            let res = self.shift_right(target_value);
            self.mem_write(target, res);
        } else {
            self.accumulator = self.shift_right(self.accumulator);
        }
    }

    fn rol(&mut self, mode: &AddressingMode) {
        if mode != &AddressingMode::Accumulator {
            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target);
            let res = self.rotate_left(target_value);
            self.mem_write(target, res);
        } else {
            self.accumulator = self.rotate_left(self.accumulator);
        }
    }

    fn ror(&mut self, mode: &AddressingMode) {
        if mode != &AddressingMode::Accumulator {
            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target);
            let res = self.rotate_right(target_value);
            self.mem_write(target, res);
        } else {
            self.accumulator = self.rotate_right(self.accumulator);
        }
    }

    fn clc(&mut self) {
        self.clear_carry_flag();
    }

    fn cld(&mut self) {
        self.clear_decimal_flag();
    }

    fn cli(&mut self) {
        self.clear_interrupt_disable();
    }

    fn clv(&mut self) {
        self.clear_overflow_flag();
    }

    fn sec(&mut self) {
        self.set_carry_flag();
    }

    fn sed(&mut self) {
        self.set_decimal_flag();
    }

    fn sei(&mut self) {
        self.set_interrupt_disable();
    }

    fn brk(&mut self) {
        self.stack_push_u16(self.program_counter + 1);
        self.php();

        self.program_counter = self.mem_read_u16(0xFFFE);
    }

    fn nop(&mut self) {}

    fn rti(&mut self) {
        self.plp();
        self.program_counter = self.stack_pop_u16();
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let target_address = self.get_operand_address(mode);
        self.program_counter = target_address;
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let target_address = self.get_operand_address(mode);
        self.stack_push_u16(self.program_counter + 1);
        self.program_counter = target_address;
    }

    fn rts(&mut self) {
        let pc = self.stack_pop_u16() + 1;
        self.program_counter = pc;
    }

    fn bcc(&mut self, mode: &AddressingMode) {
        if !self.get_carry_flag() {
            self.additional_cycles += 1;

            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target) as i8;

            if Cpu::crosses_page_boundary_i8(self.program_counter, target_value) {
                self.additional_cycles += 1;
            }

            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(target_value as i16 as u16);
        }
    }

    fn bcs(&mut self, mode: &AddressingMode) {
        if self.get_carry_flag() {
            self.additional_cycles += 1;

            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target) as i8;

            if Cpu::crosses_page_boundary_i8(self.program_counter, target_value) {
                self.additional_cycles += 1;
            }

            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(target_value as i16 as u16);
        }
    }

    fn beq(&mut self, mode: &AddressingMode) {
        if self.get_zero_flag() {
            self.additional_cycles += 1;

            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target) as i8;

            if Cpu::crosses_page_boundary_i8(self.program_counter, target_value) {
                self.additional_cycles += 1;
            }

            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(target_value as i16 as u16);
        }
    }

    fn bne(&mut self, mode: &AddressingMode) {
        if !self.get_zero_flag() {
            self.additional_cycles += 1;

            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target) as i8;

            if Cpu::crosses_page_boundary_i8(self.program_counter, target_value) {
                self.additional_cycles += 1;
            }

            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(target_value as i16 as u16);
        }
    }

    fn bmi(&mut self, mode: &AddressingMode) {
        if self.get_negative_flag() {
            self.additional_cycles += 1;

            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target) as i8;

            if Cpu::crosses_page_boundary_i8(self.program_counter, target_value) {
                self.additional_cycles += 1;
            }

            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(target_value as i16 as u16);
        }
    }

    fn bpl(&mut self, mode: &AddressingMode) {
        if !self.get_negative_flag() {
            self.additional_cycles += 1;

            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target) as i8;

            if Cpu::crosses_page_boundary_i8(self.program_counter, target_value) {
                self.additional_cycles += 1;
            }

            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(target_value as i16 as u16);
        }
    }

    fn bvs(&mut self, mode: &AddressingMode) {
        if self.get_overflow_flag() {
            self.additional_cycles += 1;

            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target) as i8;

            if Cpu::crosses_page_boundary_i8(self.program_counter, target_value) {
                self.additional_cycles += 1;
            }

            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(target_value as i16 as u16);
        }
    }

    fn bvc(&mut self, mode: &AddressingMode) {
        if !self.get_overflow_flag() {
            self.additional_cycles += 1;

            let target = self.get_operand_address(mode);
            let target_value = self.mem_read(target) as i8;

            if Cpu::crosses_page_boundary_i8(self.program_counter, target_value) {
                self.additional_cycles += 1;
            }

            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(target_value as i16 as u16);
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_value = self.mem_read(target);
        let carry_in = self.processor_status & 0b00000001;

        let acc_check = self.accumulator;

        let sum = self.accumulator as u16 + target_value as u16 + carry_in as u16;
        let result = sum as u8;

        self.accumulator = result;

        // Carry Flag
        if sum > 0xFF {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        // Overflow Flag
        if (!(acc_check ^ target_value) & (acc_check ^ result)) & 0x80 != 0 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }

        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_value = self.mem_read(target);
        let carry_in = self.processor_status & 0b00000001;

        let acc_check = self.accumulator;

        let value = target_value ^ 0xFF;
        let sum = self.accumulator as u16 + value as u16 + carry_in as u16;
        let result = sum as u8;

        self.accumulator = result;

        if sum > 0xFF {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        if ((acc_check ^ result) & (value ^ result) & 0x80) != 0 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }

        self.update_negative_and_zero_flags(self.accumulator);
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_value = self.mem_read(target);

        if target_value == self.accumulator {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }

        if self.accumulator >= target_value {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        if (self.accumulator.overflowing_sub(target_value).0) & 0b10000000 != 0 {
            self.set_negative_flag();
        } else {
            self.clear_negative_flag();
        }
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_value = self.mem_read(target);

        if target_value == self.x_register {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }

        if self.x_register >= target_value {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        if (self.x_register.overflowing_sub(target_value).0) & 0b10000000 != 0 {
            self.set_negative_flag();
        } else {
            self.clear_negative_flag();
        }
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        let target = self.get_operand_address(mode);
        let target_value = self.mem_read(target);

        if target_value == self.y_register {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }

        if self.y_register >= target_value {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        if (self.y_register.overflowing_sub(target_value).0) & 0b10000000 != 0 {
            self.set_negative_flag();
        } else {
            self.clear_negative_flag();
        }
    }

    fn isc(&mut self, mode: &AddressingMode) {
        self.inc(mode);
        self.sbc(mode);
    }

    fn trigger_nmi(&mut self) {
        self.stack_push_u16(self.program_counter);
        self.stack_push(self.processor_status | 0b00010000);
        self.sei();
        self.program_counter = self.mem_read_u16(0xFFFA) - 1
    }

    pub fn reset(&mut self) {
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn step(&mut self) -> u8 {
        match &self.ppu {
            None => (),
            Some(ppu) => {
                if ppu.borrow().poll_nmi() {
                    self.trigger_nmi();
                    println!("Nmi Triggered");
                    return 0xFF;
                }
            }
        }

        self.additional_cycles = 0;

        let opcode = self.mem_read(self.program_counter);
        let pnc = &OpCode::default();
        let op = OPCODES_MAP.get().unwrap().get(&opcode).unwrap_or(&pnc);

        #[cfg(debug_assertions)]
        let _memory = self.memory.get_memory_debug(RangeInclusive::new(0, 0xFFFF));

        self.program_counter += 1u16;
        let pc_check = self.program_counter;

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
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&op.addressing_mode),
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&op.addressing_mode),
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(&op.addressing_mode),
            0xE0 | 0xE4 | 0xEC => self.cpx(&op.addressing_mode),
            0xC0 | 0xC4 | 0xCC => self.cpy(&op.addressing_mode),
            0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&op.addressing_mode),
            0xE8 => self.inx(),
            0xC8 => self.iny(),
            0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&op.addressing_mode),
            0xCA => self.dex(),
            0x88 => self.dey(),
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(&op.addressing_mode),
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.lsr(&op.addressing_mode),
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.rol(&op.addressing_mode),
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => self.ror(&op.addressing_mode),
            0x4C | 0x6C => self.jmp(&op.addressing_mode),
            0x20 => self.jsr(&op.addressing_mode),
            0x60 => self.rts(),
            0x90 => self.bcc(&op.addressing_mode),
            0xB0 => self.bcs(&op.addressing_mode),
            0xF0 => self.beq(&op.addressing_mode),
            0xD0 => self.bne(&op.addressing_mode),
            0x30 => self.bmi(&op.addressing_mode),
            0x10 => self.bpl(&op.addressing_mode),
            0x50 => self.bvc(&op.addressing_mode),
            0x70 => self.bvs(&op.addressing_mode),
            0x18 => self.clc(),
            0xD8 => self.cld(),
            0x58 => self.cli(),
            0xB8 => self.clv(),
            0x38 => self.sec(),
            0xF8 => self.sed(),
            0x78 => self.sei(),
            0x00 => self.brk(),
            0xEA => self.nop(),
            0x40 => self.rti(),
            //Illegal Opcodes
            0xFF => self.isc(&op.addressing_mode),
            _ => {
                println!(
                    "Instruction 0x{opcode:X} at address 0x{:X} isn't valid",
                    self.program_counter - 1
                );
                return 0xFF;
            }
        }

        if self.program_counter == pc_check {
            self.program_counter += op.bytes as u16 - 1 + self.additional_cycles as u16;
        }

        let opcode = self.mem_read(self.program_counter);
        let check_op = OPCODES_MAP.get().unwrap().get(&opcode).unwrap_or(&pnc);

        if check_op.opcode == pnc.opcode {
            println!("Execution resulted in invalid pc")
        }

        op.cycles
    }

    pub fn load_rom<T: RomFileConvertible>(&mut self, rom_get: &T) {
        let rom_file = rom_get.as_rom_file();
        let prg_rom = rom_file.get_prg_rom();

        if rom_file.prg_memory.prg_rom_size > (16 * 1024) {
            self.memory.add_memory(0x8000..=0xFFFF, prg_rom)
        } else {
            self.memory.add_memory(
                0x8000..=0xFFFF,
                Box::new(MirrorMemory::new(prg_rom, 0x3FFF)),
            )
        }

        match rom_file.prg_memory.prg_ram_size {
            0 => {}
            2048 => self.memory.add_memory(
                0x6000..=0x7FFF,
                Box::new(MirrorMemory::new(
                    Box::new(Ram::new(rom_file.prg_memory.prg_ram_size as usize)),
                    0x7FF,
                )),
            ),
            4096 => self.memory.add_memory(
                0x6000..=0x7FFF,
                Box::new(MirrorMemory::new(
                    Box::new(Ram::new(rom_file.prg_memory.prg_ram_size as usize)),
                    0xFFF,
                )),
            ),
            8192 => self.memory.add_memory(
                0x6000..=0x7FFF,
                Box::new(Ram::new(rom_file.prg_memory.prg_ram_size as usize)),
            ),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Cpu {
    pub fn test_instance() -> Self {
        let mut inst = Cpu::new();
        inst.memory
            .add_memory(0x4020..=0xFFFF, Box::new(Ram::new(0xBFE0)));
        inst
    }
}

impl Cpu {
    pub fn from(state: CpuState, ppu: Rc<RefCell<Ppu>>, rom: RomFile) -> Self {
        OPCODES_MAP.get_or_init(opcode::init);

        let mut mem = MemoryMap::default();
        mem.add_memory(
            0x0..=0x1FFF,
            Box::new(MirrorMemory::new(
                Box::new(Ram::new(INTERNAL_RAM_SIZE as usize)),
                0x07FF,
            )),
        );

        mem.add_memory(0x4000..=0x4017, Box::new(Ram::new(0x18)));
        mem.add_memory(0x4018..=0x401F, Box::new(Ram::new(0x8)));

        let mut cpu = Self {
            program_counter: state.program_counter,
            stack_pointer: state.stack_pointer,
            accumulator: state.accumulator,
            x_register: state.x_register,
            y_register: state.y_register,
            processor_status: state.processor_status,
            memory: Box::new(mem),
            ppu: Some(ppu),
            additional_cycles: 0,
        };

        cpu.load_rom(&rom);

        cpu
    }
}
