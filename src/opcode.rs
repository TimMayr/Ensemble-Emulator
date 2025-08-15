use crate::cpu::AddressingMode;
use crate::cpu::AddressingMode::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub static OPCODES: OnceLock<Vec<OpCode>> = OnceLock::new();
pub static OPCODES_MAP: OnceLock<HashMap<u8, &'static OpCode>> = OnceLock::new();

#[derive(Debug)]
pub struct OpCode {
    pub opcode: u8,
    pub name: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}

pub fn init() -> HashMap<u8, &'static OpCode> {
    OPCODES
        .set(vec![
            //ADC
            OpCode::new(0x69, "ADC", 2, 2, Immediate),
            OpCode::new(0x65, "ADC", 2, 3, ZeroPage),
            OpCode::new(0x75, "ADC", 2, 4, ZeroPageX),
            OpCode::new(0x6D, "ADC", 3, 4, Absolute),
            OpCode::new(0x7D, "ADC", 3, 4, AbsoluteX),
            OpCode::new(0x79, "ADC", 3, 4, AbsoluteY),
            OpCode::new(0x61, "ADC", 2, 6, IndirectX),
            OpCode::new(0x71, "ADC", 2, 5, IndirectY),
            //AND
            OpCode::new(0x29, "AND", 2, 2, Immediate),
            OpCode::new(0x25, "AND", 2, 3, ZeroPage),
            OpCode::new(0x35, "AND", 2, 4, ZeroPageX),
            OpCode::new(0x2D, "AND", 3, 4, Absolute),
            OpCode::new(0x3D, "AND", 3, 4, AbsoluteX),
            OpCode::new(0x39, "AND", 3, 4, AbsoluteY),
            OpCode::new(0x21, "AND", 2, 6, IndirectX),
            OpCode::new(0x31, "AND", 2, 5, IndirectY),
            //ASL
            OpCode::new(0x0A, "ASL", 1, 2, Accumulator),
            OpCode::new(0x06, "ASL", 2, 5, ZeroPage),
            OpCode::new(0x16, "ASL", 2, 6, ZeroPageX),
            OpCode::new(0x0E, "ASL", 3, 6, Absolute),
            OpCode::new(0x1E, "ASL", 3, 7, AbsoluteX),
            //BCC
            OpCode::new(0x90, "BCC", 2, 2, Relative),
            //BCS
            OpCode::new(0xB0, "BCS", 2, 2, Relative),
            //BEQ
            OpCode::new(0xF0, "BEQ", 2, 2, Relative),
            //BIT
            OpCode::new(0x24, "BIT", 2, 3, ZeroPage),
            OpCode::new(0x2c, "BIT", 3, 4, Absolute),
            //BMI
            OpCode::new(0x30, "BMI", 2, 2, Relative),
            //BNE
            OpCode::new(0xD0, "BNE", 2, 2, Relative),
            //BPL
            OpCode::new(0x10, "BPL", 2, 2, Relative),
            //BRK
            OpCode::new(0x00, "BRK", 2, 7, Implied),
            //BVC
            OpCode::new(0x50, "BVC", 2, 2, Relative),
            //BVS
            OpCode::new(0x70, "BVS", 2, 2, Relative),
            //CLC
            OpCode::new(0x18, "CLC", 1, 2, Implied),
            //CLD
            OpCode::new(0xD8, "CLD", 1, 2, Implied),
            //CLI
            OpCode::new(0x58, "CLI", 1, 2, Implied),
            //CLV
            OpCode::new(0xb8, "CLV", 1, 2, Implied),
            //CMP
            OpCode::new(0xC9, "CMP", 2, 2, Immediate),
            OpCode::new(0xC5, "CMP", 2, 3, ZeroPage),
            OpCode::new(0xD5, "CMP", 2, 4, ZeroPageX),
            OpCode::new(0xCD, "CMP", 3, 4, Absolute),
            OpCode::new(0xDD, "CMP", 3, 4, AbsoluteX),
            OpCode::new(0xD9, "CMP", 3, 4, AbsoluteY),
            OpCode::new(0xC1, "CMP", 2, 6, IndirectX),
            OpCode::new(0xD1, "CMP", 2, 5, IndirectY),
            //CPX
            OpCode::new(0xE0, "CPX", 2, 2, Immediate),
            OpCode::new(0xE4, "CPX", 2, 3, ZeroPage),
            OpCode::new(0xEC, "CPX", 3, 4, Absolute),
            //CPY
            OpCode::new(0xC0, "CPY", 2, 2, Immediate),
            OpCode::new(0xC4, "CPY", 2, 3, ZeroPage),
            OpCode::new(0xCC, "CPY", 3, 4, Absolute),
            //DEC
            OpCode::new(0xC6, "DEC", 2, 5, ZeroPage),
            OpCode::new(0xD6, "DEC", 2, 6, ZeroPageX),
            OpCode::new(0xCE, "DEC", 3, 6, Absolute),
            OpCode::new(0xDE, "DEC", 3, 7, AbsoluteX),
            //DEX
            OpCode::new(0xCA, "DEX", 1, 2, Implied),
            //DEY
            OpCode::new(0x88, "DEY", 1, 2, Implied),
            //EOR
            OpCode::new(0x49, "EOR", 2, 2, Immediate),
            OpCode::new(0x45, "EOR", 2, 3, ZeroPage),
            OpCode::new(0x55, "EOR", 2, 4, ZeroPageX),
            OpCode::new(0x4D, "EOR", 3, 4, Absolute),
            OpCode::new(0x5D, "EOR", 3, 4, AbsoluteX),
            OpCode::new(0x59, "EOR", 3, 4, AbsoluteY),
            OpCode::new(0x41, "EOR", 2, 6, IndirectX),
            OpCode::new(0x51, "EOR", 2, 5, IndirectY),
            //INC
            OpCode::new(0xE6, "INC", 2, 5, ZeroPage),
            OpCode::new(0xF6, "INC", 2, 6, ZeroPageX),
            OpCode::new(0xEE, "INC", 3, 6, Absolute),
            OpCode::new(0xFE, "INC", 3, 7, AbsoluteX),
            //INX
            OpCode::new(0xE8, "INX", 1, 2, Implied),
            //INY
            OpCode::new(0xC8, "INY", 1, 2, Implied),
            //JMP
            OpCode::new(0x4C, "JMP", 3, 3, Absolute),
            OpCode::new(0x6C, "JMP", 3, 5, Indirect),
            //JSR
            OpCode::new(0x20, "JSR", 3, 6, Absolute),
            //LDA
            OpCode::new(0xA9, "LDA", 2, 2, Immediate),
            OpCode::new(0xA5, "LDA", 2, 3, ZeroPage),
            OpCode::new(0xB5, "LDA", 2, 4, ZeroPageX),
            OpCode::new(0xAD, "LDA", 3, 4, Absolute),
            OpCode::new(0xBD, "LDA", 3, 4, AbsoluteX),
            OpCode::new(0xB9, "LDA", 3, 4, AbsoluteY),
            OpCode::new(0xA1, "LDA", 2, 6, IndirectX),
            OpCode::new(0xB1, "LDA", 2, 5, IndirectY),
            //LDX
            OpCode::new(0xA2, "LDX", 2, 2, Immediate),
            OpCode::new(0xA6, "LDX", 2, 3, ZeroPage),
            OpCode::new(0xB6, "LDX", 2, 4, ZeroPageY),
            OpCode::new(0xAE, "LDX", 3, 4, Absolute),
            OpCode::new(0xBE, "LDX", 3, 4, AbsoluteY),
            //LDY
            OpCode::new(0xA0, "LDY", 2, 2, Immediate),
            OpCode::new(0xA4, "LDY", 2, 3, ZeroPage),
            OpCode::new(0xB4, "LDY", 2, 4, ZeroPageX),
            OpCode::new(0xAC, "LDY", 3, 4, Absolute),
            OpCode::new(0xBC, "LDY", 3, 4, AbsoluteX),
            //LSR
            OpCode::new(0x4A, "LSR", 1, 2, Accumulator),
            OpCode::new(0x46, "LSR", 2, 5, ZeroPage),
            OpCode::new(0x56, "LSR", 2, 6, ZeroPageX),
            OpCode::new(0x4E, "LSR", 3, 6, Absolute),
            OpCode::new(0x5E, "LSR", 3, 7, AbsoluteX),
            //NOP
            OpCode::new(0xEA, "NOP", 1, 2, Implied),
            //ORA
            OpCode::new(0x09, "ORA", 2, 2, Immediate),
            OpCode::new(0x05, "ORA", 2, 3, ZeroPage),
            OpCode::new(0x15, "ORA", 2, 4, ZeroPageX),
            OpCode::new(0x0D, "ORA", 3, 4, Absolute),
            OpCode::new(0x1D, "ORA", 3, 4, AbsoluteX),
            OpCode::new(0x19, "ORA", 3, 4, AbsoluteY),
            OpCode::new(0x01, "ORA", 2, 6, IndirectX),
            OpCode::new(0x11, "ORA", 2, 5, IndirectY),
            //PHA
            OpCode::new(0x48, "PHA", 1, 3, Implied),
            //PHP
            OpCode::new(0x08, "PHP", 1, 3, Implied),
            //PLA
            OpCode::new(0x68, "PLA", 1, 4, Implied),
            //PLP
            OpCode::new(0x28, "PLP", 1, 4, Implied),
            //ROL
            OpCode::new(0x2A, "ROL", 1, 2, Accumulator),
            OpCode::new(0x26, "ROL", 2, 5, ZeroPage),
            OpCode::new(0x36, "ROL", 2, 6, ZeroPageX),
            OpCode::new(0x2E, "ROL", 3, 6, Absolute),
            OpCode::new(0x3E, "ROL", 3, 7, AbsoluteX),
            //ROR
            OpCode::new(0x6A, "ROR", 1, 2, Accumulator),
            OpCode::new(0x66, "ROR", 2, 5, ZeroPage),
            OpCode::new(0x76, "ROR", 2, 6, ZeroPageX),
            OpCode::new(0x6E, "ROR", 3, 6, Absolute),
            OpCode::new(0x7E, "ROR", 3, 7, AbsoluteX),
            //RTI
            OpCode::new(0x40, "RTI", 1, 6, Implied),
            //RTS
            OpCode::new(0x60, "RTS", 1, 6, Implied),
            //SBC
            OpCode::new(0xE9, "SBC", 2, 2, Immediate),
            OpCode::new(0xE5, "SBC", 2, 3, ZeroPage),
            OpCode::new(0xF5, "SBC", 2, 4, ZeroPageX),
            OpCode::new(0xED, "SBC", 3, 4, Absolute),
            OpCode::new(0xFD, "SBC", 3, 4, AbsoluteX),
            OpCode::new(0xF9, "SBC", 3, 4, AbsoluteY),
            OpCode::new(0xE1, "SBC", 2, 6, IndirectX),
            OpCode::new(0xF1, "SBC", 2, 5, IndirectY),
            //SEC
            OpCode::new(0x38, "SEC", 1, 2, Implied),
            //SED
            OpCode::new(0xF8, "SED", 1, 2, Implied),
            //SEI
            OpCode::new(0x78, "SEI", 1, 2, Implied),
            //STA
            OpCode::new(0x85, "STA", 2, 3, ZeroPage),
            OpCode::new(0x95, "STA", 2, 4, ZeroPageX),
            OpCode::new(0x8D, "STA", 3, 4, Absolute),
            OpCode::new(0x9D, "STA", 3, 5, AbsoluteX),
            OpCode::new(0x99, "STA", 3, 5, AbsoluteY),
            OpCode::new(0x81, "STA", 2, 6, IndirectX),
            OpCode::new(0x91, "STA", 2, 6, IndirectY),
            //STX
            OpCode::new(0x86, "STX", 2, 3, ZeroPage),
            OpCode::new(0x96, "STX", 2, 4, ZeroPageY),
            OpCode::new(0x8E, "STX", 3, 4, Absolute),
            //STY
            OpCode::new(0x84, "STY", 2, 3, ZeroPage),
            OpCode::new(0x94, "STY", 2, 4, ZeroPageX),
            OpCode::new(0x8C, "STY", 3, 4, Absolute),
            //TAX
            OpCode::new(0xAA, "TAX", 1, 2, Implied),
            //TAY
            OpCode::new(0xA8, "TAY", 1, 2, Implied),
            //TSX
            OpCode::new(0xBA, "TSX", 1, 2, Implied),
            //TXA
            OpCode::new(0x8A, "TXA", 1, 2, Implied),
            //TXS
            OpCode::new(0x9A, "TXS", 1, 2, Implied),
            //TYA
            OpCode::new(0x98, "TYA", 1, 2, Implied),
            //Illegal Opcodes
            OpCode::new(0xFF, "ISC", 2, 7, AbsoluteX),
        ])
        .ok();

    let mut map = HashMap::new();

    for opcode in OPCODES.get().unwrap() {
        map.insert(opcode.opcode, opcode);
    }

    map
}

impl OpCode {
    pub fn new(
        opcode: u8,
        name: &'static str,
        bytes: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        Self {
            opcode,
            name,
            bytes,
            cycles,
            addressing_mode,
        }
    }
}

impl Default for OpCode {
    fn default() -> Self {
        Self {
            opcode: 0xB3,
            name: "PNC",
            bytes: 1,
            cycles: 0,
            addressing_mode: Immediate,
        }
    }
}
