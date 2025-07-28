use nesamabob::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();

    //and immediate
    cpu.mem_write(0x0, 0x29);
    cpu.mem_write(0x1, 0b01100110);

    //lda
    cpu.mem_write(0x2, 0xA9);
    cpu.mem_write(0x3, 0b11000011);

    //prt
    cpu.mem_write(0x4, 0xFF);

    //and zero page
    cpu.mem_write(0x5, 0x25);
    cpu.mem_write(0x6, 0x0052);
    cpu.mem_write(0x0052, 0b01100110);

    //lda
    cpu.mem_write(0x7, 0xA9);
    cpu.mem_write(0x8, 0b11000011);

    //prt
    cpu.mem_write(0x9, 0xFF);

    //and zero page x
    cpu.mem_write(0xA, 0x35);
    cpu.mem_write(0xB, 0x0053);

    //ldx
    cpu.mem_write(0xC, 0xA2);
    cpu.mem_write(0xD, 0x0010);

    cpu.mem_write(0x0063, 0b01100110);

    //lda
    cpu.mem_write(0xE, 0xA9);
    cpu.mem_write(0xF, 0b11000011);

    //prt
    cpu.mem_write(0x10, 0xFF);

    //and absolute
    cpu.mem_write(0x11, 0x2D);
    cpu.mem_write_u16(0x12, 0x2D2D);
    cpu.mem_write(0x2D2D, 0b01100110);

    //lda
    cpu.mem_write(0x13, 0xA9);
    cpu.mem_write(0x14, 0b11000011);

    //prt
    cpu.mem_write(0x15, 0xFF);

    //and absolute x
    cpu.mem_write(0x16, 0x3D);
    cpu.mem_write_u16(0x17, 0x3D3D);

    //ldx
    cpu.mem_write(0x18, 0xA2);
    cpu.mem_write(0x19, 0x0010);

    cpu.mem_write(0x3D4D, 0b01100110);

    //lda
    cpu.mem_write(0x1A, 0xA9);
    cpu.mem_write(0x1B, 0b11000011);

    //prt
    cpu.mem_write(0x1C, 0xFF);

    //and absolute y
    cpu.mem_write(0x1D, 0x39);
    cpu.mem_write_u16(0x1E, 0x3939);

    //ldy
    cpu.mem_write(0x1F, 0xA0);
    cpu.mem_write(0x20, 0x0010);

    cpu.mem_write(0x3949, 0b01100110);

    //lda
    cpu.mem_write(0x21, 0xA9);
    cpu.mem_write(0x22, 0b11000011);

    //prt
    cpu.mem_write(0x23, 0xFF);

    //and indirect x
    cpu.mem_write(0x24, 0x21);
    cpu.mem_write(0x25, 0x0055);

    //ldx
    cpu.mem_write(0x26, 0xA2);
    cpu.mem_write(0x27, 0x0010);

    cpu.mem_write_u16(0x0065, 0x2121);
    cpu.mem_write(0x2121, 0b01100110);

    //lda
    cpu.mem_write(0x28, 0xA9);
    cpu.mem_write(0x29, 0b11000011);

    //prt
    cpu.mem_write(0x2A, 0xFF);

    //and indirect y
    cpu.mem_write(0x2B, 0x31);
    cpu.mem_write(0x2C, 0x0054);

    cpu.mem_write_u16(0x0054, 0x3131);

    //ldy
    cpu.mem_write(0x2D, 0xA0);
    cpu.mem_write(0x2E, 0x0010);

    cpu.mem_write(0x3141, 0b01100110);

    //lda
    cpu.mem_write(0x2F, 0xA9);
    cpu.mem_write(0x30, 0b11000011);

    //prt
    cpu.mem_write(0x31, 0xFF);

    cpu.run()
}
