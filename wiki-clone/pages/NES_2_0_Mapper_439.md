# NES 2.0 Mapper 439

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_439) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_439)

**NES 2.0 Mapper 439** denotes the **YS2309** UxROM-like multicart PCB. Its UNIF MAPRs are **BMC-DS-07** and **BMC-K86B**. All registers set to $00 on reset. 

## Latch Register ($8000-$FFFF, write)
    
    
    D~[M..PP PPPP]  Address mask: $8000
       |  ++-++++- PRG A19..A14 when CPU A14=0,
       |           $3F when CPU A14=1
       +---------- Nametable mirroring, 0=Vertical, 1=Horizontal
    

This is basically an UxROM-like latch register for up to 1 MiB of PRG ROM, plus software-switchable mirroring. 

## Outer Bank Register ($6000, write)
    
    
    D~[.PPP ....]  Address mask: $E001
        +++------ PRG A19..A17 if enabled via $6001
    

## Mask Register ($6001, write)
    
    
    D~[MPPP ....]  Address mask: $E001
       |||+------- 0: PRG A17 from Latch, 1: PRG A17 from $6000
       ||+-------- 0: PRG A18 from Latch, 1: PRG A18 from $6000
       |+--------- 0: PRG A19 from Latch, 1: PRG A19 from $6000
       +---------- 1: Protect Mirroring bit in Latch from being changed
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
