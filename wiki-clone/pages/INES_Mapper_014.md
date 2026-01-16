# INES Mapper 014

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_014) | View [other pages](Special_AllPages.xhtml#INES_Mapper_014)

**iNES Mapper 014** denotes the 哥德 (Gouder) **SL-1632** PCB, used on the 8-character version of _Samurai Spirits_ by Rex Soft. Like [INES Mapper 116](INES_Mapper_116.xhtml "INES Mapper 116"), it uses the **Huang-1** ASIC together with a PAL that provides a supervisor register for selecting CHR A18 and for switching between [MMC3](MMC3.xhtml "MMC3") and [VRC2](VRC2_and_VRC4.xhtml "VRC2") modes (A0/A1, VRC2b), but moves the supervisor register address to $A131, overlapping the MMC3 and VRC2 registers there. Its UNIF board name is **UNL-SL1632**. 

According to Nestopia's mapper source code, mirroring can only be controlled in VRC2 mode, and the VRC2 PRG and mirroring registers do not respond to address mirrors. These behaviors contradict other emulators and have not been verified on original hardware. 

## Supervisor Register ($A131, write)
    
    
    Mask: $FFFF?
    
    D~7654 3210
      ---------
      A.B. C.M.
      | |  | +-- Mapper mode
      | |  |      0: VRC2
      | |  |      1: MMC3
      | |  +---- CHR A18 for PPU $0000-$0FFF
      | +------- CHR A18 for PPU $1000-$17FF
      +--------- CHR A18 for PPU $1800-$1FFF
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multi-ASIC mappers](Category_Multi_ASIC_mappers.xhtml)
