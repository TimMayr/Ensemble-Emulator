# INES Mapper 034

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_034) | View [other pages](Special_AllPages.xhtml#INES_Mapper_034)

**iNES Mapper 034** denotes the unrelated NINA-001/NINA-002 and BNROM circuit boards. The NES 2.0 Submapper differentiates the two: 

  * **Submapper 1** denotes the NINA-001/NINA-002 circuit board,
  * **Submapper 2** denotes the BNROM circuit board.



In the absence of valid submapper information, mapper 34 .NES files should be considered BNROM when the CHR-ROM size is 0-8 KiB, and NINA-001/NINA-002 when the CHR-ROM size is above 8 KiB. 

## Contents

  * 1 NINA-001/NINA-002
    * 1.1 Banks
    * 1.2 Registers
      * 1.2.1 PRG Bank Select ($7FFD, write)
      * 1.2.2 CHR Bank Select 0 ($7FFE, write)
      * 1.2.3 CHR Bank Select 1 ($7FFF, write)
    * 1.3 Hardware
  * 2 BNROM
    * 2.1 Banks
    * 2.2 Register: Bank Select ($8000-$FFFF, write)
    * 2.3 See also



# NINA-001/NINA-002

**NINA-001**

**Company** | American Video Entertainment   
---|---  
**Games** | [1 in NesCartDB](https://nescartdb.com/search/advanced?ines=34)  
**Complexity** | Discrete logic   
**Boards** | NINA-001   
**PRG ROM capacity** | 64K   
**PRG ROM window** | 32K   
**PRG RAM capacity** | 8K   
**PRG RAM window** | n/a   
**CHR capacity** | 64K   
**CHR window** | 4K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 034  
  
**NESCartDB**

[NINA-001](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=AVE-NINA-01)  
---  
[NINA-002](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=AVE-NINA-02)  
  
The NINA-001 PCB supports 64 KiB of PRG-ROM capacity and uses a microcontroller to disable the [CIC](CIC_lockout_chip.xhtml "CIC"). The NINA-002 PCB increases PRG-ROM capacity to 128 KiB and uses a stun circuit to disable the CIC. 

## Banks

  * CPU $6000-$7FFF: 8 KiB of unbanked PRG-RAM.
  * CPU $8000-$FFFF: 32 KiB window into 64 KiB (NINA-001)/128 KiB (NINA-002) of PRG-ROM
  * PPU $0000-$0FFF: 4 KiB window into 64 KiB of CHR-ROM
  * PPU $1000-$1FFF: 4 KiB window into 64 KiB of CHR-ROM
  * Nametable arrangement: Fixed vertical arrangement ("Horizontal mirroring")



## Registers

The registers overlapping PRG-RAM at the same addresses mean that reading the register's address returns the last value written to the PRG-RAM, which is also the last value written to the register. 

### PRG Bank Select ($7FFD, write)
    
    
    D~[.... ..BA] A~[0111 1111 1111 1101]
              ++- PRG A16..A15 (32 KiB bank)
    

The power-on value is undefined; games should have a reset vector and handler in all PRG-ROM banks. 

### CHR Bank Select 0 ($7FFE, write)
    
    
    D~[.... DCBA] A~[0111 1111 1111 1110]
            ++++- CHR A15..A12 (4 KiB bank) at PPU $0000
    

### CHR Bank Select 1 ($7FFF, write)
    
    
    D~[.... DCBA] A~[0111 1111 1111 1111]
            ++++- CHR A15..A12 (4 KiB bank) at PPU $1000
    

## Hardware

The NINA-001 board contains a 74LS133 (13-input NAND gate), 74LS74 (dual D flip-flop), 74HCT139 (dual 1-4 decoder), 74LS00 (quad 2-input NAND gate), two 74HCT173s (4-bit tristate D flip-flop), as well as a microcontroller labelled NINA to act as a [CIC](CIC_lockout_chip.xhtml "CIC") defeat mechanism. These 4 registers reside "on top" of PRG RAM: each write to the register goes both to the register and to the RAM location at the same address. Thus, reading the register's address returns the last value written to the RAM, which is also the last value written to the register. 

One 2-NAND and one half of the 74'139 together generate /RAMSEL. Another 2-NAND inverts that to form +RAMSEL. A third 2-NAND inverts R/W to produce +WR. The 13-NAND combines +RAMSEL, +WR, and A12…A2 to generate /REGSEL. The other half of the 74'139 uses /REGSEL, A1, and A0 to generate the latching signals for the three flip-flops. 

The final 2-NAND inverts PPU A12. PPU A12 and PPU /A12 are fed to the two 74'173s /OE inputs, implementing a simple quad 1-of-2 multiplexer. 

A **NINA-002** board exists which increased the PRG mask ROM size to 128KB and dropped the NINA microcontroller in favor of a [CIC stun circuit](CIC_lockout_chip.xhtml#Defeating "CIC lockout chip"). It is otherwise identical in function to the NINA-001 board as no other game was released on this board with >64KB PRG ROM. 

This hardware has a trivial oversize definition that supports 8 MiB PRG and 1 MiB CHR, made by replacing the three flip-flops with 74'374s. 

# BNROM

**BNROM**

**Company** | Irem, Nintendo   
---|---  
**Games** | [2 in NesCartDB](https://nescartdb.com/search/advanced?ines=34)  
**Complexity** | Discrete logic   
**Boards** | I-IM, BNROM   
**PRG ROM capacity** | 128K   
**PRG ROM window** | 32K   
**PRG RAM capacity** | None   
**CHR capacity** | 8K   
**CHR window** | n/a   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Yes   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 034, [241](INES_Mapper_241.xhtml "INES Mapper 241"), [177](INES_Mapper_177.xhtml "INES Mapper 177")  
  
**NESCartDB**

[BNROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=BNROM)  
---  
  
## Banks

  * CPU $8000-$FFFF: 32 KiB window into 128 KiB of PRG-ROM
  * PPU $0000-$1FFF: 8 KiB of unbanked CHR-RAM/-ROM
  * Nametable arrangement: Fixed; solder pad selects between Horizontal and Vertical



## Register: Bank Select ($8000-$FFFF, write)
    
    
    D~[.... ..BA] A~[1... .... .... ....]
              ++- PRG A16..A15 (32 KiB bank)
    

  * The power-on value is undefined; games should have a reset vector and handler in all PRG-ROM banks.
  * The original BNROM board is always subject to AND-type bus conflicts: the effective value is the value being written bitwise-AND'd with the PRG-ROM content at the address being written to.



## See also

[iNES Mapper 241](INES_Mapper_241.xhtml "INES Mapper 241") is a variation of BxROM with 8 KiB PRG-RAM at CPU $6000-$7FFF, avoids bus conflicts, and optionally supports an LPC speech chip. 

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
