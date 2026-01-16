# NES 2.0 Mapper 310

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_310) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_310)

**NES 2.0 Mapper 310** denotes the **K-1053** circuit board, used for the _1200-in-1 New World Multi_ multicart. It is an extension of [INES Mapper 015](INES_Mapper_015.xhtml "INES Mapper 015") that supports more than 1 MiB of PRG ROM, and 32 KiB rather than 8 KiB of CHR RAM. 

# Banks

  * CPU $8000-$FFFF: Switchable PRG-ROM banks (8/16/32 KiB)
  * PPU $0000-$1FFF: Switchable 8 KiB (out of 32 KiB) of CHR-RAM that can be write-protected
  * Nametable mirroring: Switchable horizontal/vertical settings



# Data Latch ($8000-$BFFF, write)
    
    
    Mask: $C000
    
    D~7654 3210
    ---------
    pMPP PPPP
    ||++-++++- PRG A19..A14
    |+-------- Nametable mirroring
    +--------- PRG A13 if SS=2, ignored otherwise
     Power-on and reset value: All bits clear
    

# Address and Data Latch ($C000-$FFFF, write)
    
    
    Mask: $C000
    
    A~FEDC BA98 7654 3210  D~7654 3210
      -------------------    ---------
      11.. .... .... PPSS    .... ..CC
                     ||||           ++- CHR A14..A13
                     ||++-------------- PRG banking mode
                     ||                  0: NROM-256 (PRG A14=CPU A14)
                     ||                  1: UNROM    (PRG A14..16=111 when CPU A14=1)
                     ||                  2: NROM-64  (PRG A13=p)
                     ||                  3: NROM-128
                     ++---------------- PRG A21..20
    Power-on and reset value: All bits clear
    

CHR-RAM is write-protected in PRG banking modes 0 and 3, and write-enabled in modes 1 and 2. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
