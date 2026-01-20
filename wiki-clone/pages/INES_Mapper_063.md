# INES Mapper 063

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_063) | View [other pages](Special_AllPages.xhtml#INES_Mapper_063)

iNES Mapper 063 is used for NTDEC's "Powerful 250-in-1" multicart (PCB ID **TH2291-3**) and its pirate-of-a-pirate equivalent, the "Hello Kitty 255-in-1" (PCB ID **CH-011**) multicart (Submapper 0), as well as the **82AB** PCB used by a _82-in-1_ multicart (Submapper 1). The address-latch-based bankswitching hardware supports only NROM-128 and NROM-256 games. CHR data is stored in 8 KiB of unbanked CHR-RAM that can be write-protected. 

## Address latch ($8000-$FFFF, write)

Submapper 0: 
    
    
    A~FEDC BA98 7654 3210
      -------------------
      1... .CBB BBBB BbPM
            ||| |||| |||+- Select nametable mirroring type
            ||| |||| |||    0: Vertical
            ||| |||| |||    1: Horizontal
            ||| |||| ||+-- Select PRG banking mode
            ||| |||| ||     0: NROM-128, CPU A14 replaced with b
            ||| |||| ||     1: NROM-256, CPU A14 unmodified
            |++-++++-++--- Select 16 KiB PRG-ROM bank (A14..A21)
            +------------- Select CHR-RAM write-protection
                            0: Disabled, CHR-RAM writable
                            1: Enabled, CHR-RAM write-protected
    

For the menu code to correctly detect the number of games, selecting unpopulated PRG-ROM banks must result in open bus rather than wrapped addresses. 

Submapper 1: 
    
    
    A~FEDC BA98 7654 3210
      -------------------
      1... ..CB BBBB BbPM
             || |||| |||+- Select nametable mirroring type
             || |||| |||    0: Vertical
             || |||| |||    1: Horizontal
             || |||| ||+-- Select PRG banking mode
             || |||| ||     0: NROM-128, CPU A14 replaced with b
             || |||| ||     1: NROM-256, CPU A14 unmodified
             |+-++++-++--- Select 16 KiB PRG-ROM bank (A14..A21)
             +------------ Select CHR-RAM write-protection
                            0: Disabled, CHR-RAM writable
                            1: Enabled, CHR-RAM write-protected
    

Compared to Submapper 0, Submapper 1 can be thought as having one PRG-ROM bit less, and moving the CHR-RAM Protect bit into its place. 

## See also

  * [Submapper 0 dumping thread](https://forums.nesdev.org/viewtopic.php?t=19197)
  * [Submapper 1 dumping thread](https://forums.nesdev.org/viewtopic.php?t=16368)



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
