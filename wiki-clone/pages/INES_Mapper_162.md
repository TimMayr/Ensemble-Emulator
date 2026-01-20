# INES Mapper 162

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_162) | View [other pages](Special_AllPages.xhtml#INES_Mapper_162)

**iNES Mapper 162** denotes the 外星 (Wàixīng) **FS304** circuit board, used for the following games: 

  * _Mummy - 神鬼传奇_
  * _Zelda 传说: 三神之力_ (ES-1096)
  * _法老王 - Pharaoh_ (ES-1114)
  * _火焰纹章 - 圣战的系谱_ and its title screen hack _聖火徽章 III_ (Shènghuǒ Huīzhāng III/Fire Emblem III)
  * _西游记后传_ (ES-1097)



A compatible circuit board with an unknown PCB code is used for a few games from 南晶 (Nánjīng): 

  * _梁山英雄_ (Liángshān Yīngxióng, NJ023, title screen hack of 水浒神兽, NJ019)
  * _时空斗士 - Pegasus Senya_ (Shíkōng Dòushì, NJ024, title screen hack of 圣斗士星矢: 天马之幻想, NJ022)
  * _农场小精灵_ (Nóngchǎng Xiǎojīnglíng, NJ025, title screen hack of 牧场物语 - Harvest Moon, NJ011)



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Bank Low/CHR-RAM Switch ($5000, write)
    * 2.2 Alternative PRG-ROM Bank low ($5100, write)
    * 2.3 PRG-ROM Bank High ($5200, write)
    * 2.4 Mode ($5300, write)
  * 3 Notes
  * 4 See also



# Banks

  * CPU $6000-$7FFF: 8 KiB unbanked PRG-RAM, battery-backed
  * CPU $8000-$FFFF: 32 KiB switchable PRG-ROM bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM bank, 4 KiB can be automatically switched
  * Nametable mirroring: hard-wired



# Registers

All registers are initialized to $00 on reset. 

## PRG-ROM Bank Low/CHR-RAM Switch ($5000, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      C... PPpq
      |    |||+- PRG A15 if $5300.0=1 and $5300.2=1
      |    ||+-- PRG A16 if $5300.2=1
      |    ++--- PRG A18..A17
      +--------- Automatic 4 KiB CHR-RAM switch: when PPU A13=0 (pattern table) ...
                  0: CHR A12=PPU A12 (disable auto-switch)
                  1: CHR A12=PPU A9 latched on last rise of PPU A13 (enable auto-switch)
    

Automatic 4 KiB CHR-RAM switch means that the left pattern table is used for the the top half of any nametable, and the right pattern table for the bottom half of any nametable, regardless of the scroll position. This auto-switch behavior is similar to that of [mapper 96](INES_Mapper_096.xhtml "INES Mapper 096"). 

## Alternative PRG-ROM Bank low ($5100, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... ..p.
             +-- PRG A15 if $5300.0=0
    

## PRG-ROM Bank High ($5200, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... ..PP
             ++- PRG A20..A19
    

## Mode ($5300, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... .A.B
            | +- PRG A15 mode:
            |     0: PRG A15=$5100.1
            |     1: PRG A15=1       if A=0
            |        PRG A15=$5000.0 if A=1
            +--- PRG A16 mode:
                  0: PRG A16=1
                  1: PRG A16=$5000.1
    

Or in terms of where PRG A15-16 come from: 
    
    
    $5300.2/0   PRG A16   PRG A15
    -----------------------------
          0.0      1      $5100.1
          0.1      1         1
          1.0   $5000.1   $5100.1
          1.1   $5000.1   $5000.0
    

Because reset clears all registers' bits, games will boot in 32 KiB PRG-ROM bank #2 (A16=1 and A15=$5100.1=0). 

# Notes

  * To ensure that a cartridge to be dumped is really INES Mapper 162, write $04 to $5300 and verify that $5100.1 changes the PRG bank.
  * The Waixing games run with $5300=$04, using $5100.1 to change PRG A16. The three Nanjing games run with $5300=$07, using $5000.0 to change PRG A16, except for a protection check shortly after the title screen where the described behavior of $5300=$04 is verified.



# See also

Similar mappers: [INES Mapper 163](INES_Mapper_163.xhtml "INES Mapper 163"), [INES Mapper 164](INES_Mapper_164.xhtml "INES Mapper 164"), [NES 2.0 Mapper 558](NES_2_0_Mapper_558.xhtml "NES 2.0 Mapper 558")

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
