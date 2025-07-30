use nesamabob::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.init();
    cpu.mem_write(0x0, 0xA9);
    cpu.mem_write(0x1, 0x66);
    cpu.mem_write(0x2, 0xFF);

    cpu.run()
}
