use std::cell::{Cell, RefCell};
use std::mem::discriminant;
use std::ops::RangeInclusive;
use std::rc::Rc;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::emulation::mem::apu_registers::ApuRegisters;
use crate::emulation::mem::memory_map::MemoryMap;
use crate::emulation::mem::mirror_memory::MirrorMemory;
use crate::emulation::mem::{Memory, Ram};
use crate::emulation::opcode;
use crate::emulation::opcode::{OpCode, OPCODES_MAP};
use crate::emulation::ppu::Ppu;
use crate::emulation::rom::{RomFile, RomFileConvertible};
use crate::emulation::savestate::CpuState;
use crate::util;

pub const INTERNAL_RAM_MEMORY_RANGE: RangeInclusive<u16> = 0x0..=0x1FFF;
pub const INTERNAL_RAM_SIZE: u16 = 0x800;
pub const STACK_START: u8 = 0xFF;
pub const STACK_START_ADDRESS: u16 = 0x0100;

pub const NEGATIVE_BIT: u8 = 0x80;
pub const CARRY_BIT: u8 = 0x1;
pub const ZERO_BIT: u8 = 0x2;
pub const OVERFLOW_BIT: u8 = 0x40;
pub const IRQ_BIT: u8 = 0x4;
pub const UNUSED_BIT: u8 = 0x20;
pub const BREAK_BIT: u8 = 0x10;
pub const DECIMAL_BIT: u8 = 0x8;
pub const IRQ_VECTOR_ADDR: u16 = 0xFFFE;
pub const NMI_HANDLER_ADDR: u16 = 0xFFFA;
pub const RESET_VECTOR_ADDR: u16 = 0xFFFC;
pub const UPPER_BYTE: u16 = 0xFF00;
pub const LOWER_BYTE: u16 = 0x00FF;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub processor_status: u8,
    pub memory: Box<MemoryMap>,
    pub ppu: Option<Rc<RefCell<Ppu>>>,
    pub irq_provider: Cell<bool>,
    pub master_cycle: u128,
    pub lo: u8,
    pub hi: u8,
    pub current_op: MicroOp,
    pub op_queue: Vec<MicroOp>,
    pub current_opcode: Option<OpCode>,
    pub temp: u8,
    pub ane_constant: u8,
    pub is_halted: bool,
    pub irq_pending: bool,
    pub nmi_pending: bool,
    pub nmi_detected: bool,
    pub irq_detected: bool,
    pub locked_irq_vec: u16,
    pub current_irq_vec: u16,
    pub is_in_irq: bool,
    pub prev_nmi: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        let mem = Self::get_default_memory_map();
        OPCODES_MAP.get_or_init(opcode::init);

        Self {
            program_counter: 0,
            processor_status: 0,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            memory: Box::new(mem),
            stack_pointer: 0,
            ppu: None,
            irq_provider: Cell::new(false),
            master_cycle: 0,
            lo: 0,
            hi: 0,
            current_op: MicroOp::FetchOpcode(MicroOpCallback::None),
            op_queue: vec![],
            current_opcode: None,
            temp: 0,
            ane_constant: 0xEE,
            is_halted: false,
            irq_pending: false,
            nmi_pending: false,
            nmi_detected: false,
            irq_detected: false,
            locked_irq_vec: 0,
            current_irq_vec: IRQ_VECTOR_ADDR,
            is_in_irq: false,
            prev_nmi: false,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        OPCODES_MAP.get_or_init(opcode::init);
        Self::default()
    }

    pub fn mem_read(&mut self, addr: u16) -> u8 {
        if (0x2000..=0x3FFFu16).contains(&addr) {
            if (addr - 0x2000) % 0x7 == 0 {
                if let Some(ppu) = &self.ppu {
                    ppu.borrow_mut().master_cycle = self.master_cycle;
                }
            }
        }

        self.memory.mem_read(addr)
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) { self.memory.mem_write(addr, data); }

    pub fn mem_read_u16(&mut self, addr: u16) -> u16 { self.memory.mem_read_u16(addr) }

    pub fn mem_write_u16(&mut self, addr: u16, data: u16) { self.memory.mem_write_u16(addr, data); }

    pub fn stack_pop(&mut self) -> u8 {
        let val = self.mem_read(STACK_START_ADDRESS + self.stack_pointer as u16);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        val
    }

    pub fn stack_peek(&mut self) -> u8 {
        self.mem_read(STACK_START_ADDRESS + self.stack_pointer as u16)
    }

    pub fn stack_push(&mut self, data: u8) {
        self.mem_write(STACK_START_ADDRESS + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        (hi << 8) | lo
    }

    pub fn stack_push_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn set_zero_flag(&mut self) { self.processor_status |= ZERO_BIT; }

    fn clear_zero_flag(&mut self) { self.processor_status &= !ZERO_BIT; }

    fn set_negative_flag(&mut self) { self.processor_status |= NEGATIVE_BIT }

    fn clear_negative_flag(&mut self) { self.processor_status &= !NEGATIVE_BIT }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.set_zero_flag()
        } else {
            self.clear_zero_flag()
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result & NEGATIVE_BIT != 0 {
            self.set_negative_flag();
        } else {
            self.clear_negative_flag();
        }
    }

    fn update_negative_and_zero_flags(&mut self, result: u8) {
        self.update_negative_flag(result);
        self.update_zero_flag(result);
    }

    fn set_carry_flag(&mut self) { self.processor_status |= CARRY_BIT; }

    fn clear_carry_flag(&mut self) { self.processor_status &= !CARRY_BIT; }

    fn set_overflow_flag(&mut self) { self.processor_status |= OVERFLOW_BIT; }

    fn clear_overflow_flag(&mut self) { self.processor_status &= !OVERFLOW_BIT; }

    fn set_interrupt_disable(&mut self) { self.processor_status |= IRQ_BIT; }

    fn clear_interrupt_disable(&mut self) { self.processor_status &= !IRQ_BIT; }

    fn set_decimal_flag(&mut self) { self.processor_status |= DECIMAL_BIT; }

    fn clear_decimal_flag(&mut self) { self.processor_status &= !DECIMAL_BIT; }

    pub fn get_zero_flag(&self) -> bool { (self.processor_status & ZERO_BIT) != 0 }

    pub fn get_negative_flag(&self) -> bool { (self.processor_status & NEGATIVE_BIT) != 0 }

    pub fn get_carry_flag(&self) -> bool { (self.processor_status & CARRY_BIT) != 0 }

    pub fn get_overflow_flag(&self) -> bool { (self.processor_status & OVERFLOW_BIT) != 0 }

    pub fn get_decimal_flag(&self) -> bool { (self.processor_status & DECIMAL_BIT) != 0 }

    pub fn get_interrupt_disable_flag(&self) -> bool { (self.processor_status & IRQ_BIT) != 0 }

    pub fn get_break_flag(&self) -> bool { (self.processor_status & BREAK_BIT) != 0 }

    pub fn get_unused_flag(&self) -> bool { (self.processor_status & UNUSED_BIT) != 0 }

    fn shift_left(&mut self, data: u8) -> u8 {
        let res = data << 1;

        if data & NEGATIVE_BIT != 0 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_negative_and_zero_flags(res);
        res
    }

    fn shift_right(&mut self, data: u8) -> u8 {
        let res = data >> 1;

        if data & CARRY_BIT != 0 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_negative_and_zero_flags(res);
        res
    }

    fn rotate_left(&mut self, data: u8) -> u8 {
        let mut res = data << 1;

        res |= self.processor_status & CARRY_BIT;

        if data & NEGATIVE_BIT != 0 {
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
            res |= NEGATIVE_BIT;
        }

        if data & CARRY_BIT != 0 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_negative_and_zero_flags(res);
        res
    }

    fn get_addr_latch(&self) -> u16 { ((self.hi as u16) << 8) | (self.lo as u16) }

    fn get_instructions_for_op_type(&mut self) -> Vec<MicroOp> {
        let mut instructions = vec![];

        let Some(op) = self.current_opcode else {
            return instructions;
        };

        match op.op_type {
            OpType::AccumulatorOrImplied(callback) => {
                instructions.push(MicroOp::DummyRead(callback))
            }
            OpType::ImmediateAddressing(target, callback) => {
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::PC,
                    Source::None,
                    target,
                    Source::None,
                    Source::None,
                    Target::None,
                    true,
                    callback,
                ))
            }
            OpType::AbsoluteRead(target, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::FetchOperandHi(MicroOpCallback::None));
                instructions.push(MicroOp::Read(AddressSource::AddressLatch, target, callback))
            }
            OpType::AbsoluteIndexRead(index, target, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::PC,
                    Source::None,
                    Target::HI,
                    Source::LO,
                    index,
                    Target::LO,
                    true,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadPageCrossAware(
                    AddressSource::AddressLatch,
                    index,
                    target,
                    true,
                    callback,
                ))
            }
            OpType::ZeroPageRead(target, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::Read(AddressSource::LO, target, callback))
            }
            OpType::ZeroPageIndexRead(index, target, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::DummyReadAddOffsetWriteToTarget(
                    AddressSource::LO,
                    index,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(AddressSource::Temp, target, callback))
            }
            OpType::IndexedIndirectRead(target, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::DummyReadAddOffsetWriteToTarget(
                    AddressSource::LO,
                    Source::X,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(
                    AddressSource::Temp,
                    Target::LO,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::Temp,
                    Source::Constant(1),
                    Target::HI,
                    Source::None,
                    Source::None,
                    Target::None,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(AddressSource::AddressLatch, target, callback))
            }
            OpType::IndirectIndexedRead(target, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::Read(
                    AddressSource::LO,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::LO,
                    Source::Constant(1),
                    Target::HI,
                    Source::TEMP,
                    Source::Y,
                    Target::LO,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadPageCrossAware(
                    AddressSource::AddressLatch,
                    Source::Y,
                    target,
                    true,
                    callback,
                ));
            }
            OpType::BRK(callback) => {
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::PC,
                    Source::None,
                    Target::None,
                    Source::None,
                    Source::None,
                    Target::None,
                    true,
                    MicroOpCallback::COPY(
                        AddressSource::Address(IRQ_VECTOR_ADDR),
                        Target::IrqVecCandidate,
                    ),
                ));
                instructions.push(MicroOp::StackPush(Source::PCH, MicroOpCallback::None));
                instructions.push(MicroOp::StackPush(Source::PCL, MicroOpCallback::None));
                instructions.push(MicroOp::StackPush(
                    Source::PBrk,
                    MicroOpCallback::LockIrqVec,
                ));
                instructions.push(MicroOp::Read(
                    AddressSource::IrqVec,
                    Target::PCL,
                    MicroOpCallback::SEI,
                ));
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::IrqVec,
                    Source::Constant(1),
                    Target::PCH,
                    Source::None,
                    Source::None,
                    Target::None,
                    false,
                    callback,
                ));
            }
            OpType::RTI(callback) => {
                instructions.push(MicroOp::DummyRead(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::None,
                    Source::None,
                    Target::None,
                    Source::SP,
                    Source::Constant(1),
                    Target::SP,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::StackPop(Target::P, MicroOpCallback::None));
                instructions.push(MicroOp::StackPop(Target::PCL, MicroOpCallback::None));
                instructions.push(MicroOp::StackPeek(Target::PCH, callback))
            }
            OpType::RTS(callback) => {
                instructions.push(MicroOp::DummyRead(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::None,
                    Source::None,
                    Target::None,
                    Source::SP,
                    Source::Constant(1),
                    Target::SP,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::StackPop(Target::PCL, MicroOpCallback::None));
                instructions.push(MicroOp::StackPeek(Target::PCH, MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::None,
                    Source::None,
                    Target::None,
                    Source::None,
                    Source::None,
                    Target::None,
                    true,
                    callback,
                ))
            }
            OpType::PH(src, callback) => {
                instructions.push(MicroOp::DummyRead(MicroOpCallback::None));
                instructions.push(MicroOp::StackPush(src, callback))
            }
            OpType::PL(target, callback) => {
                instructions.push(MicroOp::DummyRead(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::None,
                    Source::None,
                    Target::None,
                    Source::SP,
                    Source::Constant(1),
                    Target::SP,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::StackPeek(target, callback))
            }
            OpType::JSR(callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::DummyRead(MicroOpCallback::None));
                instructions.push(MicroOp::StackPush(Source::PCH, MicroOpCallback::None));
                instructions.push(MicroOp::StackPush(Source::PCL, MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::PC,
                    Source::None,
                    Target::PCH,
                    Source::LO,
                    Source::None,
                    Target::PCL,
                    false,
                    callback,
                ))
            }
            OpType::JmpAbsolute(callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::PC,
                    Source::None,
                    Target::PCH,
                    Source::LO,
                    Source::None,
                    Target::PCL,
                    false,
                    callback,
                ))
            }
            OpType::AbsoluteRMW(target, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::FetchOperandHi(MicroOpCallback::None));
                instructions.push(MicroOp::Read(
                    AddressSource::AddressLatch,
                    target,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(
                    Target::AddressLatch,
                    Source::TEMP,
                    false,
                    callback,
                ));
                instructions.push(MicroOp::Write(
                    Target::AddressLatch,
                    Source::TEMP,
                    false,
                    MicroOpCallback::None,
                ))
            }
            OpType::AbsoluteWrite(source, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::FetchOperandHi(MicroOpCallback::None));
                instructions.push(MicroOp::Write(Target::AddressLatch, source, true, callback));
            }
            OpType::ZeroPageRMW(target, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::Read(
                    AddressSource::LO,
                    target,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(
                    Target::LoWrite,
                    Source::TEMP,
                    false,
                    callback,
                ));
                instructions.push(MicroOp::Write(
                    Target::LoWrite,
                    Source::TEMP,
                    true,
                    MicroOpCallback::None,
                ))
            }
            OpType::ZeroPageWrite(source, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::Write(Target::LoWrite, source, true, callback));
            }
            OpType::ZeroPageIndexRMW(index, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::None,
                    Source::None,
                    Target::None,
                    Source::LO,
                    index,
                    Target::LO,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(
                    AddressSource::LO,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(
                    Target::LoWrite,
                    Source::TEMP,
                    false,
                    callback,
                ));
                instructions.push(MicroOp::Write(
                    Target::LoWrite,
                    Source::TEMP,
                    true,
                    MicroOpCallback::None,
                ));
            }
            OpType::ZeroPageIndexWrite(source, index, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::None,
                    Source::None,
                    Target::None,
                    Source::LO,
                    index,
                    Target::LO,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(Target::LoWrite, source, true, callback));
            }
            OpType::AbsoluteIndexRMW(offset, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::PC,
                    Source::None,
                    Target::HI,
                    Source::LO,
                    offset,
                    Target::LO,
                    true,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadPageCrossAware(
                    AddressSource::AddressLatch,
                    Source::X,
                    Target::TEMP,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(
                    AddressSource::AddressLatch,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(
                    Target::AddressLatch,
                    Source::TEMP,
                    false,
                    callback,
                ));
                instructions.push(MicroOp::Write(
                    Target::AddressLatch,
                    Source::TEMP,
                    true,
                    MicroOpCallback::None,
                ));
            }
            OpType::AbsoluteIndexWrite(source, offset, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::PC,
                    Source::None,
                    Target::HI,
                    Source::LO,
                    offset,
                    Target::LO,
                    true,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadPageCrossAware(
                    AddressSource::AddressLatch,
                    offset,
                    Target::None,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(Target::AddressLatch, source, true, callback));
            }
            OpType::IndexedIndirectWrite(source, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::LO,
                    Source::None,
                    Target::None,
                    Source::LO,
                    Source::X,
                    Target::TEMP,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(
                    AddressSource::Temp,
                    Target::LO,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::Temp,
                    Source::Constant(1),
                    Target::HI,
                    Source::None,
                    Source::None,
                    Target::None,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(Target::AddressLatch, source, true, callback));
            }
            OpType::JmpIndirect(callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::FetchOperandHi(MicroOpCallback::None));
                instructions.push(MicroOp::Read(
                    AddressSource::AddressLatch,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::AddressLatch,
                    Source::Constant(1),
                    Target::PCH,
                    Source::TEMP,
                    Source::None,
                    Target::PCL,
                    false,
                    callback,
                ));
            }
            OpType::IndirectIndexedWrite(source, callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::LO,
                    Source::None,
                    Target::TEMP,
                    Source::None,
                    Source::None,
                    Target::None,
                    false,
                    callback,
                ));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::LO,
                    Source::Constant(1),
                    Target::HI,
                    Source::TEMP,
                    Source::Y,
                    Target::LO,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadPageCrossAware(
                    AddressSource::AddressLatch,
                    Source::Y,
                    Target::None,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(Target::AddressLatch, source, true, callback))
            }
            OpType::Relative(callback) => {
                instructions.push(MicroOp::FetchOperandLo(callback));
            }
            OpType::IndexedIndirectRMW(callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::LO,
                    Source::None,
                    Target::None,
                    Source::LO,
                    Source::X,
                    Target::TEMP,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(
                    AddressSource::Temp,
                    Target::LO,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadWithOffsetFromU16AndAddSomething(
                    AddressSource::Temp,
                    Source::Constant(1),
                    Target::HI,
                    Source::None,
                    Source::None,
                    Target::None,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(
                    AddressSource::AddressLatch,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(
                    Target::AddressLatch,
                    Source::TEMP,
                    false,
                    callback,
                ));
                instructions.push(MicroOp::Write(
                    Target::AddressLatch,
                    Source::TEMP,
                    true,
                    MicroOpCallback::None,
                ))
            }
            OpType::IndirectIndexedRMW(callback) => {
                instructions.push(MicroOp::FetchOperandLo(MicroOpCallback::None));
                instructions.push(MicroOp::Read(
                    AddressSource::LO,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                    AddressSource::LO,
                    Source::Constant(1),
                    Target::HI,
                    Source::TEMP,
                    Source::Y,
                    Target::LO,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::ReadPageCrossAware(
                    AddressSource::AddressLatch,
                    Source::Y,
                    Target::TEMP,
                    false,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Read(
                    AddressSource::AddressLatch,
                    Target::TEMP,
                    MicroOpCallback::None,
                ));
                instructions.push(MicroOp::Write(
                    Target::AddressLatch,
                    Source::TEMP,
                    false,
                    callback,
                ));
                instructions.push(MicroOp::Write(
                    Target::AddressLatch,
                    Source::TEMP,
                    true,
                    MicroOpCallback::None,
                ))
            }
        }

        instructions
    }

    fn get_instructions_for_nmi(&mut self) -> Vec<MicroOp> {
        vec![
            MicroOp::ReadWithOffsetFromU16AndAddSomething(
                AddressSource::PC,
                Source::None,
                Target::None,
                Source::None,
                Source::None,
                Target::None,
                false,
                MicroOpCallback::None,
            ),
            MicroOp::ReadWithOffsetFromU16AndAddSomething(
                AddressSource::PC,
                Source::None,
                Target::None,
                Source::None,
                Source::None,
                Target::None,
                false,
                MicroOpCallback::None,
            ),
            MicroOp::StackPush(Source::PCH, MicroOpCallback::None),
            MicroOp::StackPush(Source::PCL, MicroOpCallback::None),
            MicroOp::StackPush(Source::PNmi, MicroOpCallback::LockIrqVec),
            MicroOp::Read(AddressSource::IrqVec, Target::PCL, MicroOpCallback::SEI),
            MicroOp::ReadWithOffsetFromU16AndAddSomething(
                AddressSource::IrqVec,
                Source::Constant(1),
                Target::PCH,
                Source::None,
                Source::None,
                Target::None,
                false,
                MicroOpCallback::ExitIrq,
            ),
        ]
    }

    fn get_instructions_for_reset(&mut self) -> Vec<MicroOp> {
        vec![
            MicroOp::ReadWithOffsetFromU16AndAddSomething(
                AddressSource::PC,
                Source::None,
                Target::None,
                Source::None,
                Source::None,
                Target::None,
                false,
                MicroOpCallback::None,
            ),
            MicroOp::ReadWithOffsetFromU16AndAddSomething(
                AddressSource::PC,
                Source::None,
                Target::None,
                Source::None,
                Source::None,
                Target::None,
                false,
                MicroOpCallback::None,
            ),
            MicroOp::StackPush(Source::None, MicroOpCallback::None),
            MicroOp::StackPush(Source::None, MicroOpCallback::None),
            MicroOp::StackPush(Source::None, MicroOpCallback::None),
            MicroOp::Read(
                AddressSource::Address(RESET_VECTOR_ADDR),
                Target::PCL,
                MicroOpCallback::SEI,
            ),
            MicroOp::Read(
                AddressSource::Address(RESET_VECTOR_ADDR + 1),
                Target::PCH,
                MicroOpCallback::ExitIrq,
            ),
        ]
    }

    pub fn trigger_nmi(&mut self) {
        let mut seq = self.get_instructions_for_nmi();
        self.nmi_pending = false;

        self.is_in_irq = true;

        self.current_op = seq.remove(0);
        self.op_queue = seq;
    }

    pub fn trigger_irq(&mut self) {
        let mut seq = self.get_instructions_for_nmi();

        self.is_in_irq = true;

        self.current_op = seq.remove(0);
        self.op_queue = seq;
    }

    pub fn reset(&mut self) {
        let mut seq = self.get_instructions_for_reset();

        self.is_in_irq = true;

        self.current_op = seq.remove(0);
        self.op_queue = seq;
    }

    pub fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<u8> {
        self.memory.get_memory_debug(range)
    }

    pub fn step(&mut self, master_cycle: u128) -> Result<(), String> {
        if self.is_halted {
            return Err(String::from("Encountered hlt"));
        }

        self.master_cycle = master_cycle;
        let op = self.current_op;

        if discriminant(&op) != discriminant(&MicroOp::BranchIncrement(Source::None))
            && !self.is_in_irq
        {
            if self.nmi_detected {
                self.nmi_pending = true;
                self.nmi_detected = false;
            }

            self.irq_pending = self.irq_detected;
        }

        let mut seq = self.execute_micro_op(&op);

        if let Some(ppu) = &self.ppu {
            let curr_nmi = ppu.borrow().poll_nmi();

            if curr_nmi && !self.prev_nmi {
                self.current_irq_vec = NMI_HANDLER_ADDR;
                self.nmi_detected = true;
            }

            self.prev_nmi = curr_nmi;
        }

        if self.irq_provider.get() {
            self.current_irq_vec = IRQ_VECTOR_ADDR;
            self.irq_detected = true;
        } else {
            self.irq_detected = false;
        }

        if !seq.is_empty() {
            // sequence head becomes next, rest get queued
            self.current_op = seq.remove(0);
            self.op_queue = seq;

            Ok(())
        } else {
            if self.nmi_pending {
                self.trigger_nmi();
                self.nmi_pending = false;
                self.irq_pending = false;
                return Ok(());
            } else if self.irq_pending && !self.get_interrupt_disable_flag() {
                self.trigger_irq();
                self.irq_provider.set(false);
                self.nmi_pending = false;
                self.irq_pending = false;
                return Ok(());
            }

            self.current_op = MicroOp::FetchOpcode(MicroOpCallback::None);
            Ok(())
        }
    }

    fn execute_micro_op(&mut self, micro_op: &MicroOp) -> Vec<MicroOp> {
        match micro_op {
            MicroOp::FetchOpcode(callback) => {
                let opcode = self.mem_read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);

                self.current_opcode = Some(
                    **OPCODES_MAP
                        .get()
                        .unwrap()
                        .get(&opcode)
                        .unwrap_or_else(|| panic!("finished at: {}", self.master_cycle)),
                );
                self.run_op(*callback);

                self.get_instructions_for_op_type()
            }
            MicroOp::FetchOperandLo(callback) => {
                self.lo = self.mem_read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);

                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::FetchOperandHi(callback) => {
                self.hi = self.mem_read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);

                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::Read(source, target, callback) => {
                if let Some(address) = self.get_u16_address(source) {
                    let val = self.mem_read(address);
                    self.write_to_target(target, val);
                }

                self.run_op(*callback);

                self.op_queue.clone()
            }
            // pre_callback denotes whether to call the callback before or after the write. In the
            // case of a dummy write we write with false so that the value from before the callback
            // gets written, and in the case of a single write we need to execute the callback
            // beforehand so that we write the updated value
            MicroOp::Write(target, src, pre_callback, callback) => {
                if *pre_callback {
                    self.run_op(*callback);
                }

                let val = self.get_src_value(src);
                self.write_to_target(target, val);

                if !pre_callback {
                    self.run_op(*callback);
                }
                self.op_queue.clone()
            }
            MicroOp::StackPush(source, callback) => {
                let src_value = self.get_src_value(source);
                self.stack_push(src_value);

                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::StackPop(target, callback) => {
                let _stack = self.get_memory_debug(Some(0x0100..=0x01FF));
                let val = self.stack_pop();
                self.write_to_target(target, val);

                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::StackPeek(target, callback) => {
                let val = self.stack_peek();
                self.write_to_target(target, val);

                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::ReadPageCrossAware(source, offset, target, schedule_read, callback) => {
                let mut page_cross = false;

                if let Some(address) = self.get_u16_address(source) {
                    let val = self.mem_read(address);
                    self.write_to_target(target, val);
                    let offset = self.get_src_value(offset);

                    if self.lo.overflowing_sub(offset).1 {
                        page_cross = true;
                    }
                };

                if page_cross {
                    if *schedule_read {
                        self.op_queue
                            .push(MicroOp::Read(*source, *target, *callback));
                    }

                    self.hi = self.hi.wrapping_add(1);
                } else {
                    self.run_op(*callback);
                }

                self.op_queue.clone()
            }
            MicroOp::DummyReadAddOffsetWriteToTarget(source, offset, target, callback) => {
                if let Some(address) = self.get_u16_address(source) {
                    self.mem_read(address);
                    let src_value = self.get_src_value(offset) as u16;
                    self.write_to_target(target, address.wrapping_add(src_value) as u8);
                }

                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::DummyRead(callback) => {
                self.mem_read(self.program_counter);
                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::ReadWithOffsetFromZPAndAddSomethingU8(
                address_source,
                offset,
                target,
                add_to_src,
                to_add,
                to_save,
                inc_pc,
                callback,
            ) => {
                if let Some(address) = self.get_u16_address(address_source) {
                    let src_value = self.get_src_value(offset);
                    let offset_address = util::add_to_low_byte(address as u8 as u16, src_value);
                    let value = self.mem_read(offset_address);
                    self.write_to_target(target, value);
                }

                let add_to = self.get_src_value(add_to_src);
                let to_add = self.get_src_value(to_add);
                let value = add_to.wrapping_add(to_add);
                self.write_to_target(to_save, value);

                if *inc_pc {
                    self.program_counter = self.program_counter.wrapping_add(1);
                }

                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::ReadWithOffsetFromU16AndAddSomething(
                address_source,
                offset,
                target,
                add_to_src,
                to_add,
                to_save,
                inc_pc,
                callback,
            ) => {
                if let Some(address) = self.get_u16_address(address_source) {
                    let src_value = self.get_src_value(offset);
                    let offset_address = util::add_to_low_byte(address, src_value);
                    let value = self.mem_read(offset_address);
                    self.write_to_target(target, value);
                }

                let add_to = self.get_src_value(add_to_src);
                let to_add = self.get_src_value(to_add);
                let value = add_to.wrapping_add(to_add);
                self.write_to_target(to_save, value);

                if *inc_pc {
                    self.program_counter = self.program_counter.wrapping_add(1);
                }

                self.run_op(*callback);

                self.op_queue.clone()
            }
            MicroOp::BranchIncrement(to_add) => {
                let add_to = self.program_counter;
                let to_add = self.get_src_value(to_add) as i8;
                let value = add_to.wrapping_add(to_add as i16 as u16);
                self.write_to_target(&Target::PCL, value as u8);

                if util::crosses_page_boundary_i8(add_to, to_add) {
                    self.op_queue.push(MicroOp::FixHiBranch(value));
                }

                self.op_queue.clone()
            }
            MicroOp::FixHiBranch(value) => {
                self.program_counter = *value;

                self.op_queue.clone()
            }
        }
    }

    fn run_op(&mut self, op: MicroOpCallback) {
        match op {
            MicroOpCallback::None => {}
            MicroOpCallback::ADC => adc(self),
            MicroOpCallback::ASL => asl(self),
            MicroOpCallback::LSR => lsr(self),
            MicroOpCallback::ROL => rol(self),
            MicroOpCallback::ROR => ror(self),
            MicroOpCallback::CLC => clc(self),
            MicroOpCallback::CLD => cld(self),
            MicroOpCallback::CLI => cli(self),
            MicroOpCallback::CLV => clv(self),
            MicroOpCallback::DEX => dex(self),
            MicroOpCallback::DEY => dey(self),
            MicroOpCallback::INX => inx(self),
            MicroOpCallback::INY => iny(self),
            MicroOpCallback::SEI => sei(self),
            MicroOpCallback::SED => sed(self),
            MicroOpCallback::SEC => sec(self),
            MicroOpCallback::TAX => tax(self),
            MicroOpCallback::TAY => tay(self),
            MicroOpCallback::TSX => tsx(self),
            MicroOpCallback::TXA => txa(self),
            MicroOpCallback::TXS => txs(self),
            MicroOpCallback::TYA => tya(self),
            MicroOpCallback::AND => and(self),
            MicroOpCallback::CMP => cmp(self),
            MicroOpCallback::CPX => cpx(self),
            MicroOpCallback::CPY => cpy(self),
            MicroOpCallback::EOR => eor(self),
            MicroOpCallback::ORA => ora(self),
            MicroOpCallback::SBC => sbc(self),
            MicroOpCallback::BIT => bit(self),
            MicroOpCallback::DEC => dec(self),
            MicroOpCallback::INC => inc(self),
            MicroOpCallback::BRANCH(condition) => branch(self, condition),
            MicroOpCallback::ALR => alr(self),
            MicroOpCallback::ANC => anc(self),
            MicroOpCallback::ANC2 => anc(self),
            MicroOpCallback::ANE => ane(self),
            MicroOpCallback::ARR => arr(self),
            MicroOpCallback::DCP => dcp(self),
            MicroOpCallback::ISB => isb(self),
            MicroOpCallback::LAX => lax(self),
            MicroOpCallback::LXA => lxa(self),
            MicroOpCallback::LAS => las(self),
            MicroOpCallback::RLA => rla(self),
            MicroOpCallback::RRA => rra(self),
            MicroOpCallback::SAX => sax(self),
            MicroOpCallback::SBX => sbx(self),
            MicroOpCallback::SHA => sha(self),
            MicroOpCallback::SHX => shx(self),
            MicroOpCallback::SHY => shy(self),
            MicroOpCallback::SLO => slo(self),
            MicroOpCallback::SRE => sre(self),
            MicroOpCallback::TAS => tas(self),
            MicroOpCallback::JAM => jam(self),
            MicroOpCallback::COPY(source, target) => copy(self, source, target),
            MicroOpCallback::LockIrqVec => self.locked_irq_vec = self.current_irq_vec,
            MicroOpCallback::SEIandLockIrqVec => {
                sei(self);
                self.locked_irq_vec = self.current_irq_vec
            }
            MicroOpCallback::ExitIrq => self.is_in_irq = false,
        }
    }

    fn get_u16_address(&self, address_source: &AddressSource) -> Option<u16> {
        match address_source {
            AddressSource::AddressLatch => Some(self.get_addr_latch()),
            AddressSource::Address(u16) => Some(*u16),
            AddressSource::LO => Some(self.get_addr_latch() as u8 as u16),
            AddressSource::Temp => Some(self.temp as u16),
            AddressSource::None => None,
            AddressSource::HI => Some(self.hi as u16),
            AddressSource::PC => Some(self.program_counter),
            AddressSource::IrqVec => Some(self.locked_irq_vec),
        }
    }

    pub fn load_rom<T: RomFileConvertible>(&mut self, rom_get: &T) {
        let rom_file = rom_get.as_rom_file();
        let prg_rom = rom_file.get_prg_rom();

        if rom_file.prg_memory.prg_rom_size > (16 * 1024) {
            self.memory.add_memory(0x8000..=0xFFFF, prg_rom)
        } else {
            self.memory.add_memory(
                0x8000..=0xFFFF,
                Memory::MirrorMemory(MirrorMemory::new(Box::new(prg_rom), 0x3FFF)),
            )
        }

        match rom_file.prg_memory.prg_ram_size {
            0 => {}
            2048 => self.memory.add_memory(
                0x6000..=0x7FFF,
                Memory::MirrorMemory(MirrorMemory::new(
                    Box::new(Memory::Ram(Ram::new(
                        rom_file.prg_memory.prg_ram_size as usize,
                    ))),
                    0x7FF,
                )),
            ),
            4096 => self.memory.add_memory(
                0x6000..=0x7FFF,
                Memory::MirrorMemory(MirrorMemory::new(
                    Box::new(Memory::Ram(Ram::new(
                        rom_file.prg_memory.prg_ram_size as usize,
                    ))),
                    0xFFF,
                )),
            ),
            8192 => self.memory.add_memory(
                0x6000..=0x7FFF,
                Memory::Ram(Ram::new(rom_file.prg_memory.prg_ram_size as usize)),
            ),
            _ => {}
        }
    }

    fn get_default_memory_map() -> MemoryMap {
        let mut mem = MemoryMap::default();
        // Internal Ram
        mem.add_memory(
            INTERNAL_RAM_MEMORY_RANGE,
            Memory::MirrorMemory(MirrorMemory::new(
                Box::new(Memory::Ram(Ram::new(INTERNAL_RAM_SIZE as usize))),
                INTERNAL_RAM_SIZE - 1,
            )),
        );

        // APU Registers
        mem.add_memory(0x4000..=0x4017, Memory::ApuRegisters(ApuRegisters::new()));
        // Unused APU Registers
        mem.add_memory(0x4018..=0x401F, Memory::Ram(Ram::new(0x8)));
        mem
    }

    pub fn get_src_value(&mut self, src: &Source) -> u8 {
        match src {
            Source::A => self.accumulator,
            Source::X => self.x_register,
            Source::Y => self.y_register,
            Source::SP => self.stack_pointer,
            Source::PCL => (self.program_counter & LOWER_BYTE) as u8,
            Source::PCH => ((self.program_counter & UPPER_BYTE) >> 8) as u8,
            Source::LO => self.lo,
            Source::HI => self.hi,
            Source::TEMP => self.temp,
            Source::Constant(val) => *val,
            Source::None => 0,
            Source::PBrk => self.processor_status | (UNUSED_BIT | BREAK_BIT),
            Source::PNmi => self.processor_status | UNUSED_BIT,
        }
    }

    fn write_to_target(&mut self, trg: &Target, val: u8) {
        match trg {
            Target::A => {
                self.accumulator = val;
                self.update_negative_and_zero_flags(self.accumulator)
            }
            Target::X => {
                self.x_register = val;
                self.update_negative_and_zero_flags(self.x_register)
            }
            Target::Y => {
                self.y_register = val;
                self.update_negative_and_zero_flags(self.y_register)
            }
            Target::SP => self.stack_pointer = val,
            Target::PCL => {
                self.program_counter = (self.program_counter & UPPER_BYTE) | (val as u16)
            }
            Target::PCH => {
                self.program_counter = (self.program_counter & LOWER_BYTE) | ((val as u16) << 8)
            }
            Target::LO => self.lo = val,
            Target::HI => self.hi = val,
            Target::TEMP => self.temp = val,
            Target::P => self.processor_status = val & (!UNUSED_BIT & !BREAK_BIT),
            Target::AddressLatch => {
                self.mem_write(self.get_addr_latch(), val);
            }
            Target::LoWrite => self.mem_write(self.lo as u16, val),
            Target::None => {}
            Target::IrqVecCandidate => unreachable!(),
        }
    }

    fn check_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::CarrySet => self.get_carry_flag(),
            Condition::CarryClear => !self.get_carry_flag(),
            Condition::ZeroSet => self.get_zero_flag(),
            Condition::ZeroClear => !self.get_zero_flag(),
            Condition::NegativeSet => self.get_negative_flag(),
            Condition::NegativeClear => !self.get_negative_flag(),
            Condition::OverflowSet => self.get_overflow_flag(),
            Condition::OverflowClear => !self.get_overflow_flag(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum MicroOp {
    FetchOpcode(MicroOpCallback),
    FetchOperandLo(MicroOpCallback),
    FetchOperandHi(MicroOpCallback),
    Read(AddressSource, Target, MicroOpCallback),
    Write(Target, Source, bool, MicroOpCallback),
    StackPush(Source, MicroOpCallback),
    StackPop(Target, MicroOpCallback),
    ReadPageCrossAware(AddressSource, Source, Target, bool, MicroOpCallback),
    DummyReadAddOffsetWriteToTarget(AddressSource, Source, Target, MicroOpCallback),
    DummyRead(MicroOpCallback),
    ReadWithOffsetFromZPAndAddSomethingU8(
        AddressSource,
        Source,
        Target,
        Source,
        Source,
        Target,
        bool,
        MicroOpCallback,
    ),
    StackPeek(Target, MicroOpCallback),
    ReadWithOffsetFromU16AndAddSomething(
        AddressSource,
        Source,
        Target,
        Source,
        Source,
        Target,
        bool,
        MicroOpCallback,
    ),
    BranchIncrement(Source),
    FixHiBranch(u16),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum Target {
    A,
    X,
    Y,
    SP,
    PCL,
    PCH,
    LO,
    HI,
    TEMP,
    None,
    P,
    AddressLatch,
    LoWrite,
    IrqVecCandidate,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum Source {
    PBrk,
    A,
    X,
    Y,
    SP,
    PCL,
    PCH,
    LO,
    HI,
    TEMP,
    Constant(u8),
    None,
    PNmi,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum AddressSource {
    AddressLatch,
    Address(u16),
    LO,
    Temp,
    None,
    HI,
    PC,
    IrqVec,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum MicroOpCallback {
    None,
    ADC,
    ASL,
    LSR,
    ROL,
    ROR,
    CLC,
    CLD,
    CLI,
    CLV,
    DEX,
    DEY,
    INX,
    INY,
    SEI,
    SED,
    SEC,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    AND,
    CMP,
    CPX,
    CPY,
    EOR,
    ORA,
    SBC,
    BIT,
    DEC,
    INC,
    BRANCH(Condition),
    LAX,
    LXA,
    ALR,
    ANC,
    ANE,
    ARR,
    DCP,
    ISB,
    LAS,
    RLA,
    RRA,
    SAX,
    SBX,
    SHA,
    SHX,
    SHY,
    SLO,
    SRE,
    JAM,
    TAS,
    ANC2,
    COPY(AddressSource, Target),
    LockIrqVec,
    SEIandLockIrqVec,
    ExitIrq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum Condition {
    CarrySet,
    CarryClear,
    ZeroSet,
    ZeroClear,
    NegativeSet,
    NegativeClear,
    OverflowSet,
    OverflowClear,
}

#[derive(Clone, Copy, Debug)]
pub enum OpType {
    ImmediateAddressing(Target, MicroOpCallback),
    AbsoluteRead(Target, MicroOpCallback),
    AbsoluteIndexRead(Source, Target, MicroOpCallback),
    ZeroPageRead(Target, MicroOpCallback),
    ZeroPageIndexRead(Source, Target, MicroOpCallback),
    AccumulatorOrImplied(MicroOpCallback),
    IndexedIndirectRead(Target, MicroOpCallback),
    IndirectIndexedRead(Target, MicroOpCallback),
    BRK(MicroOpCallback),
    RTI(MicroOpCallback),
    RTS(MicroOpCallback),
    PH(Source, MicroOpCallback),
    PL(Target, MicroOpCallback),
    JSR(MicroOpCallback),
    JmpAbsolute(MicroOpCallback),
    AbsoluteRMW(Target, MicroOpCallback),
    AbsoluteWrite(Source, MicroOpCallback),
    ZeroPageRMW(Target, MicroOpCallback),
    ZeroPageWrite(Source, MicroOpCallback),
    ZeroPageIndexRMW(Source, MicroOpCallback),
    ZeroPageIndexWrite(Source, Source, MicroOpCallback),
    AbsoluteIndexRMW(Source, MicroOpCallback),
    AbsoluteIndexWrite(Source, Source, MicroOpCallback),
    IndexedIndirectWrite(Source, MicroOpCallback),
    JmpIndirect(MicroOpCallback),
    IndirectIndexedWrite(Source, MicroOpCallback),
    Relative(MicroOpCallback),
    IndexedIndirectRMW(MicroOpCallback),
    IndirectIndexedRMW(MicroOpCallback),
}

#[cfg(test)]
impl Cpu {
    pub fn test_instance() -> Self {
        let mut inst = Cpu::new();
        inst.memory
            .add_memory(0x4020..=0xFFFF, Memory::Ram(Ram::new(0xBFE0)));

        //Test instance doesn't get reset, therefore we need to manually fix the stack
        // pointer
        inst.stack_pointer = STACK_START;
        inst
    }
}

impl Cpu {
    pub fn from(state: &CpuState, ppu: Rc<RefCell<Ppu>>, rom: &RomFile) -> Self {
        OPCODES_MAP.get_or_init(opcode::init);
        let mut opcode = None;

        if let Some(opcode_u8) = state.current_opcode {
            opcode = Some(
                **OPCODES_MAP
                    .get_or_init(opcode::init)
                    .get(&opcode_u8)
                    .expect("Invalid State"),
            );
        }

        let mut cpu = Self {
            program_counter: state.program_counter,
            stack_pointer: state.stack_pointer,
            accumulator: state.accumulator,
            x_register: state.x_register,
            y_register: state.y_register,
            processor_status: state.processor_status,
            memory: Box::new(Self::get_default_memory_map()),
            ppu: Some(ppu),
            irq_provider: Cell::new(false),
            master_cycle: state.master_cycle,
            lo: state.lo,
            hi: state.hi,
            current_op: state.current_op,
            op_queue: state.op_queue.clone(),
            current_opcode: opcode,
            temp: state.temp,
            ane_constant: state.ane_constant,
            is_halted: state.is_halted,
            irq_pending: false,
            nmi_pending: false,
            nmi_detected: false,
            irq_detected: false,
            locked_irq_vec: 0,
            current_irq_vec: 0,
            is_in_irq: false,
            prev_nmi: false,
        };

        cpu.load_rom(rom);

        cpu
    }
}

pub fn adc(cpu: &mut Cpu) {
    let target_value = cpu.temp;
    let carry_in = cpu.processor_status & CARRY_BIT;

    let acc_check = cpu.accumulator;

    let sum = cpu.accumulator as u16 + target_value as u16 + carry_in as u16;
    let result = sum as u8;

    cpu.accumulator = result;

    // Carry Flag
    if sum > LOWER_BYTE {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }

    // Overflow Flag
    if (!(acc_check ^ target_value) & (acc_check ^ result)) & NEGATIVE_BIT != 0 {
        cpu.set_overflow_flag();
    } else {
        cpu.clear_overflow_flag();
    }

    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn rol(cpu: &mut Cpu) {
    if discriminant(&cpu.current_opcode.unwrap().op_type)
        != discriminant(&OpType::AccumulatorOrImplied(MicroOpCallback::None))
    {
        let target_value = cpu.temp;
        let res = cpu.rotate_left(target_value);
        cpu.temp = res;
    } else {
        cpu.accumulator = cpu.rotate_left(cpu.accumulator);
        cpu.update_negative_and_zero_flags(cpu.accumulator);
    }
}

fn ror(cpu: &mut Cpu) {
    if discriminant(&cpu.current_opcode.unwrap().op_type)
        != discriminant(&OpType::AccumulatorOrImplied(MicroOpCallback::None))
    {
        let target_value = cpu.temp;
        let res = cpu.rotate_right(target_value);
        cpu.temp = res;
    } else {
        cpu.accumulator = cpu.rotate_right(cpu.accumulator);
        cpu.update_negative_and_zero_flags(cpu.accumulator);
    }
}

fn asl(cpu: &mut Cpu) {
    if discriminant(&cpu.current_opcode.unwrap().op_type)
        != discriminant(&OpType::AccumulatorOrImplied(MicroOpCallback::None))
    {
        let target_value = cpu.temp;
        let res = cpu.shift_left(target_value);
        cpu.temp = res;
    } else {
        cpu.accumulator = cpu.shift_left(cpu.accumulator);
        cpu.update_negative_and_zero_flags(cpu.accumulator);
    }
}

fn lsr(cpu: &mut Cpu) {
    if discriminant(&cpu.current_opcode.unwrap().op_type)
        != discriminant(&OpType::AccumulatorOrImplied(MicroOpCallback::None))
    {
        let target_value = cpu.temp;
        let res = cpu.shift_right(target_value);
        cpu.temp = res
    } else {
        cpu.accumulator = cpu.shift_right(cpu.accumulator);
        cpu.update_negative_and_zero_flags(cpu.accumulator);
    }
}

fn tax(cpu: &mut Cpu) {
    cpu.x_register = cpu.accumulator;
    cpu.update_negative_and_zero_flags(cpu.x_register);
}

fn tay(cpu: &mut Cpu) {
    cpu.y_register = cpu.accumulator;
    cpu.update_negative_and_zero_flags(cpu.y_register);
}

fn txa(cpu: &mut Cpu) {
    cpu.accumulator = cpu.x_register;
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn tya(cpu: &mut Cpu) {
    cpu.accumulator = cpu.y_register;
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn tsx(cpu: &mut Cpu) {
    cpu.x_register = cpu.stack_pointer;
    cpu.update_negative_and_zero_flags(cpu.x_register)
}

fn txs(cpu: &mut Cpu) { cpu.stack_pointer = cpu.x_register; }

fn clc(cpu: &mut Cpu) { cpu.clear_carry_flag(); }

fn cld(cpu: &mut Cpu) { cpu.clear_decimal_flag(); }

fn cli(cpu: &mut Cpu) { cpu.clear_interrupt_disable(); }

fn clv(cpu: &mut Cpu) { cpu.clear_overflow_flag(); }

fn sec(cpu: &mut Cpu) { cpu.set_carry_flag(); }

fn sed(cpu: &mut Cpu) { cpu.set_decimal_flag(); }

fn sei(cpu: &mut Cpu) { cpu.set_interrupt_disable(); }

fn dex(cpu: &mut Cpu) {
    let mod_value = cpu.x_register.wrapping_sub(1);
    cpu.x_register = mod_value;
    cpu.update_negative_and_zero_flags(cpu.x_register);
}

fn dey(cpu: &mut Cpu) {
    let mod_value = cpu.y_register.wrapping_sub(1);
    cpu.y_register = mod_value;
    cpu.update_negative_and_zero_flags(cpu.y_register);
}

fn inx(cpu: &mut Cpu) {
    let mod_value = cpu.x_register.wrapping_add(1);
    cpu.x_register = mod_value;
    cpu.update_negative_and_zero_flags(cpu.x_register);
}

fn iny(cpu: &mut Cpu) {
    let mod_value = cpu.y_register.wrapping_add(1);
    cpu.y_register = mod_value;
    cpu.update_negative_and_zero_flags(cpu.y_register);
}

fn cmp(cpu: &mut Cpu) {
    let target_value = cpu.temp;

    if target_value == cpu.accumulator {
        cpu.set_zero_flag();
    } else {
        cpu.clear_zero_flag();
    }

    if cpu.accumulator >= target_value {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }

    if cpu.accumulator.wrapping_sub(target_value) & NEGATIVE_BIT != 0 {
        cpu.set_negative_flag();
    } else {
        cpu.clear_negative_flag();
    }
}

fn cpx(cpu: &mut Cpu) {
    let target_value = cpu.temp;

    if target_value == cpu.x_register {
        cpu.set_zero_flag();
    } else {
        cpu.clear_zero_flag();
    }

    if cpu.x_register >= target_value {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }

    if cpu.x_register.wrapping_sub(target_value) & NEGATIVE_BIT != 0 {
        cpu.set_negative_flag();
    } else {
        cpu.clear_negative_flag();
    }
}

fn cpy(cpu: &mut Cpu) {
    let target_value = cpu.temp;

    if target_value == cpu.y_register {
        cpu.set_zero_flag();
    } else {
        cpu.clear_zero_flag();
    }

    if cpu.y_register >= target_value {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }

    if cpu.y_register.wrapping_sub(target_value) & NEGATIVE_BIT != 0 {
        cpu.set_negative_flag();
    } else {
        cpu.clear_negative_flag();
    }
}

fn and(cpu: &mut Cpu) {
    let target_val = cpu.temp;
    cpu.accumulator &= target_val;
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn eor(cpu: &mut Cpu) {
    let target_val = cpu.temp;
    cpu.accumulator ^= target_val;
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn ora(cpu: &mut Cpu) {
    let target_val = cpu.temp;
    cpu.accumulator |= target_val;
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn sbc(cpu: &mut Cpu) {
    let target_value = cpu.temp;
    let carry_in = cpu.processor_status & CARRY_BIT;

    let acc_check = cpu.accumulator;

    let value = target_value ^ LOWER_BYTE as u8;
    let sum = cpu.accumulator as u16 + value as u16 + carry_in as u16;
    let result = sum as u8;

    cpu.accumulator = result;

    if sum > LOWER_BYTE {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }

    if ((acc_check ^ result) & (value ^ result) & NEGATIVE_BIT) != 0 {
        cpu.set_overflow_flag();
    } else {
        cpu.clear_overflow_flag();
    }

    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn bit(cpu: &mut Cpu) {
    let target_val = cpu.temp;
    let res = cpu.accumulator & target_val;
    cpu.update_zero_flag(res);

    if target_val & NEGATIVE_BIT != 0 {
        cpu.set_negative_flag()
    } else {
        cpu.clear_negative_flag()
    }

    if target_val & OVERFLOW_BIT != 0 {
        cpu.set_overflow_flag()
    } else {
        cpu.clear_overflow_flag()
    }
}

fn inc(cpu: &mut Cpu) {
    let target_value = cpu.temp;
    let mod_value = target_value.wrapping_add(1);
    cpu.temp = mod_value;
    cpu.update_negative_and_zero_flags(cpu.temp);
}

fn dec(cpu: &mut Cpu) {
    let target_value = cpu.temp;
    let mod_value = target_value.wrapping_sub(1);
    cpu.temp = mod_value;
    cpu.update_negative_and_zero_flags(cpu.temp);
}

fn branch(cpu: &mut Cpu, condition: Condition) {
    if cpu.check_condition(condition) {
        cpu.op_queue.push(MicroOp::BranchIncrement(Source::LO))
    }
}

fn isb(cpu: &mut Cpu) {
    // Inc
    let target_value = cpu.get_src_value(&Source::TEMP);
    let mod_value = target_value.wrapping_add(1);
    cpu.write_to_target(&Target::TEMP, mod_value);
    cpu.update_negative_and_zero_flags(mod_value);

    // SBC
    let carry_in = cpu.processor_status & CARRY_BIT;

    let acc_check = cpu.accumulator;

    let value = mod_value ^ LOWER_BYTE as u8;
    let sum = cpu.accumulator as u16 + value as u16 + carry_in as u16;
    let result = sum as u8;

    cpu.accumulator = result;

    if sum > LOWER_BYTE {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }

    if ((acc_check ^ result) & (value ^ result) & NEGATIVE_BIT) != 0 {
        cpu.set_overflow_flag();
    } else {
        cpu.clear_overflow_flag();
    }

    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn alr(cpu: &mut Cpu) {
    let target_val = cpu.temp;
    cpu.accumulator &= target_val;
    cpu.update_negative_and_zero_flags(cpu.accumulator);

    cpu.accumulator = cpu.shift_right(cpu.accumulator);
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn anc(cpu: &mut Cpu) {
    let target_val = cpu.temp;
    cpu.accumulator &= target_val;
    cpu.update_negative_and_zero_flags(cpu.accumulator);

    if cpu.accumulator & NEGATIVE_BIT != 0 {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }
}

fn ane(cpu: &mut Cpu) {
    cpu.accumulator |= cpu.ane_constant;
    cpu.accumulator &= cpu.x_register;
    cpu.accumulator &= cpu.temp;
}

fn arr(cpu: &mut Cpu) {
    let target_val = cpu.temp;
    cpu.accumulator &= target_val;

    cpu.accumulator = (cpu.accumulator >> 1)
        | (match cpu.get_carry_flag() {
            true => 1,
            false => 0,
        } << 7);

    cpu.update_negative_and_zero_flags(cpu.accumulator);

    if ((cpu.accumulator >> 6) & 1) != 0 {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }

    if (((cpu.accumulator >> 6) & 1) ^ ((cpu.accumulator >> 5) & 1)) != 0 {
        cpu.set_overflow_flag();
    } else {
        cpu.clear_overflow_flag()
    }
}

fn dcp(cpu: &mut Cpu) {
    dec(cpu);
    cmp(cpu)
}

fn las(cpu: &mut Cpu) {
    let res = cpu.temp & cpu.stack_pointer;
    cpu.accumulator = res;
    cpu.x_register = res;
    cpu.stack_pointer = res;
}

fn lax(cpu: &mut Cpu) {
    cpu.accumulator = cpu.temp;
    cpu.x_register = cpu.temp;
    cpu.update_negative_and_zero_flags(cpu.accumulator)
}

fn lxa(cpu: &mut Cpu) {
    cpu.accumulator = (cpu.accumulator | cpu.ane_constant) & cpu.temp;
    cpu.x_register = cpu.accumulator;
    cpu.update_negative_and_zero_flags(cpu.accumulator)
}

fn rla(cpu: &mut Cpu) {
    let target_value = cpu.temp;
    let res = cpu.rotate_left(target_value);
    cpu.temp = res;
    cpu.accumulator &= res;
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn rra(cpu: &mut Cpu) {
    let target_value = cpu.temp;
    let res = cpu.rotate_right(target_value);
    cpu.temp = res;
    adc(cpu);
}

fn sax(cpu: &mut Cpu) {
    let res = cpu.accumulator & cpu.x_register;
    cpu.temp = res;
}

fn sbx(cpu: &mut Cpu) {
    let t = cpu.accumulator & cpu.x_register;
    let r = t.wrapping_sub(cpu.temp);
    cpu.x_register = r;

    if t >= cpu.temp {
        cpu.set_carry_flag();
    } else {
        cpu.clear_carry_flag();
    }

    if r == 0 {
        cpu.set_zero_flag();
    } else {
        cpu.clear_zero_flag();
    }

    if (r & NEGATIVE_BIT) != 0 {
        cpu.set_negative_flag();
    } else {
        cpu.clear_negative_flag();
    }
}

fn sha(cpu: &mut Cpu) { cpu.temp = cpu.accumulator & cpu.x_register & cpu.hi.wrapping_add(1); }

fn shx(cpu: &mut Cpu) {
    if !cpu.lo.overflowing_sub(cpu.y_register).1 {
        cpu.temp = cpu.x_register & cpu.hi.wrapping_add(1);
    } else {
        let res = cpu.x_register & cpu.hi;
        cpu.hi = res;
        cpu.temp = res;
    }
}

fn shy(cpu: &mut Cpu) {
    if !cpu.lo.overflowing_sub(cpu.x_register).1 {
        cpu.temp = cpu.y_register & cpu.hi.wrapping_add(1);
    } else {
        let res = cpu.y_register & cpu.hi;
        cpu.hi = res;
        cpu.temp = res;
    }
}

fn slo(cpu: &mut Cpu) {
    let target_value = cpu.temp;
    let res = cpu.shift_left(target_value);
    cpu.temp = res;

    cpu.accumulator |= res;
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn sre(cpu: &mut Cpu) {
    let target_value = cpu.temp;
    let res = cpu.shift_right(target_value);
    cpu.temp = res;

    cpu.accumulator ^= res;
    cpu.update_negative_and_zero_flags(cpu.accumulator);
}

fn tas(cpu: &mut Cpu) {
    cpu.stack_pointer = cpu.accumulator & cpu.x_register;
    cpu.temp = cpu.accumulator & cpu.x_register & cpu.hi.wrapping_add(1);
}

fn jam(cpu: &mut Cpu) { cpu.is_halted = true }

fn copy(cpu: &mut Cpu, source: AddressSource, target: Target) {
    let Some(address) = cpu.get_u16_address(&source) else {
        unreachable!()
    };

    if target == Target::IrqVecCandidate {
        cpu.current_irq_vec = address
    }
}
