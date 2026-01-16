# INES Mapper 108

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_108) | View [other pages](Special_AllPages.xhtml#INES_Mapper_108)

**NES 2.0 Mapper 108** denotes at least four different PCBs used for FDS-to-cartridge conversions. All have nametable mirroring hard-wired. 

## Contents

  * 1 Submapper 1: DH-08
  * 2 Submapper 2
  * 3 Submapper 3
  * 4 Submapper 4
  * 5 Heuristics



# Submapper 1: DH-08

Used by Whirlwind Manu's first release of Taito's _Bubble Bobble_ (LH31), and is a modified version of Kaiser's cartridge conversion, originally for [INES Mapper 142](INES_Mapper_142.xhtml "INES Mapper 142"). 

Banks: 

  * CPU $6000-$7FFF: Switchable 8 KiB bank
  * CPU $8000-$FFFF: Fixed to last 32 KiB of PRG-ROM
  * PPU $0000-$1FFF: Unbanked 8 KiB CHR-RAM



Registers: 

  * CPU $F000-$FFFF: Select 8 KiB PRG-ROM bank at CPU $6000-$7FFF. The game retains writes outside this address range, which must be ignored for the game not to crash.



# Submapper 2

Used by Whirlwind Manu's later release of Taito's _Bubble Bobble_ (LH31) that replaces CHR-RAM with CHR-ROM. UNIF board name is **UNL-BB**. 

Banks: 

  * CPU $6000-$7FFF: Switchable 8 KiB bank
  * CPU $8000-$FFFF: Fixed to last 32 KiB of PRG-ROM
  * PPU $0000-$1FFF: Switchable 8 KiB CHR-ROM bank



Registers: 

  * CPU $E000-$FFFF: Select 8 KiB PRG-ROM bank at CPU $6000-$7FFF *and* 8 KiB CHR-ROM bank at PPU $0000-$1FFF. The game retains writes outside this address range, which must be ignored for the game not to crash.



# Submapper 3

Submapper 1 but with the PRG-ROM switch responding in the full CPU $8000-$FFFF address range, which is required for both games to function: 

  * _Falsion_ (LH54)
  * _Meikyuu Jiin Dababa_ (LH28)



Banks: 

  * CPU $6000-$7FFF: Switchable 8 KiB bank
  * CPU $8000-$FFFF: Fixed to last 32 KiB of PRG-ROM
  * PPU $0000-$1FFF: Unbanked 8 KiB CHR-RAM



Registers: 

  * CPU $8000-$FFFF: Select 8 KiB PRG-ROM bank at CPU $6000-$7FFF



# Submapper 4

Like Submapper 2, but with the CHR-ROM switch responding in the full CPU $8000-$FFFF address range, and PRG-ROM banks being fixed. UNIF board name is _also_ **UNL-BB**. 

  * _Pro Wrestling_ (LE05)



Banks: 

  * CPU $6000-$7FFF: 8 KiB bank, fixed to last 8 KiB
  * CPU $8000-$FFFF: Fixed to last 32 KiB of PRG-ROM
  * PPU $0000-$1FFF: Switchable 8 KiB CHR-ROM bank



Registers: 

  * CPU $8000-$FFFF: Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF



# Heuristics

FCEUX and Mesen manage to emulate all four PCBs without disambiguation by heuristically considering the addresses that all games write to: 
    
    
    if((addr & 0x9000) == 0x8000 || addr >= 0xF000){
    	//A version of Bubble Bobble expects writes to $F000+ to switch the PRG banks
    	_prgReg = _chrReg = value;
    } else {
    	//For ProWres
    	_chrReg = value & 0x01;
    }
    

To disambiguate the four different PCBs in advance: 

  * CHR-RAM -> Submappers 1 or 3 
    * Horizontal Mirroring -> Submapper 1, otherwise 3
  * CHR-ROM -> Submappers 2 or 4 
    * More than 16 KiB of CHR-ROM -> Submapper 2, otherwise 4



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
