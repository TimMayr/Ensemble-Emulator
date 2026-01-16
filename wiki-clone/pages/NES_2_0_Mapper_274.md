# NES 2.0 Mapper 274

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_274) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_274)

**NES 2.0 Mapper 274** denotes the _Cartridge Story_ series of multicarts from RCM Group. Its UNIF board name is **BMC-80013-B**. 

  * _Cartridge Story_ (72-in-1)
  * _Cartridge Story II_ (80-in-1)
  * _Cartridge Story III_ (90-in-1, a.k.a. _90-in-1 Hwang Shinwei_)



_Cartridge Story_ only contains a single main 2 MiB PRG-ROM chip; _II_ and _III_ both add an extra PRG-ROM chip. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Inner Bank and Mirroring ($8000-$9FFF, write)
    * 2.2 Outer Bank and Chip Select ($A000-$FFFF, write)



# Banks

  * CPU $8000-$BFFF: Switchable inner and outer PRG-ROM bank, main or extra PRG-ROM chip
  * CPU $C000-$FFFF: Fixed inner and switchable outer PRG-ROM, main PRG-ROM chip only
  * PPU $0000-$1FFF: 8 KiB of CHR-RAM



# Registers

## Inner Bank and Mirroring ($8000-$9FFF, write)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      ...M BBBB
         | ++++- Select PRG A17..A14 if CPU A14=0
         +------ Select nametable mirroring
                 0: Vertical
                 1: Horizontal
    

## Outer Bank and Chip Select ($A000-$FFFF, write)
    
    
    Mask: $E000
    
    D~7654 3210  A~FEDC BA98 7654 3210
      ---------    -------------------
      .OOO oooo    1C1. .... .... ....
       ||| ||||     +------------------ Select PRG-ROM Chip if CPU A14=0
       ||| ||||                          0: Extra (default)
       ||| ||||                          1: Main
       ||| ++++- Select PRG A17..A14 if CPU A14=1
       +++------ Select PRG A20..A18
    

Notes: 

  * Since the extra PRG-ROM chip only has 16 KiB (Cartridge Story II)/64 KiB (Cartridge Story III), PRG A16-A20 are ineffective when C=1. Because this yields a PRG-ROM section of the NES 2.0 file that is not a power of two, emulators must mask them off explicitly.
  * The main PRG-ROM chip is always selected when CPU A14=1.
  * Because C is set to 0 upon reset, the cartridge will boot with the extra PRG-ROM chip mapped, if present.



Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
