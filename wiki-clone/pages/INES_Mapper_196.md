# INES Mapper 196

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_196) | View [other pages](Special_AllPages.xhtml#INES_Mapper_196)

iNES Mapper 196 denotes circuit boards by **MRCM** , who mostly made Mario-themed hacks of existing games. For copy protection purposes, the MMC3 clone's A0 input is connected to A1 or A2 instead. Additonally, one variant uses [mapper 189](INES_Mapper_189.xhtml "INES Mapper 189")-style PRG banking. Submappers denote the particular variant used: 

  * **Submapper 0** : Actual variant unknown; use the following heuristics: 
    * MMC3's CPU A0 input =1 if (CPU A1=1 or CPU A2=1 or CPU A3=1) and CPU A0=0
    * Use normal MMC3 PRG banking until CPU 6000-7FFF is written to, after which use [mapper 189](INES_Mapper_189.xhtml "INES Mapper 189")-style PRG banking
  * **Submapper 1** : MMC3 CPU A0 input=CPU A1, normal MMC3 PRG banking
  * **Submapper 2** : MMC3 CPU A0 input=CPU A2, normal MMC3 PRG banking
  * **Submapper 3** : MMC3 CPU A0 input=CPU A1, [mapper 189](INES_Mapper_189.xhtml "INES Mapper 189")-style PRG banking.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
