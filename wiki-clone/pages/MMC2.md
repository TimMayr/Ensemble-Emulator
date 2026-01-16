# MMC2

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC2) | View [other pages](Special_AllPages.xhtml#MMC2)

**MMC2**   
**PxROM**

**Company** | Nintendo   
---|---  
**Games** | [1 in NesCartDB](https://nescartdb.com/search/advanced?ines=9)  
**Complexity** | ASIC   
**Boards** | PNROM, PEEOROM   
**Pinout** | [MMC2 pinout](MMC2_pinout.xhtml "MMC2 pinout")  
**PRG ROM capacity** | 128K   
**PRG ROM window** | 8K + 24K fixed   
**PRG RAM capacity** | 8K (PC10 ver.)   
**PRG RAM window** | Fixed   
**CHR capacity** | 128K   
**CHR window** | 4K + 4K (triggered)   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H or V, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [009](MMC2.xhtml "INES Mapper 009")  
  
The **Nintendo MMC2** is an [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") [mapper](Mapper.xhtml "MMC"), used on the **PNROM** and **PEEOROM** Nintendo Game Pak boards for _Mike Tyson's Punch Out!!_. The [iNES](INES.xhtml "INES") format assigns **Mapper 009** to **PxROM**. This chip appeared in November 1987. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG ROM bank select ($A000-$AFFF)
    * 2.2 CHR ROM $FD/0000 bank select ($B000-$BFFF)
    * 2.3 CHR ROM $FE/0000 bank select ($C000-$CFFF)
    * 2.4 CHR ROM $FD/1000 bank select ($D000-$DFFF)
    * 2.5 CHR ROM $FE/1000 bank select ($E000-$EFFF)
    * 2.6 Mirroring ($F000-$FFFF)
  * 3 CHR banking
  * 4 Hardware
  * 5 Variants
  * 6 See also
  * 7 References



## Banks

  * CPU $6000-$7FFF: 8 KB PRG RAM bank (PlayChoice version only; [contains a 6264 and 74139](PRG_RAM_circuit.xhtml "PRG RAM circuit"))
  * CPU $8000-$9FFF: 8 KB switchable PRG ROM bank
  * CPU $A000-$FFFF: Three 8 KB PRG ROM banks, fixed to the last three banks
  * PPU $0000-$0FFF: Two 4 KB switchable CHR ROM banks
  * PPU $1000-$1FFF: Two 4 KB switchable CHR ROM banks



The two 4 KB PPU banks each have two 4 KB banks, which can be switched during rendering by using the special tiles $FD or $FE in either a sprite or the background. See CHR banking below. 

## Registers

### PRG ROM bank select ($A000-$AFFF)
    
    
    7  bit  0
    ---- ----
    xxxx PPPP
         ||||
         ++++- Select 8 KB PRG ROM bank for CPU $8000-$9FFF
    

### CHR ROM $FD/0000 bank select ($B000-$BFFF)
    
    
    7  bit  0
    ---- ----
    xxxC CCCC
       | ||||
       +-++++- Select 4 KB CHR ROM bank for PPU $0000-$0FFF
               used when latch 0 = $FD
    

### CHR ROM $FE/0000 bank select ($C000-$CFFF)
    
    
    7  bit  0
    ---- ----
    xxxC CCCC
       | ||||
       +-++++- Select 4 KB CHR ROM bank for PPU $0000-$0FFF
               used when latch 0 = $FE
    

### CHR ROM $FD/1000 bank select ($D000-$DFFF)
    
    
    7  bit  0
    ---- ----
    xxxC CCCC
       | ||||
       +-++++- Select 4 KB CHR ROM bank for PPU $1000-$1FFF
               used when latch 1 = $FD
    

### CHR ROM $FE/1000 bank select ($E000-$EFFF)
    
    
    7  bit  0
    ---- ----
    xxxC CCCC
       | ||||
       +-++++- Select 4 KB CHR ROM bank for PPU $1000-$1FFF
               used when latch 1 = $FE
    

### Mirroring ($F000-$FFFF)
    
    
    7  bit  0
    ---- ----
    xxxx xxxM
            |
            +- Select nametable mirroring (0: vertical; 1: horizontal)
    

## CHR banking

The main interest of the MMC2 and [MMC4](MMC4.xhtml "MMC4") mappers is that they allow switching two pairs of 4 KB CHR-ROM banks at the same time, automatically alternating between banks within one pair during rendering. When the PPU reads from specific tiles ($FD or $FE) in the pattern tables, the MMC2/4 sets a latch that switches between two different 4 KB banks. This allows the tile limit to increase from 256 to 512 with bank splits, without involving the CPU or an IRQ. 

  * PPU reads $0FD8: latch 0 is set to $FD for subsequent reads
  * PPU reads $0FE8: latch 0 is set to $FE for subsequent reads
  * PPU reads $1FD8 through $1FDF: latch 1 is set to $FD for subsequent reads
  * PPU reads $1FE8 through $1FEF: latch 1 is set to $FE for subsequent reads



Notice that latch 0 only responds to one address, but latch 1 responds to a range of addresses. This means that: 

  * The left ($0000-0FFF) pattern table only switches on the _top row_ of the 8x8 tile
  * The right ($1000-1FFF) pattern table switches on _every row_ of the 8x8 tile



With this mapper, the left pattern table ($0000) is intended for use with sprites, and the right pattern ($1000) table for background. Backgrounds require a switch on every row. Because sprites aren't constrained to an 8x8 grid, triggering on only the first row allows switching sprites to be placed closer together if needed. This nuance is absent in the [MMC4](MMC4.xhtml "MMC4"), where both pattern table switches from the entire tiles in a symmetrical fashion. 

Note that the latch is updated _after_ either pattern table byte is fetched, so the switching tiles $FD or $FE themselves are drawn using the old CHR-bank before the latch value is changed.[1] As the PPU fetches 34 background tiles per scanline (and at most 33 are drawn), if vertical [mirroring](Mirroring.xhtml "Mirroring") is used, background switching tiles can be placed past the edge of the screen where they will be unseen. 

Additionally, because unused sprite slots still perform fetches with a tile number of $FF, using 8x16 sprites will result in PPU $1000-$1FFF unexpectedly changing to the bank number written to $E000 at the ends of certain scanlines (7-14, 23-30, 39-46, etc.). 

## Hardware

The MMC2 is implemented in a [40-pin shrink-DIP package](MMC2_pinout.xhtml "MMC2 pinout"). At least two revisions are known to exist, the MMC2 and the MMC2-L. 

The PEEOROM board is used in the re-issue of _Mike Tyson's Punch-Out!!_. Unlike PNROM, and unlike most other boards used in NES Game Paks sold to the public, it can be configured to support EPROM memory through jumpers on the board. 

A pirate clone that exclusively uses discrete logic has been found and reverse-engineered.[2]

## Variants

Nintendo's [MMC4](MMC4.xhtml "MMC4"), used in the [FxROM](FxROM.xhtml "FxROM") board set, is a similar mapper with PRG RAM support and PRG bank sizes of 16kb instead of 8kb. It also suppresses the different banking behavior of the left pattern table. 

Because of the extreme similarity between the MMC2 and MMC4, it is possible to make a circuit that simulates an MMC4 from an MMC2 with the help of a [7402](7402.xhtml "7402") quad-NOR gate and a [7420](https://www.nesdev.org/w/index.php?title=7420&action=edit&redlink=1 "7420 \(page does not exist\)") 4-input NAND gate to [decode PRG RAM](PRG_RAM_circuit.xhtml "PRG RAM circuit"). The following circuit "tricks" the MMC2 into thinking the program is still in the $8000-$9FFF range when reading from $A000-$BFFF, but doesn't affect mapper writes. It also shifts all addresses left one bit so that it switches 16kB instead of 8kB banks, and it shortcuts around the different behavior for pattern tables at $0000 and $1000. 
    
    
    MMC2 A16  ----------------------------------  PRG A17
    
    MMC2 A15  ----------------------------------  PRG A16
    
    MMC2 A14  ----------------------------------  PRG A15
                    ____              ___
    MMC2 A13  -----\    `.       ,---\   `.
                    )     )o-----+    )    )o---  PRG A14
    CPU A14   -----/____,'       `---/___,'  
    
    CPU A13   ---+------------------------------  PRG A13
                 |    ___
                 +---\   `.         ___
                 |    )    )o------\   `.
                 `---/___,'         )    )o-----  MMC2 A13
                              ,----/___,'
    R/W       ----------------'
    
    GND       --------------------+-------------  MMC2 PA2
                                  |
                                  +-------------  MMC2 PA1
                                  |
                                  `-------------  MMC2 PA0
    

## See also

  * [Nintendo MMC2](http://nesdev.org/mmc2.txt) 01/29/98 by Jim Geffre.
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state and lates are inaccurate.
  * [NES Mapper list](http://www.romhacking.net/documents/362/) by Disch.



## References

  1. ↑ [nesdev forum: Glitch in the Matrix ???](https://forums.nesdev.org/viewtopic.php?t=19352)
  2. ↑ [nesdev forum: Punch Out Cartridge have only TTLs instead of MMC2!](http://forums.nesdev.org/viewtopic.php?f=9&t=10652)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers triggering on reads](Category_Mappers_triggering_on_reads.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
