# INES Mapper 031

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_031) | View [other pages](Special_AllPages.xhtml#INES_Mapper_031)

**NSF subset**

**Company** | varies (homebrew)   
---|---  
**Complexity** | CPLD   
**Boards** | unknown (homebrew)   
**PRG ROM capacity** | 1024K   
**PRG ROM window** | 4K   
**PRG RAM capacity** | None   
**CHR capacity** | 8K   
**CHR window** | n/a   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Not applicable   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 031  
  
**iNES Mapper 031** represents a mapper created to facilitate cartridge compilations of [NSF](NSF.xhtml "NSF") music. It implements a common subset of the features used by NSFs. 

PRG-ROM is bankswitched in 8 x 4 kB banks from $8000-FFFF. These are controlled by registers at $5FF8-$5FFF like the NSF mapper. The high bank at $F000-FFFF is initialized to the last bank at power-on. 

There is no CHR banking, so it is recommended to use 8 kB CHR-RAM with this mapper. 

As with [BNROM](INES_Mapper_034.xhtml "BNROM") and [UxROM](UxROM.xhtml "UxROM"), there is no mirroring, CHR bank, or IRQ control; this mapper has hardwired H or V mirroring. 

Examples: 

  * _2A03 Puritans_ ([ROM](http://rainwarrior.ca/projects/nes/2a03puritans.html))
  * _Famicompo Pico_ ([ROMs](http://rainwarrior.ca/projects/nes/pico.html))
  * _RNDM_ ([info](http://megaranmusic.com/album/rndm))
  * _EZNSF_ ([forum post](https://forums.nesdev.org/viewtopic.php?t=15204))
  * _ZENSF_ ([forum post](https://forums.nesdev.org/viewtopic.php?f=6&t=17698))
  * _Test ROMs_ ([forum post](https://forums.nesdev.org/viewtopic.php?f=3&t=13120))



## Contents

  * 1 Registers
    * 1.1 PRG bank select $5000-$5FFF
  * 2 References
  * 3 Emulator support



## Registers

### PRG bank select $5000-$5FFF
    
    
    address              data
    15      bit       0  7  bit  0
    -------------------  ---------
    0101 .... .... .AAA  PPPP PPPP
                    |||  |||| ||||
                    |||  ++++-++++- Select 4 kB PRG ROM bank at slot specified by write address.
                    +++------------ Specify 4 kB bank slot at: $8000 + (AAA * $1000)
    

The canonical write position for these registers is $5FF8-$5FFF, as used in [NSFs](NSF.xhtml "NSF"). 

At power on, the register at $5FFF is set to $FF. Startup code should be placed in the last bank. There is no change to this register on reset. 

## References

  * [Excerpt from readme.txt describing the mapper](http://forums.nesdev.org/viewtopic.php?p=128137#p128137)



## Emulator support

  * [BizHawk](http://tasvideos.org/BizHawk.html) 1.11.2
  * [FCEUX](http://www.fceux.com) 2.2.3
  * [MAME](http://mamedev.org/) 0.162
  * [Mesen](https://www.mesen.ca/) 0.9.0
  * [Nintendulator](https://www.qmtpro.com/~nes/nintendulator/) 0.975
  * [OpenEmu](https://openemu.org/) (FCEUX core) 2.0.3
  * [puNES](http://forums.nesdev.org/viewtopic.php?t=6928) 0.84



Cartridges: 

  * [Everdrive N8](Everdrive_N8.xhtml "Everdrive N8") OS V14
  * [InfiniteNESLives](http://www.infiniteneslives.com/nessupplies.php) Mapper 31 board



Categories: [CPLD mappers](Category_CPLD_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml)
