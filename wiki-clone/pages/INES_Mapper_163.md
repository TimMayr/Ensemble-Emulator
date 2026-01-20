# INES Mapper 163

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_163) | View [other pages](Special_AllPages.xhtml#INES_Mapper_163)

**iNES Mapper 163** denotes the 南晶 (Nánjīng) **FC-001** circuit board, used on most of their games, including: 

  * _牧场物语 - Harvest Moon_ (NJ011)
  * _水浒神兽_ (Shuǐhǔ Shénshòu, NJ019)
  * _暗黑破坏神 - Diablo_ (NJ037)
  * _轩辕剑外传 之 天之痕_ (Xuānyuánjiàn Wàizhuàn zhī Tiānzhīhén, NJ045)
  * _Final Fantasy IV - 最终幻想 4꞉ 光与暗 水晶纷争_ (NJ098)



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Bank Low/CHR-RAM Switch ($5000, write)
    * 2.2 PRG Bank High ($5200, write)
    * 2.3 Feedback Write ($5100-$5101, write)
    * 2.4 Feedback Read ($5500-$5501, read)
    * 2.5 Mode ($5300, write)
  * 3 Notes
  * 4 See also



# Banks

  * CPU $6000-$7FFF: 8 KiB unbanked PRG-RAM, battery-backed
  * CPU $8000-$FFFF: 32 KiB switchable PRG-ROM bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM bank, 4 KiB can be automatically switched
  * Nametable mirroring: hard-wired



# Registers

All registers are initialized to $00 on reset. 

## PRG Bank Low/CHR-RAM Switch ($5000, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      C... PPPP
      |    ++++- PRG A18..A15
      +--------- Automatic 4 KiB CHR-RAM switch: when PPU A13=0 (pattern table) ...
                  0: CHR A12=PPU A12 (disable auto-switch)
                  1: CHR A12=PPU A9 latched on last rise of PPU A13 (enable auto-switch)
    

  * Automatic 4 KiB CHR-RAM switch means that the left pattern table is used for the the top half of any nametable, and the right pattern table for the bottom half of any nametable, regardless of the scroll position. This auto-switch behavior is similar to that of [mapper 96](INES_Mapper_096.xhtml "INES Mapper 096").
  * Bits 0 and 1 are subject to the Mode register.



## PRG Bank High ($5200, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... ..PP
             ++- PRG A20..A19
    

  * Bits 0 and 1 are subject to the Mode register. 1 MiB games connect both ASIC PRG A19 and A20 outputs to ROM A19, effectively exempting this register from the bit-swap.



## Feedback Write ($5100-$5101, write)
    
    
    Mask: $FF01
    
    D~7654 3210  A~FEDC BA98 7654 3210
      ---------    -------------------
      .... .F.E    .... .... .... ...A
            | |                      +- Action on write
            | +- A=0: Value latched
            |    A=1: Flip the latched F bit when E=1
            +--- A=0: Value latched
                 A=1: Ignored, previously-latched value either kept or flipped
    

  * Bit 0 is subject to the Mode register.



## Feedback Read ($5500-$5501, read)
    
    
    Mask: $F300
    
    D~7654 3210
      ---------
      .... .F..
            +--- Inverted value of latched F bit
    

  * _暗黑破坏神 - Diablo_ (Ànhēi Pòhuàishén, NJ037) only checks that F is read back inverted.
  * _真 Samurai Spirits - 魂之利刃_ (NJ042) and _轩辕剑外传 之 天之痕_ (Xuānyuánjiàn Wàizhuàn zhī Tiān zhī Hén, NJ045) check that F is flipped after a $5101 write with E=1.



## Mode ($5300, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... .A?B
            | +- 0: Do not swap D0,D1 on writes to $5000-$5200.
            |    1: Swap D0,D1 on writes to $5000-$5200.
            +--- 0: PRG A15/A16=11b
                 1: PRG A15/A16 from $5000
    

  * This register is not subject to bit-swapping, i.e. the register is not affected by its own bit 0.
  * Because reset clears the A bit, games will boot in 32 KiB PRG-ROM bank #3.



# Notes

  * Mapper 163 cartridges should be dumped with $5300=$04 to obtain the correct PRG bank order. To verify that the cartridge is really mapper 163 ... 
    * ... rule out [INES Mapper 162](INES_Mapper_162.xhtml "INES Mapper 162") by writing $04 to $5300 and verifying that $5100.1 does not change the PRG bank;
    * ... rule out [NES 2.0 Mapper 558](NES_2_0_Mapper_558.xhtml "NES 2.0 Mapper 558") by verifying that $5000 value $01 does not result in a different PRG bank between $5300 values $04 and $05 (because the B bit is bit 1 in mapper 558).
  * _暗黑破坏神 - Diablo_ (NJ037) and its title screen hack _毁灭之神_ (NJ078) mistakenly disable automatic 4 KiB CHR-RAM switching mid-screen during the ending cutscene, causing flickering on real hardware. _魔兽世界: 恶魔猎人_ (NJ097) puts the 8x16 sprites into the wrong pattern table for automatic 4 KiB CHR-RAM switching, causing a glitched cursor on the title screen on real hardware.



# See also

  * Similar mappers: [INES Mapper 162](INES_Mapper_162.xhtml "INES Mapper 162"), [INES Mapper 164](INES_Mapper_164.xhtml "INES Mapper 164"), [NES 2.0 Mapper 558](NES_2_0_Mapper_558.xhtml "NES 2.0 Mapper 558")



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
