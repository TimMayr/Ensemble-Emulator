# INES Mapper 065

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_065) | View [other pages](Special_AllPages.xhtml#INES_Mapper_065)

  
iNES Mapper 065 refers to games that use Irem's H3001 mapper. 
    
    
     Known Games:
     --------------------------
     Daiku no Gen San 2
     Kaiketsu Yanchamaru 3
     Spartan X 2
     
     Registers:
     --------------------------
     
       $8000:  PRG Reg 0 (8k @ $8000 -or- 8k @ $C000)
       $A000:  PRG Reg 1 (8k @ $A000)
     
       $B000-$B007:  CHR regs
     
       $9000:  [X... ....]  PRG bank layout
         Very similar to VRC4 or MMC3
         %0 = bank at $8000 set by writes to $8000, bank at $C000 always $3E
         %1 = bank at $C000 set by writes to $8000, bank at $8000 always $3E
    
         bank at $E000 always $3F
    
       $9001:  [MM.. ....]  Mirroring
         %00 = Vert
         %10 = Horz
         %01,%11 = 1scA
     
       $9003:  [E... ....]  IRQ Enable (0=disabled, 1=enabled)
       $9004:  [.... ....]  Reload IRQ counter
       $9005:  [IIII IIII]  High 8 bits of IRQ Reload value
       $9006:  [IIII IIII]  Low 8 bits of IRQ Reload value
     
     
     On Powerup:
     ---------------------------
     On powerup, it appears as though PRG regs are inited to specific values:
     
       $8000 = $00
       $A000 = $01
     
     Games do rely on this and will crash otherwise.
     
     CHR Setup:
     ---------------------------
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +-------+-------+-------+-------+-------+-------+-------+-------+
         | $B000 | $B001 | $B002 | $B003 | $B004 | $B005 | $B006 | $B007 |
         +-------+-------+-------+-------+-------+-------+-------+-------+
     
     
     
     IRQs:
     ---------------------------
     
     This mapper's IRQ system is very simple.  There's a 16-bit internal down counter which (when enabled),
     decrements by 1 every CPU cycle.  When the counter reaches 0, an IRQ is fired.  The counter stops at 0 -- it
     does not wrap and isn't automatically reloaded.
     
     Any write to $9003 or $9004 will acknowledge the pending IRQ.
     
     Any write to $9004 will copy the 16-bit reload value into the counter.
     
     $9006 and $9005 set the reload value, but do not have any effect on the actual counter.  Note that $9005 is
     the HIGH bits, not the low bits.
    

## See also

  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.



Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
