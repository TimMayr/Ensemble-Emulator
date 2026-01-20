# INES Mapper 205

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_205) | View [other pages](Special_AllPages.xhtml#INES_Mapper_205)
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 205          =
     ========================
     
     
     Example Games:
     --------------------------
     15-in-1 (can be set via solder pad to show a 3-in-1 menu, with different games than the other 3-in-1 cart)
     3-in-1
     
     
     Registers:
     ---------------------------
     Regs at $6000-7FFF means no SRAM
     
       $6000-7FFF:   [.... ..MM]  Game Mode / Block
     
       $8000-FFFF:   [MMC3](MMC3.xhtml "MMC3")
     
     
     Notes:
     ---------------------------
     These multicarts select the game mode by writing to $6000-7FFF, the individual games then use traditional
     MMC3 style regs at $8000-FFFF.  The MMC3 regs only swap to pages *within* the block specifed by the game
     mode.  This can be easily emulated by ANDing the page numbers written to MMC3 with certain values, and then
     ORing them with other values, based on the selected block.
     
     Chart below to illustrate:
     
       Block    PRG-AND     PRG-OR    CHR-AND    CHR-OR
       -------------------------------------------------
        0         $1F        $00        $FF       $000
        1         $1F        $10        $FF       $080
        2         $0F        $20        $7F       $100
        3         $0F        $30        $7F       $180
     
     
     For details on MMC3, see [mapper 004](MMC3.xhtml "INES Mapper 004")
    

This mapper is extremely similar to [iNES Mapper 037](INES_Mapper_037.xhtml "INES Mapper 037"). Q1 directly connects to PRG A18 and CHR A18, and both ROMs' A17 pins are `Q0 + A17·/Q1` or `NAND(NAND(Q0,Q0),NAND(A17,NAND(Q1,Q1)))`

The PCB (UNIF board name **BMC-JC-016-2**) furthermore has a solder pad that can be set to force block 1, which nonsensically would select a PRG-AND of $1F but also with a PRG-OR of $10 and is therefore not used for any game on the multicart, to block 3. The 15-in-1's multicart menu writes a block value of 1, checks whether block 1 or block 3 is actually active, and shows a different menu depending on the result. The other 3-in-1 cartridge executes that same checking code, but shows the same menu regardless of the result. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
