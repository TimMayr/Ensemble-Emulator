# User:NovaSquirrel/Nova-7

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ANovaSquirrel/Nova-7) | View [other pages](Special_AllPages.xhtml#User_NovaSquirrel_Nova_7)

This proposal for an NES mapper combines the runtime flexibility of CHR RAM with the tile animation capability of CHR ROM. 

## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 Command Register ($5000-$57FF)
    * 2.2 Parameter Register ($5800-$5FFF)
    * 2.3 Scanline IRQ register ($4800-4FFF)
  * 3 Commands
    * 3.1 CHR Bank 0-7 ($0-7)
    * 3.2 PRG Bank 0-4 ($8-B, $E)
    * 3.3 Nametable/CHR control ($C)
    * 3.4 IRQ Control ($F)



## Overview

  * PRG ROM size: Up to 1024 KB
  * PRG ROM bank size: 8 KB
  * PRG RAM: Up to 1024 KB
  * CHR capacity: 32 KB or 128 KB CHR RAM
  * CHR bank size: .5 KB to 4 KB
  * Interrupts: scanline-based
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper; horizontal, vertical, single screen or four screen
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



CHR can be 256 KB, but 8 bits is not enough to cover the entire 256 KB with .5 KB banks and the source of the 9th bit is undefined. With RAM, this would also require two 128 KB SRAMs and hardware to select one. CHR can also be ROM but this would make the four screen mode much less useful. 

## Registers

Like with FME-7 and MMC3, configuration is accomplished by first writing the command number to the Command Register at $5000-57FF, then writing the command's parameter byte to the Parameter Register at $5800-5FFF. If $8000-$FFFF is write-protected, $8000-9FFF and $A000-BFFF will also work for the command and parameter registers, respectively. 

There are 15 commands: 

  * **$0-7** control CHR banking
  * **$8-B, E** control PRG banking
  * **$C** controls nametable mirroring and CHR mode and write protect
  * **$F** controls scanline interrupts



$D is reserved 

### Command Register ($5000-$57FF)
    
    
    7  bit  0
    ---- ----
    .... CCCC
         ||||
         ++++- The command number to invoke when writing to the Parameter Register
    

### Parameter Register ($5800-$5FFF)
    
    
    7  bit  0
    ---- ----
    PPPP PPPP
    |||| ||||
    ++++-++++- The parameter to use for this command. Writing to this register invokes the command in the Command Register.
    

### Scanline IRQ register ($4800-4FFF)

Writes go to the scanline IRQ register at $0F 

## Commands

### CHR Bank 0-7 ($0-7)

[![Nova7 modes.png](../wiki-images/Nova7_modes.png)](File_Nova7_modes_png.xhtml)

[](File_Nova7_modes_png.xhtml "Enlarge")
    
    
    7  bit  0
    ---- ----
    BBBB BBBB
    |||| ||||
    ++++-++++- The bank number to select for the specified bank.
    

Register  | CHR mode 0  | CHR mode 1  | CHR mode 2  | CHR mode 3   
---|---|---|---|---  
$0  | $0000-$03FF | $0000-$07FF | $0000-$07FF | $0000-$0FFF   
$1  | $0400-$07FF | $0800-$0BFF | $0800-$0FFF | $1000-$13FF   
$2  | $0800-$0BFF | $0C00-$0FFF | $1000-$13FF | $1400-$05FF   
$3  | $0C00-$0FFF | $1000-$07FF | $1400-$07FF | $1600-$07FF   
$4  | $1000-$13FF | $1800-$19FF | $1800-$19FF | $1800-$19FF   
$5  | $1400-$17FF | $1A00-$1BFF | $1A00-$1BFF | $1A00-$1BFF   
$6  | $1800-$1BFF | $1C00-$1DFF | $1C00-$1DFF | $1C00-$1DFF   
$7  | $1C00-$1FFF | $1E00-$1FFF | $1E00-$1FFF | $1E00-$1FFF   
  
### PRG Bank 0-4 ($8-B, $E)
    
    
    7  bit  0
    ---- ----
    RBBB BBBB
    |||| ||||
    |+++-++++- The bank number to select for the specified bank.
    +--------- If 1, bank is RAM instead of ROM
    
    
    
    Bank $8 - CPU $6000-$7FFF
    Bank $9 - CPU $8000-$9FFF
    Bank $A - CPU $A000-$BFFF
    Bank $B - CPU $C000-$DFFF
    Bank $E - CPU $E000-$FFFF
    

Loss of M2 oscillation (caused by resets) causes register $0E to revert to a value of $7F, mapping the last ROM bank in the cart into $E000-$FFFF. 

### Nametable/CHR control ($C)
    
    
    7  bit  0
    ---- ----
    .WCC SFMM
     ||| ||||
     ||| ||++- Mirroring Mode
     ||| ||     0 = Vertical
     ||| ||     1 = Horizontal
     ||| ||     2 = One Screen Mirroring from $2000 ("1ScA")
     ||| ||     3 = One Screen Mirroring from $2400 ("1ScB")
     ||| |+--- If 1, four screen mirroring in last 4KB of CHR RAM, ignores mm if on
     ||| +---- If 1, swap PPU $0xxx with $1xxx
     |++------ CHR bank switching mode
     +-------- If 0, any RAM at $8000-FFFF is write protected
    

When combined with sprite 0 or the scanline IRQ feature, this allows four-screen mirroring and a status bar at the same time, as the playfield is in CHR RAM and the status bar is in CIRAM. 

### IRQ Control ($F)
    
    
    7  bit  0
    ---- ----
    NNNN NNNN
    ++++-++++- Scanlines until IRQ
    

At the start of each scanline, the PPU freezes for a few cycles, and PPU A13 stays high for at least three consecutive cycles of PPU /RD. The mapper detects this and subtracts 1 from the value in $0F unless the value is $F0-$FF. While the value is 0, /IRQ is pulled low. Programming tip: Reading from the nametables or palette during vertical or forced blanking will cause counts unless you write $FF to port $0F. 
