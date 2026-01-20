# INES Mapper 125

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_125) | View [other pages](Special_AllPages.xhtml#INES_Mapper_125)

iNES Mapper 125 is used for Whirlwind Manu's ROM cartridge conversion of the Japanese Famicom Disk System game _モンティのドキドキ大脱走_ (_Monty no Doki Doki Daisassō_ , Monty on the Run, cartridge code LH32). Its UNIF board name is **UNL-LH32**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Bank ($6000-$7FFF)
  * 3 Notes



# Banks

  * CPU $6000-$7FFF: 8 KiB switchable PRG-ROM bank
  * CPU $8000-$9FFF: 8 KiB fixed PRG-ROM bank #12
  * CPU $A000-$BFFF: 8 KiB fixed PRG-ROM bank #13
  * CPU $C000-$DFFF: 8 KiB unbanked PRG-RAM bank
  * CPU $E000-$FFFF: 8 KiB fixed PRG-ROM bank #15
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Registers

## PRG-ROM Bank ($6000-$7FFF)
    
    
    Mask: probably $E000
    
    D~7654 3210
      ---------
      .... PPPP
           ++++- Select 8 KiB PRG-ROM bank number at CPU $6000-$7FFF
    

# Notes

  * Mirroring is hard-wired (to Vertical).
  * The conversion retains all writes to the FDS sound channel registers, so an emulator could provide the expansion sound channel even though the original Whirlwind Manu cartridge did not.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml)
