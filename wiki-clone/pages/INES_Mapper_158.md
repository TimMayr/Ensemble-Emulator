# INES Mapper 158

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_158) | View [other pages](Special_AllPages.xhtml#INES_Mapper_158)

iNES Mapper 158 represents Tengen's board **800037** , used for [Alien Syndrome](https://nescartdb.com/profile/view/325/alien-syndrome). 

It modifies [RAMBO-1](RAMBO_1.xhtml "RAMBO-1") (**mapper 64**) in the exact same way that [TLSROM](TLSROM.xhtml "TLSROM") ([118](INES_Mapper_118.xhtml "INES Mapper 118")) differs from [MMC3](MMC3.xhtml "MMC3") ([4](MMC3.xhtml "INES Mapper 004")), by which we mean the RAMBO-1's CHR A17 output is connected to CIRAM A10. 

Nestopia-1.4.0's implementation looks incomplete; it seems to ignore [bank registers 8 and 9, as well as the K bit](RAMBO_1.xhtml#Bank_select_.28.248000-.249FFE.2C_even.29 "RAMBO-1"). 

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
