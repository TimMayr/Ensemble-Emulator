use crate::emulation::cpu::OpType::{
    AbsoluteIndexRead, AbsoluteRMW, AbsoluteRead, AbsoluteWrite, AccumulatorOrImplied, ImmediateAddressing,
    IndexedIndirect, IndirectIndexed, JmpAbsolute, ZeroPageIndexRMW, ZeroPageIndexRead, ZeroPageIndexWrite, ZeroPageRMW, ZeroPageRead, ZeroPageWrite,
    BRK, JSR, PH, PL, RTI,
    RTS,
};
use crate::emulation::cpu::{MicroOpCallback, OpType, Source, Target};
use std::collections::HashMap;
use std::sync::OnceLock;

pub static OPCODES: OnceLock<Vec<OpCode>> = OnceLock::new();
pub static OPCODES_MAP: OnceLock<HashMap<u8, &'static OpCode>> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
pub struct OpCode {
    pub opcode: u8,
    pub name: &'static str,
    pub op_type: OpType,
}

pub fn init() -> HashMap<u8, &'static OpCode> {
    OPCODES
        .set(vec![
            // //ADC
            OpCode::new(
                0x69,
                "ADC",
                ImmediateAddressing(Target::TEMP, MicroOpCallback::ADC),
            ),
            OpCode::new(
                0x65,
                "ADC",
                ZeroPageRead(Target::TEMP, MicroOpCallback::ADC),
            ),
            OpCode::new(
                0x75,
                "ADC",
                ZeroPageIndexRead(Source::X, Target::TEMP, MicroOpCallback::ADC),
            ),
            OpCode::new(
                0x6D,
                "ADC",
                AbsoluteRead(Target::TEMP, MicroOpCallback::ADC),
            ),
            OpCode::new(
                0x7D,
                "ADC",
                AbsoluteIndexRead(Source::X, Target::TEMP, MicroOpCallback::ADC),
            ),
            OpCode::new(
                0x79,
                "ADC",
                AbsoluteIndexRead(Source::Y, Target::TEMP, MicroOpCallback::ADC),
            ),
            OpCode::new(
                0x61,
                "ADC",
                IndexedIndirect(Target::TEMP, MicroOpCallback::ADC),
            ),
            OpCode::new(
                0x71,
                "ADC",
                IndirectIndexed(Target::TEMP, MicroOpCallback::ADC),
            ),
            // //AND
            OpCode::new(
                0x29,
                "AND",
                ImmediateAddressing(Target::TEMP, MicroOpCallback::AND),
            ),
            // OpCode::new(0x25, "AND", 2, 3, ZeroPage),
            // OpCode::new(0x35, "AND", 2, 4, ZeroPageX),
            OpCode::new(
                0x2D,
                "AND",
                AbsoluteRead(Target::TEMP, MicroOpCallback::AND),
            ),
            OpCode::new(
                0x3D,
                "AND",
                AbsoluteIndexRead(Source::X, Target::TEMP, MicroOpCallback::AND),
            ),
            OpCode::new(
                0x39,
                "AND",
                AbsoluteIndexRead(Source::Y, Target::TEMP, MicroOpCallback::AND),
            ),
            // OpCode::new(0x21, "AND", 2, 6, IndirectX),
            // OpCode::new(0x31, "AND", 2, 5, IndirectY),
            // //ASL
            OpCode::new(0x0A, "ASL", AccumulatorOrImplied(MicroOpCallback::ASL)),
            OpCode::new(0x06, "ASL", ZeroPageRMW(Target::TEMP, MicroOpCallback::ASL)),
            OpCode::new(
                0x16,
                "ASL",
                ZeroPageIndexRMW(Source::X, MicroOpCallback::ASL),
            ),
            OpCode::new(0x0E, "ASL", AbsoluteRMW(Target::TEMP, MicroOpCallback::ASL)),
            // OpCode::new(0x1E, "ASL", 3, 7, AbsoluteX),
            // //BCC
            // OpCode::new(0x90, "BCC", 2, 2, Relative),
            // //BCS
            // OpCode::new(0xB0, "BCS", 2, 2, Relative),
            // //BEQ
            // OpCode::new(0xF0, "BEQ", 2, 2, Relative),
            // //BIT
            // OpCode::new(0x24, "BIT", 2, 3, ZeroPage),
            OpCode::new(
                0x2c,
                "BIT",
                AbsoluteRead(Target::TEMP, MicroOpCallback::BIT),
            ),
            // //BMI
            // OpCode::new(0x30, "BMI", 2, 2, Relative),
            // //BNE
            // OpCode::new(0xD0, "BNE", 2, 2, Relative),
            // //BPL
            // OpCode::new(0x10, "BPL", 2, 2, Relative),
            // //BRK
            OpCode::new(0x00, "BRK", BRK(MicroOpCallback::None)),
            // //BVC
            // OpCode::new(0x50, "BVC", 2, 2, Relative),
            // //BVS
            // OpCode::new(0x70, "BVS", 2, 2, Relative),
            // //CLC
            OpCode::new(0x18, "CLC", AccumulatorOrImplied(MicroOpCallback::CLC)),
            // //CLD
            OpCode::new(0xD8, "CLD", AccumulatorOrImplied(MicroOpCallback::CLD)),
            // //CLI
            OpCode::new(0x58, "CLI", AccumulatorOrImplied(MicroOpCallback::CLI)),
            // //CLV
            OpCode::new(0xb8, "CLV", AccumulatorOrImplied(MicroOpCallback::CLV)),
            // //CMP
            OpCode::new(
                0xC9,
                "CMP",
                ImmediateAddressing(Target::TEMP, MicroOpCallback::CMP),
            ),
            // OpCode::new(0xC5, "CMP", 2, 3, ZeroPage),
            // OpCode::new(0xD5, "CMP", 2, 4, ZeroPageX),
            OpCode::new(
                0xCD,
                "CMP",
                AbsoluteRead(Target::TEMP, MicroOpCallback::CMP),
            ),
            OpCode::new(
                0xDD,
                "CMP",
                AbsoluteIndexRead(Source::X, Target::TEMP, MicroOpCallback::CMP),
            ),
            OpCode::new(
                0xD9,
                "CMP",
                AbsoluteIndexRead(Source::Y, Target::TEMP, MicroOpCallback::CMP),
            ),
            // OpCode::new(0xC1, "CMP", 2, 6, IndirectX),
            // OpCode::new(0xD1, "CMP", 2, 5, IndirectY),
            // //CPX
            OpCode::new(
                0xE0,
                "CPX",
                ImmediateAddressing(Target::TEMP, MicroOpCallback::CPX),
            ),
            // OpCode::new(0xE4, "CPX", 2, 3, ZeroPage),
            OpCode::new(
                0xEC,
                "CPX",
                AbsoluteRead(Target::TEMP, MicroOpCallback::CPX),
            ),
            // //CPY
            OpCode::new(
                0xC0,
                "CPY",
                ImmediateAddressing(Target::TEMP, MicroOpCallback::CPY),
            ),
            // OpCode::new(0xC4, "CPY", 2, 3, ZeroPage),
            OpCode::new(
                0xCC,
                "CPY",
                AbsoluteRead(Target::TEMP, MicroOpCallback::CPY),
            ),
            // //DEC
            OpCode::new(0xC6, "DEC", ZeroPageRMW(Target::TEMP, MicroOpCallback::DEC)),
            OpCode::new(
                0xD6,
                "DEC",
                ZeroPageIndexRMW(Source::X, MicroOpCallback::DEC),
            ),
            OpCode::new(0xCE, "DEC", AbsoluteRMW(Target::TEMP, MicroOpCallback::DEC)),
            // OpCode::new(0xDE, "DEC", 3, 7, AbsoluteX),
            // //DEX
            OpCode::new(0xCA, "DEX", AccumulatorOrImplied(MicroOpCallback::DEX)),
            // //DEY
            OpCode::new(0x88, "DEY", AccumulatorOrImplied(MicroOpCallback::DEY)),
            // //EOR
            OpCode::new(
                0x49,
                "EOR",
                ImmediateAddressing(Target::TEMP, MicroOpCallback::EOR),
            ),
            // OpCode::new(0x45, "EOR", 2, 3, ZeroPage),
            // OpCode::new(0x55, "EOR", 2, 4, ZeroPageX),
            OpCode::new(
                0x4D,
                "EOR",
                AbsoluteRead(Target::TEMP, MicroOpCallback::EOR),
            ),
            OpCode::new(
                0x5D,
                "EOR",
                AbsoluteIndexRead(Source::X, Target::TEMP, MicroOpCallback::EOR),
            ),
            OpCode::new(
                0x59,
                "EOR",
                AbsoluteIndexRead(Source::Y, Target::TEMP, MicroOpCallback::EOR),
            ),
            // OpCode::new(0x41, "EOR", 2, 6, IndirectX),
            // OpCode::new(0x51, "EOR", 2, 5, IndirectY),
            // //INC
            OpCode::new(0xE6, "INC", ZeroPageRMW(Target::TEMP, MicroOpCallback::INC)),
            OpCode::new(
                0xF6,
                "INC",
                ZeroPageIndexRMW(Source::X, MicroOpCallback::INC),
            ),
            OpCode::new(0xEE, "INC", AbsoluteRMW(Target::TEMP, MicroOpCallback::INC)),
            // OpCode::new(0xFE, "INC", 3, 7, AbsoluteX),
            // //INX
            OpCode::new(0xE8, "INX", AccumulatorOrImplied(MicroOpCallback::INX)),
            // //INY
            OpCode::new(0xC8, "INY", AccumulatorOrImplied(MicroOpCallback::INY)),
            // //JMP
            OpCode::new(0x4C, "JMP", JmpAbsolute(MicroOpCallback::None)),
            // OpCode::new(0x6C, "JMP", 3, 5, Indirect),
            // //JSR
            OpCode::new(0x20, "JSR", JSR(MicroOpCallback::None)),
            // //LDA
            OpCode::new(
                0xA9,
                "LDA",
                ImmediateAddressing(Target::A, MicroOpCallback::None),
            ),
            OpCode::new(0xA5, "LDA", ZeroPageRead(Target::A, MicroOpCallback::None)),
            OpCode::new(
                0xB5,
                "LDA",
                ZeroPageIndexRead(Source::X, Target::A, MicroOpCallback::None),
            ),
            OpCode::new(0xAD, "LDA", AbsoluteRead(Target::A, MicroOpCallback::None)),
            OpCode::new(
                0xBD,
                "LDA",
                AbsoluteIndexRead(Source::X, Target::A, MicroOpCallback::None),
            ),
            OpCode::new(
                0xB9,
                "LDA",
                AbsoluteIndexRead(Source::Y, Target::A, MicroOpCallback::None),
            ),
            OpCode::new(
                0xA1,
                "LDA",
                IndexedIndirect(Target::A, MicroOpCallback::None),
            ),
            OpCode::new(
                0xB1,
                "LDA",
                IndirectIndexed(Target::A, MicroOpCallback::None),
            ),
            // //LDX
            OpCode::new(
                0xA2,
                "LDX",
                ImmediateAddressing(Target::X, MicroOpCallback::None),
            ),
            OpCode::new(0xA6, "LDX", ZeroPageRead(Target::X, MicroOpCallback::None)),
            OpCode::new(
                0xB6,
                "LDX",
                ZeroPageIndexRead(Source::Y, Target::X, MicroOpCallback::None),
            ),
            OpCode::new(0xAE, "LDX", AbsoluteRead(Target::X, MicroOpCallback::None)),
            OpCode::new(
                0xBE,
                "LDX",
                AbsoluteIndexRead(Source::Y, Target::X, MicroOpCallback::None),
            ),
            // //LDY
            OpCode::new(
                0xA0,
                "LDY",
                ImmediateAddressing(Target::Y, MicroOpCallback::None),
            ),
            OpCode::new(0xA4, "LDX", ZeroPageRead(Target::Y, MicroOpCallback::None)),
            OpCode::new(
                0xB4,
                "LDX",
                ZeroPageIndexRead(Source::X, Target::Y, MicroOpCallback::None),
            ),
            OpCode::new(0xAC, "LDY", AbsoluteRead(Target::Y, MicroOpCallback::None)),
            OpCode::new(
                0xBC,
                "LDY",
                AbsoluteIndexRead(Source::X, Target::Y, MicroOpCallback::None),
            ),
            // //LSR
            OpCode::new(0x4A, "LSR", AccumulatorOrImplied(MicroOpCallback::LSR)),
            OpCode::new(0x46, "LSR", ZeroPageRMW(Target::TEMP, MicroOpCallback::LSR)),
            OpCode::new(
                0x56,
                "LSR",
                ZeroPageIndexRMW(Source::X, MicroOpCallback::LSR),
            ),
            OpCode::new(0x4E, "LSR", AbsoluteRMW(Target::TEMP, MicroOpCallback::LSR)),
            // OpCode::new(0x5E, "LSR", 3, 7, AbsoluteX),
            // //NOP
            OpCode::new(0xEA, "NOP", AccumulatorOrImplied(MicroOpCallback::None)),
            // //ORA
            OpCode::new(
                0x09,
                "ORA",
                ImmediateAddressing(Target::TEMP, MicroOpCallback::ORA),
            ),
            // OpCode::new(0x05, "ORA", 2, 3, ZeroPage),
            // OpCode::new(0x15, "ORA", 2, 4, ZeroPageX),
            OpCode::new(
                0x0D,
                "ORA",
                AbsoluteRead(Target::TEMP, MicroOpCallback::ORA),
            ),
            OpCode::new(
                0x1D,
                "ORA",
                AbsoluteIndexRead(Source::X, Target::TEMP, MicroOpCallback::ORA),
            ),
            OpCode::new(
                0x19,
                "ORA",
                AbsoluteIndexRead(Source::Y, Target::TEMP, MicroOpCallback::ORA),
            ),
            // OpCode::new(0x01, "ORA", 2, 6, IndirectX),
            // OpCode::new(0x11, "ORA", 2, 5, IndirectY),
            // //PHA
            OpCode::new(0x48, "PHA", PH(Source::A, MicroOpCallback::None)),
            // //PHP
            OpCode::new(0x08, "PHP", PH(Source::P, MicroOpCallback::None)),
            // //PLA
            OpCode::new(0x68, "PLA", PL(Target::A, MicroOpCallback::None)),
            // //PLP
            OpCode::new(0x28, "PLP", PL(Target::P, MicroOpCallback::None)),
            // //ROL
            OpCode::new(0x2A, "ROL", AccumulatorOrImplied(MicroOpCallback::ROL)),
            OpCode::new(0x26, "ROL", ZeroPageRMW(Target::TEMP, MicroOpCallback::ROL)),
            OpCode::new(
                0x36,
                "ROL",
                ZeroPageIndexRMW(Source::X, MicroOpCallback::ROL),
            ),
            OpCode::new(0x2E, "ROL", AbsoluteRMW(Target::TEMP, MicroOpCallback::ROL)),
            // OpCode::new(0x3E, "ROL", 3, 7, AbsoluteX),
            // //ROR
            OpCode::new(0x6A, "ROR", AccumulatorOrImplied(MicroOpCallback::ROR)),
            OpCode::new(0x66, "ROR", ZeroPageRMW(Target::TEMP, MicroOpCallback::ROR)),
            OpCode::new(
                0x76,
                "ROR",
                ZeroPageIndexRMW(Source::X, MicroOpCallback::ROR),
            ),
            OpCode::new(0x6E, "ROR", AbsoluteRMW(Target::TEMP, MicroOpCallback::ROR)),
            // OpCode::new(0x7E, "ROR", 3, 7, AbsoluteX),
            // //RTI
            OpCode::new(0x40, "RTI", RTI(MicroOpCallback::None)),
            // //RTS
            OpCode::new(0x60, "RTS", RTS(MicroOpCallback::None)),
            // //SBC
            OpCode::new(
                0xE9,
                "SBC",
                ImmediateAddressing(Target::TEMP, MicroOpCallback::SBC),
            ),
            // OpCode::new(0xE5, "SBC", 2, 3, ZeroPage),
            // OpCode::new(0xF5, "SBC", 2, 4, ZeroPageX),
            OpCode::new(
                0xED,
                "SBC",
                AbsoluteRead(Target::TEMP, MicroOpCallback::SBC),
            ),
            OpCode::new(
                0xFD,
                "SBC",
                AbsoluteIndexRead(Source::X, Target::TEMP, MicroOpCallback::SBC),
            ),
            OpCode::new(
                0xF9,
                "SBC",
                AbsoluteIndexRead(Source::Y, Target::TEMP, MicroOpCallback::SBC),
            ),
            // OpCode::new(0xE1, "SBC", 2, 6, IndirectX),
            // OpCode::new(0xF1, "SBC", 2, 5, IndirectY),
            // //SEC
            OpCode::new(0x38, "SEC", AccumulatorOrImplied(MicroOpCallback::SEC)),
            // //SED
            OpCode::new(0xF8, "SED", AccumulatorOrImplied(MicroOpCallback::SED)),
            // //SEI
            OpCode::new(0x78, "SEI", AccumulatorOrImplied(MicroOpCallback::SEI)),
            // //STA
            OpCode::new(0x85, "STA", ZeroPageWrite(Source::A, MicroOpCallback::None)),
            OpCode::new(
                0x95,
                "STA",
                ZeroPageIndexWrite(Source::A, Source::X, MicroOpCallback::None),
            ),
            OpCode::new(0x8D, "STA", AbsoluteWrite(Source::A, MicroOpCallback::None)),
            // OpCode::new(0x9D, "STA", 3, 5, AbsoluteX),
            // OpCode::new(0x99, "STA", 3, 5, AbsoluteY),
            // OpCode::new(0x81, "STA", 2, 6, IndirectX),
            // OpCode::new(0x91, "STA", 2, 6, IndirectY),
            // //STX
            OpCode::new(0x86, "STX", ZeroPageWrite(Source::X, MicroOpCallback::None)),
            OpCode::new(
                0x96,
                "STX",
                ZeroPageIndexWrite(Source::X, Source::Y, MicroOpCallback::None),
            ),
            OpCode::new(0x8E, "STX", AbsoluteWrite(Source::X, MicroOpCallback::None)),
            // //STY
            OpCode::new(0x84, "STY", ZeroPageWrite(Source::Y, MicroOpCallback::None)),
            OpCode::new(
                0x94,
                "STY",
                ZeroPageIndexWrite(Source::Y, Source::X, MicroOpCallback::None),
            ),
            OpCode::new(0x8C, "STY", AbsoluteWrite(Source::Y, MicroOpCallback::None)),
            // //TAX
            OpCode::new(0xAA, "TAX", AccumulatorOrImplied(MicroOpCallback::TAX)),
            // //TAY
            OpCode::new(0xA8, "TAY", AccumulatorOrImplied(MicroOpCallback::TAY)),
            // //TSX
            OpCode::new(0xBA, "TSX", AccumulatorOrImplied(MicroOpCallback::TSX)),
            // //TXA
            OpCode::new(0x8A, "TXA", AccumulatorOrImplied(MicroOpCallback::TXA)),
            // //TXS
            OpCode::new(0x9A, "TXS", AccumulatorOrImplied(MicroOpCallback::TXS)),
            // //TYA
            OpCode::new(0x98, "TYA", AccumulatorOrImplied(MicroOpCallback::TYA)),
            // //Illegal Opcodes
            // OpCode::new(0xFF, "ISC", 2, 7, AbsoluteX),
            // OpCode::new(0xE3, "ISC", 2, 7, IndirectX),
            OpCode::new(0x3A, "NOP", AccumulatorOrImplied(MicroOpCallback::None2)),
        ])
        .ok();

    let mut map = HashMap::new();

    for opcode in OPCODES.get().unwrap() {
        map.insert(opcode.opcode, opcode);
    }

    map
}

impl OpCode {
    pub fn new(opcode: u8, name: &'static str, op_type: OpType) -> Self {
        Self {
            opcode,
            name,
            op_type,
        }
    }
}

impl Default for OpCode {
    fn default() -> Self {
        Self {
            opcode: 0xB3,
            name: "PNC",
            op_type: ImmediateAddressing(Target::LO, MicroOpCallback::None),
        }
    }
}
