# INES Mapper 191

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_191) | View [other pages](Special_AllPages.xhtml#INES_Mapper_191)

**iNES Mapper 191** designates a board containing an MMC3 clone. It operates identically to [iNES Mapper 119](INES_Mapper_119.xhtml "INES Mapper 119") with two differences: 

  * CHR RAM is 2 KiB, instead of 8 KiB like on 119. (This can be overridden with [NES 2.0](NES_2_0.xhtml "NES 2.0").)
  * Bit 7 of the CHR bank number (A17) switches between CHR ROM and CHR RAM, instead of bit 6 (A16) like on 119.


    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 191          =
     ========================
     
     
     aka:
     --------------------------
     Pirate MMC3 variant
     
     
     Example Game:
     --------------------------
     Sugoro Quest - Dice no Senshitachi (As)
     
     
     Notes:
     --------------------------
     This mapper is a modified [MMC3](MMC3.xhtml "MMC3") (or is based on MMC3?).
     
     In addition to any CHR-ROM present, there is also an additional 2k of CHR-RAM which is selectable.  Bit 7 of
     each CHR reg selects RAM or ROM (1=RAM, 0=ROM)
     
     Apart from that, this mapper behaves exactly like your typical MMC3.  See [mapper 004](MMC3.xhtml "INES Mapper 004") for details.
    

See also: 

  * Waixing's other MMC3-based mappers with both CHR RAM and CHR ROM- [074](INES_Mapper_074.xhtml "INES Mapper 074"), [192](INES_Mapper_192.xhtml "INES Mapper 192"), [194](INES_Mapper_194.xhtml "INES Mapper 194"), [195](INES_Mapper_195.xhtml "INES Mapper 195")
  * Waixing's VRC4-based mappers with both CHR RAM and CHR ROM- [252](INES_Mapper_252.xhtml "INES Mapper 252"), [253](INES_Mapper_253.xhtml "INES Mapper 253")



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml)
