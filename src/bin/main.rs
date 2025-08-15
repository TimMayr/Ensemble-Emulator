use nesamabob::cpu::Cpu;
use nesamabob::nes::{MASTER_CYCLES_PER_FRAME, Nes};
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
    nes.load_rom(&String::from("./tests/ram_after_reset"));
    nes.init();
    nes.run(MASTER_CYCLES_PER_FRAME as u128 * 6000u128);
    nes.save_state("states/ram_after_reset.bin");
}

#[allow(dead_code)]
fn run_branch_01_remainder(nes: &mut Nes) {
    nes.load_state("states/branch_basic_01-nmi-enabled.bin");
    nes.run(u128::MAX);
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
