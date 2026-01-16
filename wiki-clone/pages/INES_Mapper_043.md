# INES Mapper 043

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_043) | View [other pages](Special_AllPages.xhtml#INES_Mapper_043)

**iNES Mapper 043** is used for the **TONY-I** and **YS-612** circuit boards, both containing conversions of _Super Mario Brothers 2_ (Japanese) from Famicom Disk System to ROM cartridge. There are two 32 KiB, one 2 KiB and one 8 KiB PRG-ROM chip. iNES-format ROM images first include the data of both 32 KiB ROM chips, then the data of the 2 KiB chip repeated four times, then the data of the 8 KiB ROM chip, for a total of 80 KiB of PRG-ROM. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Bank Select ($4022)
    * 2.2 IRQ Control ($4122 on TONY-I, $8122 on YS-612)
  * 3 Notes



# Banks

  * CPU $5000-$5FFF: 2 KiB PRG-ROM bank, repeated once, from 2 KiB PRG-ROM chip
  * CPU $6000-$7FFF: 8 KiB PRG-ROM bank, fixed to #2, from 2x32 KiB PRG-ROM chips
  * CPU $8000-$9FFF: 8 KiB PRG-ROM bank, fixed to #1, from 2x32 KiB PRG-ROM chips
  * CPU $A000-$BFFF: 8 KiB PRG-ROM bank, fixed to #0, from 2x32 KiB PRG-ROM chips
  * CPU $C000-$DFFF: 8 KiB PRG-ROM bank, switchable, from 2x32 KiB PRG-ROM chips
  * CPU $E000-$FFFF: 8 KiB PRG-ROM bank from 8 KiB PRG-ROM chip
  * PPU $0000-$1FFF: unbanked 8 KiB CHR-ROM



# Registers

## PRG Bank Select ($4022)
    
    
    Mask: $71FF
    
    Bit 7654 3210
        ---------
        .... .CCC
              +++- Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF.
    

The actual bank number is: 
    
    
    Value  Bank#
    ------------
    0      4
    1      3
    2      4
    3      4
    4      4
    5      7
    6      5
    7      6
    

## IRQ Control ($4122 on TONY-I, $8122 on YS-612)
    
    
    Mask: $71FF?
    
    Bit 7654 3210
        ---------
        .... ...I
                +- 0: Acknowledge and disable IRQ, reset counter
                   1: Enable IRQ
    

When enabled, the 12-bit IRQ counter increases on every M2 cycle until it overflows, upon which an IRQ is fired. 

# Notes

  * FCEUX emulates a multicart extract of _Mr. Mary 2_ under this mapper number. Since that particular SMB2J conversion was never released in single-cartridge form, its description of is part of the [multicarts' description](NES_2_0_Mapper_357.xhtml "NES 2.0 Mapper 357").
  * A modified iNES ROM file of YS-612 has different title screen variants in the four representations of the 2 KiB ROM chip. Nintendulator implements a dialog to switch between two of them.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml)
