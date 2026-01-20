# MMC1

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC1) | View [other pages](Special_AllPages.xhtml#MMC1)

**MMC1**   
**SxROM**

**Company** | Nintendo, others   
---|---  
**Games** | [390 in NesCartDB](https://nescartdb.com/search/advanced?ines=1)  
**Complexity** | ASIC   
**Boards** | SKROM, SLROM,  
SNROM, others   
**Pinout** | [MMC1 pinout](MMC1_pinout.xhtml "MMC1 pinout")  
**PRG ROM capacity** | 256 KiB (512 KiB)   
**PRG ROM window** | 16 KiB + 16 KiB fixed or 32 KiB   
**PRG RAM capacity** | 32 KiB   
**PRG RAM window** | 8 KiB   
**CHR capacity** | 128 KiB   
**CHR window** | 4 KiB + 4 KiB or 8 KiB   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H, V, or 1, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [001](MMC1.xhtml "INES Mapper 001"), [105](NES_EVENT.xhtml "INES Mapper 105"), [155](MMC1.xhtml "INES Mapper 155")  
  
The **Nintendo MMC1** is a [mapper](Mapper.xhtml "MMC") [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") used in Nintendo's [SxROM](MMC1.xhtml "SxROM"), [NES-EVENT](NES_EVENT.xhtml "NES-EVENT") and **2ME** circuit boards. It first appeared in the April of 1987. 

**iNES Mapper 1** and **155** denote the **SxROM** and **2ME** circuit boards mounting the MMC1B (iNES Mapper 1) or MMC1A (iNES Mapper 155) revision of the ASIC; Mapper 1 is used if the revision is not known or irrelevant. Three submappers of mapper 1/155 are valid: 

  * **Submapper 5** denotes the SEROM/SHROM/SH1ROM circuit boards with 32 KiB of unbanked PRG-ROM.
  * **Submapper 6** denotes the Famicom Network System's **2ME** circuit board.
  * **Submapper 7** denotes the Kaiser **KS-7058** that mounts the **KS 203** MMC1 clone, but with hard-wired nametable arrangement.



All other SxROM variants are denoted by their functional PRG/CHR-ROM/RAM sizes in the [NES 2.0](NES_2_0.xhtml "NES 2.0") header. Without NES 2.0, the PRG-RAM size has to be assumed; 32 KiB are sufficient for compatibility with all known titles. 

[INES Mapper 105](NES_EVENT.xhtml "INES Mapper 105") denotes the NES-EVENT circuit board. 

## Contents

  * 1 Banks
  * 2 Interface
    * 2.1 Examples
      * 2.1.1 Shift register
      * 2.1.2 Reset
    * 2.2 Consecutive-cycle writes
  * 3 Registers
    * 3.1 Load register ($8000-$FFFF)
    * 3.2 Control (internal, $8000-$9FFF)
    * 3.3 CHR bank 0 (internal, $A000-$BFFF)
    * 3.4 CHR bank 1 (internal, $C000-$DFFF)
    * 3.5 PRG bank (internal, $E000-$FFFF)
  * 4 SxROM connection variants
    * 4.1 SEROM, SHROM and SH1ROM
    * 4.2 SNROM
      * 4.2.1 CHR bank 0 (internal, $A000-$BFFF)
      * 4.2.2 CHR bank 1 (internal, $C000-$DFFF)
    * 4.3 SOROM, SUROM and SXROM
      * 4.3.1 CHR bank 0 (internal, $A000-$BFFF)
      * 4.3.2 CHR bank 1 (internal, $C000-$DFFF)
    * 4.4 SZROM
      * 4.4.1 CHR bank 0 (internal, $A000-$BFFF)
      * 4.4.2 CHR bank 1 (internal, $C000-$DFFF)
    * 4.5 Summary
  * 5 2ME
    * 5.1 Control (internal, $8000-9FFF)
    * 5.2 CHR bank 0 (internal, $A000-BFFF)
    * 5.3 PRG bank (internal, $E000-FFFF)
  * 6 SxROM board types
  * 7 ASIC Revisions
  * 8 References
  * 9 See also



## Banks

  * CPU $6000-$7FFF: 8 KB PRG-RAM bank, (optional)
  * CPU $8000-$BFFF: 16 KB PRG-ROM bank, either switchable or fixed to the first bank
  * CPU $C000-$FFFF: 16 KB PRG-ROM bank, either fixed to the last bank or switchable
  * PPU $0000-$0FFF: 4 KB switchable CHR bank
  * PPU $1000-$1FFF: 4 KB switchable CHR bank



Through writes to the MMC1 control register, it is possible for the program to swap the fixed and switchable PRG-ROM banks or to set up 32 KB PRG bankswitching (like [BNROM](INES_Mapper_034.xhtml "BNROM")), but most games use the default setup, which is similar to that of [UxROM](UxROM.xhtml "UxROM"). 

## Interface

Unlike almost all other mappers, the MMC1 is configured through a serial port in order to reduce its pin count. CPU $8000-$FFFF is connected to a common shift register. Writing a value with bit 7 set ($80 through $FF) to any address in $8000-$FFFF clears the shift register to its initial state. To change a register's value, the CPU writes five times with bit 7 clear and one bit of the desired value in bit 0 (starting with the low bit of the value). On the first four writes, the MMC1 shifts bit 0 into a shift register. On the fifth write, the MMC1 copies bit 0 and the shift register contents into an internal register selected by bits 14 and 13 of the address, and then it clears the shift register. Only on the fifth write does the address matter, and even then, only bits 14 and 13 of the address matter because the mapper doesn't see the lower address bits (similar to the mirroring seen with [PPU registers](PPU_registers.xhtml "PPU registers")). After the fifth write, the shift register is cleared automatically, so writing again with bit 7 set to clear the shift register is not needed. 

### Examples

#### Shift register

To switch a bank, a program will execute code similar to the following: 
    
    
    ;
    ; Sets the switchable PRG-ROM bank to the value of A.
    ;
                  ;  A          MMC1_SR  MMC1_PB
    setPRGBank:   ;  000edcba    10000             Start with an empty shift register (SR).  The 1 is used
      sta $E000   ;  000edcba -> a1000             to detect when the SR has become full.
      lsr a       ; >0000edcb    a1000
      sta $E000   ;  0000edcb -> ba100
      lsr a       ; >00000edc    ba100
      sta $E000   ;  00000edc -> cba10
      lsr a       ; >000000ed    cba10
      sta $E000   ;  000000ed -> dcba1             Once a 1 is shifted into the last position, the SR is full.
      lsr a       ; >0000000e    dcba1             
      sta $E000   ;  0000000e    dcba1 -> edcba    A write with the SR full copies D0 and the SR to a bank register
                  ;              10000             ($E000-$FFFF means PRG bank number) and then clears the SR.
      rts
    

But because only the fifth write sets the destination register, the following equivalent (if [obfuscated](Watermarking.xhtml#Instruction_encoding "Watermarking")) subroutine changes the PRG-ROM bank in the same manner: 
    
    
    setPRGBank:
      sta $8765
      lsr a
      sta $FACE
      lsr a
      sta $BA11
      lsr a
      sta $AD2E
      lsr a
      sta $EAD5
      rts
    

#### Reset

To reset the mapper, which clears the shift register and sets the PRG-ROM bank mode to 3 (fixing the last bank at $C000 and allowing the 16 KB bank at $8000 to be switched), one need only do a single write to any ROM address with a 1 in bit 7: 
    
    
    resetMapper:
      lda #$80
      sta $8000
      rts
    

Commonly, reset is done with an increment on a negative value to save 2 bytes. Because increment performs two writes, first writing the old value before the incremented one, it is recommended (such as for wider emulator compatibility) that the increment target a value that is negative on both writes, such as the INC opcode ($EE) itself: 
    
    
    resetMapper:
      inc resetMapper
      rts
    

Note that some games do a reset write to each of the 4 registers, even though it is only necessary to do 1 reset write to any register. It is suspected these games do this because of discrete logic implementations of MMC1 on development boards such as the MMC MULTI CHECKER-02, which uses separate shift registers for each MMC1 register.[1]

### Consecutive-cycle writes

When the serial port is written to on consecutive cycles, it ignores every write after the first. In practice, this only happens when the CPU executes read-modify-write instructions, which first write the original value before writing the modified one on the next cycle.[2] This restriction only applies to the data being written on bit 0; the bit 7 reset is never ignored. _Bill & Ted's Excellent Adventure_ does a reset by using INC on a ROM location containing $FF and requires that the $00 write on the next cycle is ignored. _Shinsenden_ , however, uses illegal instruction $7F (RRA abs,X) to set bit 7 on the second write and will crash after selecting the みる (look) option if this reset is ignored.[3] This write-ignore behavior appears to be intentional and is believed to ignore all consecutive write cycles after the first even if that first write does not target the serial port.[4]

## Registers

The following description generally applies to all boards mounting an MMC1 ASIC; refer to the connection variants section on how some registers bits have different meanings on particular circuit boards. 

### Load register ($8000-$FFFF)
    
    
    7  bit  0
    ---- ----
    Rxxx xxxD
    |       |
    |       +- Data bit to be shifted into shift register, LSB first
    +--------- A write with bit set will reset shift register
                and write Control with (Control OR $0C), 
                locking PRG-ROM at $C000-$FFFF to the last bank.
    

On consecutive-cycle writes, writes to the shift register (D0) after the first are ignored. See Consecutive-cycle writes for more details. 

### Control (internal, $8000-$9FFF)
    
    
    4bit0
    -----
    CPPMM
    |||||
    |||++- [Nametable](PPU_nametables.xhtml "Nametable") arrangement: (0: one-screen, lower bank; 1: one-screen, upper bank;
    |||               2: horizontal arrangement ("vertical mirroring", PPU A10); 
    |||               3: vertical arrangement ("horizontal mirroring", PPU A11) )
    |++--- PRG-ROM bank mode (0, 1: switch 32 KB at $8000, ignoring low bit of bank number;
    |                         2: fix first bank at $8000 and switch 16 KB bank at $C000;
    |                         3: fix last bank at $C000 and switch 16 KB bank at $8000)
    +----- CHR-ROM bank mode (0: switch 8 KB at a time; 1: switch two separate 4 KB banks)
    

Although [some tests have found that all versions of the MMC1 seems to reliably power on in the last bank](https://forums.nesdev.org/viewtopic.php?t=6766) (by setting the "PRG-ROM bank mode" to 3); [other tests have found that this is fragile](https://forums.nesdev.org/viewtopic.php?f=2&t=14958). Several commercial games have reset vectors every 32 KiB, but not every 16, so evidently PRG-ROM bank mode 2 doesn't seem to occur randomly on power-up. 

### CHR bank 0 (internal, $A000-$BFFF)
    
    
    4bit0
    -----
    CCCCC
    |||||
    +++++- Select 4 KB or 8 KB CHR bank at PPU $0000 (low bit ignored in 8 KB mode)
    

MMC1 can do CHR banking in 4KB chunks. Known carts with CHR-RAM have 8 KiB, so that makes 2 banks. RAM vs ROM doesn't make any difference for address lines. For carts with 8 KiB of CHR (be it ROM or RAM), MMC1 follows the common behavior of using only the low-order bits: the bank number is in effect ANDed with 1. 

### CHR bank 1 (internal, $C000-$DFFF)
    
    
    4bit0
    -----
    CCCCC
    |||||
    +++++- Select 4 KB CHR bank at PPU $1000 (ignored in 8 KB mode)
    

### PRG bank (internal, $E000-$FFFF)
    
    
    4bit0
    -----
    RPPPP
    |||||
    |++++- Select 16 KB PRG-ROM bank (low bit ignored in 32 KB mode)
    +----- MMC1B and later: PRG-RAM chip enable (0: enabled; 1: disabled; ignored on MMC1A)
           MMC1A: Bit 3 bypasses fixed bank logic in 16K mode (0: fixed bank affects A17-A14;
           1: fixed bank affects A16-A14 and bit 3 directly controls A17)
    

The high bit does not select a PRG-ROM bank. MMC1 with 512 KiB was supported by re-using a line from the CHR banking controls. (See below.) 

MMC1A's 'R' bit logic was originally designed to support a hypothetical circuit board that connects the MMC1's A17 output to PRG-RAM A13, meaning a circuit board with 128 KiB of PRG-ROM and 16 KiB of PRG-RAM. Such a circuit board was never produced; games needing 16 KiB of PRG-RAM ended up using the SOROM or SZROM circuit boards that support more than 128 KiB of PRG-ROM and bank-switch PRG-RAM using other means. 

## SxROM connection variants

### SEROM, SHROM and SH1ROM

Boards designed for 32 KiB PRG-ROM (SEROM, SHROM, and SH1ROM), such as _Dr. Mario_ , do not connect PRG A14 to the MMC1; instead A14 is connected directly to the NES. As a result, register $E000 is completely without function, and the entire 32 KiB of PRG-ROM appear unbanked from CPU $8000-$FFFF. For compatibility with these, an emulator may switch to PRG bank 0 at power-on. 

### SNROM

SNROM, which only supports 8 KiB of CHR-ROM/-RAM, uses the upper CHR bank select line coming out of the mapper (CHR A16; bit 4 of bank number) as an additional chip enable for the PRG-RAM.[[1]](http://forums.nesdev.org/viewtopic.php?t=7045)

#### CHR bank 0 (internal, $A000-$BFFF)
    
    
    4bit0
    -----
    ExxxC
    |   |
    |   +- Select 4 KB CHR-RAM bank at PPU $0000 (ignored in 8 KB mode)
    +----- PRG-RAM disable (0: enable, 1: open bus)
    

#### CHR bank 1 (internal, $C000-$DFFF)
    
    
    4bit0
    -----
    ExxxC
    |   |
    |   +- Select 4 KB CHR-RAM bank at PPU $1000 (ignored in 8 KB mode)
    +----- PRG-RAM disable (0: enable, 1: open bus) (ignored in 8 KB mode)
    

### SOROM, SUROM and SXROM

All three address only 8 KiB of CHR-ROM/-RAM. They repurpose the upper CHR lines to increase the PRG-RAM size from 8 to 16 KiB (SOROM) or 32 KiB (SXROM), and to increase the addressable PRG-ROM size from 256 to 512 KiB (SUROM and SXROM). 

#### CHR bank 0 (internal, $A000-$BFFF)
    
    
    4bit0
    -----
    PSSxC
    ||| |
    ||| +- Select 4 KB CHR-RAM bank at PPU $0000 (ignored in 8 KB mode)
    |++--- Select 8 KB PRG-RAM bank
    +----- Select 256 KB PRG-ROM bank
    

#### CHR bank 1 (internal, $C000-$DFFF)
    
    
    4bit0
    -----
    PSSxC
    ||| |
    ||| +- Select 4 KB CHR-RAM bank at PPU $1000 (ignored in 8 KB mode)
    |++--- Select 8 KB PRG-RAM bank (ignored in 8 KB mode)
    +----- Select 256 KB PRG-ROM bank (ignored in 8 KB mode)
    

The SOROM board only implements the upper `S` bit, while the SUROM board only implements the `P` bit. For SXROM, the upper `S` (bit 3) selects the SRAM's A14, and the lower `S` (bit 2) selects A13[5]. 

The 256 KB PRG bank selection applies to all the PRG area, _including the normally "fixed" bank_. It is therefore sometimes described as an "outer bank". 

In 4KB CHR bank mode, SNROM's `E` bit and SO/U/XROM's `P` and `S` bits in both CHR bank registers must be set to the same values, or the PRG-ROM and/or RAM will be bankswitched/enabled as the PPU renders. As there is not much of a reason to use 4 KB bankswitching with CHR-RAM, it is wise for programs to just set 8 KB bankswitching mode in the [Control register](MMC1.xhtml#Control_\(internal,_%248000-%249FFF\) "MMC1"). 

### SZROM

SZROM addresses only 64 KiB of CHR-ROM/-RAM and uses the remaining CHR line to increase the PRG-RAM size from 8 to 16 KiB. 

#### CHR bank 0 (internal, $A000-$BFFF)
    
    
    4bit0
    -----
    RCCCC
    |||||
    |++++- Select 4 KB CHR-ROM bank at PPU $0000 (low bit ignored in 8 KB mode)
    +----- Select 8 KB PRG-RAM bank
    

#### CHR bank 1 (internal, $C000-$DFFF)
    
    
    4bit0
    -----
    RCCCC
    |||||
    |++++- Select 4 KB CHR-ROM bank at PPU $1000 (ignored in 8 KB mode)
    +----- Select 8 KB PRG-RAM bank (ignored in 8 KB mode)
    

SZROM behaves similarly to SOROM, except that the PRG-RAM bank is a controlled by a different bit, and enough CHR is present that 4K bankswitching is desirable. The only currently-known game on SZROM is _A Ressha de Ikou_. 

SZROM can be detected by a NES2.0 header specifying 8 KiB of PRG-RAM, 8 KiB of PRG-NVRAM, and 16 KiB or more of CHR. 

### Summary

The connection variants of SxROM that apply to registers $A000 and $C000 can be amalgamated as follows: 
    
    
    4bit0
    -----
    EDCBA
    |||||
    ||||+- CHR A12
    |||+-- CHR A13 if CHR >= 16 KiB
    ||+--- CHR A14 if CHR >= 32 KiB; and PRG-RAM A13 if PRG-RAM size is 32 KiB
    |+---- CHR A15 if CHR >= 64 KiB; and PRG-RAM A13 if PRG-RAM size is 16 KiB
    |                                or PRG-RAM A14 if PRG-RAM size is 32 KiB
    +----- CHR A16 if CHR = 128 KiB; and PRG-ROM A18 if PRG-ROM size is 512 KiB
    

The `E` bit also acts as a PRG-RAM disable for SNROM (PRG-ROM size <= 256 KiB, CHR-RAM size = 8 KiB, PRG-RAM size = 8 KiB), though this is merely for write protection and not strictly required for compatible emulation. 

The `D` bit used by SOROM (16k PRG-RAM) and SXROM (32 KiB PRG-RAM) controls a different address line depending on the board type. Using A13 for `D` with both boards and A14 for `C` will work, but will break SXROM battery save file compatibility with standard implementations. 

## 2ME

This board, used with the [Famicom Network System](Family_Computer_Network_System.xhtml "Famicom Network System") (FCNS), features 64 big-endian 16-bit words of EEPROM in addition to up to 32 KB of battery-backed PRG-RAM. Because FCNS cartridges are not on the PPU bus, all PPU-related MMC1 outputs are repurposed for EEPROM and PRG-RAM storage. The EEPROM's data output is mapped to $6000-7FFF bit 0 when enabled, and its command and data interface can be found in the 93LC46 datasheet. Note that open bus for this board is card open bus, which is open bus on the card side of the FCNS, not the console side. 

2ME is assigned [NES 2.0 MMC1 submapper 6](NES_2_0_submappers.xhtml#001:_MMC1 "NES 2.0 submappers") and uses an MMC1B. 

#### Control (internal, $8000-9FFF)
    
    
    7  bit  0
    ---- ----
    ...I PPEE
       | ||||
       | ||++- EEPROM CS (01 = enabled, otherwise disabled)
       | ++--- PRG-ROM bank mode (see [standard register definition](MMC1.xhtml#Control_\(internal,_%248000-%249FFF\) "MMC1"))
       +------ EEPROM DI enable (0 = DI forced to 0, 1 = DI output enabled)
    

The graphics-related bits here are repurposed for EEPROM enables. Because the MMC1's PPU A12-10 inputs are grounded on the 2ME board, settings that would normally vary based on PPU fetches such as the nametable arrangement and active CHR register are instead fixed. 

  * The CIRAM A10 output is used as the EEPROM's CS input and will be always true in upper-bank single screen arrangement and always false otherwise.
  * The lowest CHR register bit is used for the EEPROM's DI input, but can only be non-zero in 4 KB banking mode. The second CHR register ($C000-DFFF) is unused in this mode because the PPU inputs are fixed such that they'll never point into the upper half of the pattern table space.



#### CHR bank 0 (internal, $A000-BFFF)
    
    
    7  bit  0
    ---- ----
    ...O RRCI
       | ||||
       | |||+- EEPROM DI
       | ||+-- EEPROM CLK
       | |+--- PRG-RAM A13
       | +---- PRG-RAM A14
       +------ PRG-RAM /CE, and EEPROM DO +OE (0 = PRG-RAM enabled, 1 = EEPROM DO enabled)
    

  * EEPROM DI allows sending commands or data to the EEPROM. It is forced to 0 if 4 KB banking is not enabled via $8000 bit 4.
  * EEPROM CLK is used to transition to the next bit when reading from or writing to the EEPROM.
  * PRG-RAM banking uses the same bit order as SXROM.
  * The PRG-RAM /CE and EEPROM DO +OE bit acts as a selector for $6000-7FFF between PRG-RAM access and EEPROM read. The PRG-RAM enable matches the behavior of SNROM. While EEPROM is selected, bits 7-1 always return card open bus and bit 0 returns any data currently outputted by the EEPROM or card open bus otherwise.



#### PRG bank (internal, $E000-FFFF)
    
    
    7  bit  0
    ---- ----
    ...D PPPP
       | ||||
       | ++++- PRG-ROM bank (see [standard register definition](MMC1.xhtml#PRG_bank_\(internal,_%24E000-%24FFFF\) "MMC1"))
       +------ $6000-7FFF disable (0 = enabled, 1 = disabled)
    

If $6000-7FFF is disabled, then neither PRG-RAM nor EEPROM can be accessed through this region and reads return card open bus. EEPROM can still be written, however, because the DI input does not use this region. 

## SxROM board types

The following SxROM boards are known to exist: 

Board | PRG-ROM | PRG-RAM | CHR | Comments   
---|---|---|---|---  
[SAROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SAROM) | 64 KB | 8 KB | 16 / 32 / 64 KB ROM | NES only   
[SBROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SBROM) | 64 KB |  | 16 / 32 / 64 KB ROM | NES only   
[SCROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SCROM) | 64 KB |  | 128 KB ROM | NES only   
[SC1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SC1ROM) | 64 KB |  | 128 KB ROM | Uses [7432](7432.xhtml "7432") for 28-pin CHR-ROM   
[SEROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SEROM) | 32 KB |  | 16 / 32 / 64 KB ROM |   
[SFROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SFROM) | 128 / 256 KB |  | 16 / 32 / 64 KB ROM |   
[SF1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SF1ROM) | 256 KB |  | 64 KB ROM | [PRG uses standard 32-pin EPROM pinout](http://forums.nesdev.org/viewtopic.php?p=139126#p139126)  
[SFEXPROM](https://nescartdb.com/profile/view/745) | 256 KB |  | 64 KB ROM | [Patches PRG at runtime](http://forums.nesdev.org/viewtopic.php?t=1371) to correct a bad mask ROM run.   
[SGROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SGROM) | 128 / 256 KB |  | 8 KB RAM/ROM |   
[SHROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SHROM) | 32 KB |  | 128 KB ROM | NES only   
[SH1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SH1ROM) | 32 KB |  | 128 KB ROM | Uses [7432](7432.xhtml "7432") for 28-pin CHR-ROM   
[SIROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SIROM) | 32 KB | 8 KB | 16 / 32 / 64 KB ROM | Japan Only   
[SJROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SJROM) | 128 / 256 KB | 8 KB | 16 / 32 / 64 KB ROM |   
[SKROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SKROM) | 128 / 256 KB | 8 KB | 128 KB ROM |   
[SLROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SLROM) | 128 / 256 KB |  | 128 KB ROM |   
[SL1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SL1ROM) | 64 / 128 / 256 KB |  | 128 KB ROM | Uses [7432](7432.xhtml "7432") for 28-pin CHR-ROM   
[SL2ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SL2ROM) | 128 / 256 KB |  | 128 KB ROM | [CHR uses standard 32-pin EPROM pinout](http://forums.nesdev.org/viewtopic.php?t=12657)  
[SL3ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SL3ROM) | 256 KB |  | 128 KB ROM | Uses [7432](7432.xhtml "7432") for 28-pin CHR-ROM   
[SLRROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SLRROM) | 128 / 256 KB |  | 128 KB ROM | Difference from SLROM unknown   
[SMROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SMROM) | 256 KB |  | 8 KB RAM | Japan Only   
[SNROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SNROM) | 128 / 256 KB | 8 KB | 8 KB RAM/ROM |   
[SOROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SOROM) | 128 / 256 KB | 16 KB | 8 KB RAM/ROM |   
[STROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=STROM) | Not MMC1, actually [NROM](NROM.xhtml "NROM")  
[SUROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SUROM) | 512 KB | 8 KB | 8 KB RAM/ROM |   
[SXROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=SXROM) | 128 / 256 / 512 KB | 32 KB | 8 KB RAM/ROM | Japan Only   
[SZROM](https://www.crazysmart.net.au/phpBB3/viewtopic.php?f=10&t=8&p=9#p9) | 128 / 256 KB | 16 KB | 16-64 KB ROM | Japan Only   
  
  * SLxROM boards are functionally identical to SLROM, but with different chip pinouts. Some of them have an additional [74HC32](7432.xhtml "7432") chip to combine PPU /RD and PPU /A13 into a single enable signal for the CHR-ROM chip that has only 28 pins.
  * SMROM is functionally identical to SGROM, but features two 128 KB PRG-ROM chips instead of one 256 KB. Only a very early MMC1 game in Japan, _[Hokkaidou Rensa Satsujin: Ohoutsuku ni Shouyu](https://nescartdb.com/profile/view/3777)_ , is known to have used this board, and one of very few Nintendo-made boards which combine smaller ROM chips to get a bigger ROM.
  * One SNROM game for Famicom, _[Morita Shogi](https://nescartdb.com/profile/view/3479)_ , uses an 8 KiB CHR-ROM instead of CHR-RAM. The [6264](6264_static_RAM.xhtml "6264 static RAM") pinout is nearly identical to the pinout of an 8 KiB [mask ROM](Mask_ROM_pinout.xhtml "Mask ROM pinout"), except for pins 26 and 27. On the 6264, these are a positive chip enable (CS2) and negative write enable (/WE) respectively; on the mask ROM, they may be additional positive chip enables. Either way, they're high during reads.



Solder pad config (SAROM, SJROM, SKROM, SNROM, SUROM, and SXROM only)
    

  * PRG-RAM retaining data : 'SL' disconnected, Battery, D1, D2, R1 R2 and R3 present.
  * PRG-RAM not retaining data : 'SL' connected, leave slots for Battery, D1, D2, R1, R2 and R3 free.



Even if the SOROM and SZROM boards utilizes a battery, it is connected to only one PRG-RAM chip. The first RAM chip will not retain its data, but the second one will. 

## ASIC Revisions

At least seven different versions of the MMC1 are known to exist: MMC1, MMC1A, MMC1B1, MMC1B1-H, MMC1B2, MMC1B2F, MMC1B3. The known differences relate to bit 4 of $E000. 

MMC1A
    PRG-RAM is always enabled at $6000-$7FFF. Bit 4 of $E000 causes bit 3 to directly control PRG-ROM A17 instead of going through the fixed bank logic.
MMC1B
    PRG-RAM is enabled by default but can by disabled by bit 4 of $E000. $E000 bit 3 never bypasses the fixed bank.

The MMC1 most commonly exists in a [24-pin shrink-DIP package](MMC1_pinout.xhtml "MMC1 pinout"). An SOIC-24 incarnation MMC1B2F has been observed inside of a JRA-PAT Famicom Network System card. 

Boards using an MMC1 may contain a battery connected to the PRG-RAM's power line to preserve the data. Boards doing so will allow extra circuitry to be used, with 2 diodes and 2 resistors. A diode is needed from both voltage sources: The battery and the NES 5V, so that one cannot supply current to the other, and there is a resistor in series with the battery as part of UL compliance. A pull-down resistor is needed on the CE line so that the SRAM is disabled when the MMC1 isn't powered. Finally, the battery powered SRAMs have an additional larger decoupling capacitor to make sure voltage transitions are smooth. Very early NES-SNROM-03 and lower revisions lacks that capacity, and saves are lost much more easily on those boards. 

Nintendo transitioned from the original MMC1 (manufactured by ROHM) to the MMC1A (manufactured probably by Ricoh) around the 39th week of 1988. (Based on comparison of otherwise identical SMB/DH/WCTM carts from 38th and 39th weeks of '88). Two games ([1](https://nescartdb.com/profile/view/3736), [2](https://nescartdb.com/profile/view/3458)) are known to rely on MMC1A behavior, writing values between $10 and $1F to the PRG bank register, write to WRAM at $6000-$7FFF, and expect the write to succeed. 

AX5904 is a third-party clone of the MMC1A. 

## References

  1. ↑ [Forum thread:](https://forums.nesdev.org/viewtopic.php?t=25065) MMC MULTI CHECKER-02 development board
  2. ↑ [6502_cpu.txt](http://nesdev.org/6502_cpu.txt). See the section labelled Instruction Timing, subsections Absolute addressing, Read-Modify-Write instructions
  3. ↑ [Forum thread:](https://forums.nesdev.org/viewtopic.php?t=23730) _Shinsenden_ crash and MMC1 reset findings
  4. ↑ [Forum post:](https://forums.nesdev.org/viewtopic.php?p=91562#p91562) MMC1 write investigation
  5. ↑ [Forum post:](http://forums.nesdev.org/viewtopic.php?t=4596) Tracing the SXROM PCB



## See also

  * [Programming MMC1](Programming_MMC1.xhtml "Programming MMC1")
  * [MMC1 pinout](MMC1_pinout.xhtml "MMC1 pinout")
  * ["Nintendo MMC1 info for 8-bit NES carts" by Matthew J. Richey](http://nesdev.org/mmc1.txt)
  * [MMC1 doc by Kevin Horton](http://kevtris.org/mappers/mmc1/index.html)
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information on SUROM/SXROM carts (512 KiB carts, 1024 KiB carts) is inaccurate
  * [US Patent 4,949,298](https://patents.google.com/patent/US4949298A/en)
  * [Forum topic "MMC1A fixed bank behavior difference" with test ROM, January 2022](https://forums.nesdev.org/viewtopic.php?t=23619)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with large PRG RAM](Category_Mappers_with_large_PRG_RAM.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
