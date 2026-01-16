# INES Mapper 122

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_122) | View [other pages](Special_AllPages.xhtml#INES_Mapper_122)

**iNES Mapper 122** denotes the **JY043** circuit board, a simpler bootleg variant of Kaiser's [KS-7058](MMC1.xhtml "MMC1") circuit board. 

# Overview

  * CPU $8000-$FFFF: 32 KiB unbanked PRG-ROM
  * PPU $0000-$0FFF: 4 KiB switchable window into 32 KiB CHR-ROM
  * PPU $1000-$1FFF: 4 KiB switchable window into 32 KiB CHR-ROM
  * Nametable arrangement: Selected via solder pad



# Registers

There are two writable registers in the CPU $8000-$FFFF address range. 

**Register 1: Responds to even addresses from CPU $8000-$FFFE**
    
    
    D~[.... .CBA] A~[1... .... .... ...0]
             +++- 4 KiB CHR-ROM bank (CHR A14..A12)
                  for PPU $0000-$0FFF
    

**Register 2: Responds to odd addresses from CPU $8001-$FFFF**
    
    
    D~[.... .CBA] A~[1... .... .... ...1]
             +++- 4 KiB CHR-ROM bank (CHR A14..A12)
                  for PPU $1000-$1FFF
    

# Notes

  * iNES Mapper 122 had at one time been used by the DOS-based fwNES emulator to denote Sunsoft-1, now assigned to [Mapper 184](INES_Mapper_184.xhtml "INES Mapper 184").
  * JY043 had previously been assigned together with KS-7058 and the BBK Keyboard Famiclone's mapper as [INES Mapper 171](INES_Mapper_171.xhtml "INES Mapper 171").



Categories: [INES Mappers](Category_INES_Mappers.xhtml)
