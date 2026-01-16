# NES 2.0 Mapper 543

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_543) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_543)

NES 2.0 Mapper 543 denotes the unmarked printed circuit board of the 1996 無敵智カ卡 5-in-1 (CH-501) multicart. It has 2 MiB of PRG-ROM, 2x32 KiB of battery-backed PRG-RAM, and 32 KiB (8 KiB used) of CHR-RAM. Bankswitching is provided by an AX5904 [MMC1](MMC1.xhtml "MMC1") clone connected in [SOROM](SxROM.xhtml "SOROM") or [SNROM](SxROM.xhtml "SNROM") configurations, plus an additional PAL providing outer bank switching functionality. The circuit board has provisions for additional 2 MiB of PRG-ROM and a third battery-backed 32 KiB PRG-RAM chip. 

## Outer Bank Register ($5000, write)
    
    
    Mask: $F000
    
    D~7654 3210
      ---------
      .... S...
           +---- Serial data
    

The outer bank register is similar to the MMC1 registers in that it expects the four-bit bank number as serial data, with the lowest bit sent first. Unlike the MMC1's serial register, there does not seem to be a means of resetting the shift register, and its width is four bits instead of five. It selects the upper PRG-ROM address lines and SRAM chips as follows: 
    
    
    D~3210
      ----
      1PpP
      |||+- PRG A18, SRAM #1 A14, SRAM #2 A13
      ||+-- PRG A19, SRAM #1 (bit clear)/#2 (bit set) chip select
      |+--- PRG A20, SRAM #2 A14
      +---- 1 during game-play, 0 during menu
    
    

  * MMC1 CHR A15 is connected to SRAM #1 A13.
  * Because the two SRAM chips are connected differently, PRG A19 implicitly selects between SOROM (0) and SNROM (1) configurations.
  * Since both SRAM chips are battery-backed, the original SOROM distinction between a volatile and non-volatile 8 KiB bank does not apply here.



This means that games $08 and $09 use the [SOROM](SxROM.xhtml "SOROM") configuration, with the MMC1's CHR A15 output selecting between two inner 8 KiB banks, and PRG A18 selecting between two outer 16 KiB banks in the first 32 KiB SRAM chip. Games $0A, $0B, $0E and $0F use the [SNROM](SxROM.xhtml "SNROM") configuration, with the MMC1's CHR A15 output ignored, and PRG A18 and A20 selecting four outer 8 KiB banks in the second 32 KiB SRAM chip. Games $0C and $0D would use the same save RAM space as games $08 and $09 and are therefore invalid and unused by the cartridge. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with large PRG RAM](Category_Mappers_with_large_PRG_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
