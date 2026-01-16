# User:Ddribin/PPU Sandbox

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ADdribin/PPU_Sandbox) | View [other pages](Special_AllPages.xhtml#User_Ddribin_PPU_Sandbox)

## Contents

  * 1 PPU Register Overview
  * 2 PPUCR1: PPU Control Register 1
    * 2.1 Bit 7 - NMIE: NMI Enable
    * 2.2 Bit 6 - MSE: Master/Slave Enable
    * 2.3 Bit 5 - SSZ: Sprite Size
    * 2.4 Bit 4 - BPT: Background Pattern Table
    * 2.5 Bit 3 - SPT: Sprite Pattern Table
    * 2.6 Bit 2 - VDN: VRAM Increment Down
    * 2.7 Bits 1, 0 - NTA1 and NTA0: Base Nametable Address 1 and 0
  * 3 PPUCR2: PPU Control Register 2
    * 3.1 Bit 7 - INB: Intensify Blues
    * 3.2 Bit 6 - ING: Intensify Greens
    * 3.3 Bit 5 - INR: Intensify Reds
    * 3.4 Bit 4 - SRE: Sprite Render Enable
    * 3.5 Bit 3 - BRE: Background Render Enable
    * 3.6 Bit 2 - SCD: Sprite Clip Disable
    * 3.7 Bit 1 - BCD: Background Clip Disable
    * 3.8 Bit 0 - GSE: Grayscale Enable
  * 4 PPUSR: PPU Status Register
    * 4.1 Bit 7 - VBL: Vertical Blank
    * 4.2 Bit 6 - S0H: Sprite 0 Hit
    * 4.3 Bit 5 - SOV: Sprite Overflow
    * 4.4 Bits 4..0 - Res: Reserved
  * 5 CA65 Definitions



## PPU Register Overview

The PPU exposes only eight memory-mapped registers to the CPU. These nominally sit at $2000 through $2007 in the CPU's address space, but because they're incompletely decoded, they're mirrored in every 8 bytes from $2008 through $3FFF, so a write to $3456 is the same as a write to $2006. 

PPU Registers  Address | Name | (Alt. Names?) | Function   
---|---|---|---  
$2000 | PPUCR1 | PPUCTRL PCR | PPU Control Register 1   
$2001 | PPUCR2 | PPUMASK PPUMR PMR | PPU Control Register 2   
$2002 | PPUSR | PPUSTATUS PSR | PPU Status Register   
$2003 | OAMAR | OAMADDR OAR | OAM Address Register   
$2004 | OAMDR | OAMDATA ODR | OAM Data Register   
$2005 | PPUSCR | PPUSCROLL PSCR | PPU Scroll Register   
$2006 | PPUAR | PPUADDR PAR | PPU Address Register   
$2007 | PPUDR | PPUDATA PDR | PPU Data Register   
$4014 | OAMDMA | ODMA | OAM DMA   
  
## PPUCR1: PPU Control Register 1

Bit | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0   
---|---|---|---|---|---|---|---|---  
$2000  | NMIE | MSE | SSZ | BPT | SPT | VDN | NTA1 | NTA0   
Read/Write  | W | W | W | W | W | W | W | W   
Initial Value  | 0 | X | 0 | 0 | 0 | 0 | 0 | 0   
  
### Bit 7 - NMIE: NMI Enable

Setting NMIE to one causes an NMI to be generated at the start of the vertical blanking interval. 

### Bit 6 - MSE: Master/Slave Enable

Has no effect on the NES. 

### Bit 5 - SSZ: Sprite Size

0: 8x8; 1: 8x16 

### Bit 4 - BPT: Background Pattern Table

Background pattern table address (0: $0000; 1: $1000) 

### Bit 3 - SPT: Sprite Pattern Table

Sprite pattern table address for 8x8 sprites (0: $0000; 1: $1000) 

### Bit 2 - VDN: VRAM Increment Down

VRAM address increment per CPU read/write of PPUDATA (0: increment by 1, going across; 1: increment by 32, going down) 

### Bits 1, 0 - NTA1 and NTA0: Base Nametable Address 1 and 0

NTA1 | NTA0 | Base VRAM Address   
---|---|---  
0 | 0 | $2000 (Nametable 0)   
0 | 1 | $2400 (Nametable 1)   
1 | 0 | $2800 (Nametable 2)   
1 | 1 | $2C00 (Nametable 3)   
  
## PPUCR2: PPU Control Register 2

Bit | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0   
---|---|---|---|---|---|---|---|---  
$2001  | INB | ING | INR | SRE | BRE | SCD | BCD | GSE   
Read/Write  | W | W | W | W | W | W | W | W   
Initial Value  | 0 | 0 | 0 | 0 | 0 | X | X | 0   
  
### Bit 7 - INB: Intensify Blues

Intensify blues and darken other colors. 

### Bit 6 - ING: Intensify Greens

Intensify greens and darken other colors. 

### Bit 5 - INR: Intensify Reds

Intensify reds and darken other colors. 

### Bit 4 - SRE: Sprite Render Enable

Enable sprite rendering. 

### Bit 3 - BRE: Background Render Enable

Enable background rendering. 

### Bit 2 - SCD: Sprite Clip Disable

Control sprite clipping in leftmost 8 pixels of screen (0: clip; 1: display). 

### Bit 1 - BCD: Background Clip Disable

Control background clipping in leftmost 8 pixels of screen (0: clip; 1: display). 

### Bit 0 - GSE: Grayscale Enable

0: normal color; 1: AND all palette entries with 0x30, effectively producing a monochrome display; note that colour emphasis STILL works when this is on! 

## PPUSR: PPU Status Register

Bit | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0   
---|---|---|---|---|---|---|---|---  
$2002  | VBL | S0H | SOV | \--- | \--- | \--- | \--- | \---   
Read/Write  | R | R | R | R | R | R | R | R   
Initial Value  | X | 0 | X | X | X | X | X | X   
  
### Bit 7 - VBL: Vertical Blank

### Bit 6 - S0H: Sprite 0 Hit

### Bit 5 - SOV: Sprite Overflow

### Bits 4..0 - Res: Reserved

## CA65 Definitions
    
    
    .define bit2mask(bitnum) (1 << bitnum)
    .define bits2mask(bits, bitnum) (bits << bitnum)
    
            ;; PPU Registers
            ppucr1 := $2000
            nmie = bit2mask(7)
            mse  = bit2mask(6)
            ssz  = bit2mask(5)
            bpt  = bit2mask(4)
            spt  = bit2mask(3)
            vdn  = bit2mask(2)
            nta1 = bit2mask(1)
            nta0 = bit2mask(0)
    
            nta_2000 = bits2mask(%00, nta0)
            nta_2400 = bits2mask(%01, nta0)
            nta_2800 = bits2mask(%10, nta0)
            nta_2c00 = bits2mask(%11, nta0)
    
            ppucr2 := $2001
            inb = bit2mask(7)
            ing = bit2mask(6)
            inr = bit2mask(5)
            sre = bit2mask(4)
            bre = bit2mask(3)
            scd = bit2mask(2)
            bcd = bit2mask(1)
            gse= bit2mask(0)
    
            ppusr := $2002
            vbl = bit2mask(7)
            s0h = bit2mask(6)
            sov = bit2mask(5)
    
            oamaddr := $2003
            oamdata := $2004
            ppuscroll := $2005
            ppuaddr := $2006
            ppuaddr := $2007
            oamdma := $4014
    
