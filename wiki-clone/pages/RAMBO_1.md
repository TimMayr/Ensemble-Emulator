# RAMBO-1

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/RAMBO-1) | View [other pages](Special_AllPages.xhtml#RAMBO_1)

**RAMBO-1**

**Company** | Tengen   
---|---  
**Games** | [5 in NesCartDB](https://nescartdb.com/search/advanced?ines=64)  
**Complexity** | ASIC   
**Boards** | 800032   
**Pinout** | [Tengen RAMBO-1 pinout](Tengen_RAMBO_1_pinout.xhtml "Tengen RAMBO-1 pinout")  
**PRG ROM capacity** | 256K   
**PRG ROM window** | 3×8K + 8K fixed   
**PRG RAM capacity** | None   
**CHR capacity** | 256K   
**CHR window** | 1Kx8 or 2Kx2 + 1Kx4   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H or V, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | Yes   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [064](RAMBO_1.xhtml "INES Mapper 064"), [158](INES_Mapper_158.xhtml "INES Mapper 158")  
  
**NESCartDB**

[iNES 064](https://nescartdb.com/search/advanced?ines=64)  
---  
[800032](https://nescartdb.com/search/advanced?unif=TENGEN-800032)  
  
    _For the mapper used in the game "Rambo", see [UxROM](UxROM.xhtml "UxROM")._

The **Tengen RAMBO-1** is an [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") [mapper](Mapper.xhtml "MMC"), canonically designated as **mapper 64**. This mapper is basically Tengen's version of the [MMC3](MMC3.xhtml "MMC3"), but with some extra features. The RAMBO-1 came as a [40-pin PDIP](Tengen_RAMBO_1_pinout.xhtml "Tengen RAMBO-1 pinout"). A variant with different mirroring control is [mapper 158](INES_Mapper_158.xhtml "INES Mapper 158"). 

Example games: 

  * _Klax_
  * _Skull and Crossbones_
  * _Shinobi_
  * _Rolling Thunder_
  * _Hard Drivin' (prototype)_



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Bank select ($8000-$9FFE, even)
      * 2.1.1 CHR Banks
      * 2.1.2 PRG Banks
    * 2.2 Bank data ($8001-$9FFF, odd)
    * 2.3 Mirroring ($A000-$BFFE, even)
    * 2.4 Unknown ($A001-$BFFF, odd)
  * 3 IRQ registers
    * 3.1 IRQ latch ($C000-$DFFE, even)
    * 3.2 IRQ mode select / reload ($C001-$DFFF, odd)
    * 3.3 IRQ acknowledge / disable ($E000-$FFFE, even)
    * 3.4 IRQ enable ($E001-$FFFF, odd)
  * 4 IRQ counter operation
  * 5 Variants
  * 6 See also



## Banks

  * CPU $8000-$9FFF: 8 KiB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG ROM bank
  * CPU $C000-$DFFF: 8 KiB switchable PRG ROM bank
  * CPU $E000-$FFFF: 8 KiB PRG ROM bank, fixed to the last bank
  * PPU -- Three selectable configurations: 
    1. 1 KiB switchable CHR banks at $0000, $0400, $0800, $0C00, $1000, $1400, $1800, $1C00
    2. 2 KiB switchable CHR banks at $0000, $0800; 1 KiB switchable CHR banks at $1000, $1400, $1800, $1C00
    3. 2 KiB switchable CHR banks at $1000, $1800; 1 KiB switchable CHR banks at $0000, $0400, $0800, $0C00



## Registers

The RAMBO-1 has four pairs of registers at $8000-$9FFF, $A000-$BFFF, $C000-$DFFF, and $E000-$FFFF - even addresses ($8000, $8002, etc.) select the low register and odd addresses ($8001, $8003, etc.) select the high register in each pair. 

### Bank select ($8000-$9FFE, even)
    
    
    7  bit  0
    ---- ----
    CPKx RRRR
    |||  ||||
    |||  ++++- Specify which bank register to update on next write to Bank Data register
    |||          0000: R0: Select 2 (K=0) or 1 (K=1) KiB CHR bank at PPU $0000 (or $1000)
    |||          0001: R1: Select 2 (K=0) or 1 (K=1) KiB CHR bank at PPU $0800 (or $1800)
    |||          0010: R2: Select 1 KiB CHR bank at PPU $1000-$13FF (or $0000-$03FF)
    |||          0011: R3: Select 1 KiB CHR bank at PPU $1400-$17FF (or $0400-$07FF)
    |||          0100: R4: Select 1 KiB CHR bank at PPU $1800-$1BFF (or $0800-$0BFF)
    |||          0101: R5: Select 1 KiB CHR bank at PPU $1C00-$1FFF (or $0C00-$0FFF)
    |||          0110: R6: Select 8 KiB PRG ROM bank at $8000-$9FFF (or $C000-$DFFF)
    |||          0111: R7: Select 8 KiB PRG ROM bank at $A000-$BFFF
    |||          1000: R8: If K=1, Select 1 KiB CHR bank at PPU $0400 (or $1400)
    |||          1001: R9: If K=1, Select 1 KiB CHR bank at PPU $0C00 (or $1C00)
    |||          1111: RF: Select 8 KiB PRG ROM bank at $C000-$DFFF (or $8000-$9FFF)
    ||+------- Full 1 KiB CHR bank mode  0: two 2 KiB banks at $0000-$0FFF (or $1000-$1FFF)
    ||                                   1: four 1 KiB banks at $0000-$0FFF (or $1000-$1FFF)
    |+-------- PRG ROM bank mode  0: $8000-$9FFF uses bank selected with R6
    |                                $A000-$BFFF uses bank selected with R7
    |                                $C000-$DFFF uses bank selected with RF
    |                             1: $8000-$9FFF uses bank selected with RF
    |                                $A000-$BFFF uses bank selected with R7
    |                                $C000-$DFFF uses bank selected with R6
    +--------- CHR A12 inversion  0: two 2 KiB banks (or four 1 KiB banks) at $0000-$0FFF
                                     four 1 KiB banks at $1000-$1FFF
                                  1: two 2 KiB banks (or four 1 KiB banks) at $1000-$1FFF
                                     four 1 KiB banks at $0000-$0FFF
    

#### CHR Banks

When $8000 & $80 | is $00 | is $80   
---|---|---  
PPU Bank | Value of RAMBO-1 register   
$0000-$03FF | R0 | R2   
$0400-$07FF | K=0: R0  
K=1: R8 | R3   
$0800-$0BFF | R1 | R4   
$0C00-$0FFF | K=0: R1  
K=1: R9 | R5   
$1000-$13FF | R2 | R0   
$1400-$17FF | R3 | K=0: R0  
K=1: R8   
$1800-$1BFF | R4 | R1   
$1C00-$1FFF | R5 | K=0: R1  
K=1: R9   
  
With 2KiB banks (where K=0), only even-numbered CHR banks can be selected. The lowest bit in the register is ignored and instead passes PPU A10 directly through to CHR A10. 

#### PRG Banks

Because the values in R6, R7, RF, and $8000 are unspecified at power on, the reset vector must point into $E000-$FFFF, and code must initialize these before jumping out of $E000-$FFFF. 

When $8000 & $40 | is $00 | is $40   
---|---|---  
CPU Bank | Value of RAMBO-1 register   
$8000-$9FFF | R6 | RF   
$A000-$BFFF | R7 | R7   
$C000-$DFFF | RF | R6   
$E000-$FFFF | (-1) | (-1)   
  
  * (-1) : the last bank



### Bank data ($8001-$9FFF, odd)

All eight bits are used for a new value for the bank based on last value written to Bank select register (as mentioned above) 

### Mirroring ($A000-$BFFE, even)
    
    
    7  bit  0
    ---- ----
    xxxx xxxM
            |
            +- [Mirroring](Mirroring.xhtml "Mirroring") (0: vertical; 1: horizontal)
    

This applies to **mapper 64** only (see Variants below). 

### Unknown ($A001-$BFFF, odd)

This register is either not implemented or has undiscovered functionality on RAMBO-1. The MMC3/MMC6 equivalent of this register controls external and internal PRG-RAM access. There is not presently any reason to believe that RAMBO-1 has any support for PRG-RAM. 

## IRQ registers

### IRQ latch ($C000-$DFFE, even)

All eight bits of this register specifies the IRQ counter reload value. When the IRQ counter is zero (or a reload is requested through $C001), this value will be copied into the IRQ counter at the end of the current scanline. 

### IRQ mode select / reload ($C001-$DFFF, odd)
    
    
    7  bit  0
    ---- ----
    xxxx xxxM
            |
            +- IRQ mode select (0: Scanline Mode, 1: CPU Cycle Mode)
    

Writing to this register also clears the IRQ counter so that it will be reloaded at next clock, or the next scanline, depending on the selected mode. This also resets the prescaler in cycle mode, so the next clock will occur 4 cycles later. 

### IRQ acknowledge / disable ($E000-$FFFE, even)

Writing any value to this register will disable counter interrupts AND acknowledge any pending interrupts. 

### IRQ enable ($E001-$FFFF, odd)

Writing any value to this register will enable counter interrupts. Note that writing to this register does not acknowledge the IRQ if already set. 

## IRQ counter operation

There are two IRQ modes: PPU A12 mode (also known as _scanline mode_) and CPU cycle mode. 

In _scanline mode_ , the counter is clocked using a very similar method to that used by the [MMC3](MMC3.xhtml "MMC3") and follows the same restrictions. In comparison to the [MMC3](MMC3.xhtml "MMC3"), the actual interrupt triggers slightly later. Specifically, it is delayed until [M2 falls twice after the PPU A12 rise](http://forums.nesdev.org/viewtopic.php?p=117323#p117323) that would have triggered the MMC3 interrupt. 

In _CPU cycle mode_ , the counter is clocked every 4 CPU cycles. The actual interrupt triggers [one M2 cycle later](http://forums.nesdev.org/viewtopic.php?p=117461#p117461) than one would naively expect. 

Whichever IRQ mode is being used, the counter behaves the following way: 

**When the IRQ is clocked by _scanline_ or _CPU cycle_ modes:**

  * **IF** $C001 was written to after previous clock: 
    * reload IRQ counter with IRQ reload value; if non zero, this value is **ORed with 1** (see notes).
  * **ELSE IF** IRQ counter is 0: 
    * reload IRQ counter with IRQ reload value.
  * **ELSE**
    * Decrement IRQ counter by 1.
  * If IRQ counter is now 0 **AND** IRQs are enabled: 
    * trigger IRQ after 4 CPU cycles.



**Notes:**

  * It's still unknown how the extra kick works. _Klax_ still requires **+1** to run perfectly.
  * Most emulators run _Skull & Crossbones_ with a glitched scanline on the "Continue" screen OR during the game (score bar).



Following these rules in scanline counter mode has demonstrated accurate behavior, but this needs to be refined and merged with the description above: 
    
    
    On M2 falling edge:
       If PA12 = 0:
           If 4-bit counter < 16: 4-bit counter++.
           else keep the same value.
       If PA12 = 1:
           If 4-bit counter = 16: Clock the 8-bit counter.
           Always reset 4-bit counter to 0.
    
    On 8-bit counter clock:
       If value = 0:
           If IRQ is enabled, trigger IRQ (wait 1 more cycle before doing that though, regardless of PA12 next cycle...)
           Always reload counter with value in $C000.
       Else value--.
    
    On Write to $C001 (reset):
       Reload 8-bit counter with value in $C000
       Set 4-bit counter to 17.
    

## Variants

[Mapper 158](INES_Mapper_158.xhtml "INES Mapper 158"), used for _Alien Syndrome_ , has mirroring like [mapper 118](INES_Mapper_118.xhtml "INES Mapper 118") ([TLSROM](TLSROM.xhtml "TLSROM")), where CIRAM A10 is connected to CHR A17, and bit 7 of each CHR bank mapped into PPU $0000-$0FFF controls which page of CIRAM is used for the corresponding nametable in $2000-$2FFF. 

## See also

  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
