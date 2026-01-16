# INES Mapper 117

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_117) | View [other pages](Special_AllPages.xhtml#INES_Mapper_117)

iNES Mapper 117 is used for _Crayon Shin-Chan (Ch)_ and _San Guo Zhi 4 - Chi Bi Feng Yun_ by Future Media. 

## Banks

  * CPU $8000-$9FFF: 8 KB switchable PRG ROM bank.
  * CPU $A000-$BFFF: 8 KB switchable PRG ROM bank.
  * CPU $C000-$DFFF: 8 KB switchable PRG ROM bank.
  * CPU $E000-$FFFF: 8 KB PRG ROM bank, fixed to the last bank.


  * PPU $0000-$03FF: 1 KB switchable CHR ROM bank.
  * PPU $0400-$07FF: 1 KB switchable CHR ROM bank.
  * PPU $0800-$0BFF: 1 KB switchable CHR ROM bank.
  * PPU $0C00-$0FFF: 1 KB switchable CHR ROM bank.
  * PPU $1000-$13FF: 1 KB switchable CHR ROM bank.
  * PPU $1400-$17FF: 1 KB switchable CHR ROM bank.
  * PPU $1800-$1BFF: 1 KB switchable CHR ROM bank.
  * PPU $1C00-$1FFF: 1 KB switchable CHR ROM bank.



## Registers
    
    
    $8000/$8001/$8002
    7  bit  0
    ---- ----
    PPPP PPPP
    |||| ||||
    ++++-++++- swaps 8 KB PRG ROM bank at CPU $8000/$A000/$C000.
    
    
    
    $A000/$A001/$A002/$A003/$A004/$A005/$A006/$A007
    7  bit  0
    ---- ----
    CCCC CCCC
    |||| ||||
    ++++-++++- swaps 1 KB CHR ROM bank at PPU $400*(address AND 7).
    
    
    
    $A008/$A009/$A00A/$A00B/$A00C/$A00D/$A00E/$A00F
    * Unknown, writes should be ignored.
    
    
    
    $C000/$C001/$C002
    * Any write to these registers sets the IRQ counter AND latch to the value written.
    * IRQs are also acknowledged.
    
    
    
    $E000
    7  bit  0
    ---- ----
    xxxx xxxI
            |
            +-- IRQ enable/disable.
    

  


## IRQ operation

  * The IRQ counter value selects PPU scanline MINUS $14 (?) to trigger an IRQ.
  * If the IRQ counter matches the scanline number, and IRQs are enabled, and the latched value isn't zero, an IRQ is triggered. The counter is set to zero.


