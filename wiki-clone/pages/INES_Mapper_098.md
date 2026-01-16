# INES Mapper 098

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_098) | View [other pages](Special_AllPages.xhtml#INES_Mapper_098)

iNES Mapper 098 has been earmarked for an in-development mapper intended to make GBC -> NES ports easier with modest hardware manufacturing costs. 

Exact hardware specs are yet-to-be-finalized. But key features include: 

  * UNROM-style bankswitching with support for up to 4MB flash ROM, optionally self-flashable
  * 32kB of CHR-RAM, divided into 4kB pages
  * Optional support for up to 32kB of switchable PRG-RAM
  * 8x8 color attributes (8x1 achievable with raster tricks)
  * Cheap scanline IRQ partially driven by software
  * 4-screen layout using parts of CHR-RAM, or 2/1-screen layout using CIRAM


