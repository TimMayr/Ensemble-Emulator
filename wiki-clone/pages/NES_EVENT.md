# NES-EVENT

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES-EVENT) | View [other pages](Special_AllPages.xhtml#NES_EVENT)

**NES-EVENT**

**Company** | Nintendo   
---|---  
**Games** | [1 in NesCartDB](https://nescartdb.com/search/advanced?ines=105)  
**Complexity** | ASIC   
**Boards** | NES-EVENT   
**PRG ROM capacity** | 256K   
**PRG ROM window** | 16K + 16K fixed or 32K   
**PRG RAM capacity** | None   
**CHR capacity** | 8K RAM   
  
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H, V, or 1, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | Yes   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [105](NES_EVENT.xhtml "INES Mapper 105")  
  
**NESCartDB**

[iNES 105](https://nescartdb.com/search/advanced?ines=105)  
---  
  
  
**NES-EVENT** is the [MMC1](MMC1.xhtml "MMC1")-based board used for _Nintendo World Championships_ , a multicart including timed versions of _Super Mario Bros._ , _Rad Racer_ , and _Tetris_. Like [SUROM](SxROM.xhtml "SUROM") and [SXROM](SxROM.xhtml "SXROM"), it repurposes the MMC1 CHR address bits, and MMC1 wired this way is [mapper 105](NES_EVENT.xhtml "INES Mapper 105"). 

The menu music resembles selected tracks from _Hello Kitty World_. 

## Contents

  * 1 Parts
  * 2 Register
  * 3 Disch Notes
  * 4 References



## Parts

  * 256K of PRG ROM as two 27C010 UV EPROMs
  * 8K of CHR RAM
  * 8K of PRG RAM
  * Several 4000-series and 7400-series discrete ICs



## Register

MMC1 register $A000 is mapped like this: 
    
    
    43210
    |||||
    ||||+- Not used (ordinarily selects bottom or top half of CHR RAM)
    |+++-- 0-3: Select 32 KiB bank in $8000-$FFFF from lower 128KB ROM
    |      4-7: Normal MMC1 behavior from upper 128KB ROM
    +----- 0: Run timer
           1: Reset timer
    

The first 32 KiB is hardwired until the timer is started (write 0 then 1 to bit 4) for the first time. 

The other ICs implement a 30-bit up counter clocked by M2 that fires an IRQ when it reaches a high enough value. DIP switches set bits 28-25 of the counter's initial value. This results in a range of roughly 5 to 10 minutes on an NTSC console. 

## Disch Notes
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 105          =
     ========================
     
     aka
     --------------------------
     NES-EVENT
     
     
     Example Game:
     --------------------------
     Nintendo World Championships 1990
     
     
     Notes:
     ---------------------------
     This mapper is an [MMC1](MMC1.xhtml "MMC1") with crazy wiring and a huge 30-bit CPU cycle driven IRQ counter.  Registers are all
     internal and not directly accessable -- and the latch must be written to 1 bit at a time -- just like on a
     normal MMC1.  For details on how regs are written to, see [mapper 001](MMC1.xhtml "INES Mapper 001").
     
     This mapper has 8k CHR-RAM, and it is not swappable.
     
     
     Registers:
     ---------------------------
     
     Note that like a normal MMC1, registers are internal and not accessed directly.
     
     
       $8000-9FFF:   [.... PSMM]  Same as MMC1 (but CHR mode bit isn't used)
     
       $A000-BFFF:   [...I OAA.]
            I = IRQ control / initialization toggle
            O = PRG Mode/Chip select
            A = PRG Reg 'A'
     
       $C000-DFFF:   [.... ....]  Unused
     
       $E000-FFFF:   [...W BBBB]
            W = WRAM disable (same as MMC1)
            B = PRG Reg 'B'
     
     
     
     Powerup / Reset / Initialization:
     ---------------------------
     
       On powerup and reset, the first 32k of PRG (from the first PRG chip) is selected at $8000 *no matter what*.
     PRG cannot be swapped until the mapper has been "initialized" by setting the 'I' bit to 0, then to '1'.  This
     toggling will "unlock" PRG swapping on the mapper.
     
       Note 'I' also controls the IRQ counter (see below)
     
     
     PRG Setup:
     ---------------------------
     
       There are 2 PRG chips, each 128k.  The 'O' bit selects between the chips, and also determines which PRG Reg
     is used to select the page.
     
       O=0:  Use first PRG chip (first 128k), use 'A' PRG Reg, 32k swap
       O=1:  Use second PRG chip (second 128k), use 'B' PRG Reg, MMC1 style swap
     
       In addition, if the mapper has not been "unlocked", the first 32k of the first chip is always selected
     regardless (as if $A000 contained $00).
     
       Modes as listed below:
     
                       $8000   $A000   $C000   $E000
                     +-------------------------------+
     Uninitialized:  |             { 0 }             |  <-- use first 128k
                     +-------------------------------+
     O=0:            |             $A000             |  <-- use first 128k
                     +-------------------------------+
     O=1, P=0:       |            <$E000>            |  <-- use second 128k
                     +-------------------------------+
     O=1, P=1, S=0:  |     { 0 }     |     $E000     |  <-- use second 128k
                     +---------------+---------------+
     O=1, P=1, S=1:  |     $E000     |     {$07}     |  <-- use second 128k
                     +---------------+---------------+
     
     
     
     
     IRQ Counter:
     ---------------------------
     
       The 'I' bit in $A000 controls the IRQ counter.  When cleared, the IRQ counter counts up every cycle.  When
     set, the IRQ counter is reset to 0 and stays there (does not count), and the pending IRQ is acknowledged.
     
       The cart has 4 dipswitches which control how high the counter must reach for an IRQ to be generated.
     
       The IRQ counter is 30 bits wide.. when it reaches the following value, an IRQ is fired:
     
       [1D CBAx xxxx xxxx xxxx xxxx xxxx xxxx]
         ^ ^^^
         | |||
         either 0 or 1, depending on the corresponding dipswitch.
     
     So if all dipswitches are open (use '0' above), the counter must reach $20000000.
     If all dipswitches are closed (use '1' above), the counter must reach $3E000000.
     etc
     
       In the official tournament, 'C' was closed, and the others were open, so the counter had to reach $2800000.
    

Note that: 

  * PPU A12 is still connected to the MMC1 (so the register at $C000 exists, and the bit in $8000 to control CHR banking mode still exists, and writing a value with that bit set would make the register at $C000 relevant), and
  * MMC1 PRG A14 is connected to both PRG ROMs (so writes to $8000 could select either 16+16 banking mode, and then writes to $E000 could cause either the first or second 16K of the first ROM to be mapped at both $8000 and $C000 before the timer was unlocked)



## References

  * [Kevin Horton's reverse engineering of NES-EVENT](http://kevtris.org/mappers/nes_custom/NES_EVENT.html)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
