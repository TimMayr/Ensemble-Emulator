# INES Mapper 091

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_091) | View [other pages](Special_AllPages.xhtml#INES_Mapper_091)

iNES Mapper 091 was used by the original release of _Super Fighter III_ from an unknown publisher, but is more commonly known by a clone board from J.Y. Company. 

INES Mapper 091 submapper table   
---  
Submapper # | Board | Games   
0 | JY830623C | _Street Fighter 3_ , _Mortal Kombat II_ , _Dragon Ball Z 2_ , _Mario & Sonic 2_  
0 | YY840238C | _1995 Super HIK 4-in-1 (JY-016)_ , _1995 Super HiK 4-in-1 (JY-017)_  
1 | EJ-006-1 | _Super Fighter III_  
  
Submapper 0 has a PA12-based IRQ counter and hard-wired mirroring; submapper 1 an M2-based IRQ counter and selectable mirroring. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 CHR-ROM Bank Select ($6000-$6003, write)
    * 2.2 Nametable Mirroring Select ($6004-$6005, write, submapper 1 only)
    * 2.3 IRQ Counter Register ($6006-$6007, write, submapper 1 only)
    * 2.4 PRG-ROM Bank Select ($7000-$7001, write)
    * 2.5 IRQ Stop/Acknowledge ($7006, write)
    * 2.6 IRQ Start/Reset ($7007, write)
    * 2.7 Outer Bank register ($8000-$9FFF, write, submapper 0 only)
  * 3 Notes



# Banks

  * CPU $8000-$9FFF: 8 KiB switchable PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG-ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG-ROM bank, hard-wired to last bank
  * PPU $0000-$07FF: 2 KiB switchable CHR-ROM bank
  * PPU $0800-$0FFF: 2 KiB switchable CHR-ROM bank
  * PPU $1000-$17FF: 2 KiB switchable CHR-ROM bank
  * PPU $1800-$1FFF: 2 KiB switchable CHR-ROM bank



# Registers

## CHR-ROM Bank Select ($6000-$6003, write)

Mask: $F003/$F007 (submapper 0/1) 

  * $6000: Select 2 KiB CHR-ROM bank at PPU $0000-$07FF
  * $6001: Select 2 KiB CHR-ROM bank at PPU $0800-$0FFF
  * $6002: Select 2 KiB CHR-ROM bank at PPU $1000-$17FF
  * $6003: Select 2 KiB CHR-ROM bank at PPU $1800-$1FFF



## Nametable Mirroring Select ($6004-$6005, write, submapper 1 only)

Mask: $F007 

  * $6004: Select horizontal mirroring, value ignored
  * $6005: Select vertical mirroring, value ignored



## IRQ Counter Register ($6006-$6007, write, submapper 1 only)

Mask: $F007 

  * $6006: Set IRQ Counter LSB
  * $6007: Set IRQ Counter MSB



## PRG-ROM Bank Select ($7000-$7001, write)

Mask: $F003/$F007 (submapper 0/1) 

  * $7000: Select 8 KiB PRG-ROM bank at CPU $8000-$9FFF
  * $7001: Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF



## IRQ Stop/Acknowledge ($7006, write)

Mask: $F003 (submapper 0)/$F007 (submapper 1) 

Value is ignored. 

## IRQ Start/Reset ($7007, write)

Mask: $F003/$F007 (submapper 0/1) 

Value is ignored. 

## Outer Bank register ($8000-$9FFF, write, submapper 0 only)

Mask: unknown 
    
    
    A~FEDC BA98 7654 3210
      -------------------
      .... .... .... .PPC
                      ||+- Select outer 512 KiB CHR-ROM bank (CHR A19)
                      ++-- Select outer 128 KiB PRG-ROM bank (PRG A17-A18)
    

# Notes

  * Submapper 0: the IRQ counter is a binary counter clocked by PPU A12, and simply counts a fixed number of 64 unfiltered rises of PPU A12.
  * Submapper 1: the IRQ counter is a binary counter clocked by the M2 signal with a factor of 5/4, meaning counting down by five every fourth M2 cycle.
  * Submapper 1's _Super Fighter III_ runs a hardware test when resetting with Select+Start+B pressed. The test exists in J.Y. Company's hacked versions as well, but was not adjusted for the simpler hardware and thus will fail some tests.
  * The commonly available ROM image of _Super Fighter III_ is a mapper hack for [INES Mapper 197](INES_Mapper_197.xhtml "INES Mapper 197").



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with fixed-timing scanline IRQs](Category_Mappers_with_fixed_timing_scanline_IRQs.xhtml)
