# NES 2.0 Mapper 446

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_446) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_446)

**NES 2.0 Mapper 446** denotes Mindkids' **SMD172B_FPGA** circuit board. It is used for some large multicarts from _Pixel Games_ as well as for homebrew games such as: 

  * _Jay and Silent Bob Mall Brawl_
  * _Blazing Rangers_ /_炎のRangerman_



## Contents

  * 1 Supervisor Registers
    * 1.1 Mapper Selection ($5000, write)
    * 1.2 Outer 8 KiB PRG-ROM Bank LSB ($5001, write)
    * 1.3 Outer 8 KiB PRG-ROM Bank MSB ($5002, write)
    * 1.4 Inner 8 KiB PRG-ROM Bank Mask ($5003, write)
    * 1.5 Flags ($5004, write)
    * 1.6 CHR RAM protect ($5005, write)
    * 1.7 Outer 8 KiB CHR-RAM Bank ($5006, write)



# Supervisor Registers

## Mapper Selection ($5000, write)
    
    
    D~[L..M MMMM]
       |  +-++++-- Mapper for the FPGA to emulate
       +---------- 1=Lock supervisor registers until reset
    

The set of mappers that the FPGA can emulate differs across cartridges and is selected by the NES 2.0 Submapper. 

NES 2.0 Submapper | $5000.0-4 | Mapper   
---|---|---  
0 | 01 | [SKROM](SxROM.xhtml "SKROM")  
1 | 00 | [UNROM](UxROM.xhtml "UNROM")  
1 | 01 | [TKROM](TxROM.xhtml "TKROM")  
1 | 02 | [BNROM](INES_Mapper_034.xhtml "BNROM")  
1 | 03 | [CNROM](CNROM.xhtml "CNROM")  
1 | 04 | [ANROM](AxROM.xhtml "ANROM")  
1 | 05 | [SKROM](SxROM.xhtml "SKROM")  
1 | 06 | [SNROM](SxROM.xhtml "SNROM")  
1 | 07 | [SUROM](SxROM.xhtml "SUROM")  
1 | 08 | [GNROM](GxROM.xhtml "GNROM")  
1 | 09 | [PNROM](MMC2.xhtml "PNROM")  
1 | 0A | [HKROM](MMC6.xhtml "HKROM")  
1 | 0B | [INES Mapper 152](INES_Mapper_152.xhtml "INES Mapper 152")  
1 | 0E | [TKSROM](TKSROM.xhtml "TKSROM")  
1 | 0F | [INES Mapper 189](INES_Mapper_189.xhtml "INES Mapper 189")  
1 | 10 | [INES Mapper 026](VRC6.xhtml "INES Mapper 026")  
1 | 12 | [INES Mapper 022](VRC2_and_VRC4.xhtml "INES Mapper 022")  
1 | 15 | [INES Mapper 025](VRC2_and_VRC4.xhtml "INES Mapper 025")  
1 | 18 | [INES Mapper 023](VRC2_and_VRC4.xhtml "INES Mapper 023")  
1 | 1A | [VRC1](VRC1.xhtml "VRC1")  
2 | 00 | [UNROM](UxROM.xhtml "UNROM")  
2 | 01 | [TKROM](TxROM.xhtml "TKROM")  
2 | 02 | [BNROM](INES_Mapper_034.xhtml "BNROM")  
2 | 03 | [CNROM](CNROM.xhtml "CNROM")  
2 | 04 | [ANROM](AxROM.xhtml "ANROM")  
2 | 05 | [SKROM](SxROM.xhtml "SKROM")  
2 | 06 | [SNROM](SxROM.xhtml "SNROM")  
2 | 07 | [SUROM](SxROM.xhtml "SUROM")  
2 | 08 | [GNROM](GxROM.xhtml "GNROM")  
2 | 09 | [TLROM](TxROM.xhtml "TLROM")  
2 | 0A | [HKROM](MMC6.xhtml "HKROM")  
2 | 0B | [INES Mapper 152](INES_Mapper_152.xhtml "INES Mapper 152")  
2 | 0D | [TKROM](TxROM.xhtml "TKROM")  
2 | 0E | [TKSROM](TKSROM.xhtml "TKSROM")  
2 | 0F | [INES Mapper 189](INES_Mapper_189.xhtml "INES Mapper 189")  
2 | 10 | [INES Mapper 026](VRC6.xhtml "INES Mapper 026")  
2 | 12 | [INES Mapper 022](VRC2_and_VRC4.xhtml "INES Mapper 022")  
2 | 15 | [INES Mapper 025](VRC2_and_VRC4.xhtml "INES Mapper 025")  
2 | 18 | [INES Mapper 023](VRC2_and_VRC4.xhtml "INES Mapper 023")  
2 | 1A | [VRC1](VRC1.xhtml "VRC1")  
3 | 01 | [H3001](INES_Mapper_065.xhtml "INES Mapper 065")  
  
## Outer 8 KiB PRG-ROM Bank LSB ($5001, write)
    
    
    D~[BBBB BBBB]
       ++++-++++- PRG A20..A13
    

## Outer 8 KiB PRG-ROM Bank MSB ($5002, write)
    
    
    D~[BBBB BBBB]
       ++++-++++- PRG A28..A21
    

## Inner 8 KiB PRG-ROM Bank Mask ($5003, write)
    
    
    D~[MMMM MMMM]
       ++++-++++- PRG A20..A13 mask
    

The register has an inverted meaning in Submapper 2 vs. Submappers 0-1. In Submappers 0-1, an "1" bit means that the Outer Bank bit is used. In Submapper 2, an "1" bit means that the Inner Bank bit is used. 

## Flags ($5004, write)
    
    
    D~[..CC ...M]
         ||    +- Mirroring (V=1)
         ++------ Inner CHR-RAM address mask
                  0x: 256 KiB
                  10: 128 KiB
                  11: 32 KiB
    

The mirroring bit has no effect when selecting an FPGA mapper with software-selectable mirroring. 

## CHR RAM protect ($5005, write)
    
    
    D~[.... .P..]
             +- 1: Write-protect CHR-RAM
    

## Outer 8 KiB CHR-RAM Bank ($5006, write)
    
    
    D~[...B BBBB]
          +-++++- CHR A17..A13
    

Categories: [Mappers with flash save](Category_Mappers_with_flash_save.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multi-ASIC mappers](Category_Multi_ASIC_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
