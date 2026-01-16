# NES 2.0 Mapper 368

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_368) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_368)

[![](../wiki-images/YUNG8-front.jpg)](File_YUNG8_front_jpg.xhtml)

[](File_YUNG8_front_jpg.xhtml "Enlarge")

PCB front

[![](../wiki-images/YUNG8-back.jpg)](File_YUNG8_back_jpg.xhtml)

[](File_YUNG8_back_jpg.xhtml "Enlarge")

PCB back (mirrored, pin 31 on the left)

NES 2.0 Mapper 368 denotes the Yung-08 cartridge conversion PCB of the disk system game _Super Mario Bros. 2 (Japan)_. It is the "parent" of the [N-32](INES_Mapper_050.xhtml "INES Mapper 050") and [Mr. Mary 2](NES_2_0_Mapper_357.xhtml "NES 2.0 Mapper 357") conversions, as both retain (jumped-over) protection-reading code from this version. 

There are two 32 KiB PRG-ROM chips, one 2 KiB PRG-ROM, and one 8 KiB CHR-ROM chip. The ROM image's 128 KiB PRG-ROM part is arranged as follows: 

  1. Offset $00000: First PRG-ROM chip (32 KiB)
  2. Offset $08000: Second PRG-ROM chip (32 KiB)
  3. Offset $10000: Third PRG-ROM chip, repeated 32 times (64 KiB)



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Bank Select ($4022, write)
    * 2.2 Latch write/IRQ Control ($4122, write)
    * 2.3 Latch read ($4122, read)



# Banks

  * CPU $6000-$7FFF: 8 KiB PRG-ROM bank, fixed to #2
  * CPU $8000-$9FFF: 8 KiB PRG-ROM bank, fixed to #1
  * CPU $A000-$BFFF: 8 KiB PRG-ROM bank, fixed to #0
  * CPU $C000-$DFFF: 8 KiB PRG-ROM bank, switchable
  * CPU $E000-$FFFF: 8 KiB PRG-ROM bank, fixed to #8 (2 KiB PRG-ROM repeated four times)
  * PPU $0000-$1FFF: unbanked 8 KiB CHR-ROM



# Registers

## PRG Bank Select ($4022, write)
    
    
    Mask: $F1FF
    
    Bit 7654 3210
        ---------
        .... .CCC
              +++- Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF
    

The actual bank number is: 
    
    
    Value  Bank#
    ------------
    0      4
    1      3
    2      5
    3      3
    4      6
    5      3
    6      7
    7      3
    

## Latch write/IRQ Control ($4122, write)
    
    
    Mask: $F1FF
    
    D~7654 3210
      ---------
      ..LL .L.I
        ||  | +- 0: Acknowledge and disable IRQ, reset counter
        ||  | |  1: Enable IRQ
        ++--+-+- Bits to be read-back
    

When enabled, the 12-bit IRQ counter increases on every M2 cycle until it overflows, upon which an IRQ is fired. 

## Latch read ($4122, read)
    
    
    Mask: $F1FF
    
    D~7654 3210
      ---------
      10LL 1L1L
        ++--+-+- Latched bits
    

When holding the A button while pressing START to begin in World A-1, the game checks the latch's functionality and returns to the title screen if the check fails. 

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml)
