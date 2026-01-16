# NES 2.0 Mapper 461

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_461) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_461)

**NES 2.0 Mapper 461** denotes the **CM-9309** PCB, used for the _6-in-1 Wuzi Gun_ [MMC1](MMC1.xhtml "MMC1")-based multicart, consisting of the two MMC1 games _Operation Wolf_ and _Mechanized Attack_ plus four NROM-128 games. There is one 256 KiB CHR ROM chip and one 32 KiB CHR ROM chip, with the 256 KiB one being stored first in the .NES file. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    A~[011. .... .... pMPP]
                      ||++-- PRG A15..A14 in NROM-128 mode
                      |+---- PRG Mode and CHR ROM chip select
                      ||      0: NROM-128 mode PRG, second (32 KiB) CHR chip
                      ||      1: MMC1 mode PRG mode, first (256 KiB) CHR chip
                      ++---- PRG A17..A16
                      +----- CHR A17
                      +----- 0: 128 KiB inner PRG bank in MMC1 mode (for Operation Wolf)
                             1: 64 KiB inner PRG bank in MMC1 mode (for Mechanized Attack)
    

This register connects to the MMC1's WRAM interface. The MMC1 clone mimics an MMC1A, so the WRAM interface cannot be disabled. The MMC1's registers select mirroring as well as 128 KiB PRG and 128 KiB CHR banks 

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
