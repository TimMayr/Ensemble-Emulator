# NES 2.0 Mapper 513

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_513) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_513)

iNES 2.0 Mapper 513 is used for 美少女 夢工場三 (Měi Shàonǚ Mèng Gōngchǎng III), an unauthorized port by Sachen of one of the Princess Maker games. Its UNIF board name is **UNL-SA-9602B**. It uses a standard [MMC3](MMC3.xhtml "MMC3") clone with 32 KiB battery-backed CHR-RAM, no CHR-ROM, and the MMC3's CHR bank register bits 6 and 7 repurposed as 512 KiB outer PRG bank numbers. 

## Banks

  * CPU $8000-$9FFF (or $C000-$DFFF): 8 KB switchable PRG ROM bank; outer bank is applied
  * CPU $A000-$BFFF: 8 KB switchable PRG ROM bank; outer bank is applied
  * CPU $C000-$DFFF (or $8000-$9FFF): 8 KB PRG ROM bank, fixed to the second-last bank; outer bank is NOT applied
  * CPU $E000-$FFFF: 8 KB PRG ROM bank, fixed to the last bank; outer bank is NOT applied



## Registers

All same as normal MMC3, except that the CHR Bank Select registers ($8000 values 0-5) have a slightly different meaning: 
    
    
    7  bit  0
    ---- ----
    PP.C CCCC
    || +-++++-- Select 1 KiB CHR-RAM bank
    ++--------- Select 512 KiB PRG-ROM bank (outer bank)
    

## Notes

  * The two fixed 8 KiB PRG Banks ($C000-$DFFF or $8000-$9FFF, $E000-$FFFF) are hard-wired to 512 KiB outer bank 0.
  * The current dump of the game has a PRG ROM size of 1536 KiB. FCEUX emulates outer PRG ROM bank 3 being identical to bank 2. On real hardware, selecting bank 3 would activate two memory chips at the same time, which may potentially damage them.
  * As with [SOROM, SUROM and SXROM](MMC1.xhtml#SOROM.2C_SUROM_and_SXROM "MMC1"), if the several CHR bank registers do not specify the same outer PRG ROM bank, then PRG ROM presumably would be bankswitched as the PPU renders.
  * [PCB image and analysis](http://forums.nesdev.org/viewtopic.php?f=9&p=219216)



Categories: [Mappers with battery-backed CHR-RAM](Category_Mappers_with_battery_backed_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
