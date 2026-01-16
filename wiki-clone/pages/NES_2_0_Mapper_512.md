# NES 2.0 Mapper 512

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_512) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_512)

NES 2.0 Mapper 512 is used for 中國大亨 (Zhōngguó Dàhēng, often incorrectly rendered in Japanese as Chūgoku Taitei), a Chinese board game similar to Game of the Goose by Sachen. It uses an MMC3 clone with both CHR-ROM and at least 4 KiB of CHR-RAM, 4 KiB of additional VRAM, plus 8 KiB of battery-backed WRAM at CPU $6000-&7FFF. 

## Registers

Mask: probably $E001 
    
    
       $8000, $8001, $A000, $A001, $C000, $C001, $E000, $E001: As normal MMC3.
    

Mask: unknown, but probably $C100 
    
    
    $4100: Set CHR and VRAM mode
            0: CHR-ROM, CIRAM with MMC3-determined mirroring
            1: CHR-ROM, Cartridge VRAM
            2: CHR-RAM, CIRAM with MMC3-determined mirroring
            3: Seemingly identical to 2
    

## Notes

  * The game uses four pages of cartridge VRAM for the main game board, _plus_ the two pages of CIRAM for the status bar, for a total of six nametable pages.
  * While the fact that cartridge VRAM and CHR-RAM cannot be enabled at the same time might suggest that the same 4 KiB of RAM are used for both purposes, such is not the case, as the game board is expected to retain its content while Chinese characters are written into CHR-RAM.
  * Even as the game uses WRAM at CPU $6000-$7FFF, it fails to enable it by writing $80 to $A001. Apparently, the MMC3 clone does not allow disabling WRAM.



Categories: [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
