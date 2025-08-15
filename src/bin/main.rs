use nesamabob::cpu::Cpu;
use nesamabob::nes::Nes;
use nesamabob::ppu::Ppu;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
fn main() {
    // let rom = RomFile::load(&String::from("./tests/1.Branch_Basics.nes"));
    // println!("{:?}", rom);

    let cpu = Cpu::new();
    let ppu = Rc::new(RefCell::new(Ppu::default()));
    let mut nes = Nes::new(cpu, ppu);
    run_branch_01_remainder(&mut nes)
}

#[allow(dead_code)]
fn run_branch_01_remainder(nes: &mut Nes) {
    nes.load_state("states/branch_basic_01-nmi-enabled.bin");
    nes.run(u64::MAX);
}

#[allow(dead_code)]
fn run_branch_01_nmi_enabled(nes: &mut Nes) {
    nes.load_rom(&String::from("./tests/1.Branch_Basics.nes"));
    nes.init();
    nes.run(228084);
    nes.save_state("states/branch_basic_01-nmi-enabled.bin");
}

#[allow(dead_code)]
fn run_branch_01_first_sync(nes: &mut Nes) {
    nes.load_rom(&String::from("./tests/1.Branch_Basics.nes"));
    nes.init();
    nes.run(347184);
    nes.save_state("states/branch_basic_01-sync-1-successful.bin");
}
