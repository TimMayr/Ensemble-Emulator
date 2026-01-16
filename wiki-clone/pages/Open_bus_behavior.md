# Open bus behavior

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Open_bus_behavior) | View [other pages](Special_AllPages.xhtml#Open_bus_behavior)

When the CPU attempts to read from an address which has no devices active, the result is **open bus behavior**. The value that results from a read in such a region is undefined, but on specific hardware predictable results may appear. 

Clone versions of the hardware, and flash cartridges like the [PowerPak](PowerPak.xhtml "PowerPak") or [Everdrive](Everdrive_N8.xhtml "Everdrive") may differ in their open bus behavior from a standard NES.[1]

This commonly effects: 

  * The [controller read](Controller_reading.xhtml#Unconnected_data_lines_and_open_bus "Controller reading") ports at $4016 and $4017.
  * Game bugs that read from $4020-7FFF when nothing is mapped in these ranges.
  * [CNROM variant](CNROM.xhtml "INES Mapper 185") games that can unmap PPU CHR memory to use the PPU open bus as a copy protection test.



## Contents

  * 1 CPU open bus
    * 1.1 Examples
    * 1.2 Indirect indexed instruction example
  * 2 PPU open bus
  * 3 Electrical basis
  * 4 See also
  * 5 References



## CPU open bus

On a standard NES, reading open bus repeats the last value that was read from the bus before this read. Relevant cases: 

  * Absolute addressed instructions will read the high byte of the address (the last byte of the operand).
  * Indexed addressing uses the high byte of the _base_ address read from the instruction's operand; the index register does not yet apply when that value is read.
  * Indirect addressing will fetch the high byte of the pointer last. The Y index does not yet apply. See below for an example.
  * A [DMC](APU_DMC.xhtml "DMC") DMA may also alter the last value read if it interrupts an instruction.



Open bus can be limited to _part_ of a byte. The controller ports ($4016, $4017) affect only bits 4-0. Bits 7-5 repeat the corresponding bits from the previous read, usually 010 from the high byte $40.[1]

Few games rely on CPU open bus behavior. In most cases of commercial games that do, it seems to be unintentional due to a programming error. 

A few games read the region $6000-7FFF but have no WRAM present here. This can be a problem for emulators, as the original [iNES](INES.xhtml "INES") file format had no way to specify a lack of WRAM, leaving the emulator to provide WRAM behavior in that region by default. The [NES 2.0](NES_2_0.xhtml "NES 2.0") format corrects this with a WRAM size field. 

Some cartridges seem to have a weak non-deterministic effect on the values read from the open bus regions[2]. Where this has been observed, it seems to have an OR effect, overriding bits of the expected open-bus value (especially D7) as 1 instead of 0, and it tends to have greater frequency near the end of a 256-byte page. 

### Examples

  * **Low G Man** : WRAM instead of open bus behavior may cause a crash in Chapter 1 Scene 3B a few seconds into the boss fight music, or graphical glitches when using the boomerang weapon.[3]
  * **Battletoads & Double Dragon**: At the end of the first level, reads a values from ~$600X when the Abobo boss finished destroying the wall. A value of $00 read here will crash the game.[4]
  * **Captain Planet** , **Dirty Harry** , **Infiltrator** , **Mad Max** , **Paperboy** , **The Last Starfighter** : Games by Mindscape that rely on the controller port being exactly $41 for a button press to be recognized.[5]
  * **Castlevania** : Just after the introduction will read a few values from ~$7FFX before the first stage begins. (This is a benign bug: the value read appears to be inconsequential.)



### Indirect indexed instruction example

This example will illustrate cycle by cycle[6] the values read by the _LDA (indirect), Y_ instruction. 

    **LDA ($04), Y**

  * $05,$04 = $73FA
  * Y = $31


Cycle  | Operation  | Read address  | Value read   
---|---|---|---  
1 | read opcode | PC | $B1   
2 | read operand | PC | $04   
3 | read low byte of indirection | $0004 | $FA   
4 | read high byte of indirection | $0005 | $73   
5 | read early indirect value | $732B | $73 (open bus)   
6 | read fixed indirect value | $742B | $73 (open bus)   
  
Note from this example that the open bus behavior always returns the last value read on the data bus. Note that the last byte read before the open bus is the high byte of the indirection pointer. Even though the address of both subsequent reads are affected by Y, both times the open bus will read back the _last_ read value. 

## PPU open bus

The PPU has two data buses: the I/O bus, used to communicate with the CPU, and the video memory bus. Their open bus behavior differs. 

Writes to any PPU port, including the nominally read-only status port at $2002, load a value onto the PPU's I/O bus. Reading a value from $2004 or $2007 loads a byte from OAM, video memory, or the palette onto this bus. Reading the PPU's status port loads a value onto bits 7-5 of the bus, leaving the rest unchanged. Reading any PPU port, including write-only ports $2000, $2001, $2003, $2005, $2006, returns the PPU I/O bus's value.[1]

Open bus on the video memory bus behaves differently. Video memory's data bus is multiplexed with the low byte of the address bus on [pins 31 through 38](PPU_pinout.xhtml "PPU pin out and signal description"). Thus a read from an address with no memory connected will usually return the low byte of the address. This was used as a limited form of copy protection by a few games using [a configuration of CNROM](CNROM.xhtml "INES Mapper 185"). 

## Electrical basis

A data bus behaves as a dynamic latch. Once a low or high voltage is placed on each line of the bus, the capacitance of the long traces that make up the data bus allows it to "float" in place to be read again. 

Each memory or I/O circuit has a set of pins for [enables](https://en.wikipedia.org/wiki/Chip_select "wikipedia:Chip select"), or input signals that cause the circuit to take an action when all are in the correct state. For example, a [RAM](6264_static_RAM.xhtml "6264 static RAM") may have /CE1, CE2, and /OE. The / means it is active when the input is low (close to ground voltage); otherwise, it is active high (close to power voltage). Normally, the circuit's [three-state outputs](https://en.wikipedia.org/wiki/Three-state_logic "wikipedia:Three-state logic") are held in a [high impedance](https://en.wikipedia.org/wiki/high_impedance "wikipedia:high impedance") (hi-Z) state, effectively disconnected from the data bus. Only when all enables are in the correct state (in this case 0, 1, and 0 respectively) does the circuit output a value as voltages to the data bus, erasing the voltages already there. 

A circuit called a decoder generates enable signals based on other signals, especially address signals. The 74LS139 decoder in the Control Deck (U3 in the [Famicom schematic](File_Neswires_jpg.xhtml "File:Neswires.jpg")) decodes the cartridge ROM at $8000-$FFFF, primary work RAM at $0000-$1FFF, and PPU I/O at $2000-$3FFF, based on the CPU's clock divider output (M2) and CPU address bits 15-13. The CPU itself decodes reads of $4015 (APU status) and $4016-$4017 (controller read). But no circuit in the Control Deck decodes reads from $4000-$4014 or $4018-$7FFF. Cartridges with WRAM at $6000-$7FFF contain an additional circuit to decode WRAM, whether [by itself](PRG_RAM_circuit.xhtml "PRG RAM circuit") or as part of an [MMC](Mapper.xhtml "Mapper"). But when the CPU reads an address that no circuit decodes, all it sees on its data inputs is whatever was left to float on the data bus. 

## See also

  * [Bus conflict](Bus_conflict.xhtml "Bus conflict")



## References

  1. ↑ 1.0 1.1 1.2 [Forum post](https://forums.nesdev.org/viewtopic.php?p=143759#p143759): Riding the open bus
  2. ↑ [Forum post](https://forums.nesdev.org/viewtopic.php?p=284555#p284555): _MMC5 exact timings_ \- An investigation of open-bus areas of several cartridges.
  3. ↑ [Forum post](https://forums.nesdev.org/viewtopic.php?f=2&t=13105): Low G Man "plays nice with emulators" patch
  4. ↑ [Forum post](https://forums.nesdev.org/viewtopic.php?f=9&t=15222): Battletoads Double Dragon Powerpak Freeze
  5. ↑ [Forum post](https://forums.nesdev.org/viewtopic.php?f=9&t=3698): PowerPAK, Games that don't register input
  6. ↑ [6502_cpu.txt](http://nesdev.org/6502_cpu.txt): includes cycle by cycle instruction reference


