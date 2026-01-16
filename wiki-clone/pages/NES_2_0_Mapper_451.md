# NES 2.0 Mapper 451

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_451) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_451)

**NES 2.0 Mapper 451** is used for the homebrew game _Haratyler HP/MP_. It is basically a homebrew TLROM-like circuit board that implements the MMC3 register's in an unusual fashion, and saves the high score to flash ROM. The game executes the AMIC A29040B flash ROM chip's "Software ID" command; if the manufacturer and model ID do not match the expected values ($37 and $86), or it detects that WRAM exists, the game replaces the first level with an unwinnable boss fight. _Haratyler MP_ additionally provides MP3 playback from a built-in microSD card, similar to [GTMP3](GTROM.xhtml#GTMP3 "GTROM"). 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 MP3 Playback Controller ($6000-$7FFF)
    * 2.2 Mirroring Register ($A000-$BFFF)
    * 2.3 IRQ Register ($C000-$DFFF)
    * 2.4 Bank Register ($E000-$FFFF)



# Banks

  * CPU $8000-$9FFF: 8 KiB fixed PRG ROM bank $00
  * CPU $A000-$BFFF: 8 KiB switchable PRG ROM bank $10+x
  * CPU $C000-$DFFF: 8 KiB switchable PRG ROM bank $20+x
  * CPU $E000-$FFFF: 8 KiB fixed PRG ROM bank $30
  * PPU $0000-$1FFF: 8 KiB switchable CHR RAM bank from 16 KiB total



# Registers

## MP3 Playback Controller ($6000-$7FFF)

## Mirroring Register ($A000-$BFFF)
    
    
    A~[101. .... .... ...M]
                         +- 0: Vertical, 1: Horizontal
    

## IRQ Register ($C000-$DFFF)
    
    
    A~[110. .... VVVV VVVV]
    

A write to this register amounts to the following writes on a normal MMC3: 

  * $C000=VVVVVVVV -1
  * $C001=0
  * If V=$FF: $E000, otherwise $E001



## Bank Register ($E000-$FFFF)
    
    
    A~[111. .... .... ..BA]
                        |+- PRG/CHR A13
                        +-- PRG A16
    

This means effectively: 
    
    
    BA    CPU $A000   CPU $C000   PPU $0000
     0        $10         $20        0
     1        $11         $21        1
     2        $18         $28        0
     3        $19         $29        1
    

Together with the fixed banks, this means that only PRG ROM banks $00/$10/$11/$18/$19/$20/$21/$28/$29/$30 are accessible. The rest serve as padding for the flash ROM chip's 64 KiB sector size. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with flash save](Category_Mappers_with_flash_save.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
