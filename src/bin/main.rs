use nesamabob::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.init();
    cpu.load_rom();

    cpu.run()
}
