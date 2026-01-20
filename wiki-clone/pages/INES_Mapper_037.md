# INES Mapper 037

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_037) | View [other pages](Special_AllPages.xhtml#INES_Mapper_037)

iNES Mapper 037 represents the Nintendo of Europe multicart "Super Mario Bros. + Tetris + Nintendo World Cup". It glues together 3 [MMC3](MMC3.xhtml "MMC3")-compatible games in a single pak. 

## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 Outer Bank Select ($6000-$7FFF)
    * 2.2 All other registers ($8000-$FFFF)
  * 3 Hardware



## Overview

  * PRG ROM size: 256 KiB
  * PRG ROM bank size: 8 KiB inner / 64 or 128 KiB outer
  * PRG RAM: Impossible
  * CHR capacity: 256 KiB ROM
  * CHR bank size: 1 and 2 KiB inner / 128 KiB outer
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper.
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



## Registers

This game, like [Super Spike V'Ball + Nintendo World Cup](INES_Mapper_047.xhtml "INES Mapper 047"), replaces PRG RAM with a single register to enforce multiple "jail cells" containing each game. 

The Outer Bank Select register is cleared by the [CIC](CIC_lockout_chip.xhtml "CIC lockout chip") reset line, so if this multicart is used in a console without a functioning lockout chip, only a full power cycle will get back to the menu. 

### Outer Bank Select ($6000-$7FFF)
    
    
    7  bit  0
    xxxx xQBB
          │││
          │└┴── If 3, forces PRG A16 high regardless of Q bit; otherwise
          └──── the MMC3's PRG A16 is ANDed with this bit before going to PRG ROM.
                 Additionally connected to A17 on both PRG and CHR ROM.
    

In case that wasn't clear: 

Value written | PRG window | 128kB CHR window   
---|---|---  
0,1,2 | $00000-$0FFFF (64kB) | $00000-$1FFFF   
3 | $10000-$1FFFF (64kB) | $00000-$1FFFF   
4,5,6 | $20000-$3FFFF (128kB) | $20000-$3FFFF   
7 | $30000-$3FFFF (64kB) | $20000-$3FFFF   
  
As far as the MMC3 is concerned, this _is_ the PRG-RAM, so you will need to [enable writes to PRG-RAM](MMC3.xhtml#PRG_RAM_protect_.28.24A001-.24BFFF.2C_odd.29 "MMC3") to update it. Despite that the MMC3 thinks this register is RAM, the register itself does not, so it is not readable and attempting to read it will return open bus. 

### All other registers ($8000-$FFFF)

See [MMC3](MMC3.xhtml "MMC3"). 

## Hardware

Since this pak was only ever released with epoxy covering wirebonded silicon dice, the following is guesswork: 

It is very likely that the support hardware is a [74HC161](74161.xhtml "74161") latch and a [74HC00](7400.xhtml "7400") quad NAND gate, based on the specific order that the traces enter the epoxy blob. The latch, just like in the discrete logic mappers that use it, latches the bottom 3 bits of the data bus when it is written to. (Below, we call the latched form of D0→Q0, D1→Q1, and D2→Q2). The NAND gates are arranged to calculate 
    
    
    (MMC3 PRG A16 out) = M16 (for brevity)
    (PRG ROM A16 in) = Q0·Q1 + Q2·M16 = NAND(NAND(Q0,Q1),NAND(Q2,M16))
    (PRG ROM A17 in) = (CHR ROM A17 in) = Q2
    

Further testing has indicated that the internal products (Q0 NAND Q1, Q2 NAND M16) are brought out to test pads for factory verification. 

The fourth NAND gate is used to invert CIC +RESET to generate 74'161 /CLEAR. 

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
