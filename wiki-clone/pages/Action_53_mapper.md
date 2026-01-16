# Action 53 mapper

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Action_53_mapper) | View [other pages](Special_AllPages.xhtml#Action_53_mapper)

**Action 53**

**Company** | InfiniteNESLives   
---|---  
**Complexity** | CPLD   
**Boards** |   
**PRG ROM capacity** | 8M (512K/256K inner)   
**PRG ROM window** | 32K or 16K/16K   
**PRG RAM capacity** | None   
**CHR capacity** | 32K   
**CHR window** | 8K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H, V, or 1-screen switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [028](Action_53_mapper.xhtml "INES Mapper 028")  
  
The **Action 53 mapper** allows making a multicart of games that use multiple discrete mappers. It was assigned to [iNES Mapper 028](Action_53_mapper.xhtml "INES Mapper 028"). 

Examples: 

  * _[STREEMERZ: Action 53 Function 16 Volume One](Action_53.xhtml "Action 53")_
  * _[Double Action 53](Action_53.xhtml "Action 53")_
  * _[Action 53 Vol. 3: Revenge of the Twins](Action_53.xhtml "Action 53")_



  


## Contents

  * 1 Registers
    * 1.1 $5000: Register select
    * 1.2 $00: CHR bank
    * 1.3 $01: Inner bank
    * 1.4 $80: Mode
    * 1.5 $81: Outer bank
  * 2 Power up state
  * 3 Configurations
  * 4 Reference implementations
  * 5 Hardware implementations
  * 6 Implementation notes
  * 7 See also
  * 8 Notes
  * 9 External links



## Registers

$5000-$5FFF
    Register select
$8000-$FFFF
    Register value

There are no bus conflicts. 

### $5000: Register select
    
    
    7654 3210
    S       R
    |       +- Select register
    +--------- 0: User registers; 1: Supervisor registers
    

In a multicart, registers $00 and $01 change the bank within a game, and registers $80 and $81 remain constant throughout a given game's execution. Games ported from [SGROM or SNROM](MMC1.xhtml "SxROM") may rewrite register $80, usually to change mirroring. Games ported from SUROM may change register $81's value to access the first or second 256 KiB of the ROM. 

### $00: CHR bank
    
    
    7654 3210
       M   BB
       |   ++- Set CHR RAM A14-A13
       +------ Set mirroring mode bit 0 if H/V mirroring is disabled
    

### $01: Inner bank
    
    
    7654 3210
       M BBBB
       | ++++- Set current PRG ROM bank
       +------ Set mirroring mode bit 0 if H/V mirroring is disabled
    

If the current mirroring mode is one of the 1-screen modes (0 or 1), writes to registers $00 and $01 change bit 0 of the mirroring mode to D4 of the written value. (This simulates the mirroring control of [AxROM](AxROM.xhtml "AxROM").) If the current mirroring mode is vertical or horizontal (2 or 3), D4 is ignored. 

### $80: Mode
    
    
    7654 3210
      SS PPMM
      || ||++- Nametable mirroring mode
      || ++--- PRG bank mode
      ++------ PRG outer bank size
    

Mirroring modes  Mode | Effect | A10 output | Effect of write to register $00 or $01   
---|---|---|---  
0 | 1-screen lower bank | 0 | D4 changes bit 0 of the mirroring mode   
1 | 1-screen upper bank | 1 | D4 changes bit 0 of the mirroring mode   
2 | Vertical | PPU A10 | D4 is ignored   
3 | Horizontal | PPU A11 | D4 is ignored   
  
While the mirroring mode is 0 or 1 (1-screen), bit 0 can be written in three places: bit 0 of $80, bit 4 of $00, or bit 4 of $01. 

[![](../wiki-images/Multi-discrete_PRG_bank.png)](File_Multi_discrete_PRG_bank_png.xhtml)

Logic table, with o = $81 value and i = $01 value

PRG bank mode  Mode | Simulates | Effect   
---|---|---  
0, 1 | BNROM/AOROM | Current 32 KiB bank in $8000-$FFFF   
2 | UNROM (#180) | Fixed bottom half of outer bank in $8000-$BFFF  
Current bank in $C000-$FFFF   
3 | UNROM (#2) | Current bank in $8000-$BFFF  
Fixed top half of outer bank in $C000-$FFFF   
  
When the fixed bank ($8000-$BFFF in mode 2 or $C000-$FFFF in mode 3) is accessed, it treats accesses to the fixed bank the same way as accesses in mode 0 with 32K: the outer bank bits are passed straight through. For example, this would allow the fixed $C000 bank in mode 3 128K to be set to 16K bank 7 (as in mapper 2) or 1, 3, or 5. In mode 2 128K, the fixed $8000 bank could be configured as 16K bank 0 (as in mapper 180) or 2, 4, or 6. 

PRG outer bank size  Size | Effect   
---|---  
0 | A15 and up controlled by outer bank (32 KiB)   
1 | A16 and up controlled by outer bank (64 KiB)   
2 | A17 and up controlled by outer bank (128 KiB)   
3 | A18 and up controlled by outer bank (256 KiB)   
  
Again, when a fixed bank is being accessed, this is temporarily forced to 32K, allowing all outer bank bits to come through. 

These are the outputs on A22-A14 in each of the 12 combinations of mode and size, with **o** used for outer bank bits and **i** used for inner bank bits: 

Mode value | PRG bank mode | Outer bank size | Bank in $8000 | Bank in $C000   
---|---|---|---|---  
$00-$07 | 32 KiB | 32 KiB | `oooooooo0` | `oooooooo1`  
$08-$0B | Fixed $8000 | 32 KiB | `oooooooo0` | `ooooooooi`  
$0C-$0F | Fixed $C000 | 32 KiB | `ooooooooi` | `oooooooo1`  
$10-$17 | 32 KiB | 64 KiB | `oooooooi0` | `oooooooi1`  
$18-$1B | Fixed $8000 | 64 KiB | `oooooooo0` | `oooooooii`  
$1C-$1F | Fixed $C000 | 64 KiB | `oooooooii` | `oooooooo1`  
$20-$27 | 32 KiB | 128 KiB | `ooooooii0` | `ooooooii1`  
$28-$2B | Fixed $8000 | 128 KiB | `oooooooo0` | `ooooooiii`  
$2C-$2F | Fixed $C000 | 128 KiB | `ooooooiii` | `oooooooo1`  
$30-$37 | 32 KiB | 256 KiB | `oooooiii0` | `oooooiii1`  
$38-$3B | Fixed $8000 | 256 KiB | `oooooooo0` | `oooooiiii`  
$3C-$3F | Fixed $C000 | 256 KiB | `oooooiiii` | `oooooooo1`  
  
For all of these cases, the "o"s come from the **topmost** outer bank bits and the "i"s come from the **bottommost** inner bank bits. 

### $81: Outer bank
    
    
    7654 3210
    BBBB Bbbb
    ++++-++++- Set outer PRG ROM bank
    

When the outer bank size is set greater than 32K, the least significant bits are ignored. 

Bits 7 through 3 always control PRG ROM A22 through A18. Bits 2-0 control A17-A15 only when the outer bank size is small enough to require them. 

Many implementations recognize only the lower 4 or 6 bits for two reasons: memory cost and the practical limit of 2 MiB PRG ROM in an [iNES](INES.xhtml "INES") ROM image. The PowerPak uses only the low 4 bits, as it has only 512 KiB of RAM for PRG ROM. And by the 2010s, 5-volt 8-bit parallel flash memories larger than 2Mx8 had become hard to find. An implementation supporting [NES 2.0 large ROMs](NES_2_0.xhtml#Byte_9_\(Upper_bits_of_ROM_size\) "NES 2.0") should [recognize all bits](Oversize.xhtml "Oversize") for a maximum of 8 MiB. 

One document about this mapper describes a register at $4444 with unknown purpose. The released hardware does not respond to this address.[1]

## Power up state

At power on, the last 16 KiB of the ROM is mapped into $C000-$FFFF. The rest of the state is unspecified. The mapper state is unchanged on reset. 

_Non-normative:_ Once a program boots, it may set reg $81 = $FF and reg $80 = $02 to get into oversize-BNROM mode in the last bank. 

_Non-normative:_ If desired, games in a multicart can be patched with an appropriate reset stub to allow returning to the menu. The Action 53 build tool does this semi-automatically for NROM games. 

## Configurations

NROM-128 ([#0](NROM.xhtml "INES Mapper 000"))
    Outer bank size 0, PRG mode 2 or 3, mirroring H or V, select $01
NROM-256 ([#0](NROM.xhtml "INES Mapper 000"))
    Outer bank size 0, PRG mode 0, mirroring H or V, select $01
CNROM ([#3](CNROM.xhtml "INES Mapper 003"))
    Outer bank size 0, PRG mode 0, mirroring H or V, select $00
BNROM ([#34](INES_Mapper_034.xhtml "INES Mapper 034"))
    Outer bank size 1-3, PRG mode 0, mirroring H or V, select $01
BNROM oversize ([#34](INES_Mapper_034.xhtml "INES Mapper 034") as emulated)
    Outer bank size 0, PRG mode 0, mirroring H or V, select $81, and modify bus-conflict-avoidance table for position within multicart
UNROM (common) ([#2](UxROM.xhtml "INES Mapper 002"))
    Outer bank size 1-3, PRG mode 3, mirroring H or V, select $01
UNROM (Crazy Climber and MGC 2011) ([#180](INES_Mapper_180.xhtml "INES Mapper 180"))
    Outer bank size 1-3, PRG mode 2, mirroring H or V, select $01
AOROM ([#7](AxROM.xhtml "INES Mapper 007"))
    Outer bank size 1-3, PRG mode 0, mirroring 1-screen, select $01

## Reference implementations

See [Action 53 mapper/Reference implementations](Action_53_mapper_Reference_implementations.xhtml "Action 53 mapper/Reference implementations") for functions in Python and 6502 assembly language that calculate the bank number output to PRG ROM A20-A14 as described above. These may be used to verify emulator or hardware implementations. 

Supported in the following emulators: 

  * FCEUX (2.2.2; bugs prior to 2.3.0)
  * Bizhawk [c217768](https://github.com/TASVideos/BizHawk/commit/c217768871617ba9d6343896a47123f7027e392d)
  * Nintendulator
  * Mesen



## Hardware implementations

Two [implementations in Verilog](Action_53_mapper_Verilog.xhtml "Action 53 mapper/Verilog") are designed for use on a CPLD. One has been tested on a PowerPak as [MAP1C.MAP](https://forums.nesdev.org/viewtopic.php?p=102718#p102718). 

The MiSTer FPGA computer includes an [implementation in SystemVerilog](https://github.com/Kitrinx/NES_MiSTer/blob/7d1a94b38817a1eede8bfbbb5bfbc7b76b823824/rtl/mappers/generic.sv#L958-L1164). 

## Implementation notes

Input pins: 2 power, 16 signal 

  * Power and ground
  * CPU D7, D5-D0 (D6 used only in 4-8 MB version)
  * CPU A14-A12, /PRGSEL, M2, R/W
  * PPU A12-A10



Output pins: 12 signal 

  * CHR RAM A14-A13
  * CIRAM A10
  * PRG ROM A20-A14, /CE
  * Optional PRG RAM enable ($6000-$7FFF)



A CPLD requires one macrocell per bit of state and one for each output pin controlled by a multiplexer, plus possibly a couple more for more complex operations. Depending on maximum PRG ROM size (512 KiB to 8 MiB), this mapper requires 18 to 22 bits of state and 7 multiplexed outputs, which fits comfortably in a 32- or 36-cell CPLD. 

  * Register select: 2 bits
  * Register $00: 2 bits (D4 is directed to register $80)
  * Register $01: 4 bits (D4 is directed to register $80)
  * Register $80: 6 bits
  * Register $81: 4, 6, or 8 bits
  * A17-A14: 4 multiplexed outputs
  * CIRAM A10: 1 multiplexed output
  * PRG ROM /CE: 1 multiplexed output
  * PRG RAM /CE: 1 multiplexed output



After synthesizing a 2 MiB design and laying fitting within a XC9536XL CPLD, 27/36 Macrocells were consumed (75%). Additionally this design requires 25/34 available pins on the XC9536XL. 

Adding WRAM control requires 2 Macrocells and 2 pins. 

Lowering to 1 MB by shaving off PRG ROM A20 would save 1 Macrocell and 1 pin, if desired. 

## See also

  * [Action 53 manual](Action_53_manual.xhtml "Action 53 manual")



## Notes

  1. ↑ [Everynes](https://problemkaputt.de/everynes.htm#mapper28action53homebrewxin1) mentions a $4444 write. The menu program's author has used writes to $4444 for debugging purposes, without any associated hardware functionality related to this mapper.



## External links

  * [action53 github](https://github.com/pinobatch/action53) \- build tool to create multi-game ROM
  * [test28](https://forums.nesdev.org/viewtopic.php?p=215345#p215345), a comprehensive test ROM
  * [Forum post announcing reservation of #28](https://forums.nesdev.org/viewtopic.php?p=101111#p101111)
  * [Forum post announcing implementation in NESICIDE](https://forums.nesdev.org/viewtopic.php?p=101970#p101970)



Categories: [CPLD mappers](Category_CPLD_mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
