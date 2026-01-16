# NES 2.0 Mapper 406

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_406) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_406)

**NES 2.0 Mapper 406** is used for the homebrew game _Haradius Zero_. It is basically a homebrew [TLROM](TxROM.xhtml "TLROM") circuit board that connects the [MMC3](MMC3.xhtml "MMC3")'s address lines differently, and saves the high score to flash ROM. The game executes the flash ROM chip's "Software ID" command; if the manufacturer and model ID do not match the expected values, or it detects that WRAM exists, the game replaces the first level with an unwinnable boss fight. 

**Submapper 0** denotes the 60-pin release of the game (saying "AXR" at the bottom of the title screen). The MMC3's A0 input is actually connected to CPU A1, so that $8000/$8002 rather than $8000/$8001 are differentiated. The Flash ROM chip is a Macronix MX29F040 (manufacturer ID=$C2, model ID=$A4). 

**Submapper 1** denotes the 72-pin release of the game (saying "INL" at the bottom of the title screen). The MMC3's $8xxx and $Exxx registers are swapped, so that $8000/$8001 enables/disable IRQ generation, and $E000/$E001 write to bank registers. The Flash ROM chip is an AMD AM29F040 (manufacturer ID=$01, model ID=$A4). 

Categories: [Mappers with flash save](Category_Mappers_with_flash_save.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
