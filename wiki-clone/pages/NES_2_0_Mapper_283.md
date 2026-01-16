# NES 2.0 Mapper 283

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_283) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_283)

NES 2.0 Mapper 283 is used for the _Block Family 6-in-1/7-in-1_ multicarts. Its UNIF board name is **BMC-GS-2004** (additional 8 KiB PRG-ROM chip present) and **BMC-GS-2013** (no additional 8 KiB PRG-ROM chip present). 

# Banks

  * CPU $6000-$7FFF: 8 KiB PRG-ROM bank, fixed to last 8 KiB bank within the first 256 KiB block, or if present, to the content of an additional 8 KiB PRG-ROM chip.
  * CPU $8000-$FFFF: 32 KiB switchable PRG-ROM bank.
  * PPU $0000-$1FFF: 8 KiB of CHR-RAM.



# Data Latch

  * CPU $8000-$FFFF: Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF.



# Notes

  * Mirroring is fixed and denoted in the NES header.
  * The 8 KiB PRG-ROM chip is present and located in the very last PRG-ROM bank if the number of PRG-ROM banks is a power of two _plus one_. Its data is repeated once since the iNES format only allows for 16 KiB PRG-ROM banks.



Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
