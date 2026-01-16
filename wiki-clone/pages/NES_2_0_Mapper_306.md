# NES 2.0 Mapper 306

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_306) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_306)

NES 2.0 Mapper 306 is used for Kaiser's ROM cartridge conversion of the Japanese Famicom Disk System game _Exciting Basket_ (_Double Dribble_ in North America and Europe). Its UNIF board name is **UNL-KS7016**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Bank ($D903-$D943)
  * 3 Notes



# Banks

  * CPU $6000-$7FFF: 8 KiB switchable PRG-ROM bank
  * CPU $8000-$FFFF: 32 KiB fixed PRG-ROM bank #3
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Registers

## PRG-ROM Bank ($D903-$D943)
    
    
    Mask: $D903 (according to FCEUX)
    
    A~FEDC BA98 7654 3210
      -------------------
      11.1 1..1 .Mpp PP11
                 ||| ++-- Bits 0-1 of 8 KiB PRG-ROM bank number at CPU $6000-$7FFF
                 |++----- Bits 2-3 of 8 KiB PRG-ROM bank number at CPU $6000-$7FFF if M=1
                 +------- 0=Ignore pp, switchable bank number is 10PP
                          1=Obey pp, switchable bank number is ppPP
    

# Notes

  * Mirroring is hard-wired (to Vertical).
  * Switchable bank numbers $0C-$0F are forced to be mirrors of $0B.
  * The conversion retains all writes to the FDS sound channel registers, so an emulator could provide the expansion sound channel even though the original Kaiser cartridge did not.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml)
