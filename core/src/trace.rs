use std::fs::OpenOptions;
use std::io::Write;

use crate::emulation::cpu::{
    Cpu, OpType, Source, CARRY_BIT, DECIMAL_BIT, IRQ_BIT, NEGATIVE_BIT, OVERFLOW_BIT, UNUSED_BIT,
    ZERO_BIT,
};
use crate::emulation::nes::Nes;
use crate::emulation::opcode;
use crate::emulation::opcode::OpCode;
use crate::util::add_to_low_byte;

pub struct TraceLog {
    pub log: String,
    pub output: String,
}
impl Default for TraceLog {
    fn default() -> Self { Self::new(String::from("./trace-log.txt")) }
}

impl TraceLog {
    pub fn new(path: String) -> Self {
        Self {
            log: String::from(""),
            output: path,
        }
    }

    pub fn trace(&mut self, nes: &Nes) {
        let mut cpu = nes.cpu.clone();
        let ppu = nes.ppu.borrow();
        let current_opcode = cpu.current_opcode.unwrap();

        let relevant_mem_start = cpu.program_counter.wrapping_sub(1);
        let relevant_mem_end =
            relevant_mem_start.wrapping_add(opcode::get_bytes_for_opcode(current_opcode) as u16);
        let relevant_mem = cpu.get_memory_debug(Some(relevant_mem_start..=relevant_mem_end));
        let mem_formatted = relevant_mem
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<Vec<_>>()
            .join(" ");

        let descriptor_string = get_opcode_descriptor(current_opcode, &mut cpu);

        self.log += format!(
            "{:04X}  {:<8} {:>4} {:<27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:3},{:3} CYC:{}\n",
            cpu.program_counter.wrapping_sub(1),
            mem_formatted,
            current_opcode.name,
            descriptor_string,
            cpu.accumulator,
            cpu.x_register,
            cpu.y_register,
            cpu.processor_status | UNUSED_BIT,
            cpu.stack_pointer,
            ppu.scanline,
            ppu.dot,
            (cpu.master_cycle / 12 ).wrapping_sub(1)
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
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.output.clone())
            .unwrap_or_else(|e| panic!("Error saving log: \n{}", e));

        file.write_all(self.log.as_bytes()).expect("error");
    }
}

pub fn get_str_for_src(source: Source) -> String {
    match source {
        Source::X => String::from("X"),
        Source::Y => String::from("Y"),
        Source::None => String::new(),
        _ => String::new(),
    }
}

pub fn get_opcode_descriptor(opcode: OpCode, cpu: &mut Cpu) -> String {
    match opcode.op_type {
        OpType::ImmediateAddressing(..) => {
            format!("#${:02X}", cpu.memory.mem_read_debug(cpu.program_counter))
        }
        OpType::AccumulatorOrImplied(..) => {
            let Some(opcode) = cpu.current_opcode else {
                return String::new();
            };

            match opcode {
                OpCode {
                    name: "LSR" | "ASL" | "ROR" | "ROL",
                    ..
                } => String::from("A"),
                _ => String::new(),
            }
        }
        OpType::AbsoluteRead(..) | OpType::AbsoluteRMW(..) | OpType::AbsoluteWrite(..) => {
            let address = ((cpu
                .memory
                .mem_read_debug(cpu.program_counter.wrapping_add(1))
                as u16)
                << 8)
                | cpu.memory.mem_read_debug(cpu.program_counter) as u16;
            format!(
                "${:04X} = {:02X}",
                address,
                cpu.memory.mem_read_debug(address)
            )
        }
        OpType::AbsoluteIndexRead(source, ..)
        | OpType::AbsoluteIndexRMW(source, ..)
        | OpType::AbsoluteIndexWrite(_, source, ..) => {
            let address = ((cpu
                .memory
                .mem_read_debug(cpu.program_counter.wrapping_add(1))
                as u16)
                << 8)
                | cpu.memory.mem_read_debug(cpu.program_counter) as u16;

            let reg_string = get_str_for_src(source);

            let effective_address = address.wrapping_add(cpu.get_src_value(&source) as u16);

            format!(
                "${:04X},{} @ {:04X} = {:02X}",
                address,
                reg_string,
                effective_address,
                cpu.memory.mem_read_debug(effective_address)
            )
        }
        OpType::ZeroPageRead(..) | OpType::ZeroPageRMW(..) | OpType::ZeroPageWrite(..) => {
            let address = cpu.memory.mem_read_debug(cpu.program_counter);
            format!(
                "${:02X} = {:02X}",
                address,
                cpu.memory.mem_read_debug(address as u16)
            )
        }
        OpType::ZeroPageIndexRead(source, ..)
        | OpType::ZeroPageIndexRMW(source, ..)
        | OpType::ZeroPageIndexWrite(_, source, ..) => {
            let address = cpu.memory.mem_read_debug(cpu.program_counter);

            let reg_string = get_str_for_src(source);

            let effective_address = address.wrapping_add(cpu.get_src_value(&source));

            format!(
                "${:02X},{} @ {:02X} = {:02X}",
                address,
                reg_string,
                effective_address,
                cpu.memory.mem_read_debug(effective_address as u16)
            )
        }
        OpType::IndexedIndirectRead(..)
        | OpType::IndexedIndirectRMW(_)
        | OpType::IndexedIndirectWrite(..) => {
            let address = cpu.memory.mem_read_debug(cpu.program_counter);

            let effective_address = address.wrapping_add(cpu.get_src_value(&Source::X));
            let lookup_addr = ((cpu
                .memory
                .mem_read_debug((effective_address.wrapping_add(1)) as u16)
                as u16)
                << 8)
                | cpu.memory.mem_read_debug(effective_address as u16) as u16;

            let val = cpu.memory.mem_read_debug(lookup_addr);

            format!(
                "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                address, effective_address, lookup_addr, val
            )
        }
        OpType::IndirectIndexedRead(..)
        | OpType::IndirectIndexedRMW(_)
        | OpType::IndirectIndexedWrite(..) => {
            let address = cpu.memory.mem_read_debug(cpu.program_counter);

            let effective_addr =
                ((cpu.memory.mem_read_debug((address.wrapping_add(1)) as u16) as u16) << 8)
                    | cpu.memory.mem_read_debug(address as u16) as u16;

            let lookup_addr = effective_addr.wrapping_add(cpu.get_src_value(&Source::Y) as u16);

            let val = cpu.memory.mem_read_debug(lookup_addr);

            format!(
                "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                address, effective_addr, lookup_addr, val
            )
        }
        OpType::BRK(_) | OpType::RTI(..) | OpType::RTS(..) | OpType::PH(..) | OpType::PL(..) => {
            String::new()
        }
        OpType::JSR(_) | OpType::JmpAbsolute(_) => {
            let address = ((cpu
                .memory
                .mem_read_debug(cpu.program_counter.wrapping_add(1))
                as u16)
                << 8)
                | cpu.memory.mem_read_debug(cpu.program_counter) as u16;
            format!("${:04X}", address)
        }
        OpType::JmpIndirect(_) => {
            let address = ((cpu
                .memory
                .mem_read_debug(cpu.program_counter.wrapping_add(1))
                as u16)
                << 8)
                | cpu.memory.mem_read_debug(cpu.program_counter) as u16;

            let val = ((cpu.memory.mem_read_debug(add_to_low_byte(address, 1)) as u16) << 8)
                | cpu.memory.mem_read_debug(address) as u16;
            format!("(${:04X}) = {:04X}", address, val)
        }
        OpType::Relative(..) => {
            let base_address = cpu.program_counter.wrapping_add(1);
            let offset = cpu.memory.mem_read_debug(cpu.program_counter) as i8;
            let val = base_address.wrapping_add(offset as i16 as u16);

            format!("${:04X}", val)
        }
    }
}
