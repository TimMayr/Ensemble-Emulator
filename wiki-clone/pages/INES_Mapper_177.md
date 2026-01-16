# INES Mapper 177

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_177) | View [other pages](Special_AllPages.xhtml#INES_Mapper_177)

iNES Mapper 177 is used for games from 恒格电子 (Hénggé Diànzǐ). It's basically oversize [BNROM](INES_Mapper_034.xhtml "BNROM") without bus conflicts, with an additional mirroring control, and with 8 KiB of battery-backed WRAM at CPU $6000-$7FFF. 
    
    
                7  bit  0
                ---------
    $8000-FFFF: ..MP PPPP
                  |+-++++- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF
                  +------- Select nametable mirroring
                           0: Vertical
                           1: Horizontal
    

Some Hengedianzi games use hard-wired mirroring and do not set the M bit correctly; they are set to [INES Mapper 241](INES_Mapper_241.xhtml "INES Mapper 241") instead. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
