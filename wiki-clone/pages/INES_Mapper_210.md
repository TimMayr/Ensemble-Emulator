# INES Mapper 210

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_210) | View [other pages](Special_AllPages.xhtml#INES_Mapper_210)

**Namco 175**

**Company** | Namco   
---|---  
**Games** | [4 in NesCartDB](https://nescartdb.com/search/advanced?ines=210)  
**Complexity** | ASIC   
**Boards** | 110F0B   
**PRG ROM capacity** | 512K   
**PRG ROM window** | 8Kx3 + 8K fixed   
**PRG RAM capacity** | 8K   
**PRG RAM window** | 8K   
**CHR capacity** | 256K   
**CHR window** | 1Kx8   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H or V fixed   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 210  
  
**Namco 340**

**Company** | Namco   
---|---  
**Games** | [8 in NesCartDB](https://nescartdb.com/search/advanced?ines=210)  
**Complexity** | ASIC   
**Boards** | CS000x   
**PRG ROM capacity** | 512K   
**PRG ROM window** | 8Kx3 + 8K fixed   
**PRG RAM capacity** | none   
**PRG RAM window** | none   
**CHR capacity** | 256K   
**CHR window** | 1Kx8   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H, V, or 1 switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 210  
  
iNES Mapper 210 is used to denote Famicom boards bearing either the Namco 175 (Submapper 1) or Namco 340 (Submapper 2) ASIC. Both can be thought of as cost-reduced versions of the [Namco 163](INES_Mapper_019.xhtml "INES Mapper 019"): 

Function | N163 | N175 | N340   
---|---|---|---  
IRQ counter | Yes | No | No   
ROM nametables | Yes | No | No   
Optional WRAM | Yes | Yes | No   
Expansion audio | Yes | No | No   
ASIC-internal RAM | Yes | No | No   
Mirroring | Selectable (extended) | Hard-wired H/V | Selectable H/V/0   
  
## Contents

  * 1 Game list
    * 1.1 Namco 175 (Submapper 1)
    * 1.2 Namco 340 (Submapper 2)
  * 2 Banks
  * 3 Registers
    * 3.1 CHR Select ($8000-$BFFF) w
    * 3.2 External PRG RAM enable ($C000-$C7FF) w (Namco 175 only)
    * 3.3 PRG Select 1 ($E000-$E7FF) w
    * 3.4 PRG Select 2 ($E800-$EFFF) w
    * 3.5 PRG Select 3 ($F000-$F7FF) w
  * 4 Notes
  * 5 See also



## Game list

### Namco 175 (Submapper 1)

Name | Battery | WRAM | Mirroring   
---|---|---|---  
Chibi Maruko-chan: Uki Uki Shopping | No | No | V   
Dream Master | No | No | H   
Family Circuit '91 | Yes | 2 KiB | V   
Famista '91 | No | No | V   
Heisei Tensai Bakabon | No | No | V   
  
### Namco 340 (Submapper 2)

  * Famista '92
  * Famista '93
  * Famista '94
  * Splatterhouse: Wanpaku Graffiti
  * Top Striker
  * Wagyan Land 2
  * Wagyan Land 3



## Banks

  * CPU $6000-$7FFF: 2 KB PRG RAM bank, mirrored four times (Namco 175 only)
  * CPU $8000-$9FFF: 8 KB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KB switchable PRG ROM bank
  * CPU $C000-$DFFF: 8 KB switchable PRG ROM bank
  * CPU $E000-$FFFF: 8 KB PRG ROM bank, fixed to the last bank
  * PPU $0000-$03FF: 1 KB switchable CHR bank
  * PPU $0400-$07FF: 1 KB switchable CHR bank
  * PPU $0800-$0BFF: 1 KB switchable CHR bank
  * PPU $0C00-$0FFF: 1 KB switchable CHR bank
  * PPU $1000-$13FF: 1 KB switchable CHR bank
  * PPU $1400-$17FF: 1 KB switchable CHR bank
  * PPU $1800-$1BFF: 1 KB switchable CHR bank
  * PPU $1C00-$1FFF: 1 KB switchable CHR bank



## Registers

The 175 and 340 have 12 registers within $8000-$FFFF. Each register occupies a range of $800 bytes, so $8000-$87FF all refers to one register, $8800-$8FFF all refers to another register, and so on. 

### CHR Select ($8000-$BFFF) w

Value CPU writes | Behavior   
---|---  
$00-$FF | Selects 1KB page of CHR-ROM   
Write to CPU address | 1KB CHR bank affected   
---|---  
$8000-$87FF | $0000-$03FF   
$8800-$8FFF | $0400-$07FF   
$9000-$97FF | $0800-$0BFF   
$9800-$9FFF | $0C00-$0FFF   
$A000-$A7FF | $1000-$13FF   
$A800-$AFFF | $1400-$17FF   
$B000-$B7FF | $1800-$1BFF   
$B800-$BFFF | $1C00-$1FFF   
  
### External PRG RAM enable ($C000-$C7FF) w (Namco 175 only)
    
    
    7  bit  0
    ---- ----
    .... ...E
            |
            +- 1: Enable PRG RAM
    

### PRG Select 1 ($E000-$E7FF) w
    
    
    7  bit  0
    ---- ----
    MMPP PPPP
    |||| ||||
    ||++-++++- Select 8KB page of PRG-ROM at $8000
    ++-------- Namco 340 only: Select mirroring
                 0: One-screen A
                 1: Vertical
                 2: One-screen B
                 3: Horizontal
    

### PRG Select 2 ($E800-$EFFF) w
    
    
    7  bit  0
    ---- ----
    ..PP PPPP
      || ||||
      ++-++++- Select 8KB page of PRG-ROM at $A000
    

### PRG Select 3 ($F000-$F7FF) w
    
    
    7  bit  0
    ---- ----
    ..PP PPPP
      || ||||
      ++-++++- Select 8KB page of PRG-ROM at $C000
    

## Notes

  * _Family Circuit '91_ relies on its 2 KiB of WRAM being correctly mirrored throughout the $6000-$7FFF address range.
  * Many INES Mapper 210 games are incorrectly set to [INES Mapper 019](INES_Mapper_019.xhtml "INES Mapper 019").
  * In the absence of a NES 2.0 header with submapper 1 or 2 to distinguish between Namco 175 and 340, the following heuristics have been suggested: 
    * If $E000.6 or $E000.7 are ever set, this should be a Namcot 340.
    * If the game ever reads from or writes to PRG RAM, this is a Namco 175 with external RAM.
    * If the game ever writes to $C000, this is probably a Namco 175.



## See also

  * [Naruko's blog's post on mirroring on the 340](http://d.hatena.ne.jp/na6ko/20110501/1304181038)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml)
