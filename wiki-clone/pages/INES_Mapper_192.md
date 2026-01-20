# INES Mapper 192

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_192) | View [other pages](Special_AllPages.xhtml#INES_Mapper_192)

**iNES Mapper 192** is used to designate the Waixing **FS308** circuit board, which is similar in capability to TQROM ([iNES Mapper 119](INES_Mapper_119.xhtml "INES Mapper 119")). It uses a clone of the [MMC3](MMC3.xhtml "MMC3") but uses the CHR bank number in such a way so as to allow [both CHR ROM and CHR RAM to be used simultaneously](MMC3_with_CHR_ROM_and_CHR_RAM.xhtml "MMC3 with CHR ROM and CHR RAM") for displaying Chinese characters. 

BizHawk supports this mapper as of r3973; FCEUX and FCEU-MM have supported them for a long time but definitely as of 26-nov-2012, although CRC checks for m192<->m074 may be lacking. 

## Registers

Registers resemble [MMC3](MMC3.xhtml "MMC3"), except CHR ROM banks 8, 9, 10, and 11 are replaced with CHR RAM. Other (unknown) inaccuracies compared to an authentic MMC3 may be present. This is effectively the same as [INES Mapper 074](INES_Mapper_074.xhtml "INES Mapper 074") but with twice as much CHR RAM. If the [NES 2.0](NES_2_0.xhtml "NES 2.0") header is present, 74 and 192 should be emulated identically, as NES 2.0 specifies CHR RAM size elsewhere. 
    
    
     Example Game:
     --------------------------
     Ying Lie Qun Xia Zhuan
     
     
     Notes:
     --------------------------
     This mapper is a modified MMC3 (or is based on MMC3?).
     
     In addition to any CHR-ROM present, there is also an additional 4k of CHR-RAM which is selectable.
     
     CHR Pages $08-$0B are CHR-RAM, other pages are CHR-ROM.
    

Mappers [252](INES_Mapper_252.xhtml "INES Mapper 252") and [253](INES_Mapper_253.xhtml "INES Mapper 253"), also used for Waixing's localizations, are similar but use a VRC4. 

Also compare: [iNES Mapper 191](INES_Mapper_191.xhtml "INES Mapper 191"), [iNES Mapper 194](INES_Mapper_194.xhtml "INES Mapper 194"), [iNES Mapper 195](INES_Mapper_195.xhtml "INES Mapper 195")

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml)
