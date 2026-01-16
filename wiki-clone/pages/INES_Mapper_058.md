# INES Mapper 058

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_058) | View [other pages](Special_AllPages.xhtml#INES_Mapper_058)

**iNES Mapper 058** is used for a number of simple NROM-/CNROM-based multicarts: 

  * _21-in-1 (AS-5321)_
  * _50-in-1 (WQ1806 B)_
  * _55-in-1 (WQ2006 B)_
  * _68-in-1 (HKX5268)_
  * _75-in-1 (WQ1905 E)_
  * _86-in-1 (AP-5486)_
  * _92-in-1 (WQ1605 E)_
  * _97-in-1 (WQ1708 B)_



## Address Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      1... .... MSCC CPPP
                |||| |+++- PRG A16..A14
                ||++-+---- CHR A15..A13
                |+-------- PRG Mode
                |           0: NROM-256 (PRG A14=CPU A14)
                |           1: NROM-128
                +--------- Nametable mirroring
                            0: Vertical
                            1: Horizontal
    

## Notes

  * [INES Mapper 213](INES_Mapper_213.xhtml "INES Mapper 213") is a duplicate.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
