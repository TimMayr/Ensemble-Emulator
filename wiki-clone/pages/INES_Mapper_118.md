# INES Mapper 118

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_118) | View [other pages](Special_AllPages.xhtml#INES_Mapper_118)

**TxSROM** is used to designate [TKSROM](TKSROM.xhtml "TKSROM") and [TLSROM](TLSROM.xhtml "TLSROM") boards, both of which use the [Nintendo MMC3](MMC3.xhtml "Nintendo MMC3") in a nonstandard way. The only known difference between these boards and [TKROM](TxROM.xhtml "TKROM") and [TLROM](TxROM.xhtml "TLROM") is the mirroring configuration. The CHR A17 line connects directly to CIRAM A10 line instead of MMC3's CIRAM A10 output, to compensate for the MMC3's lack of single-screen [mirroring](Mirroring.xhtml "Mirroring"). The [iNES](INES.xhtml "INES") format assigns **iNES Mapper 118** to these boards. 

Example games: 

  * _Armadillo_
  * _Pro Sport Hockey_



## Contents

  * 1 Registers
    * 1.1 Bank data ($8001-$9FFF, odd)
    * 1.2 Mirroring ($A000-$BFFE, even)
  * 2 Variants
  * 3 References



## Registers

The behavior of these boards differs from that of a typical MMC3 board in the use of the upper CHR address line. This board relies on the fact that the MMC3's CHR bank circuit ignores A13 when calculating CHR A10-A17, responding to [nametable](PPU_nametables.xhtml "Nametable") fetches from $2000-$2FFF the same way as fetches from the first pattern table at $0000-$0FFF. This means that the 1KB/2KB banking scheme used for CHR bankswitching is active even during nametable fetches while the CHR ROM/RAM is disabled. 

However, on these particular boards, the CHR bankswitching directly affects the [mirroring](Mirroring.xhtml "Mirroring") mapping the nametable RAM. This allows programs to select which nametable is mapped to each slot, much like CHR banks are mapped to pattern table slots, in either two 2KB banks (allowing only single-screen or horizontal mirroring) or four 1KB banks (allowing all mirroring modes one can think of, because this is equal to the size of a nametable) at the price of mapping the 1KB CHR banks to the first pattern table by setting bit 7 of $8000. If the IRQ counter is being used in a standard way, this involves having sprites bankswitched in 2KB banks and backgrounds in 1KB banks. 

### Bank data ($8001-$9FFF, odd)
    
    
    7  bit  0
    ---- ----
    MDDD DDDD
    |||| ||||
    |+++-++++- New bank value, based on last value written to Bank select register
    |          0: Select 2 KB CHR bank at PPU $0000-$07FF (or $1000-$17FF);
    |          1: Select 2 KB CHR bank at PPU $0800-$0FFF (or $1800-$1FFF);
    |          2: Select 1 KB CHR bank at PPU $1000-$13FF (or $0000-$03FF);
    |          3: Select 1 KB CHR bank at PPU $1400-$17FF (or $0400-$07FF);
    |          4: Select 1 KB CHR bank at PPU $1800-$1BFF (or $0800-$0BFF);
    |          5: Select 1 KB CHR bank at PPU $1C00-$1FFF (or $0C00-$0FFF);
    |          6, 7: as standard MMC3
    |
    +--------- [Mirroring](Mirroring.xhtml "Mirroring") configuration, based on the last value
               written to Bank select register
               0: Select Nametable at PPU $2000-$27FF
               1: Select Nametable at PPU $2800-$2FFF
               Note : Those bits are ignored if corresponding CHR banks
               are mapped at $1000-$1FFF via $8000.
               
               2 : Select Nametable at PPU $2000-$23FF
               3 : Select Nametable at PPU $2400-$27FF
               4 : Select Nametable at PPU $2800-$2BFF
               5 : Select Nametable at PPU $2C00-$2FFF
               Note : Those bits are ignored if corresponding CHR banks
               are mapped at $1000-$1FFF via $8000.
    

### Mirroring ($A000-$BFFE, even)
    
    
    7  bit  0
    ---- ----
    xxxx xxxM
            |
            +- [Mirroring](Mirroring.xhtml "Mirroring")
               This bit is bypassed by the configuration described above, so writing here has no effect.
    

Note: In theory, the CHR limitation is 256 KB like all [MMC3](MMC3.xhtml "MMC3") boards. But because CHR A17 has another usage, having a CHR greater than 128 KB would require very careful CHR ROM layout because CHR bankswitching and mirroring will be linked through the same selection bits. Probably for this reason, official Nintendo TLSROM boards doesn't allow for 256 KB CHR-ROMs. However, a mapper 118 game that uses a third party MMC3/board, using 1-screen mirroring could draw the playfield with the lower 128 KB of CHR ROM and the lower nametable, and draw the status bar and menus with the upper 128 KB of CHR ROM and the upper nametable. Sprite tile banks could go in whatever space remains in either or both halves. 

## Variants

Mappers [158](INES_Mapper_158.xhtml "INES Mapper 158") and [207](INES_Mapper_207.xhtml "INES Mapper 207") do the exact same thing but with Tengen's [RAMBO-1](RAMBO_1.xhtml "RAMBO-1") and Taito's [X1-005](INES_Mapper_080.xhtml "INES Mapper 080"), respectively. [Mapper 95](INES_Mapper_095.xhtml "INES Mapper 095") is almost the same thing, but with a [reduced MMC3 made by Namco](INES_Mapper_206.xhtml "INES Mapper 206") instead of a full MMC3 and with the nametable select bit on A15 instead of A17 because the Namco mapper has only six CHR bank bits. 

## References

  * [NES Mapper list](http://www.romhacking.net/documents/362/) by Disch.



Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
