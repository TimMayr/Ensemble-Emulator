# NES 2.0 Mapper 396

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_396) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_396)

**NES 2.0 Mapper 396** denotes the 晶太 **YY850437C** and Realtec _GN-51_ multicart PCBs, basically an [UNROM](UxROM.xhtml "UNROM") mapper with an outer bank register. Its UNIF MAPR is **BMC-830752C**. 

  * _1995 Super 8-in-1_ (JY-050 rev0)
  * _Super 8-in-1 Gold Card Series_ (JY-085)
  * _Super 8-in-1 Gold Card Series_ (JY-086)
  * _2-in-1_ (Realtec PG-07, GN-51 PCB)



## Outer Bank Register ($A000-$BFFF)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      .Mm. .ppp
       ||   +++- PRG A19..A17
       |+------- Mirroring (GN-51), 1=Horizontal
       +-------- Mirroring (YY850437C), 1=Horizontal
    

No bus conflicts. Since no multicart menu sets the respective "other" mirroring bit, submapper-less emulation is possible by selecting horizontal mirroring if either bit is 1, and vertical mirroring if both are 0. 

## Inner Bank Register ($8000-$9FFF, $C000-$FFFF)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      .... .PPP
            +++- PRG A16..A14 if CPU A14=0
    

No bus conflicts. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
