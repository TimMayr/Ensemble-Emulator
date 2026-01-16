# INES Mapper 206

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_206) | View [other pages](Special_AllPages.xhtml#INES_Mapper_206)

**Namco 118, Tengen MIMIC-1**   
**DxROM**

**Company** | Namco, Tengen, others   
---|---  
**Games** | [39 in NesCartDB](https://nescartdb.com/search/advanced?ines=206)  
**Complexity** | ASIC   
**Boards** | 34xx, DxROM   
**PRG ROM capacity** | 128K   
**PRG ROM window** | 8K + 8K + 16K fixed   
**PRG RAM capacity** | None   
**CHR capacity** | 64K   
**CHR window** | 2Kx2 + 1Kx4   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H/V, or 4   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 206, [76](INES_Mapper_076.xhtml "INES Mapper 076"), [88](INES_Mapper_088.xhtml "INES Mapper 088"), [154](INES_Mapper_154.xhtml "INES Mapper 154"), [95](INES_Mapper_095.xhtml "INES Mapper 095")  
  
iNES Mapper 206 is the simpler predecessor of the [MMC3](MMC3.xhtml "MMC3"), and was used by Tengen and Namco. Chips used include "Tengen MIMIC-1" and "[Namcot 118](Namcot_108_family_pinout.xhtml "Namcot 108 family pinout")", and the boards made by Nintendo of America that used this mapper are NES-[DxROM](DxROM.xhtml "DxROM"). Many ROMS using this mapper are incorrectly listed as using MMC3, but will usually work if emulated with MMC3, and the mirroring is correct, as if they were on a [TEROM or TFROM board](TxROM.xhtml "TxROM"). 

Compared to MMC3: 

  * There are no IRQs
  * There is no WRAM support
  * PRG always has the last two 8KiB banks fixed to the end.
  * CHR always gives the left pattern table (0000-0FFF) the two 2KiB banks, and the right pattern table (1000-1FFF) the four 1KiB banks.
  * Mirroring is hardwired, one game uses 4-screen mirroring (Gauntlet, DRROM).
  * CHR size limit is 64KiB, PRG size limit is 128KiB.
  * There are no control registers in the $A000-$FFFF range. 
    * Writing to RAM while executing from $8000-9FFF can cause erratic mapper writes. See [Errata](INES_Mapper_206.xhtml#Errata "INES Mapper 206") below.



## Contents

  * 1 Registers
    * 1.1 Bank select ($8000-$9FFE, even)
    * 1.2 Bank data ($8001-$9FFF, odd)
  * 2 Errata
  * 3 Submappers
  * 4 Variants
  * 5 References



## Registers

Register mask: $E001 

### Bank select ($8000-$9FFE, even)
    
    
    7  bit  0
    ---- ----
    xxxx xRRR
          |||
          +++- Specify which bank register to update on next write to Bank Data register
               0: Select 2 KB CHR bank at PPU $0000-$07FF
               1: Select 2 KB CHR bank at PPU $0800-$0FFF
               2: Select 1 KB CHR bank at PPU $1000-$13FF
               3: Select 1 KB CHR bank at PPU $1400-$17FF
               4: Select 1 KB CHR bank at PPU $1800-$1BFF
               5: Select 1 KB CHR bank at PPU $1C00-$1FFF
               6: Select 8 KB PRG ROM bank at $8000-$9FFF
               7: Select 8 KB PRG ROM bank at $A000-$BFFF
    

See [MMC3](MMC3.xhtml "MMC3") and note the absence of any control bits in the upper five bits of this register. 

### Bank data ($8001-$9FFF, odd)
    
    
    7  bit  0
    ---- ----
    xxdd DDDd
      || ||||
      ++-++++- New bank value, based on last value written to bank select register (mentioned above)
    

Only bits 5-1 exist for the two 2 KiB CHR banks, only bits 5-0 exist for the four 1 KiB CHR banks, and only bits 3-0 exist for the two 8 KiB PRG banks. These eight bank registers are identical to those of [MMC3](MMC3.xhtml "MMC3"), except that only 128 KiB PRG and 64 KiB CHR are supported. 

## Errata

On Namco 108, writing to $0000-1FFF while executing from $8000-9FFF may be seen by the mapper as a write to $8000-9FFF, causing spurious bankswitches.[1] Contemporary software avoids this problem by never executing code from this region. This is presumably why _Babel no Tou_ , despite having only 32 KiB PRG, uses bankswapping and only executes bank 0 and 1 code when mapped into $A000-BFFF. Namco 109 and MIMIC-1 are not affected by this bug. It is not known if Namco 118 and 119 are affected, but if the ten's digit indicates manufacturer, then it may be that 118 is affected and 119 is not. 

## Submappers

_Babel no Tou_ , on the PCB **3401** , is the only game with 32 KiB PRG that allows—and uses!—PRG banking. All other games with 32 KiB PRG connect CPU A13 and CPU A14 directly to the PRG ROM, but fortunately they initialize their PRG registers to work on a normal board. Submapper 0 indicates the normal PCB that allows PRG banking, while submapper 1 indicate the **3407** , **3417** and **3451** PCBs with unbanked 32 KiB PRG-ROM. 

## Variants

[Mapper 76](INES_Mapper_076.xhtml "INES Mapper 076") increases CHR to 128KiB by inflating the 1KiB CHR banks to 2KiB and making the originally-2KiB banks inaccessible. 

Mapper [88](INES_Mapper_088.xhtml "INES Mapper 088") increases CHR to 128KiB by connecting PPU's A12 line to the CHR ROM's A16 line, making tiles in $0000 and $1000 come from disjoint sections of ROM. Because an undersize ROM on a mapper 88 board behaves identically to mapper 206, emulators may treat these mapper numbers as synonymous. 

[Mapper 154](INES_Mapper_154.xhtml "INES Mapper 154") starts with mapper 88, then adds mapper-controlled one-screen mirroring. 

[Mapper 95](INES_Mapper_095.xhtml "INES Mapper 095") uses the MSB to control mirroring by connecting CHR A15 to CIRAM A10, much as CHR A17 controls CIRAM A10 in [TxSROM](INES_Mapper_118.xhtml "INES Mapper 118"). 

## References

    

  * [FCEUX's implementation](https://sourceforge.net/p/fceultra/code/HEAD/tree/fceu/trunk/src/boards/206.cpp)
  * [Enri](http://cmpslv3.stars.ne.jp/)'s reverse-engineered schematic of PCBs 3407 and 3416: <http://cmpslv3.stars.ne.jp/Famic/Fcmp206.htm>



  1. ↑ [Forum thread:](https://forums.nesdev.org/viewtopic.php?t=13297) Naruko's report on Namco 108 banking problems when executing from $8000-9FFF



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
