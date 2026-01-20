# INES Mapper 115

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_115) | View [other pages](Special_AllPages.xhtml#INES_Mapper_115)

**iNES Mapper 115** denotes the 卡聖 (Kǎshèng) SFC-02B/-03/-004 boards, bearing an [MMC3](MMC3.xhtml "MMC3")-clone with a 256 KiB outer CHR-ROM bank register and an NROM-like PRG-ROM register that can override the MMC3 clone's PRG-ROM bank. Used by: 

  * _A.V. 究極麻將 II_ (_A.V. Jiūjí Májiàng II_ , Kǎshèng re-release, originally released by 哥德 (Gouder) on [INES Mapper 116](INES_Mapper_116.xhtml "INES Mapper 116"))
  * _包青天_ (_Bāo Qīngtiān_ , Justice Pao)
  * _餓狼伝説 Special_ (_Garō Densetsu Special_ , Kǎshèng re-release, originally released by 哥德 (Gouder) on [INES Mapper 116](INES_Mapper_116.xhtml "INES Mapper 116"))
  * _雷電 II - Thunderbolt Fighting Plane_ (_Léidiàn II_)
  * _三國春秋: 四川省_ (_Sānguó Chūnqiū: Sìchuān Shěng_)
  * _幽☆遊☆白書 Final_ (_Yū Yū Hakusho Final_ , both Chinese and English versions)
  * _1997 Super HiK 16-in-1_ multicart



## Contents

  * 1 Registers
    * 1.1 Mode/NROM Bank Register ($6000, write)
    * 1.2 Outer CHR-ROM Bank Register ($6001, write)
    * 1.3 Solder Pad Register ($6002, read)
    * 1.4 MMC3-compatible registers ($8000-$FFFF)
  * 2 Notes
  * 3 Similar Mappers



# Registers

## Mode/NROM Bank Register ($6000, write)
    
    
    Mask: $E003
    
    D~[MPN. PPPp]
       |||  ++++- If M=1: PRG A17..A14
       ||+------- 0: If M=1: PRG A14=p (NROM-128)
       ||         1: If M=1: PRG A14=CPU 14 (NROM-256)
       |+-------- PRG A18
       +--------- 0: MMC3 mode, PRG A17..A14=MMC3 PRG A17..A14
                  1: NROM mode, PRG A17..A14=PPPp and "N" bit applies
    Power-on value: $00
    

## Outer CHR-ROM Bank Register ($6001, write)
    
    
    Mask: $E003
    
    D~[.... ...C]
               +- CHR A18
    Power-on value: $00
    

## Solder Pad Register ($6002, read)
    
    
    Mask: $E003
    
    D~[.... .VVV]
             +++- Solder Pad value
    

## MMC3-compatible registers ($8000-$FFFF)

Mask: $E001, see [MMC3](MMC3.xhtml "MMC3"). 

# Notes

  * Unlike many similar mappers, the $600x registers are not connected to the MMC3 clone's WRAM interface and thus function regardless of whether WRAM is enabled or not.
  * IRQ behavior resembles "Sharp" behavior, i.e. a latch value of zero causes an IRQ on every scanline, on which the Chinese version of 幽☆遊☆白書 Final (Yuu Yuu Hakusho Final) depends.



# Similar Mappers

  * [INES Mapper 114](INES_Mapper_114.xhtml "INES Mapper 114") has the same $6000 and $6001 registers, but additionally scrambles MMC3 register addresses and indices. Also, Mapper 114's IRQ behavior resembles "NEC" behavior, i.e. a latch value of zero disables IRQ.
  * [INES Mapper 187](INES_Mapper_187.xhtml "INES Mapper 187") has a similar NROM override register, placed at $5000
  * [INES Mapper 248](INES_Mapper_248.xhtml "INES Mapper 248") is a duplicate of Mapper 115.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
