# Talk:INES Mapper 111

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_111) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_111)

## Like MMC1, except direct access to registers?

NesterJ's source code has a definition for Mapper 111 that implies that it is similar to the MMC1 except that the registers are directly accessible instead of needing to be fed 1 bit at a time. It is intended for a Chinese translation of Ninja Gaiden. 

8000-9FFF: ...C PPMM (Control register) 

A000-BFFF: ...C CCCC (CHR bank 0) 

C000-DFFF: ...C CCCC (CHR bank 1) 

E000-FFFF: ...R PPPP (PRG bank) 

The code is poorly organized, so I don't know if the reset bit (originally set by writing 0x80 to any register) is implemented at all. 

~~184.97.108.106~~ 00:38, 29 March 2015 (MDT) 
