# NES 2.0 Mapper 515

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_515) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_515)

iNES 2.0 Mapper 515 is used for 패밀리 노래방 _Family Noraebang_ , a Korean karaoke cartridge that uses UNROM-style PRG banking, has provisions for an expansion cartridge, an ADC for Microphone input, and a clone of the Yamaha [YM2413](https://en.wikipedia.org/wiki/Yamaha_YM2413) chip for expansion audio: the K-663A. 

## Contents

  * 1 Banks
  * 2 PRG-ROM bankswitching
  * 3 Expansion sound
  * 4 Microphone input
  * 5 See also



## Banks

  * CPU $8000-$BFFF: 16 KB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KB PRG ROM bank, fixed to the last bank



## PRG-ROM bankswitching
    
    
    Mask $E00F:
      W 8000: [RBBB BBBB] - R:0-select external ROM; 1-select internal ROM; B-select bank at $8000-$BFFF
                            $C000-$FFFF is always last bank of internal ROM
    

The board does not seem to suffer from bus conflicts. The existing ROM images are arranged similarly to [INES Mapper 188](INES_Mapper_188.xhtml "INES Mapper 188"): The last 1 MiB of PRG-ROM are the main cartridge, the first (total PRG-ROM size minus 1 MiB) are the expansion cartridge. 

## Expansion sound
    
    
    Mask: $E003:
      W 6000: YM2413 index
      W 6001: YM2413 data
      R 6000: YM2413 read back test register contents
    

Note that the board does not allow the 2A03 audio to be mixed with the YM2413 output. 

## Microphone input
    
    
    Mask: $E003:
       R 6003: [D... ....] - SPI (ADC) data
    Mask not yet known:
       W C002: [S... ....] - SPI (ADC) chip select
       W C003: [K... ....] - SPI clock / ADC conversion clock
    

## See also

[http://forums.nesdev.org/viewtopic.php?f=9&t=16124](http://forums.nesdev.org/viewtopic.php?f=9&t=16124)
