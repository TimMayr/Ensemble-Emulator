# ROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/ROM) | View [other pages](Special_AllPages.xhtml#ROM)

**Read-only memory** is any of several types of computer memory designed to hold constant data and cannot be modified during the ordinary course of operation. 

## Solid state ROM

These forms of ROM are integrated circuits with no moving parts: 

[Mask ROM](https://en.wikipedia.org/wiki/Mask_ROM "wikipedia:Mask ROM")
    Program is encoded on the "mask", or the shape of the integrated circuit, when the chip is fabricated. During the NES's commercial era, retail NES games were mass-produced as mask ROM because they were very cheap in volume.
[Programmable read-only memory](https://en.wikipedia.org/wiki/Programmable_read-only_memory "wikipedia:Programmable read-only memory") (PROM), One-time programmable (OTP)
    Early PROMs were an array of fuses that could be changed from 1 to 0. Later PROMs were UVEPROMs without a window.
[Ultraviolet erasable programmable read-only memory](https://en.wikipedia.org/wiki/EPROM "wikipedia:EPROM") (UVEPROM)
    Variant of PROM using field-effect transistors that turn back to 1 upon exposure to strong ultraviolet light through a window over the chip. In general, one would erase one of them by putting it in a box that shines UV on them for about 20 minutes. Developer carts during the NES's commercial era used this sort of EPROM.
[Electrically erasable programmable read-only memory](https://en.wikipedia.org/wiki/EEPROM "wikipedia:EEPROM") (EEPROM)
    Variant of PROM using floating-gate transistors that can be set to 0 using electricity or set to 1 using more electricity. Early solid-state disk devices were EEPROM based.
[Flash memory](https://en.wikipedia.org/wiki/Flash_memory "wikipedia:Flash memory")
    EEPROM that can be erased a page at a time, where a page can be several thousand bytes or more. Modern developer and retail carts use flash memory.

An NES Game Pak has a PRG ROM connected to the [CPU](CPU.xhtml "CPU") and either a second CHR ROM or a CHR RAM (or, rarely, both) connected to the [PPU](PPU.xhtml "PPU"). 

Solid-state ROMs are measured by the number of words times the length of each word. On the NES and Super NES, each word is 8 bits; on the Sega Genesis it is 16 bits. The size in words of the vast majority of ROMs is a power of 2: 16384, 32768, etc. [Some Super NES carts support multiple PRG ROM chips](http://forums.nesdev.org/viewtopic.php?p=72781#p72781) allowing a non-power-of-two as a sum of different ROM sizes, but on the NES, no Nintendo board supports more than one PRG ROM or more than one CHR ROM, except for some very early mapper 0 boards ([RTROM and STROM](NROM.xhtml#Variants "NROM")) that took two 8 KiB PRG ROMs and one 8 KiB CHR ROM. 

## Optical disc

[CD-ROM](https://en.wikipedia.org/wiki/CD-ROM "wikipedia:CD-ROM")
    An optical disc that holds data to be copied into a RAM and used there. There are three kinds of CD: CD-ROM acts like Mask ROM, CD-R acts like PROM, and CD-RW with [packet writing](https://en.wikipedia.org/wiki/packet_writing "wikipedia:packet writing") acts like flash memory. Access to CD is much slower than access to solid-state ROM.

## ROM images

The term **ROM** can also refer to a **ROM image** , a computer file that represents the entire contents of a ROM or a set of ROMs. The majority of NES ROM images are distributed in the [iNES](INES.xhtml "INES") or [NES 2.0](NES_2_0.xhtml "NES 2.0") format. 
