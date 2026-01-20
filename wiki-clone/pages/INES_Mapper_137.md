# INES Mapper 137

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_137) | View [other pages](Special_AllPages.xhtml#INES_Mapper_137)

  
iNES Mapper 137 is used to represent the Sachen 8259D board. It is similar to the [Sachen 8259](Sachen_8259.xhtml "Sachen 8259")A-C boards, but has some important differences. The information here comes primarily from FCEU-mm and may have some inaccuracies. 

## Contents

  * 1 Overview
  * 2 Variants
  * 3 Banks
  * 4 Registers
    * 4.1 Register Select ($4100)
    * 4.2 Register Data ($4101)
    * 4.3 CHR Select (Internal 0-3)
    * 4.4 CHR Top Bits (Internal 4)
    * 4.5 PRG Bank (Internal 5)
    * 4.6 CHR 3 Bit #3 (Internal 6)
    * 4.7 Mode and Mirroring Select (Internal 7)
  * 5 See also



## Overview

  * PRG ROM size: up to 128 KiB
  * PRG ROM bank size: 32 KiB
  * PRG RAM: No
  * CHR capacity: up to 32KiB ROM
  * CHR bank size: 1 KiB / 4KiB
  * Nametable mirroring: Mapper controlled
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



## Variants

There is only one known 8259D game, _The Great Wall_. It has 32KiB PRG ROM and 32KiB CHR ROM. It has been seen in the wild as iNES 137 and UNIF "UNL-Sachen-8259D". 

## Banks

  * CPU $8000-$FFFF: 32KiB switchable PRG ROM bank
  * PPU $0000-$03FF: 1KiB switchable CHR ROM bank
  * PPU $0400-$07FF: 1KiB switchable CHR ROM bank
  * PPU $0800-$0BFF: 1KiB switchable CHR ROM bank
  * PPU $0C00-$0FFF: 1KiB switchable CHR ROM bank
  * PPU $1000-$1FFF: fixed to last 4KiB of CHR ROM



## Registers

Registers should be masked with $C101. 

### Register Select ($4100)
    
    
    7  bit  0
    ---- ----
    .... .rrr
          |||
          +++- Select register number for next data write
    

### Register Data ($4101)
    
    
    7  bit  0
    ---- ----
    .... .ddd
          |||
          +++- Output data to register selected by $4100.
    

### CHR Select (Internal 0-3)
    
    
    7  bit  0
    ---- ----
    .... .ccc
          |||
          +++- Select lower 3 bits of 1KiB CHR block for PPU $0000/$0400/$0800/$0C00
    

### CHR Top Bits (Internal 4)
    
    
    7  bit  0
    ---- ----
    .... .DCB
          |||
          ||+- Select bit #4 for CHR register 1
          |+-- Select bit #4 for CHR register 2
          +--- Select bit #4 for CHR register 3
    

### PRG Bank (Internal 5)
    
    
    7  bit  0
    ---- ----
    .... .ppp
          |||
          +++- Select 32KiB PRG bank for CPU $8000
    

### CHR 3 Bit #3 (Internal 6)
    
    
    7  bit  0
    ---- ----
    .... ...E
            |
            +- Select bit #3 for CHR register 3
    

Putting all of this together, the 4 CHR registers have the following bit layouts: 
    
    
    7  bit  0
    ---- ----
    ...0 0ccc  CHR register 0
    ...B 0ccc  CHR register 1
    ...C 0ccc  CHR register 2
    ...D Eccc  CHR register 3
    

So only CHR register 3 can access all possible locations in CHR ROM. 

### Mode and Mirroring Select (Internal 7)

The only game only writes 0x2 to this register (i.e. mm=1, s=0) and wants vertical mirroring/horizontal layout. Underdocumentation has caused someone to set the "4-screen" bit in the header for _The Great Wall_ , but this is unnecessary and erroneous. 

The below is deduction, and differs from the normal Sachen 8259 behavior, because some rewiring is necessary for the reduced bank size. 
    
    
    7  bit  0
    ---- ----
    .... .mms
          |||
          ||+- Enable "simple" mode. (mirroring is fixed to _H_ , and banks become weird)
          ++-- Select mirroring (0 = _H_ , 1 = _V_ , 2 = (0,1,1,1), 3 = 1scA)
    

The upper two bits of the address are still controlled by the mapper when in "simple" mode, even though the three LSBs now all come from register 0. "Simple" mode doesn't affect the behavior of the external 74-series ICs. 

## See also

  * <http://cah4e3.shedevr.org.ru/%5Blst%5D-sachen-mappers.txt>



Categories: [INES Mappers](Category_INES_Mappers.xhtml)
