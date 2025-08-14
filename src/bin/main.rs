use nesamabob::cpu::Cpu;
use nesamabob::nes::Nes;
use nesamabob::ppu::PpuStub;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // let rom = RomFile::load(&String::from("./tests/1.Branch_Basics.nes"));
    // println!("{:?}", rom);

    let cpu = Cpu::new();
    let ppu = Rc::new(RefCell::new(PpuStub::default()));
    let mut nes = Nes::new(cpu, ppu);
    nes.cpu
        .load_rom(&String::from("./tests/1.Branch_Basics.nes"));

    nes.init();
    nes.run();
}
