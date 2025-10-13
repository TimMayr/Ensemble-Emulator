use std::fs::File;
use std::io;
use std::io::BufRead;

use crate::emulation::emu::{Console, Consoles};
use crate::emulation::nes::Nes;
use crate::frontend::Frontends;

#[test]
fn nestest() {
    let mut emu = Consoles::Nes(Nes::default());
    emu.set_trace_log_path(Some(String::from("./tests/outputs/nestest_headless.log")));

    emu.load_rom(&String::from("./tests/nes-test-roms/nestest_headless.nes"));
    emu.power();
    emu.run(&mut Frontends::default())
        .expect("Error running test");

    let file1 = File::open("./tests/outputs/nestest_headless.log").expect("Error running test");
    let file2 = File::open("./tests/outputs-compare/nestest_headless_good.log")
        .expect("Error running test");

    let lines1: Vec<_> = io::BufReader::new(file1)
        .lines()
        .take(8980)
        .collect::<Result<_, _>>()
        .expect("Error running test");
    let lines2: Vec<_> = io::BufReader::new(file2)
        .lines()
        .take(8980)
        .collect::<Result<_, _>>()
        .expect("Error running test");

    assert_eq!(lines1, lines2);
}
