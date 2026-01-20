# INES Mapper 113

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_113) | View [other pages](Special_AllPages.xhtml#INES_Mapper_113)

**Mapper 113**

**Company** | _unknown_  
---|---  
**Boards** | _unknown_  
**PRG ROM capacity** | 256K   
**PRG ROM window** | 32K   
**PRG RAM capacity** | None   
**CHR capacity** | 128K   
**CHR window** | 8k   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H or V, controlled by mapper   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 113  
  
iNES Mapper 113 denotes the HES **NTD-8** circuit board, used to create multicarts made for a bunch of [NINA-03/06](NINA_003_006.xhtml "NINA-003-006") games. 

Example Games: 

  * _HES 6-in-1_
  * _Mind Blower Pak_
  * _Total Funpak_



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Control Register
  * 3 Variants
  * 4 References



## Banks

  * CPU $8000-$FFFF: 32 KB switchable PRG ROM bank (max 256 KB)
  * PPU $0000-$1FFF: 8 KB switchable CHR ROM bank (max 128 KB)



## Registers

### Control Register

This register is located at addresses: 
    
    
    F        bit      0
    ---- ---- ---- ----
    010x xxx1 xxxx xxxx
    

That is, $4100-$41FF, $4300-$43FF, $45xx, $47xx, ..., $5Dxx, and $5Fxx. 
    
    
    7  bit  0
    ---- ----
    MCPP PCCC
    |||| ||||
    |+||-|+++-- Select 8 KB CHR ROM bank for PPU $0000-$1FFF
    | ++-+----- Select 32 KB PRG ROM bank for CPU $8000-$FFFF
    +---------- Mirroring control (0 = Horizontal, 1 = Vertical)
    

Note the high bit of the CHR select is shifted away from the others. 

## Variants

This mapper extends the [NINA-03/06](NINA_003_006.xhtml "NINA-003-006") with an extra 4 bits of control register. The low 4 bits written behave the same, but the high 4 bits add [mirroring](Mirroring.xhtml "Mirroring") control, 1 extra bit of CHR select, and 2 extra bits of PRG select. 

## References

  * [NES mapper list by Disch](http://www.romhacking.net/documents/362/)



Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
