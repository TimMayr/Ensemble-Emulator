# NES 2.0 Mapper 348

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_348) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_348)

NES 2.0 Mapper 348 is used for multicarts using _830118C_ -numbered PCBs such as _1994 New Series Red Pig 7-in-1_. Its UNIF board name is **BMC-830118C**. 

## Contents

  * 1 Registers
    * 1.1 MMC3-compatible registers ($8000-$FFFF, write)
    * 1.2 Outer Bank Register ($6800, write)
  * 2 GNROM-like Mode
  * 3 Similar Mappers



# Registers

## MMC3-compatible registers ($8000-$FFFF, write)

These registers function identically to a normal [MMC3](MMC3.xhtml "MMC3"), except in GNROM mode. 

## Outer Bank Register ($6800, write)
    
    
    Mask: probably $E000
    
    D~7654 3210
      ---------
      .... OO..
           ++--- Select 128 KiB outer PRG-ROM bank (i.e. PRG A17-A18)
                 and 128 KiB outer CHR-ROM bank (i.e. CHR A17-A18)
    

  * This bank register functions as a substitute for WRAM and can only be written to if $A001 bit 7 is set and $A001 bit 6 is clear.
  * The Outer Bank Register's PRG/CHR A17-A18 replace the MMC3's.



# GNROM-like Mode

If PRG A17 and PRG A18 are both 1 (i.e. if $6800 AND $0C==$0C), standard MMC3 PRG-ROM banking is replaced with an odd GNROM-ish mode, in which the MMC3's CPU A14 input is held low and PRG A14 is directly connected to CPU A14, resulting in the following bank layout: 

  * CPU $8000-$BFFF: MMC3 banks effective in the $8000-$BFFF range but as if their bank register values were AND'd with $FD
  * CPU $C000-$FFFF: MMC3 banks effective in the $8000-$BFFF range but as if their bank register values were OR'd with $02



# Similar Mappers

[NES 2.0 Mapper 315](NES_2_0_Mapper_315.xhtml "NES 2.0 Mapper 315") is similar but with the Outer Bank Register bits being arranged slightly differently. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
