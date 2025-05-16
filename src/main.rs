use iced_x86::{BlockEncoder, BlockEncoderOptions, Code, Instruction, InstructionBlock, Register};
use object::{write::{Object, StandardSegment, Symbol, SymbolSection}, BinaryFormat, Endianness, SectionKind, SymbolKind};
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Encode `mov rax, 60; xor rdi, rdi; syscall` (exit(0))
    let instructions = [
        Instruction::with2(Code::Mov_r64_imm64, Register::RAX, 60)?,
        Instruction::with2(Code::Xor_r64_rm64, Register::RDI, Register::RDI)?,
        Instruction::with(Code::Syscall),
    ];

    let rip = 0x1000; // Virtual start of code section
    let block = InstructionBlock::new(&instructions, rip);
    let result = BlockEncoder::encode(64, block, BlockEncoderOptions::NONE)?;    let code = result.code_buffer;

    // Build ELF64 object
    let mut obj = Object::new(BinaryFormat::Elf, object::Architecture::X86_64, Endianness::Little);

    // Create executable section
    let section_id = obj.add_section(
        obj.segment_name(StandardSegment::Text).to_vec(),
        b".text".to_vec(),
        SectionKind::Text,
    );

    obj.section_mut(section_id).set_data(code.clone(), 16); // 16-byte alignment

    // Add entry symbol
    let symbol_id = obj.add_symbol(Symbol {
        name: b"_start".to_vec(),
        value: 0,
        size: code.len() as u64,
        kind: SymbolKind::Text,
        scope: object::SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(section_id),
        flags: object::SymbolFlags::None,
    });

    // Write binary to disk
    let elf_data = obj.write()?;
    let mut file = File::create("output.o")?;
    file.write_all(&elf_data)?;
    println!("Written ELF executable to ./output.o");

    Ok(())

    // let content = fs::read("Action in New York (Europe).nes").expect("File not found");
    //
    // print_in_range(&content, &0, &17);
    //
    // println!("Identification String:");
    // print_in_range(&content, &0, &4);
    // println!("PRG_ROM:");
    // print_in_range(&content, &4, &5);
    // println!("CHR_ROM:");
    // print_in_range(&content, &5, &6);
    // println!("Flags 6:");
    // print_in_range_binary(&content, &6, &7);
    // println!("Flags 7:");
    // print_in_range_binary(&content, &7, &8);
    // println!("Mapper MSB/Submapper:");
    // print_in_range_nibbles_u4(&content, &8, &9);
}

fn print_at_indices(content: &Vec<u8>, indices: &Vec<usize>) {
    for (i, &c) in content.iter().enumerate() {
        if indices.contains(&i) {
            if c.is_ascii_graphic() || c == b' ' {
                print!("{}", c as char);
            } else {
                print!("<{:#02X?}>", c);
            };
        }
    }
    println!()
}

fn print_in_range(content: &Vec<u8>, start: &u32, end: &u32) {
    for (i, &c) in content.iter().enumerate() {
        if i >= *start as usize && i < *end as usize {
            if c.is_ascii_graphic() || c == b' ' {
                print!("{}", c as char);
            } else {
                print!("<{:#02X?}>", c);
            };
        }
    }
    println!()
}

fn print_in_range_binary(content: &Vec<u8>, start: &u32, end: &u32) {
    for (i, &c) in content.iter().enumerate() {
        if i >= *start as usize && i < *end as usize {
            print!("{:08b}", c)
        }
    }
    println!()
}

fn print_in_range_nibbles_u4(content: &Vec<u8>, start: &i32, end: &i32) {
    for (i, &c) in content.iter().enumerate() {
        if i >= *start as usize && i < *end as usize {
            print!("{}", c >> 4);
            print!(" ");
            print!("{}", c & 0x0F);
        }
    }
    println!()
}
