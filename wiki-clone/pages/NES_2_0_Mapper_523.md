# NES 2.0 Mapper 523

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_523) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_523)

NES 2.0 Mapper 523 is used for the 晶科泰 (Jncota) re-release of the game 封神榜: 伏魔三太子 (Fēngshénbǎng: Fúmó Sān Tàizǐ), originally released by C&E on [INES Mapper 246](INES_Mapper_246.xhtml "INES Mapper 246"). It is basically identical to the [Waixing FS005](INES_Mapper_176.xhtml "INES Mapper 176") board, except that ... 

  * Mirroring is hard-wired (to Horizontal in Fēngshénbǎng's case),
  * CHR-ROM A10 is directly connected to PPU A10,
  * CHR-ROM A11-A18 are connected to the MMC3 clone's CHR A10-A17 outputs,
  * the MMC3 clone's PPU A10 input is connected to Ground.



With the MMC3 clone set to [Extended MMC3 Mode](INES_Mapper_176.xhtml#MMC3-compatible_registers_\(%248000/%248001,_%24A000,_%24C000/%24C001,_%24E000/%24E001\) "INES Mapper 176"), the bank registers at $8000 therefore change from their normal to the following meanings: 
    
    
    $0: Select 2 KiB CHR bank at PPU $0000-$07FF (or $1000-$17FF)
    $1: Select 2 KiB CHR bank at PPU $0800-$0FFF (or $1800-$1FFF)
    $2: Select 2 KiB CHR bank at PPU $1000-$17FF (or $0000-$07FF)
    $3: No function
    $4: Select 2 KiB CHR bank at PPU $1800-$1FFF (or $0800-$0FFF)
    $5: No function
    $6: Select 8 KiB PRG ROM bank at $8000-$9FFF (or $C000-$DFFF)
    $7: Select 8 KiB PRG ROM bank at $A000-$BFFF
    $8: Select 8 KiB PRG ROM bank at $C000-$DFFF (or $8000-$9FFF)
    $9: Select 8 KiB PRG ROM bank at $E000-$FFFF
    $A: No function
    $B: No function
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
