# NES 2.0 Mapper 286

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_286) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_286)

NES 2.0 Mapper 286 is used for Benshieng multicarts. Its UNIF mapper is **BMC-BS-5**. The hardware latches the address rather than the data bus bits. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 CHR-ROM Select ($8000-$9FFF)
    * 2.2 PRG-ROM Select ($A000-$BFFF)
  * 3 Notes



# Banks

  * CPU $8000-$9FFF: 8 KiB switchable PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG-ROM bank
  * CPU $C000-$DFFF: 8 KiB switchable PRG-ROM bank
  * CPU $E000-$FFFF: 8 KiB switchable PRG-ROM bank
  * PPU $0000-$07FF: 2 KiB switchable CHR-ROM bank
  * PPU $0800-$0FFF: 2 KiB switchable CHR-ROM bank
  * PPU $1000-$17FF: 2 KiB switchable CHR-ROM bank
  * PPU $1800-$1FFF: 2 KiB switchable CHR-ROM bank



# Registers

## CHR-ROM Select ($8000-$9FFF)
    
    
    FEDC BA98 7654 3210
    -------------------
    100. RR.. .... BBBB
         ||        ++++- Select 8 KiB CHR-ROM bank number
         ||              in range selected by address bits 8-9
         ++------------- Select 8 KiB CHR-ROM bank
                         00: PPU $0000-$07FF
                         01: PPU $0800-$0FFF
                         10: PPU $1000-$17FF
                         11: PPU $1800-$1FFF
    

## PRG-ROM Select ($A000-$BFFF)

The AND mask to which the PRG-ROM select responds is determined in bits 4-7 by a DIP switch. 
    
    
    FEDC BA98 7654 3210
    -------------------
    101. RR.. DDDD BBBB
         ||   |||| ++++- Select 8 KiB PRG-ROM bank number
         ||   ||||       in range selected by address bits 8-9
         ||   ++++------ AND mask from DIP switch
         ||              01: Setting #1
         ||              02: Setting #2
         ||              04: Setting #3
         ||              08: Setting #4
         ++------------- Select 8 KiB PRG-ROM bank
                         00: CPU $8000-$9FFF
                         01: CPU $A000-$BFFF
                         10: CPU $C000-$DFFF
                         11: CPU $E000-$FFFF
    

# Notes

  * The initial PRG bank values are 0xF.
  * Mirroring is fixed to vertical.
  * The multicart menu code has a programming error: when pressing Up from the first position to move the selection to the last game, subsequent game selections launch the wrong game.



Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
