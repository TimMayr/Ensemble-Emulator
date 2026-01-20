# INES Mapper 050

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_050) | View [other pages](Special_AllPages.xhtml#INES_Mapper_050)

**iNES Mapper 050** denotes the **N-32** conversion of _Super Mario Bros. 2 (J)_. A hack of the [YUNG-08](NES_2_0_Mapper_368.xhtml "NES 2.0 Mapper 368") conversion, it replaces references to "Mario" with "Romeo". Its PCB code is **761214**. 
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 050          =
     ========================
     
     Example Game:
     --------------------------
     Super Mario Bros. (JU) (Alt Levels)   (SMB2j pirate cart)
     
     
     Notes:
     ---------------------------
     No PRG-RAM.  PRG setup is bizarre, as is the scrambled PRG reg.
     
     
     Registers:
     ---------------------------
     
     Range,Mask:   $4020-5FFF, $4120
     
     
       $4020:  [.... HLLM]
         L,M,H = Low, middle, and high bits of PRG Reg
     
     
       $4120:  [.... ...E]  IRQ Enable (0=Disabled, 1=Enabled)
     
     
     
     PRG Setup:
     ---------------------------
     
           $6000   $8000   $A000   $C000   $E000  
         +-------+-------+-------+-------+-------+
         | {$0F} | { 8 } | { 9 } | $4020 | {$0B} |
         +-------+-------+-------+-------+-------+
     
     
     IRQs:
     ---------------------------
     
       Writing to $4120 with E=0 will disable IRQs, acknowledge the pending IRQ, and reset the IRQ counter to 0.
     Writing with E=1 will just enable IRQs (but will not change anything else).
     
       When enabled, The IRQ counter will count up every CPU cycle.  When it makes the transition from
     $0FFF->$1000, and IRQ is generated.  The counter appears to be a full 16-bits (so it will not wrap until
     $FFFF)
    

The purported 16-bit counter seems unlikely. Both FCEUX and Nestopia disable the counter when it fires. The actual hardware is more likely a 13-bit counter with its MSB inverted into the Famicom's /IRQ line. 

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml)
