# INES Mapper 061

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_061) | View [other pages](Special_AllPages.xhtml#INES_Mapper_061)

**iNES Mapper 61** , when using CHR ROM, denotes the NTDEC **0324** PCB, used for two _HQ 高品質合卡 15-in-1_ multicarts. A compatible PCB with ID **GS-2017** mounts 8 KiB of unbanked CHR RAM instead, used for the _方塊外傳 9合1 - Tetris Family 9-in-1_ and _RCM 20-in-1_ multicarts. 

## Address Latch ($8000-$FFFF, write)
    
    
    A~[1... CCCC M.pN PPPP]
            |||| | || ++++- PRG A18..A15
            |||| | |+------ 0: PRG A14=CPU A14 (NROM-256)
            |||| | |        1: PRG A14=p (NROM-128)
            |||| | +------- PRG A14 if M=1
            |||| +--------- Mirroring, 0=Vertical, 1=Horizontal
            ++++----------- CHR A16..A13 (when using CHR ROM)
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
