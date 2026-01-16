# INES Mapper 096

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_096) | View [other pages](Special_AllPages.xhtml#INES_Mapper_096)

iNES Mapper 096 was used with the two games that use the [Oeka Kids tablet](Oeka_Kids_tablet.xhtml "Oeka Kids tablet"). It is specifically designed to turn the PPU into an all-points-addressable 2bpp bitmap without needing timed code, a scanline counter, or [audio channel abuse](APU_DMC.xhtml#Usage_of_DMC_for_syncing_to_video "APU DMC"). 

Games: 

  * _Oeka Kids: Anpanman no Hiragana Daisuki_
  * _Oeka Kids: Anpanman to Oekaki Shiyou!!_



## Contents

  * 1 Overview
  * 2 Banks
  * 3 Registers
    * 3.1 Outer Bank Control (CPU $8000-$FFFF), bus conflicts
    * 3.2 Inner Bank Control (PPU $2000-$2FFF)
  * 4 Hardware
  * 5 Disch's Notes
  * 6 See Also



## Overview

  * PRG ROM size: 128 KiB
  * PRG ROM bank size: 32 KiB
  * PRG RAM: No
  * CHR capacity: 32 KiB RAM
  * CHR bank size: 4 KiB inner / 16 KiB outer
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Hardwired Vertical mirroring
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): partly



## Banks

  * CPU $8000-$FFFF: 32 KiB switchable PRG ROM bank
  * PPU $0000-$0FFF: 4 KiB switchable CHR RAM bank
  * PPU $1000-$1FFF: 4 KiB semi-fixed CHR RAM bank (banks 3 or 7 only)



## Registers

### Outer Bank Control (CPU $8000-$FFFF), bus conflicts
    
    
    7  bit  0
    xxxx xCPP
          |||
          |++- Select 32KiB PRG bank
          +--- Select 16KiB outer CHR bank
    

### Inner Bank Control (PPU $2000-$2FFF)

No bus conflicts: 
    
    
    13  PPU address  0
     DD xxCC xxxx xxxx
     ||   ||
     ||   ++---------- Select 4 KiB inner CHR RAM bank for PPU $0000-$0FFF
     ++--------------- Latch CC bits when 'DD' changes from anything else to '10'
    

Note that this effect does not pay attention to the PPU read or write strobes; it reacts continuously to all changes of the PPU's address bus. 

Sprites fetched from PPU $0000-$0FFF will always come from banks 0 or 4 because the CC bits are clear for the sprite dummy nametable fetches. 

## Hardware

The game uses a [74161](74161.xhtml "74161"), [7402](7402.xhtml "7402"), and [7474](7474.xhtml "7474"). The 74161 is wired exactly the same as in all of Nintendo's discrete logic mappers, complete with bus conflicts. 

The 7402 generates the clock signal for the 7474 by calculating `(PPU /A13) NOR (PPU A12)`, which is equivalent to `(PPU A13) AND (PPU /A12)`. On a rising edge of this signal, the 7474 latches the value of `PPU A9` and `PPU A8`, i.e. when the PPU fetches the nametable after having fetched from the pattern tables. Because `PPU /RD` isn't involved and attribute fetches are immediately after nametable fetches, attribute fetches are ignored. Another two NOR gates take these two latched values and produce 4+4F CHR banking. 

Technically, this banking is NOR logic, not OR logic. This difference is invisible to the software, however. (This means that the fixed bank is actually bank 0 or 4, and that reading from $2000 actually selects bank 3 or 7. The outer bank is not inverted: mapping the 3 C bits above to what actually appears on the RAM's address lines provides the order 3,2,1,0,7,6,5,4) 

To rephrase: it is the act of changing the PPU address from $0xxx, $1xxx, or $3xxx _to_ $2xxx that latches A9 and A8. There is no special logic that guarantees attribute fetches don't cause bankswitching, and Y scroll values greater than 240 will choose the same bank as reads from any of the rest of $23xx. It is only because of the PPU's fetch pattern that attribute fetches have no impact. 

## Disch's Notes
    
    
     Subset of Disch's original notes:  
    
    
    
     Long story short:
     
       A nametable spans from $2000-$23BF   ($23C0-$23FF are the attribute table).
       The mapper breaks the NT up like so:
     
          $2000-20FF = use CHR page 0
          $2100-21FF = use CHR page 1
          $2200-22FF = use CHR page 2
          $2300-23BF = use CHR page 3
     
       the other nametables at $2400, $2800, $2C00 are broken up in the same fashion.
     
     
     
     
     Long story long:
     
       PPU Address lines are modified as the PPU fetches tiles, and also when the game manually changes the PPU
     address (via the second write to $2006 --- or by the increment after read/writing $2007).  The mapper
     monitors every change to the PPU Address lines, and when it lies within a certain range, it swaps the
     appropriate CHR page in.
     
       It will only swap CHR when the address falls between $2000-2FFF.  $3xxx will not trigger a swap.
     
       When all that checks out, bits 8 and 9 (Addr AND $0300) select the 4k CHR page to swap in to $0000.
     
       Note that the mapper does not distinguish between PPU driven line changes and game driven line changes.
     This means that games can manually swap the CHR page by doing specific writes to $2006:
     
     LDA #$20
     STA $2006
     STA $2006   ; Addr set to $20xx -- CHR page 0 selected
     
     LDA #$21
     STA $2006
     STA $2006   ; Addr set to $21xx -- CHR page 1 selected
     
       And in fact, games would HAVE to do that to select CHR, since that's the only way to fill CHR RAM with the
     desired data.  So make sure your emu supports this.
    

## See Also

  * NES Mapper list by Disch [[1]](http://www.romhacking.net/documents/362/)



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
