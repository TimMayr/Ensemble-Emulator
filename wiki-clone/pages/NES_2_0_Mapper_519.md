# NES 2.0 Mapper 519

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_519) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_519)

NES 2.0 Mapper 519 is used for Chinese-language multicarts designated by the UNIF board name **UNL-EH8813A**. It latches both the CPU address and data buses. 
    
    
    A~[1... ...L pDPP PPPP] D~[MCCC CCCC]
               | |||| ||||     |||| ||++-- Select Inner 8 KiB CHR-ROM bank at PPU $0000-$1FFF
               | |||| ||||     |+++-++---- Select Outer 32 KiB CHR-ROM bank at PPU $0000-$1FFF (*)
               | |||| ||||     +---------- Select Nametable mirroring type (0=Vertical, 1=Horizontal) (*)
               | ||++-++++---------------- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF/$C000-$FFFF (*)
               | |+----------------------- Read DIP switch mode (*)
               | |                           0: Disabled, reads from $8000-$FFFF normal
               | |                           1: Enabled, when reading from $8000-$FFFF:
               | |                               Bitwise AND address with $FFF0 and OR with the DIP switch setting ($0-$F)
               | +------------------------ PRG-ROM banking mode (*)
               |                             0: NROM-256 (32 KiB, bit 0 ignored)
               |                             1: NROM-128 (16 KiB, $8000-$BFFF mirrored at $C000-$FFFF)
               +-------------------------- 1=Lock all fields marked (*)
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
