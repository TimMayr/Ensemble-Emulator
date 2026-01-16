# INES Mapper 132

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_132) | View [other pages](Special_AllPages.xhtml#INES_Mapper_132)

INES Mapper 132 is used to denote three compatible boards used by games from TXC Corporation: 

  * TXC 01-22003-400 PCB: 72 pins with [CIC](CIC_lockout_chip.xhtml "CIC") stun circuit, [TXC 05-00002-010](TXC_05_00002_010_pinout.xhtml "TXC 05-00002-010 pinout") ASIC, max. 64 KiB PRG-ROM, max. 32 KiB CHR-ROM
  * TXC 01-22111-100 PCB: 60 pins, TXC 05-00002-010 ASIC, max. 64 KiB PRG-ROM, max. 32 KiB CHR-ROM
  * TXC 01-22270-000 PCB: 60 pins, TXC 05-00002-010 ASIC or 74LS161 latch, max. 64 KiB PRG-ROM, max. 32 KiB CHR-ROM

Title | Cartridge code | PRG-ROM | CHR-ROM | Notes   
---|---|---|---|---  
棋王 (Qíwáng, Chinese Chess) | MGC-001 | 64 KiB | 32 KiB |   
Creatom | MGC-003 | 64 KiB | 32 KiB |   
小瑪琍 (Xiǎo Mǎlí) | MGC-005 | 16 KiB | 8 KiB | TXC re-release, no bankswitching or copy-protection. Cartridge label titles it _Bingo_.   
麻将方块 (Mahjong Block) | MGC-008 | 32 KiB | 32 KiB | TXC re-release   
Venice Beach Volley | MGC-010 | 32 KiB | 32 KiB | TXC re-release   
Rad Racket - Deluxe Tennis II | MGC-011 | 32 KiB | 32 KiB | TXC re-release   
  
Its UNIF board name is **UNL-22211**. 

## Contents

  * 1 Banks
  * 2 Registers
  * 3 TXC 01-22270-000 PCB with 74LS161 latch
  * 4 Notes
  * 5 Similar Mappers
  * 6 See also



## Banks

  * CPU $8000-$FFFF: 32 KiB switchable PRG ROM bank
  * PPU $0000-$1FFF: 8 KiB switchable CHR ROM bank



## Registers

Mapper 132 uses a custom IC (real number 05-00002-010, often with fake markings) serving as a latch, adder and inverter. There are six registers: "P", "R", and Output (three bits each); "S", inCrement, and and inVert (one bit each). 
    
    
    Mask: $E100
     read $4100: [xxxx SRRR]
                  |||| ||||
                  |||| ++++- Copy internal registers 'RRR' and**'S' XOR 'V'** to data bus.
                  ++++------ open bus
    Mask: $E103
     write $4100: If Increment is set, internal register 'RRR' <- 'RRR'+1
                  Otherwise, if Invert is clear, copy internal register 'PPP' to 'RRR'
                             if Invert is set, copy '~PPP' to 'RRR'
                  'S' is not changed at all.
     write $4101: [.... ...V] - Invert Mode
     write $4102: [.... SPPP] - Copy data bus to internal registers 'S' and 'PPP'.
                                'S' can be read back immediately; 'PPP' must be copied using $4100 first.
     write $4103: [.... ...C] - Increment Mode
    Mask: $8000
     write $8000: copy internal register 'RRR' to PRG A15, CHR A14, and CHR A13 banking pins, in order.
    

Games will also check the lower three or four bits of $4100 for the correct value after several increment and inversion operations as a copy-protection measure. 

## TXC 01-22270-000 PCB with 74LS161 latch

Instead of the 05-00002-010 ASIC, the 01-22270-000 PCB variant can also use a simple 74LS161 latch, which would make the board function as [MHROM](GxROM.xhtml "MHROM"). 

## Notes

  * _戰國四川省_ (Zhànguó Sìchuān Shěng, original version of AVE's _Tiles of Fate_) is set to Mapper 132 in GoodNES 3.23b. That ROM image is actually a mapper hack with the PRG-ROM code unmodified but the CHR-ROM banks rearranged to work as Mapper 132; the correct mapper is [INES Mapper 173](INES_Mapper_173.xhtml "INES Mapper 173"). That mapper hack only works on certain emulators' implementation of Mapper 132, not on the above implementation based on studying the circuit board.
  * The TXC re-release of _小瑪琍_ (Xiǎo Mǎlí), with cartridge code MGC-005, uses the 01-22111-100 board as well, but since it does not use any bankswitching whatsoever, it can be emulated as mapper 0.
  * _麻将方块 (Mahjong Block)_ (TXC re-release, headerless CRC32 0ACFC3CD) is commonly set to [INES Mapper 173](INES_Mapper_173.xhtml "INES Mapper 173"), but since it expects Output bit 1 to select CHR A14, it actually uses INES Mapper 132. Note that the original _Super Mega_ release of the game uses [INES Mapper 172](INES_Mapper_172.xhtml "INES Mapper 172").



## Similar Mappers

  * On games with only 32 KiB PRG-ROM, Mapper 132 is almost identical to [INES Mapper 136](INES_Mapper_136.xhtml "INES Mapper 136") except in the value read from $4100 due to Mapper 136 having four (28-pin JV001 ASIC) versus Mapper 132 having three adder bits (24-pin 05-00002-010 ASIC).
  * On games with only 32 KiB PRG-ROM, [INES Mapper 173](INES_Mapper_173.xhtml "INES Mapper 173") is similar to Mapper 132 except that CHR A14 is taken from the inverse of the Invert bit rather than Output bit 1.
  * [INES Mapper 036](INES_Mapper_036.xhtml "INES Mapper 036") is somewhat similar, using the same 05-00002-010 ASIC. It utilizes CPU bits 4 and 5 instead of 2 for PRG-ROM banking and moves CHR-ROM banking to a separate data latch.



## See also

[PCB images ](http://forums.nesdev.org/viewtopic.php?f=3&t=15961&p=213362#p213362)
