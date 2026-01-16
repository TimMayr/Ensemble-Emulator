# INES Mapper 067

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_067) | View [other pages](Special_AllPages.xhtml#INES_Mapper_067)

  
**iNES Mapper 067** represents the [Sunsoft-3](Sunsoft_3_pinout.xhtml "Sunsoft 3 pinout") mapper, used in _Fantasy Zone II_ (J), _Mito Koumon II - Sekai Manyuu Ki_ , and the [Vs. System](Vs__System.xhtml "Vs. System") game _Vs. Platoon_. 

## Contents

  * 1 Overview
  * 2 Banks
  * 3 Registers
    * 3.1 Interrupt Acknowledge ($8000)
    * 3.2 CHR bank 0…3 ($8800..$BFFF)
    * 3.3 IRQ load ($C800, write twice)
    * 3.4 IRQ enable ($D800)
    * 3.5 Mirroring ($E800)
    * 3.6 PRG bank ($F800)
  * 4 IRQ Operation
  * 5 References



## Overview

  * PRG ROM size: Hardware supports at most 256 KiB
  * PRG ROM bank size: 16 KiB
  * PRG RAM: Unused, but mapper IC provides RAM enables.
  * CHR bank size: 2 KiB
  * CHR ROM size: Hardware supports at most 128 KiB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper (Horizontal, Vertical, 1-screen)
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



## Banks

  * CPU $8000-$BFFF: 16 KiB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG ROM bank, fixed to the last bank
  * PPU $0000-$07FF: 2 KiB switchable CHR bank
  * PPU $0800-$0FFF: 2 KiB switchable CHR bank
  * PPU $1000-$17FF: 2 KiB switchable CHR bank
  * PPU $1800-$1FFF: 2 KiB switchable CHR bank



## Registers

### Interrupt Acknowledge ($8000)

Mask: $8800 

Any write to this address or any of its mirrors acknowledges a pending IRQ. 

### CHR bank 0…3 ($8800..$BFFF)

Mask: $F800 

Note that the hardware only has six pins for CHR banking, for a limit of 128KB of CHR. 

Write to CPU address  | 2KB CHR bank affected   
---|---  
$8800 | $0000-$07FF   
$9800 | $0800-$0FFF   
$A800 | $1000-$17FF   
$B800 | $1800-$1FFF   
  
### IRQ load ($C800, write twice)

Mask: $F800 

Write the high then low byte of a 16-bit CPU cycle count, much like [PPUADDR](PPU_registers.xhtml "PPU registers"). This directly affects the current count, not a reload value. The write state is reset by writing to the register at $D800. 

### IRQ enable ($D800)

Mask: $F800 
    
    
    7  bit  0
    ...P ....
       |
       +------ 0: Pause counter; 1: Count
    

While bit 4 is true, the 16-bit count decreases by 1 every CPU cycle. Whenever the count wraps from $0000 to $FFFF, the mapper asserts an IRQ and pauses itself. Writes reset a latch such that the next $C800 write goes to the _high_ byte of the count. 

Despite previous notes, writes to this register do not acknowledge the IRQ. 

If counting is enabled, the External IRQ pin is also capable of asserting an IRQ. No existing hardware uses this functionality. 

### Mirroring ($E800)

Mask: $F800 
    
    
    7  bit  0
    .... ..MM
           ||
           ++- Nametable mirroring (0=vertical, 1=horizontal, 2=1scA, 3=1scB)
                aka connect VRAM A10 to (0=PPU A10, 1=PPU A11, 2=Gnd, 3=Vcc)
    

### PRG bank ($F800)

Mask: $F800 
    
    
    7  bit  0
    ...X PPPP
       | ||||
       | ++++- select a 16 KiB CHR ROM bank at CPU $8000-$BFFF. $C000-$FFFF is fixed to the last bank of PRG ROM.
       +------ 1 bit latch, present but unused. Could be combined with an external OR gate to increase PRG capacity to 512KB.
    

## IRQ Operation

$C800 is a write-twice register (similar to $2005 and $2006). The first write sets the **high** 8 bits of the IRQ counter, and the second write sets the **low** 8 bits. This directly changes the actual IRQ counter – not a reload value. 

In addition to enabling/disabling counting, any write to $D800 will reset the toggle so that the next write to $C800 will be the first write. 

The IRQ counter, when enabled, counts down every CPU cycle. When it wraps ($0000→FFFF), it disables itself and triggers an IRQ. 

## References

  * [Disch's Mapper Notes](http://www.romhacking.net/documents/362/)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
