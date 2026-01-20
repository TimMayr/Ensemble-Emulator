# INES Mapper 207

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_207) | View [other pages](Special_AllPages.xhtml#INES_Mapper_207)

iNES Mapper 207 represents the board used for _Fudou Myouou Den_. 

It modifies Taito's X1-005 ([80](INES_Mapper_080.xhtml "INES Mapper 080")) in the exact same way that [TLSROM](TLSROM.xhtml "TLSROM") ([118](INES_Mapper_118.xhtml "INES Mapper 118")) differs from [MMC3](MMC3.xhtml "MMC3") ([4](MMC3.xhtml "INES Mapper 004")), by which we mean the X1-005's CHR A17 output is connected to CIRAM A10. Thus, it achieves 1scA/1scB/H controllable mirroring instead of H/V. 
    
    
     ========================
     =  Mapper 207          =
     ========================
     
      Registers:
     ---------------------------
     
       $7EF0:  [MCCC CCCC]
          M = Mirroring 0
          C = CHR Reg 0
     
       $7EF1:  [MCCC CCCC]
          M = Mirroring 1
          C = CHR Reg 1
     
       $7EF2-7EF5:  CHR Regs 2-5
     
       $7EF6-7EF7:  was old mirroring control register, now ignored
     
       $7EF8-7EFF:  see [iNES Mapper 080](INES_Mapper_080.xhtml "INES Mapper 080")
     
     CHR Setup:
     ---------------------------
     
            $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
          +---------------+---------------+-------+-------+-------+-------+
          |    <$7EF0>    |    <$7EF1>    | $7EF2 | $7EF3 | $7EF4 | $7EF5 |
          +---------------+---------------+-------+-------+-------+-------+
     
     Mirroring:
     ---------------------------
     
       [ $7EF0 ][ $7EF0 ]
       [ $7EF1 ][ $7EF1 ]
     
     Mirroring bit of appropriate reg selects NTA or NTB
    

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
