# NES 2.0 Mapper 261

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_261) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_261)

**NES 2.0 Mapper 261** was assigned by FCEUX for the **810544-C-A1** and **NTDEC 2746** multicart circuit boards. Its UNIF board name is **BMC-810544-C-A1**. 

  * _200-in-1 - Elfland_
  * _14-in-1_ (NTDEC)



# Address latch
    
    
    A~1... ..PP PmpM CCCC
             || |||| ++++- Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF
             || |||+------ Select nametable mirroring type
             || |||         0: Vertical
             || |||         1: Horizontal
             || ||+------- Lowest bit of 16 KiB PRG-ROM bank number
             || ||         at CPU $8000-$BFFF/$C000-$FFF in NROM-128 mode
             || |+-------- PRG-ROM banking mode
             || |           0: NROM-128 (16 KiB PRG-ROM at $8000-$BFFF, mirrored)
             || |           1: NROM-256 (32 KiB PRG-ROM at $8000-$FFFF)
             ++-+--------- Select 32 KiB PRG-ROM bank in NROM-256 mode/
                           bits 1-3 of 16 KiB PRG-ROM bank in NROM-128 mode
    

The first game on that multicart, _Elfland_ , is actually a CNROM game that writes to the multicart address latch to switch CHR-ROM banks. 

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
