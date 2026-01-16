# NES 2.0 Mapper 468

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_468) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_468)

**NES 2.0 Mapper 468** denotes **BlazePro** multicarts based on the Altera MAX II CPLD. It can emulate sixteen mappers at once, selected via the MSB nibble at $5700. Different multicarts program the CPLD with different sets of mappers, denoted via the NES 2.0 submapper field. The LSB nibble of $5700 contains mapper-specific flags. 

Submapper | $5700.4-7 | Mapper   
---|---|---  
0 | 0 | [SxROM](MMC1.xhtml "SxROM")  
3 | 2 | [SxROM](MMC1.xhtml "SxROM")  
0 | 1 | [SUROM](SxROM.xhtml "SUROM")  
0 | 4 | [ANROM](AxROM.xhtml "ANROM")/[BNROM](INES_Mapper_034.xhtml "BNROM")  
1 | 4 | [ANROM](AxROM.xhtml "ANROM")/[BNROM](INES_Mapper_034.xhtml "BNROM")  
0 | 6 | [AOROM](AxROM.xhtml "AOROM")  
1 | 6 | [AOROM](AxROM.xhtml "AOROM")  
0 | 10 | [PNROM](MMC2.xhtml "PNROM")  
1 | 0 | [TxROM](TxROM.xhtml "TxROM"), 256 KiB CHR   
1 | 1 | [TxROM](TxROM.xhtml "TxROM"), 128 KiB CHR   
1 | 2 | [TxSROM](INES_Mapper_118.xhtml "TxSROM")  
0 | 8 | [FxROM](FxROM.xhtml "FxROM")  
0 | 9 | [UNROM](UxROM.xhtml "UNROM"), [IF-12 (Holy Diver)](INES_Mapper_078.xhtml "INES Mapper 078")  
0 | 11 | [UOROM](UxROM.xhtml "UOROM")  
1 | 7 | [UxROM](UxROM.xhtml "UxROM")  
3 | 7 | [UxROM](UxROM.xhtml "UxROM")  
0 | 5 | (C)NROM, [BF9097](INES_Mapper_071.xhtml#Mirroring_\(%248000-%249FFF\) "INES Mapper 071")  
1 | 5 | (C)NROM, [BF9097](INES_Mapper_071.xhtml#Mirroring_\(%248000-%249FFF\) "INES Mapper 071")  
0 | 12 | [GNROM](GxROM.xhtml "GNROM")  
0 | 13 | [Color Dreams](Color_Dreams.xhtml "Color Dreams")  
1 | 12 | [GNROM](GxROM.xhtml "GNROM")  
1 | 13 | [Color Dreams](Color_Dreams.xhtml "Color Dreams")  
2 | 0 | [VRC2](VRC2_and_VRC4.xhtml "VRC2") A0/A1   
2 | 1 | [VRC4](VRC2_and_VRC4.xhtml "VRC4") A0/A1   
2 | 2 | [VRC2](VRC2_and_VRC4.xhtml "VRC2") A1/A0   
2 | 3 | [VRC4](VRC2_and_VRC4.xhtml "VRC4") A1/A0   
3 | 0 | [VRC6](VRC6.xhtml "VRC6"), 256 KiB CHR   
3 | 1 | [VRC6](VRC6.xhtml "VRC6"), 128 KiB CHR   
4 | 0 | [VRC1](VRC1.xhtml "VRC1")  
4 | 1 | [VRC7](VRC7.xhtml "VRC7")  
4 | 4 | [VRC3](VRC3.xhtml "VRC3")  
0 | 14 | [Nanjing](INES_Mapper_163.xhtml "INES Mapper 163")  
1 | 14 | [Nanjing](INES_Mapper_163.xhtml "INES Mapper 163")  
0 | 7 | [SMB2J](INES_Mapper_040.xhtml "INES Mapper 040")  
5 | 0 | [FME-7](Sunsoft_FME_7.xhtml "FME-7")  
  
Categories: [Multi-ASIC mappers](Category_Multi_ASIC_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
