# User:Lidnariq/Discrete Logic Toolbox

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ALidnariq/Discrete_Logic_Toolbox) | View [other pages](Special_AllPages.xhtml#User_Lidnariq_Discrete_Logic_Toolbox)

The **Discrete Logic Toolbox** is an exercise in building interesting NES extensions using minimal additional hardware. Sometimes it gets a little overly ambitious. 

## Contents

  * 1 0 ICs
  * 2 1 IC
  * 3 2 ICs
  * 4 One latch and one other IC
  * 5 One latch and more than one other IC
    * 5.1 CHR compression



## 0 ICs

  * [iNES Mapper 099](INES_Mapper_099.xhtml "INES Mapper 099"): 
    * (Famicom) Connect OUT2 from [expansion port](Expansion_port.xhtml "Famicom expansion port pinout") to /IRQ; connect /IRQ to CHR A13: lose ability to use APU IRQs.
    * (NES) Connect OUT2 from [expansion port](Expansion_port.xhtml "NES expansion port pinout") to unused EXP pins; connect those pin to CHR A13. (Optionally, connect OUT1 also)



## 1 IC

  * One [7400](7400.xhtml "7400"): [User:Zzo38/Mapper C](User_Zzo38_Mapper_C.xhtml "User:Zzo38/Mapper C") (MMC4-style CHR bankswitching, BNROM-style PRG bankswitching)
  * One AY-3-8910: [User:Zzo38/Mapper A](User_Zzo38_Mapper_A.xhtml "User:Zzo38/Mapper A") ([expansion audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio") and GNROM-style banking)
  * Any one inverter (⅙ [7404](7404.xhtml "7404"), 05, 06, 07, 14, 16, 19, ¼ [7400](7400.xhtml "7400"), 01, [7402](7402.xhtml "7402"), 03, 24, 25, 26, 28, 33, 36, 37, 38, 39, 86, ⅓ 7410, 12, 27, ½ 7413, 18, 20, 22, 23, 25, 40) 
    * Prevent bus conflicts on bankswitch writes by making `ROM /OE ← NOT R/W`
  * Selecting logic such as 7420, [74138](74138.xhtml "74138"), [74139](74139.xhtml "74139")
    * [PRG RAM circuit](PRG_RAM_circuit.xhtml "PRG RAM circuit") (Add up to 8KiB WRAM mapped from $6000-$7fff)
    * Split 32KiB into 16KiB RAM from $8000-$BFFF and 16KiB ROM from $C000-$FFFF
  * [7485](7485.xhtml "7485") (4-bit comparator) 
    * [NROM-368](NROM_368.xhtml "NROM-368") (Map up to 46KiB from $4800-$FFFF)
    * Map up to 44KiB from $5000-$FFFF without bus conflicts
  * Any latch with an active-low clock enable, such as [74161](74161.xhtml "74161"), [74377](74377.xhtml "74377")
    * Most of Nintendo's discrete-logic mappers ([AxROM](AxROM.xhtml "AxROM"), [BNROM](INES_Mapper_034.xhtml "BNROM"), [CNROM](CNROM.xhtml "CNROM"), [GNROM or MHROM](GxROM.xhtml "GNROM")) 
      * [UxROM](UxROM.xhtml "UxROM") can be emulated on a BNROM-class board by manually mirroring the upper bank before burning. (A 128KiB UNROM game with banks 01234567 would become a 256 KiB BNROM with banks 0717273747576777)
    * Arbitrary GNROM-style mappers with 32KiB PRG banks and 8KiB CHR banks
    * Mapper-controlled 1-screen mirroring (a la AxROM)
    * Oeka Kids-style dynamic banking of CHR with zones as small as the size of an attribute byte (32x32 pixel), by connecting `Latch./CLKEN ← /PPUA13` and `Latch.CLK ← /RD`
    * Additionally, a simple circuit (diode, resistor, capacitor) can automatically clear a 74161 on reset
  * Any sufficiently large binary counter, such as (74)4020 
    * Interrupts for 2ⁿ X every 2⁽ⁿ⁺¹⁾ X, where X could be (A12 rises = scanlines·8, cpu cycles, PPU reads)
  * 74253 (Tristatable dual 4-input multiplexer) in lieu of CHR ROM, plus 8 ≈5kΩ resistors: 
    * Game Genie style low-resolution graphics, where each 4-by-4 pixel zone is individually controllable and can have any color `D3…D0←SEL(A3…A2,A11…A8)` and `D7…D4←SEL(A3…A2,A7…A4)`
  * 74'153 or 74'157, plus 8 ≈5kΩ resistors: 
    * Extends [iNES Mapper 218](INES_Mapper_218.xhtml "INES Mapper 218") to allow selective disabling of 1kB NT RAM to allow individual access of bitplanes without palette hacks. `CIRAM/SEL ← SEL(A3,A10,A11)`, `SEL/E ← A13`, `D0…D7 ← A12 through resistors`



## 2 ICs

  * Any obsolete RAM such as 74170, 670, 189, 219, 289 plus decoding logic ([7432](7432.xhtml "7432")) 
    * 4 (for the two 74?70) or 16 (for the three 74??9) independently controlled banks. 
      * This is not exactly the same as [User:Zzo38/Mapper B](User_Zzo38_Mapper_B.xhtml "User:Zzo38/Mapper B"); we omit the fixed 4KiB topmost bank for simplicity.
  * 7485 + 74(4078) (comparator+8-input NOR) 
    * Map almost 48KiB from $4020-$FFFF without bus conflicts
  * Any latch plus decoding logic, such as 7400, 02, 32, 133, 138 + 7474, 173, 174, 176 
    * GNROM-style mappers as made by not-Nintendo
  * Timer/counter plus decoding logic, such as 7400 + 555/(74)4020/74123 
    * Acknowledgable interrupts
  * Inverter (such as 7400, 7404, 7486) + 8KB RAM 
    * Map 8KB RAM into PPU $0xxx and $2xxx for 4KB CHR-RAM slice and 4-screen mirroring, inverter makes `NOT PPU A12` to decode 4KB ROM (or 4KB window) in $1xxx



## One latch and one other IC

"Latch" in this case means a [74161](74161.xhtml "74161") or a [74377](74377.xhtml "74377"), i.e. those with an active-low clock enable. 

  * Masking logic, such as 7400, 02, 08, 32 
    * [UxROM](UxROM.xhtml "UxROM") and similar with a fixed bank of PRG, to a maximum of 256 KiB (16 KiB × 2⁴)
    * PPU equivalent of UxROM ([CPROM](CPROM.xhtml "CPROM"), [Racermate](INES_Mapper_168.xhtml "INES Mapper 168")), with a fixed bank of CHR, to a maximum of 64 KiB (4 KiB × 2⁴)
  * Inverting masking logic, such as 7400, 02 
    * UxROM without bus conflicts and similar with a fixed bank of PRG or CHR (max 3 bits of banking)
  * a quad 1-of-2 multiplexer (74157) 
    * Two independently controllable banks
    * Two independently controllable banks and two duplicate fixed banks
  * Tristatable quad 1-2 multiplexer (74'257) and four ≈5kΩ resistors 
    * Two independently controllable banks and two different fixed banks (128kiB PRG example: Three resistors pull ROM A14..A16 high. The last connects CPU A13→ROM A13 but can be overridden by the mux)
  * ½ dual 1-of-4 multiplexer (74153) 
    * Controllable horizontal/vertical/1screen mirroring
  * A single 1-of-2 multiplexer (7400) 
    * Controllable horizontal/vertical mirroring (as used by Holy Diver)



## One latch and more than one other IC

The traditional emulator oversize variant of [UxROM](UxROM.xhtml "UxROM") requires an additional 7432 OR gate in order to keep the upper bank fixed to the absolutely topmost bank. Much like the preceding note that BNROM can emulate UNROM, an alternative board that resembles [SUROM](MMC1.xhtml "SxROM") can emulate an oversize UxROM by omitting the second 7432 and having each 16th bank be duplicates. 

### CHR compression

CHR compression techniques allow the use of a smaller CHR ROM, or automatic conversion of data in CHR RAM, for animation or palette swapping. The `ControlLine` mentioned below could be a PPU address line or a latch output. The forms mentioned here explicitly modify the entire PPU space; more useful and restrictive versions (that only affect some of the PPU address space) are possible by adding additional ICs such as a [74138](74138.xhtml "74138") with a tristateable latch or AND gates. 

  * 7486 (quad XOR) 
    * Reorder colors of fetched tiles from 0123 to 0213 by `CHR A3 ← ControlLine XOR PPU A3`
    * Flip fetched tiles vertically by `CHR A0…A2 ← ControlLine XOR PPU A0…A2`
  * 74283 (4-bit full adder) 
    * Allows tile cycling of each 16 tile range by `CHR A4…A7 ← Latched4bit + PPU A4…A7`. (Remove upper address line to reduce range)
  * Two 74283 (making an 8-bit full adder) 
    * Vertical scrolling of each 32 tile range by `CHR A0…A2,A4…A8 ← Latched8bit + A0…A2,A4…A8`.
  * Two [74157](74157.xhtml "74157") (quad 1-of-2 multiplexer) 
    * Flip fetched tiles horizontally by `PPU D0…D7 ← SELECT(ControlLine,CHR D0…D7,CHR D7…D0)`—can reorder columns arbitrarily (ROM only)
  * Two 7486 (quad XOR) 
    * Reorder colors of fetched tiles from 0123 to 3210 by `PPU D0…D7 ← CHR D0…D7 XOR ControlLine` (ROM only)


