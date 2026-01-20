# INES Mapper 121

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_121) | View [other pages](Special_AllPages.xhtml#INES_Mapper_121)

iNES Mapper 121 is used for the 卡聖 (Kǎshèng) A9711 and A9713 [MMC3](MMC3.xhtml "MMC3")-clone-bearing protected boards. 

A9711 (256 KiB PRG-ROM): 

  * _MK6_ (title screen hack of [_幽☆遊☆白書 Final_](INES_Mapper_115.xhtml "INES Mapper 115"))
  * _Sonic & Knuckles 5_ (hack of _Somari_ , originally released by 哥德 (Gouder) on [INES Mapper 116](INES_Mapper_116.xhtml "INES Mapper 116"))
  * _Sonic 3D Blast 6_ (hack of _Somari_ , originally released by 哥德 (Gouder) on [INES Mapper 116](INES_Mapper_116.xhtml "INES Mapper 116"))
  * _Street Fighter Zero 2 '97_ (not [_Street Fighter Zero 2_](INES_Mapper_187.xhtml "INES Mapper 187"))
  * _Super Real Bout 97_ (hack of _餓狼伝説 Special_ , originally released by 哥德 (Gouder) on INES Mapper 116)
  * _The Panda Prince/Super Lion King 2/熊貓太子_ (_Xióngmāo Tàizǐ_)
  * _Ultimate Mortal Kombat 3_ (hack of [_Mortal Kombat 3_](INES_Mapper_123.xhtml "INES Mapper 123"))
  * _Ultimate Mortal Kombat 4_ (hack of [_Mortal Kombat 3_](INES_Mapper_123.xhtml "INES Mapper 123"))
  * _幽☆遊☆白書 97 V_ (_Yū Yū Hakusho 97 V_ , title screen hack of [_幽☆遊☆白書 Final_](INES_Mapper_115.xhtml "INES Mapper 115"))



Only _Street Fighter Zero 2 '97_ has 512 KiB of CHR-ROM; CHR A18 (the second half of CHR-ROM) is directly selected by PPU A12, regardless of what the MMC3 is set to do. This means that the "left" pattern table always uses the first and the "right" pattern table always uses the second 256 KiB half of CHR-ROM. 

A9713 (512 KiB PRG-ROM): 

  * _3-in-1: Earthworm Jim 2+4, The Lion King_



## Contents

  * 1 Registers
    * 1.1 Protection Array Read ($5000-$5FFF)
    * 1.2 Protection Array Index ($5000-$5FFF)
    * 1.3 Outer Bank Register ($5180, A9713 only)
    * 1.4 CHR A18 Control ($8000-$9FFF, even; A9711 only)
    * 1.5 MMC3-compatible registers ($8000-$FFFF)
    * 1.6 Protection Latch ($8001)
    * 1.7 Protection Index ($8003)
  * 2 Errata
  * 3 Similar Mappers



# Registers

## Protection Array Read ($5000-$5FFF)

Mask: probably $F000 

Returns the value from the four-byte protection array ($83, $83, $42, $00) indexed by the previous write to $5000-$5FFF. 

## Protection Array Index ($5000-$5FFF)
    
    
    Mask: probably $F000
    
    D~7654 3210
      ---------
      .... ..II
             ++- Select index into protection array on next $5000-$5FFF read
    

[FCEUX' source code](https://github.com/asfdfdfd/fceux/blob/master/src/boards/121.cpp) claims that address bit 8 switches between two protection arrays, the second one being $00, $02, $02, $03. 

## Outer Bank Register ($5180, A9713 only)
    
    
    Mask: $F180
    
    D~7654 3210
      ---------
      M... ....
      +--------- Select outer 256 KiB PRG-ROM and CHR-ROM bank
    

## CHR A18 Control ($8000-$9FFF, even; A9711 only)
    
    
    Mask: $E001
    
    D~7654 3210
      ---------
      M... ....
      +--------- Select CHR A18 Mode
                  0: CHR A18=inverted PPU A12
                  1: CHR A18=PPU A12
    

  


## MMC3-compatible registers ($8000-$FFFF)

Mask: $E001, see [MMC3](MMC3.xhtml "MMC3"). 

## Protection Latch ($8001)

Mask: $E003 

If the previous write to the Protection Index register ($8003) selected indices $26, $28 or $2A, the value written to $8001 --- with the lower six bits reversed -- is immediately applied. Otherwise, it will only be applied upon the next write to the $8003. Regardless of the $8003 setting, the MMC3 clone will always receive the write as well. 

## Protection Index ($8003)

Mask: $E003 

Bits 0-5 select a protection index register. If a valid index has been selected, the value previously written to $8001 --- with the lower six bits reversed --- is applied. If an invalid index has been selected, the $A000/$C000/$E000 banks selected via $8003 expire, and normal MMC3 PRG-ROM banking is restored. 

  * Index $26: Select 8 KiB PRG-ROM bank at CPU $E000-$FFFF using current bit-reversed $8001 value, and apply subsequent writes to $8001 to this index as well.
  * Index $28: Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF using current bit-reversed $8001 value, and apply subsequent writes to $8001 to this index as well.
  * Index $2A: Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF using current bit-reversed $8001 value, and apply subsequent writes to $8001 to this index as well.
  * Index $2C: Select 8 KiB PRG-ROM bank at CPU $E000-$FFFF using current bit-reversed $8001 value _if it is not zero_ , but do not apply subsequent writes to $8001 to this index.
  * Index $2F: Do nothing, but keep the $A000/$C000/$E000 banks previously selected via $8003.
  * Indices $20/$29/$2B/$3C/$3F: Select 8 KiB PRG-ROM bank at CPU $E000-$FFFF using current bit-reversed $8001 value, but do not apply subsequent writes to $8001 to this index.
  * All other indices: Restore normal MMC3 PRG-ROM banking.



Writing to $8003 also has the side effect of writing the same value to the MMC clone's $8000 register. 

# Errata

  * GoodNES 3.23b has several INES Mapper 121 images erroneously set to 187.



# Similar Mappers

  * iNES Mapper [187](INES_Mapper_187.xhtml "INES Mapper 187") has the same CHR A18 behavior as the A9711 board, adds an NROM override register, and drops the complex protection.



Categories: [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
