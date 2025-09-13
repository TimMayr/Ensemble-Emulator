use crate::mem::{Memory, Rom};
use crate::nes::Nes;

#[test]
fn test_nmi_vector() {
    let mut nes = Nes::default();

    //Create and initialize new Rom
    let mut rom = Rom::new(0xBFE0);
    //Set Reset vector to 0x4020
    rom.init(0xBFDA, 0x20);
    rom.init(0xBFDB, 0x40);
    //Load 0x20 to acc from 4020
    rom.init(0x0, 0xA9);
    rom.init(0x1, 0x20);

    //Attach new Rom memory device to cpu
    nes.cpu.memory.add_memory(0x4020..=0xFFFF, Box::new(rom));

    //Manually force an nmi
    nes.ppu.borrow().nmi_requested.set(true);

    nes.reset();

    nes.cpu.step();
    assert_eq!(nes.cpu.accumulator, 0x20)
}
