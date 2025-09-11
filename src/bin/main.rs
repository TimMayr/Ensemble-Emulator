use nesamabob::nes::{Nes, MASTER_CYCLES_PER_FRAME};

#[allow(dead_code)]
fn main() {
    // let rom = RomFile::load(&String::from("./tests/1.Branch_Basics.nes"));
    // println!("{:?}", rom);

    let mut nes = Nes::default();
    nes.load_rom(&String::from("./tests/ram_after_reset"));
    nes.reset();
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
    nes.reset();
    nes.run(228084);
    nes.save_state("states/branch_basic_01-nmi-enabled.bin");
}

#[allow(dead_code)]
fn run_branch_01_first_sync(nes: &mut Nes) {
    nes.load_rom(&String::from("./tests/1.Branch_Basics.nes"));
    nes.reset();
    nes.run(347184);
    nes.save_state("states/branch_basic_01-sync-1-successful.bin");
}
