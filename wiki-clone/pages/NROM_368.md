# NROM-368

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NROM-368) | View [other pages](Special_AllPages.xhtml#NROM_368)

**NROM-368** is a name for a theoretical extension to all mappers incapable of banking PRG, such as [NROM](NROM.xhtml "NROM"), [CNROM](CNROM.xhtml "CNROM"), [CPROM](CPROM.xhtml "CPROM"), [Sunsoft 1](INES_Mapper_184.xhtml "INES Mapper 184"), and [CHR-less](INES_Mapper_218.xhtml "INES Mapper 218"), allowing 46 KiB of linearly addressed ROM instead of 32 KiB. The name comes from the naming scheme for Nintendo's NROM boards, as 368 kilobits of PRG ROM are addressable. Its original intent was to provide more space for a game written in C, as cc65 tends not to be good at optimizing for space. 

To date it has not been used for any ROM release. 

## Format

The PRG ROM is 47104 bytes in size. Due to constraints of the [iNES](INES.xhtml "INES") format, it is padded at the beginning with 2048 bytes of ignored data so that it is an even multiple of 16384 bytes; the rest is loaded in order into $4800-$7FFF, $8000-$BFFF, and $C000-$FFFF. 

So an [iNES](INES.xhtml "INES") or [NES 2.0](NES_2_0.xhtml "NES 2.0") image would look like this: 

  1. 16 bytes: Header. PRG ROM size must be 3. Trainer and battery are forbidden; NES 2.0 PRG RAM size must be 0.
  2. 2048 bytes: Ignored.
  3. 47104 bytes: PRG ROM mapped to $4800-$FFFF.
  4. 8192× _n_ bytes: CHR ROM mapped to PPU $0000-$1FFF.



The [UNIF](UNIF.xhtml "UNIF") encapsulation should ignore all padding and just have `PRG0` be exactly 47104 bytes. 

## Hardware

Just as the addition of PRG RAM and bus conflict avoidance to these mappers takes [one chip to decode](PRG_RAM_circuit.xhtml "PRG RAM circuit"), the addition of $4800-$7FFF also takes one chip that uses /ROMSEL, M2, and A14-A11 to construct an enable signal for the PRG ROM. This is a [74HC85](7485.xhtml "7485") comparator. 

    _TO DO: Once the circuit is tested on a real PCB, details of how to wire up the '85 will be given here._

A14 through A0 go to the PRG ROM as is, and /ROMSEL goes to A15. When burning the EPROM, you have to rearrange the 16 KiB segments of the PRG ROM into the order 1, 2, 0, 0, as /ROMSEL is inverted compared to A15. 

## References

  * [BBS topic](http://forums.nesdev.org/viewtopic.php?t=8921), including [wiring instructions](http://forums.nesdev.org/viewtopic.php?p=94322#p94322)



Categories: [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml)
