# NES 2.0 Mapper 346

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_346) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_346)

NES 2.0 Mapper 346 is used for Kaiser's cartridge conversion of the FDS game _Zanac_. Its UNIF board name is **UNL-KS7012**. 

# Banks

  * CPU $6000-$7FFF: Unbanked 8 KiB of PRG-RAM
  * CPU $8000-$FFFF: Selectable 32 KiB PRG-ROM bank, defaults to #1
  * PPU $0000-$1FFF: Unbanked 8 KiB of CHR-RAM



Mirroring is hard-wired. 

# Registers

The actual address masks are unknown. 

  * CPU $E0A0: Select 32 KiB PRG-ROM bank #0 at CPU $8000-$FFFF.
  * CPU $EE36: Select 32 KiB PRG-ROM bank #1 at CPU $8000-$FFFF.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
