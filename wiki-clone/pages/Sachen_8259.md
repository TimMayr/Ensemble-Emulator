# Sachen 8259

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sachen_8259) | View [other pages](Special_AllPages.xhtml#Sachen_8259)

Sachen 8259 describes a series of boards by Sachen. The following information was gathered primarily from FCEU-mm and while it appears to be a good enough behavioral description to run many ROMs, it is likely incomplete. 

## Contents

  * 1 Overview
  * 2 Variants
  * 3 Banks
  * 4 Registers
    * 4.1 Reg Select ($4100)
    * 4.2 Reg Data ($4101)
    * 4.3 Chr Select (Internal 0-3)
    * 4.4 Chr Outer Bank (Internal 4)
    * 4.5 Prg Bank (Internal 5)
    * 4.6 Mode and Mirroring Select (Internal 7)
  * 5 CHR Rewiring (A and C variants)
  * 6 Errata
  * 7 References



## Overview

  * PRG ROM size: up to 128 KiB
  * PRG ROM bank size: 32 KiB
  * PRG RAM: No
  * CHR capacity: up to 512KiB ROM, depending on variant
  * CHR bank size: 2 KiB
  * Nametable mirroring: Mapper controlled
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



Also see [Sachen SA8259A pinout](Sachen_SA8259A_pinout.xhtml "Sachen SA8259A pinout"). 

## Variants

There are four known variants of the board. The B variant is described below, as well as the A and C variants which alter behavior by rewiring CHR banks. The D variant behaves much differently and is described in [iNES Mapper 137](INES_Mapper_137.xhtml "INES Mapper 137"). 

iNES Mapper | PCB Label | UNIF Name | CHR Shift   
---|---|---|---  
138 | _unknown_ | UNL-Sachen-8259B | 0   
141 | TC-A003-72 | UNL-Sachen-8259A | 1   
139 | _unknown_ | UNL-Sachen-8259C | 2   
[137](INES_Mapper_137.xhtml "INES Mapper 137") | _unknown_ | UNL-Sachen-8259D | -1 (approximately)   
  
## Banks

  * CPU $8000-$FFFF: 32KiB switchable PRG ROM bank
  * PPU $0000-$07FF: 2KiB switchable CHR ROM bank
  * PPU $0800-$0FFF: 2KiB switchable CHR ROM bank
  * PPU $1000-$17FF: 2KiB switchable CHR ROM bank
  * PPU $1800-$1FFF: 2KiB switchable CHR ROM bank



## Registers

Registers should be masked with $C101. 

### Reg Select ($4100)
    
    
    7  bit  0
    ---- ----
    .... .rrr
          |||
          +++- Select register number for next data write
    

### Reg Data ($4101)
    
    
    7  bit  0
    ---- ----
    .... .ddd
          |||
          +++- Output data to register selected by $4100.
    

### Chr Select (Internal 0-3)
    
    
    7  bit  0
    ---- ----
    .... .ccc
          |||
          +++- Select lower 3 bits of 2KiB CHR block for PPU $0000/$0800/$1000/$1800
    

### Chr Outer Bank (Internal 4)
    
    
    7  bit  0
    ---- ----
    .... .bbb
          |||
          +++- Select upper 3 bits of all four 2KiB CHR blocks
    

### Prg Bank (Internal 5)
    
    
    7  bit  0
    ---- ----
    .... .ppp
          |||
          +++- Select 32KiB PRG bank for CPU $8000
    

### Mode and Mirroring Select (Internal 7)
    
    
    7  bit  0
    ---- ----
    .... .mms
          |||
          ||+- Enable "simple" mode.
          ++-- Select mirroring (0 = V, 1 = H, 2 = (0,1,1,1), 3 = 1scA)
    

When simple mode is enabled, mirroring is fixed to V, ignoring the m bits, and all 4 CHR banks use the CHR bank 0 register. 

## CHR Rewiring (A and C variants)

The B variant described above has 6 bit CHR regs covering 2KiB blocks, giving a CHR ROM total of up to 128KiB. The A and C variants rewire the board to increase this limit to 256KiB and 512KiB respectively. This is done by shifting up the CHR ROM address outputs on the mapper and connecting the lower lines directly to the PPU. To be specific: 
    
    
    B: Passes PPU A[10:0] through to CHR ROM A[10:0]
       Uses PPU A[12:11] to choose chr reg
       Passes 6 bits of chr reg to CHR ROM A[16:11]
    A: Passes PPU A[11:0] through to CHR ROM A[11:0]
       Uses PPU A[12:11] to choose chr reg
       Passes 6 bits of chr reg to CHR ROM A[17:12]
    C: Passes PPU A[12:0] through to CHR ROM A[12:0]
       Uses PPU A[12:11] to choose chr reg
       Passes 6 bits of chr reg to CHR ROM A[18:13]
    

This can be emulated by shifting the CHR regs left 1 or 2 bits, and inserting the appropriate 1 or 2 bits of PPU address in the low bits. Note that when s = 1, CHR reg 0 is used for all banks, but the extra address line(s) still leak through on the A and C boards. This means that on a C board, when s = 1, the board can be considered to have a single 8KiB CHR ROM bank for pattern tables. 

## Errata

  * The game _Q-Boy_ , using a PCB labelled **2M-RAM-COB** , replaces CHR-ROM with 8 KiB of unbanked CHR-RAM, and is usually set to mapper 141. Emulators must therefore distinguish a banked CHR-ROM variant with a CHR shift of 1, and an unbanked CHR-RAM variant, of mapper 141.
  * kevtris had originally assigned mapper 141 to be used just for _Q-Boy_ , and mapper **135** instead of 141 for the 8259A CHR-ROM variant with a CHR shift of 1.[1]



## References

  1. ↑ NES/fami Core Release Notes for Analogue NT Mini Jailbreak firmware


