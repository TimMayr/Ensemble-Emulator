# NES 2.0 Mapper 262

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_262) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_262)

NES 2.0 Mapper 262 is used for _Street Heroes/侍魂 (Shìhún)_ by Sachen. It uses an [MMC3](MMC3.xhtml "MMC3") clone with both CHR-ROM and 8 KiB of CHR-RAM, as well 4 KiB of additional VRAM for four-screen mirroring. Its UNIF board name is **UNL-SHERO**. 

## Registers

Mask: probably $E001 
    
    
       $8000, $8001, $A000, $A001, $C000, $C001, $E000, $E001: As normal MMC3.
    

Mask: unknown, but probably $C100 
    
    
    $4100 (read): Read jumper setting
       Bit 7654 3210
           ---------
           J... ....
           +--------- Jumper value
                      0: Game title is _Street Heroes_
                      1: Game title is _侍魂 (Shìhún)_ (Samurai Spirits)
    
    $4100 (write): High CHR-ROM bits and CHR-ROM/-RAM selection
       Bit 7654 3210
           ---------
           .M.. 8421
            |   |||+- Select 256 KiB CHR-ROM bank at PPU $1000-$17FF
            |   ||+-- Select 256 KiB CHR-ROM bank at PPU $1800-$1FFF
            |   |+--- Select 256 KiB CHR-ROM bank at PPU $0800-$0FFF
            |   +---- Select 256 KiB CHR-ROM bank at PPU $0000-$07FF
            +-------- Select CHR-ROM/-RAM
                      0: CHR-ROM
                      1: CHR-RAM
    

Categories: [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
