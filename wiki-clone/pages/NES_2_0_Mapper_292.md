# NES 2.0 Mapper 292

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_292) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_292)

**NES 2.0 Mapper 292** is used for the unlicensed game _Dragon Fighter_ from "Flying Star", not to be confused with the licensed game from Natsume of the same name. Its PCB name is "BMW8544", its UNIF board name is **UNL-DRAGONFIGHTER**. It's an [MMC3](MMC3.xhtml "MMC3") clone with custom CHR banking both for protection purposes and to access 512 KiB of CHR-ROM. PRG banking, mirroring and scanline IRQ work as on a normal MMC3. 

## Contents

  * 1 Banks
  * 2 Extra Registers
    * 2.1 Extra Index Register ($6000-$7FFF, write)
    * 2.2 Extra Data Register #0 ($6000-$7FFF, read)
    * 2.3 Extra Data Register #1 ($6000-$7FFF, read)
    * 2.4 Latch Register (Write)
  * 3 Effective CHR banks
    * 3.1 PPU $0000-$07FF
    * 3.2 PPU $0800-$0FFF
    * 3.3 PPU $1000-$1FFF
  * 4 Errata
  * 5 See also



# Banks

  * CPU $8000-$9FFF: 8 KiB switchable/fixed PRG-ROM window
  * CPU $A000-$BFFF: 8 KiB switchable PRG-ROM window
  * CPU $C000-$DFFF: 8 KiB switchable/fixed PRG-ROM window
  * CPU $E000-$FFFF: 8 KiB fixed PRG-ROM window
  * PPU $0000-$07FF: 2 KiB switchable CHR-ROM bank
  * PPU $0800-$0FFF: 2 KiB switchable CHR-ROM bank
  * PPU $1000-$1FFF: 4 KiB switchable CHR-ROM bank



# Extra Registers

There are four extra registers that are filled by writing or reading (!) from the CPU $6000-$7FFF address range when PRG-RAM is enabled in the MMC3's A001 register. 

## Extra Index Register ($6000-$7FFF, write)
    
    
    D~[ABR. ....]
       ||+-------- Select Extra Data Register #0 or #1 on next $6000-$7FFF read
       ++--------- Always 1
    

It's hypothesized that clearing the A and B bits may switch the two CHR pattern table halves to normal MMC3 CHR banking, but this has not yet been verified on real hardware. The game keeps both bits enabled at all times. 

## Extra Data Register #0 ($6000-$7FFF, read)
    
    
    D~[XXXX XXXX]
       ++++-++++- XOR value for 2 KiB CHR-ROM bank at PPU $0000-$07FF
    

This register is filled by _reading_ from $6000-$7FFF when the Extra Index Register's R bit is 0. It is filled with the value that had been latched during the last CPU write to any address. 

## Extra Data Register #1 ($6000-$7FFF, read)
    
    
    D~[.BAA AAAA]
        |++-++++- 4 KiB CHR-ROM bank at PPU $1000-$1FFF (CHR A17..A12)
        +-------- Upper bit of 2 KiB CHR-ROM bank at PPU $0800-$0FFF (CHR A18)
    

This register is filled by _reading_ from $6000-$7FFF when the Extra Index Register's R bit is 1. It is filled with the value that had been latched during the last CPU write to any address. 

## Latch Register (Write)

All CPU writes, including writes to zero page memory, are visible on the cartridge connector. The mapper keeps the data of the last CPU write to any address in a temporary latch register, so it can store that data in one of its two Extra Data registers when so requested by _reading_ from $6000-$7FFF. The game writes the desired values to zero page. 

# Effective CHR banks

The address lines of CHR-ROM are effectively determined as follows: 

### PPU $0000-$07FF
    
    
    CHR A10: PPU A10
    CHR A11: Extra Register 0 Bit 0 XOR MMC3 A11 (MMC3 register 0, bit 1)
    CHR A12: Extra Register 0 Bit 1 XOR MMC3 A12 (MMC3 register 0, bit 2)
    CHR A13: Extra Register 0 Bit 2 XOR MMC3 A13 (MMC3 register 0, bit 3)
    CHR A14: Extra Register 0 Bit 3 XOR MMC3 A14 (MMC3 register 0, bit 4)
    CHR A15: Extra Register 0 Bit 4 XOR MMC3 A15 (MMC3 register 0, bit 5)
    CHR A16: Extra Register 0 Bit 5 XOR MMC3 A16 (MMC3 register 0, bit 6)
    CHR A17: Extra Register 0 Bit 6 XOR MMC3 A17 (MMC3 register 0, bit 7)
    CHR A18: Extra Register 0 Bit 7
    Or in C code: chrBank2K =extraRegister[0] ^ (mmc3Register[0] >> 1);
    

Note that when Bit 7 in the MMC3's index register is set, MMC3 registers 2/3 instead of register 0 apply. 

### PPU $0800-$0FFF
    
    
    CHR A10: PPU A10
    CHR A11: MMC3 A11 (i.e. MMC3 register 1 bit 1)
    CHR A12: MMC3 A12 (i.e. MMC3 register 1 bit 2)
    CHR A13: MMC3 A13 (i.e. MMC3 register 1 bit 3)
    CHR A14: MMC3 A14 (i.e. MMC3 register 1 bit 4)
    CHR A15: MMC3 A15 (i.e. MMC3 register 1 bit 5)
    CHR A16: MMC3 A16 (i.e. MMC3 register 1 bit 6)
    CHR A17: MMC3 A17 (i.e. MMC3 register 1 bit 7)
    CHR A18: Extra Register 1 Bit 6
    Or in C code: chrBank2K =((extraRegister[1] << 1) & 0x80) ^ (mmc3Register[1] >> 1);
    

Note that when Bit 7 in the MMC3's index register is set, MMC3 registers 4/5 instead of register 1 apply. 

### PPU $1000-$1FFF
    
    
    CHR A10: PPU A10
    CHR A11: PPU A11
    CHR A12: Extra Register 1 Bit 0
    CHR A13: Extra Register 1 Bit 1
    CHR A14: Extra Register 1 Bit 2
    CHR A15: Extra Register 1 Bit 3
    CHR A16: Extra Register 1 Bit 4
    CHR A17: Extra Register 1 Bit 5
    CHR A18: Always 0, i.e. restricted to the first 256 KiB CHR-ROM half.
    Or in C code: chrBank4K =extraRegister[1] & 0x3F;
    

# Errata

  * Contrary to original thought, the PRG-ROM bank at CPU $8000-$9FFF is not determined by any extra register.
  * The game has single-frame graphical glitches on [real hardware](https://www.youtube.com/watch?v=ba-aT2s6SlY) when a metasprite is changed to a different animation cel. The reason is that OAM data is updated one frame later than the CHR bank registers. A "better than original hardware" emulation could correct this problem by applying changes to the CHR bank registers (both MMC3 and extra) affecting PPU 0000-0FFF only at the next sprite DMA ($4014 write) while applying changes affecting PPU 1000-1FFF immediately.



# See also

[FCEUX emulation source code](https://github.com/TASEmulators/fceux/blob/master/src/boards/BMW8544.cpp)

Categories: [Mappers triggering on reads](Category_Mappers_triggering_on_reads.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
