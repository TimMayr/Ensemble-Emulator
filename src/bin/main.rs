use nesamabob::rom::Rom;

fn main() {
    let rom = Rom::load(String::from(
        "D:\\Stuff\\Dev\\EmulatorThingimabob\\tests\\1.Branch_Basics.nes",
    ));

    println!("{rom:?}")
}
