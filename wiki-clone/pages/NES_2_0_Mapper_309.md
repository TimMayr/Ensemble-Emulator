# NES 2.0 Mapper 309

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_309) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_309)

NES 2.0 Mapper 309 is used for Whirlwind Manu's ROM cartridge conversion of the Japanese Famicom Disk System game _愛戦士ニコル_ (_Ai Senshi Nicol_ , cartridge code LH51). Its UNIF board name is **UNL-LH51**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Bank ($8000-9FFF)
    * 2.2 Mirroring ($E000-9FFF)
  * 3 Notes



# Banks

  * CPU $6000-$7FFF: 8 KiB unbanked PRG-RAM
  * CPU $8000-$9FFF: 8 KiB switchable PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB fixed PRG-ROM bank #13
  * CPU $C000-$DFFF: 8 KiB fixed PRG-ROM bank #14
  * CPU $E000-$FFFF: 8 KiB fixed PRG-ROM bank #15
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Registers

## PRG-ROM Bank ($8000-9FFF)
    
    
    Mask: probably $E000
    
    D~7654 3210
      ---------
      .... PPPP
           ++++- Select 8 KiB PRG-ROM bank number at CPU $8000-$9FFF
    

## Mirroring ($E000-9FFF)
    
    
    Mask: probably $E000
    
    D~7654 3210
      ---------
      .... M...
           +---- Select nametable mirroring type
                  0: Vertical
                  1: Horizontal
    

  


# Notes

  * This particular conversion of the game has broken sound [even on real hardware](https://www.youtube.com/watch?v=Rgt9cWk0yh0). Other conversions of the game that run under [INES Mapper 042](INES_Mapper_042.xhtml "INES Mapper 042") do not suffer from this problem.
  * The conversion retains all writes to the FDS sound channel registers, so an emulator could provide the expansion sound channel even though the original Whirlwind Manu cartridge did not. Given that the sound routine itself is broken however, doing such a thing may not be worthwhile.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml)
