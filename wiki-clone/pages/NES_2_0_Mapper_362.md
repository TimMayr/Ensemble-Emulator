# NES 2.0 Mapper 362

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_362) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_362)

NES 2.0 Mapper 362 is used for an early VRC4-clone-based multicart PCB (830506C) from J.Y. Company, preceding the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"): 

  * _1995 Super HiK 4-in-1 (JY-005)_



The [VRC4](VRC2_and_VRC4.xhtml "VRC4") clone is wired in the VRC4f configuration, i.e. address 0x001s to A0 and address 0x002s to A1. As each individual game on that multicart only has 128 KiB of PRG/CHR-ROM, the two highest CHR-ROM bank bits are repurposed as outer bank bits: 

### CHR Select 0 high ($B001)
    
    
    7  bit  0
    ---------
    ...O OLLL
       | |+++- Select D4..D6 KiB of 1 KiB CHR-ROM bank at PPU $0000-$03FF
       +-+---- Select outer 128 KiB PRG/CHR-ROM bank when PPU $0000-$03FF is accessed
    

  * The high CHR Select register ($B004) is ignored.
  * The other seven high CHR bank select registers continue the pattern for the other seven 1 KiB PPU banks.



## Notes

  * As with [SOROM, SUROM and SXROM](MMC1.xhtml#SOROM.2C_SUROM_and_SXROM "MMC1"), if the several CHR bank registers do not specify the same outer bank, then PRG-ROM will would be bankswitched as the PPU renders.
  * As with [NES 2.0 Mapper 298](NES_2_0_Mapper_298.xhtml "NES 2.0 Mapper 298"), the VRC4 clone's F003 register only acknowledges IRQs but does not move the 'A' control bit to the 'E' control bit.



Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
