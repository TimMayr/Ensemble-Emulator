use crate::emulation::board::{Board, CpuBus};
use crate::emulation::cpu::{
    OpType, Source, CARRY_BIT, DECIMAL_BIT, IRQ_BIT, NEGATIVE_BIT, OVERFLOW_BIT, UNUSED_BIT,
    ZERO_BIT,
};
use crate::emulation::opcode;
use crate::emulation::opcode::{get_opcode, OpCode};
use crate::emulation::savestate::SaveState;
use crate::util::add_to_low_byte;

pub struct TraceLog {
    pub log: String,
}
impl Default for TraceLog {
    fn default() -> Self { Self::new() }
}

impl TraceLog {
    pub fn new() -> Self {
        Self {
            log: String::from(""),
        }
    }

    pub fn trace(&mut self, nes: SaveState) {
        let board = Board::from(&nes.board);
        let cpu = &board.cpu;
        let current_opcode = get_opcode(cpu.current_opcode.unwrap().opcode).unwrap();

        let relevant_mem_start = cpu.program_counter.wrapping_sub(1);
        let relevant_mem_end =
            relevant_mem_start.wrapping_add(opcode::get_bytes_for_opcode(current_opcode) as u16);
        let relevant_mem: Vec<u8> = board.get_range(relevant_mem_start..=relevant_mem_end);
        let mem_formatted = relevant_mem
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<Vec<_>>()
            .join(" ");

        let descriptor_string = get_opcode_descriptor(current_opcode, &board);

        self.log += format!(
            "{:04X}  {:<8} {:>4} {:<27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}\n",
            cpu.program_counter.wrapping_sub(1),
            mem_formatted,
            current_opcode.name,
            descriptor_string,
            cpu.accumulator,
            cpu.x_register,
            cpu.y_register,
            cpu.processor_status | UNUSED_BIT,
            cpu.stack_pointer,
            nes.total_cycles / 12
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
}

pub fn get_str_for_src(source: Source) -> String {
    match source {
        Source::X => String::from("X"),
        Source::Y => String::from("Y"),
        Source::None => String::new(),
        _ => String::new(),
    }
}

pub fn get_opcode_descriptor(opcode: OpCode, board: &Board) -> String {
    match opcode.op_type {
        OpType::ImmediateAddressing(..) => {
            format!(
                "#${:02X}",
                CpuBus::read_debug(board, board.cpu.program_counter)
            )
        }
        OpType::AccumulatorOrImplied(..) => {
            if board.cpu.current_opcode.is_none() {
                return String::new();
            }

            match opcode {
                OpCode {
                    name: "LSR" | "ASL" | "ROR" | "ROL",
                    ..
                } => String::from("A"),
                _ => String::new(),
            }
        }
        OpType::AbsoluteRead(..) | OpType::AbsoluteRMW(..) | OpType::AbsoluteWrite(..) => {
            let address = ((CpuBus::read_debug(board, board.cpu.program_counter.wrapping_add(1))
                as u16)
                << 8)
                | CpuBus::read_debug(board, board.cpu.program_counter) as u16;
            format!(
                "${:04X} = {:02X}",
                address,
                CpuBus::read_debug(board, address)
            )
        }
        OpType::AbsoluteIndexRead(source, ..)
        | OpType::AbsoluteIndexRMW(source, ..)
        | OpType::AbsoluteIndexWrite(_, source, ..) => {
            let address =
                (CpuBus::read_debug(board, board.cpu.program_counter.wrapping_add(1)) as u16) << 8
                    | (CpuBus::read_debug(board, board.cpu.program_counter) as u16);

            let reg_string = get_str_for_src(source);

            let val = match source {
                Source::X => board.cpu.x_register,
                Source::Y => board.cpu.y_register,
                _ => 0,
            };

            let effective_address = address.wrapping_add(val as u16);

            format!(
                "${:04X},{} @ {:04X} = {:02X}",
                address,
                reg_string,
                effective_address,
                CpuBus::read_debug(board, effective_address)
            )
        }
        OpType::ZeroPageRead(..) | OpType::ZeroPageRMW(..) | OpType::ZeroPageWrite(..) => {
            let address = CpuBus::read_debug(board, board.cpu.program_counter);
            format!(
                "${:02X} = {:02X}",
                address,
                CpuBus::read_debug(board, address as u16)
            )
        }
        OpType::ZeroPageIndexRead(source, ..)
        | OpType::ZeroPageIndexRMW(source, ..)
        | OpType::ZeroPageIndexWrite(_, source, ..) => {
            let address = CpuBus::read_debug(board, board.cpu.program_counter);

            let reg_string = get_str_for_src(source);

            let val = match source {
                Source::X => board.cpu.x_register,
                Source::Y => board.cpu.y_register,
                _ => 0,
            };

            let effective_address = address.wrapping_add(val);

            format!(
                "${:02X},{} @ {:02X} = {:02X}",
                address,
                reg_string,
                effective_address,
                CpuBus::read_debug(board, effective_address as u16)
            )
        }
        OpType::IndexedIndirectRead(..)
        | OpType::IndexedIndirectRMW(_)
        | OpType::IndexedIndirectWrite(..) => {
            let address = CpuBus::read_debug(board, board.cpu.program_counter);

            let effective_address = address.wrapping_add(board.cpu.x_register);
            let lookup_addr =
                ((CpuBus::read_debug(board, effective_address.wrapping_add(1) as u16) as u16) << 8)
                    | CpuBus::read_debug(board, effective_address as u16) as u16;

            let val = CpuBus::read_debug(board, lookup_addr);

            format!(
                "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                address, effective_address, lookup_addr, val
            )
        }
        OpType::IndirectIndexedRead(..)
        | OpType::IndirectIndexedRMW(_)
        | OpType::IndirectIndexedWrite(..) => {
            let address = CpuBus::read_debug(board, board.cpu.program_counter);

            let effective_addr =
                ((CpuBus::read_debug(board, address.wrapping_add(1) as u16) as u16) << 8)
                    | CpuBus::read_debug(board, address as u16) as u16;

            let lookup_addr = effective_addr.wrapping_add(board.cpu.y_register as u16);

            let val = CpuBus::read_debug(board, lookup_addr);

            format!(
                "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                address, effective_addr, lookup_addr, val
            )
        }
        OpType::BRK(_) | OpType::RTI(..) | OpType::RTS(..) | OpType::PH(..) | OpType::PL(..) => {
            String::new()
        }
        OpType::JSR(_) | OpType::JmpAbsolute(_) => {
            let address = ((CpuBus::read_debug(board, board.cpu.program_counter.wrapping_add(1))
                as u16)
                << 8)
                | CpuBus::read_debug(board, board.cpu.program_counter) as u16;
            format!("${:04X}", address)
        }
        OpType::JmpIndirect(_) => {
            let address = ((CpuBus::read_debug(board, board.cpu.program_counter.wrapping_add(1))
                as u16)
                << 8)
                | CpuBus::read_debug(board, board.cpu.program_counter) as u16;

            let val = ((CpuBus::read_debug(board, add_to_low_byte(address, 1)) as u16) << 8)
                | CpuBus::read_debug(board, address) as u16;
            format!("(${:04X}) = {:04X}", address, val)
        }
        OpType::Relative(..) => {
            let base_address = board.cpu.program_counter.wrapping_add(1);
            let offset = CpuBus::read_debug(board, board.cpu.program_counter) as i8;
            let val = base_address.wrapping_add(offset as i16 as u16);

            format!("${:04X}", val)
        }
    }
}
