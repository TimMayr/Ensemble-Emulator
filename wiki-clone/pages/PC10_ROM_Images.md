# PC10 ROM-Images

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PC10_ROM-Images) | View [other pages](Special_AllPages.xhtml#PC10_ROM_Images)

Playchoice 10 ROM images can be stored in two formats: 

## iNES Format

PC10 games in [iNES](INES.xhtml "INES") format are indicated by Bit1 of Byte 7 of the iNES header. If the flag is set, then the file should contain some additional entries after the PRG ROM and CHR ROM areas: 

  * 8Kbyte INST ROM (containing data and Z80 code for instruction screens)
  * 16 bytes RP5H01 PROM Data output (needed to decrypt the INST ROM)
  * 16 bytes RP5H01 PROM CounterOut output (needed to decrypt the INST ROM) (usually constant: 00,00,00,00,FF,FF,FF,FF,00,00,00,00,FF,FF,FF,FF)



The two required PROM sections are missing in older ROM images. A tool for upgrading such incomplete dumps can be found at <http://problemkaputt.de/pc10make.zip>

Note: Some very old ROM images don't have the PC10 flag set in the header, and, instead, they declare the 8K INST ROM as an additional CHR ROM bank. 

## MAME Format

Instead of using a single ROM image file, MAME stores all ROM, EPROM, and PROM chips in separate files. 

The PROM data is typically stored in a file called "security.prm". It contains only the 16 Data bytes (not the CounterOut bytes). All bits in the PROM file are inverted, and the bit ordering is reversed: bit0 (the first bit of the PROM's serial bitstream) is stored in bit7 of the 1st byte of the file). 

## PC10 Emulators

The iNES format is used by no$nes. The MAME format is used by MAME. 

Categories: [File formats](Category_File_formats.xhtml)
