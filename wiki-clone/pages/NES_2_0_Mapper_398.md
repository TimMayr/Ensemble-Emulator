# NES 2.0 Mapper 398

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_398) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_398)

**NES 2.0 Mapper 398** denotes the 晶太 YY840820C circuit board, used for a single multicart: 

  * _1995 Super HiK 5-in-1 - 新系列米奇老鼠組合卡_ (JY-048)



It adds an outer bank register to a [VRC4](VRC2_and_VRC4.xhtml "VRC4") clone (A0/A1, VRC4f) together with ridiculously complex logic to implement a GNROM-like mode. The first 128 KiB of PRG-ROM and 512 KiB (first chip) of CHR-ROM contain _World Hero_ , the second 128 KiB of PRG-ROM and 128 KiB (second chip) of CHR-ROM contain four CNROM-256 games. 

## Outer Bank Register ($8000-$FFFF)
    
    
    Mask: $8000
     
    A~FEDC BA98 7654 3210
      -------------------
      1... .... BA.. ....
                ++-------- PRG A17..A16
                |+-------- CHR A16
                +--------- CHR chip select (CHR A19 in NES 2.0 ROM image)
                |           0: first (512 KiB)
                |           1: second (128 KiB)
                +--------- Mode Select
                            0: VRC4, first CHR chip
                            1: GNROM-like, second CHR chip
    Power-on value: $C0
    

The Outer Bank Register overlaps the VRC4 registers in the same address range. 

## GNROM-like mode operation

In the GNROM-like mode (Outer Bank Register 'B' bit =1), the address bits come from the following sources: 
    
    
    PRG-ROM A13: CPU A13
    PRG-ROM A14: CPU A14
    PRG-ROM A15: VRC4 CHR A12
    PRG-ROM A16: Outer Bank Register 'A' bit
    PRG-ROM A17: Outer Bank Register 'B' bit, i.e. always 1
    CHR-ROM A10: PPU A10
    CHR-ROM A11: PPU A11
    CHR-ROM A12: PPU A12
    CHR-ROM A13: VRC4 CHR A10
    CHR-ROM A14: VRC4 CHR A11
    CHR-ROM A15: VRC4 CHR A12
    CHR-ROM A16: Outer Bank Register 'A' bit
    

As with SOROM, SUROM and SXROM, if the several CHR bank registers' CHR A12 bit does not specify the same outer bank, then PRG-ROM will would be bankswitched as the PPU renders. 

Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
