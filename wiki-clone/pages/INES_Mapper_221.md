# INES Mapper 221

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_221) | View [other pages](Special_AllPages.xhtml#INES_Mapper_221)

iNES Mapper 221 is used for multicarts using the NTDEC N625092 PCB. They have 8 KiB of unbanked CHR-RAM at PPU $0000-$1FFF. Their UNIF board name is **BMC-N625092**. 

## Mode/Outer Bank Register ($8000-$BFFF)
    
    
    A~FEDC BA98 7654 3210
      -------------------
      10.. ..Bp ...O OOPM
             ||    | |||+- Select nametable mirroring type
             ||    | |||    0: Vertical
             ||    | |||    1: Horizontal
             ||    | ||+-- Select NROM-128/Other PRG-ROM modes
             ||    | ||     0: NROM-128 (Inner Bank selects 16 KiB PRG-ROM bank
             ||    | ||        at CPU $8000-$BFFF mirrored at CPU $C000-$FFFF)
             ||    | ||     1: Other mode (decided by bit 8)
             ||    +-++--- Select 128 KiB Outer PRG-ROM bank (PRG A17-A19)
             |+----------- Select PRG-ROM mode if bit 1=1
             |              0: NROM-128 (Inner Bank SHR 1 selects 32 KiB PRG-ROM
             |                 bank at CPU $8000-$FFFF)
             |              1: UNROM (Inner Bank selects 16 KiB PRG-ROM bank at
             |                 CPU $8000-$BFFF, CPU $C000-$FFFF fixed to Inner Bank #7)
             +------------ Select 1 MiB Outer PRG-ROM bank (PRG A20)
    

## Inner Bank Register ($C000-$FFFF)
    
    
    A~FEDC BA98 7654 3210
      -------------------
      11.. .... .... CIII
                     |+++- 16/32 KiB (depending on PRG-ROM mode) Inner Bank number
                     +---- Select CHR-RAM write-protection
                            0: Disabled, CHR-RAM write-enabled
                            1: Enabled, CHR-RAM write-protected
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
