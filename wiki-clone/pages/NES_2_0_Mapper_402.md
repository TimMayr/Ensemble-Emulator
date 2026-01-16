# NES 2.0 Mapper 402

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_402) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_402)

**NES 2.0 Mapper 402** denotes the **J-2282** circuit board, used by the _22-in-1 Olympic Games_ multicart with 512 KiB PRG-ROM and unbanked 8 KiB CHR-RAM. 
    
    
    A~[1... FC.. MN.P PPPp]
            ||   || +-++++- PRG A18..A14
            ||   |+-------- 0: PRG A14=CPU A14 (NROM-256)
            ||   |          1: PRG A14=p (NROM-128)
            ||   +--------- 0: Vertical mirroring
            ||              1: Horizontal mirroring
            |+------------- 0: CHR-RAM write-protected
            |               1: CHR-RAM write-enabled
            +-------------- 0: CPU $6000-$7FFF open bus
                            1: CPU $6000-$7FFF mirror of $E000-$FFFF
    

The F bit is set to play the FDS-sourced version of _Volleyball_. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
