use nesamabob::cpu::Cpu;
use nesamabob::nes::Nes;
use nesamabob::ppu::PpuStub;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
fn main() {
    // let rom = RomFile::load(&String::from("./tests/1.Branch_Basics.nes"));
    // println!("{:?}", rom);

    let cpu = Cpu::new();
    let ppu = Rc::new(RefCell::new(PpuStub::default()));
    let mut nes = Nes::new(cpu, ppu);
    run_branch_01_init(&mut nes)
}

#[allow(dead_code)]
fn run_branch_01_remainder(nes: &mut Nes) {
    nes.load_state("states/branch_basic_01-init.bin");
    nes.run();
}

#[allow(dead_code)]
fn run_branch_01_init(nes: &mut Nes) {
    nes.load_rom(&String::from("./tests/1.Branch_Basics.nes"));
    nes.init();
    nes.run();
    nes.save_state("states/branch_basic_01-init.bin");
}
