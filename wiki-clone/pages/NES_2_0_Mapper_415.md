# NES 2.0 Mapper 415

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_415) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_415)

**NES 2.0 Mapper 415** denotes the **0353** circuit board for a cartridge conversion of the FDS game _Roger Rabbit_ , renamed to _Lucky Rabbit_. 

# Banks

  * CPU $6000-$7FFF: Switchable 8 KiB PRG-ROM bank
  * CPU $8000-$FFFF: Fixed last 32 KiB PRG-ROM bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Data latch
    
    
    Mask: $8000
    D~[...M PPPP]
          | ++++- PRG A16..A13 at CPU $6000-$7FFF
          +------ 0=Vertical, 1=Horizontal mirroring
    

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml)
