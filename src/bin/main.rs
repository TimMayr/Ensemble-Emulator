use nesamabob::cpu::Cpu;

fn main() {
    // let rom = RomFile::load(&String::from("./tests/1.Branch_Basics.nes"));
    // println!("{:?}", rom);

    let mut cpu = Cpu::new();
    cpu.load_rom(&String::from("./tests/1.Branch_Basics.nes"));
    cpu.reset();
    cpu.run()
}
