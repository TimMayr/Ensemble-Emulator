# INES Mapper 147

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_147) | View [other pages](Special_AllPages.xhtml#INES_Mapper_147)

INES Mapper 147 is used to denote the Sachen 3018 board, used by one game: _Chinese Kungfu: 少林武者_ (TC-011) and its Western localization _Challenge of the Dragon_ (not to be confused with the Color Dreams game of the same name). The original 60-pin Chinese version has 128 KiB of PRG-ROM and 128 KiB of CHR-ROM, while the 72-pin Western localization only has 64 KiB of PRG-ROM and 128 KiB of CHR-ROM. 

## Contents

  * 1 Banks
  * 2 Registers (60-pin release)
  * 3 Registers (72-pin release)
  * 4 Notes
  * 5 See also



## Banks

  * CPU $8000-$FFFF: 32 KiB switchable PRG ROM bank
  * PPU $0000-$1FFF: 8 KiB switchable CHR ROM bank



## Registers (60-pin release)

The original 60-pin release uses a custom IC (marked "JV001") serving as a latch, adder and inverter. There are five registers: Input (6 bits), Output (6 bits), Register (6 bits), Mode (1 bit) and Invert (1 bit). 
    
    
    Mask: $E103
    Read $4100-$4103: [RRRR RR..]: Read Register (with its bits 0-5), connected to the CPU data bus bits 2-7. Register bits 4-5 (CPU bits 6-7) are inverted if Invert==1. CPU bits 0-1 are open bus.
    Write $4100: When Mode==0: Bits 0-5 of Register := Input, bits 0-3 being inverted if Invert==1.
                 When Mode==1: Bits 0-3 of Register incremented by one, bits 4-5 unaffected.
    Write $4101: Invert := Written value bit 2.
    Write $4102: Input := Written value bits 2-7.
    Write $4103: Mode := Written value bit 2.
    Write $8000-$FFFF: Output := Register; written value is ignored.
    

The six Output bits have the following functions: 
    
    
    Written CPU bit:       76543210
    Internal Register bit: 543210..
                           --------
                           ||||||
                           +----+-- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF (PRG A15-A16)
                            ++++--- Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF (CHR A13-A16)
    

The two PRG-ROM bits are split up in this way due to board topology: the lower bit goes directly to the A15 pin of the two 64 KiB PRG-ROM chips, while the higher bit selects which 64 KiB PRG-ROM chip is enabled. 

## Registers (72-pin release)

The 72-pin release of the game has only 64 KiB instead of 128 KiB of PRG-ROM, and uses a simple data latch instead of the JV001 ASIC, with the data bits having compatible functions as the original value written to $4102 on the JV001 IC: 
    
    
     Mask: $4103
     Bus conflicts in the mirror at $C102-$FFFE
     $4102: [PCCC CP..] - Select 32 KiB PRG bank and 8 KiB CHR bank
    

## Notes

  * Despite the hardware differences between the 60- and 72-pin versions of the game, both a more complex emulation of the JV001 chip as well as a simple emulation of the data latch will work with both versions' ROM images, as the 60-pin version neither uses the adder or inverter functionalities of the JV001 chip nor checks the result available from reading $4100, and the 72-pin version retains the complete $4101/$4102/$4103 write sequence that would be necessary to operate the JV001 IC.
  * The GoodNES 3.23b ROM image of _Chinese Kungfu: 少林武者_ is defective despite being tagged with [!]: the wrong CHR banks are selected during attract mode and in the second area of gameplay. A good redump is available.



## See also

  * [PCB images](http://forums.nesdev.org/viewtopic.php?f=3&t=15961&p=213357#p213357)


