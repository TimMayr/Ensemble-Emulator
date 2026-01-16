# VRC3

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC3) | View [other pages](Special_AllPages.xhtml#VRC3)

**VRC3**

**Company** | Konami   
---|---  
**Games** | [1 in NesCartDB](https://nescartdb.com/search/advanced?ines=73)  
**Complexity** | ASIC   
**Boards** | 350685   
**Pinout** | [VRC3 pinout](VRC3_pinout.xhtml "VRC3 pinout")  
**PRG ROM capacity** | 128K   
**PRG ROM window** | 16K   
**PRG RAM capacity** | 8K   
**PRG RAM window** | 8K, fixed   
**CHR capacity** | 8K RAM   
**CHR window** | 8K, fixed   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | Yes   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [073](VRC3.xhtml "INES Mapper 073")  
  
The Konami VRC3 is an [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") [mapper](Mapper.xhtml "MMC") apparently used only for the game [Salamander](https://nescartdb.com/profile/view/1783/salamander). The [iNES](INES.xhtml "INES") format assigns **mapper 73** to VRC3. 

  


## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Select ($F000-$FFFF)
    * 2.2 IRQ Latch 0 ($8000-$8FFF)
    * 2.3 IRQ Latch 1 ($9000-$9FFF)
    * 2.4 IRQ Latch 2 ($A000-$AFFF)
    * 2.5 IRQ Latch 3 ($B000-$BFFF)
    * 2.6 IRQ Control ($C000-$CFFF)
    * 2.7 IRQ Acknowledge ($D000-$DFFF)
  * 3 IRQ Counter Operation
  * 4 References



  


## Banks

  * CPU $6000-$7FFF: optional 8 KiB PRG RAM bank
  * CPU $8000-$BFFF: 16 KiB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG ROM bank, fixed to the last bank



## Registers

### PRG Select ($F000-$FFFF)
    
    
    7  bit  0
    ---------
    .... .PPP
          |||
          +++- Select 16 KB PRG ROM at $8000
    

  


### IRQ Latch 0 ($8000-$8FFF)
    
    
    7  bit  0
    ---------
    .... LLLL
         ||||
         ++++- Bits 0-3 of IRQ latch.
    

### IRQ Latch 1 ($9000-$9FFF)
    
    
    7  bit  0
    ---------
    .... LLLL
         ||||
         ++++- Bits 4-7 of IRQ latch.
    

### IRQ Latch 2 ($A000-$AFFF)
    
    
    7  bit  0
    ---------
    .... LLLL
         ||||
         ++++- Bits 8-11 of IRQ latch.
    

### IRQ Latch 3 ($B000-$BFFF)
    
    
    7  bit  0
    ---------
    .... LLLL
         ||||
         ++++- Bits 12-15 of IRQ latch.
    

  


### IRQ Control ($C000-$CFFF)
    
    
    7  bit  0
    ---------
    .... .MEA
          |||
          ||+- IRQ Enable on Acknowledgement (see $D000)
          |+-- IRQ Enable (1=enabled)
          +--- IRQ Mode (1=8-bit counter, 0=16-bit counter)
    

Any write to this register will acknowledge the pending IRQ. If this register is written to with the **E** bit set, the 16-bit IRQ counter is reloaded with the 16-bit Latch value (set by writes to $8000-$BFFF). 

  


### IRQ Acknowledge ($D000-$DFFF)
    
    
    7  bit  0
    ---------
    .... ....
    

Any write to this register will acknowledge the pending IRQ. In addition, the **A** control bit moves to the **E** control bit, enabling or disabling IRQs. Writes to this register do not affect the current state of the IRQ counter. 

  


## IRQ Counter Operation

VRC3 IRQs operate differently from other VRCs. The counter is 16 bits instead of 8 bits, and there is no scanline mode, only CPU cycle mode. Other aspects, however, are very similar. 

IRQs on this mapper are generated through a 16-bit counter, which is incremented each CPU cycle (if enabled). When it overflows from $FFFF, an IRQ is generated, and the counter is reloaded with the 16-bit latch value. 

When the IRQ mode bit (**M**) is set, the upper 8 bits of the counter are ignored (and never incremented) and IRQs are generated when the lower 8 bits overflow from $FF. When the IRQ is triggered, only the low 8 bits are reloaded from the latch value. Reloading with a write to $C000 with **E** set will always reload all 16 bits, regardless of mode. 

When IRQs are disabled (**E** control bit clear), the IRQ counter does not increment. 

Writing to the acknowledge register at $D000 automatically moves control bit **A** into control bit **E** , allowing the IRQ to be easily re-enabled while acknowledging. 

## References

  * [Disch's Mapper Notes](http://www.romhacking.net/documents/362/)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
