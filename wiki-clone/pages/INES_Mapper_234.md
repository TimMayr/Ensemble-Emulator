# INES Mapper 234

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_234) | View [other pages](Special_AllPages.xhtml#INES_Mapper_234)

iNES Mapper 234 represents the Maxi 15 multicart, which allows the combination of slightly-modified 32k/32k [CNROM](CNROM.xhtml "CNROM") and 64k/64k [NINA-03](NINA_003_006.xhtml "NINA-003-006") games. The only instance of this board was released with 512k/512k, but seems to be laid out in a way that would support a total of 1M/1M in 4 512k ROMs. 

## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 Outer Bank Control ($FF80-$FF9F)
    * 2.2 Lockout defeat control ($FFC0-$FFDF)
    * 2.3 Inner Bank Control ($FFE8-$FFF7)



## Overview

  * PRG ROM size: 512 KiB or 1MiB
  * PRG ROM bank size: 32 KiB inner / 32 or 64 KiB outer
  * PRG RAM: No
  * CHR capacity: 512 KiB or 1MiB ROM
  * CHR bank size: 8 KiB inner / 32 or 64 KiB outer
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper.
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): Yes



## Registers

Typical for the Atari 2600, but strange for the NES: Registers lie at $FF80-$FFF7 but bankswitching happens on the _value that was read_. Bus conflicts are thus avoided by storing the library of desired bankswitch values in ROM. Writes to the registers will produce bus conflicts. 

All registers are cleared on reset. 

### Outer Bank Control ($FF80-$FF9F)
    
    
    7  bit  0
    MOQq BBBb  
    |||| ||||
    |||| ++++-- Block selection
    |||+------- ROMs 3+4 Disable (0=normal, 1=disable ROMs 3+4 even if selected by Q)
    ||+-------- ROM switch (0=select ROMs 1+2, 1=select ROMs 3+4)
    |+--------- Mode (0=CNROM, 1=NINA-03)
    +---------- Mirroring (0=Vertical, 1=Horizontal)
    

North American and later Australian cartridge versions were distributed with only ROMs 1+2 populated. The early Australian cartridge version uses three chips and comes with ROMs 1+2+3 populated; for the latter, the Q and q bits must be emulated. 

The q bit seems to have been intended to have been an extra address line for ROMs 3+4, enabling a total of 1.5M/1.5M in the cartridge, but instead of just being routed to the A19 line for ROMs 3+4, it instead also is ORed with the Output Enable signal and so disables them. 

Once any of the Q, q, BBB, or b bits are set, neither this register nor the Lockout defeat register can be updated. However, the Inner Bank Control Register can be set even when updates to the other registers are disabled. 

### Lockout defeat control ($FFC0-$FFDF)
    
    
    7  bit  0
    .... ..LL
           ||
           ++-- Lockout defeat (charge pump drive)
    

Emulators do not need to implement the lockout defeat control register. 

### Inner Bank Control ($FFE8-$FFF7)
    
    
    7  bit  0
    .cCC ...P
     |||    |
     |||    +-- PRG page
     +++------- CHR page
    

Both CNROM and NINA-03 modes select a 32kB bank of PRG ROM at $8000 and an 8kB bank of CHR ROM at $0000. 

In CNROM mode, the 32kB PRG ROM bank is BBBb and the 8kB CHR ROM bank is BBBbCC. 

In NINA-03 mode, the 32kB PRG ROM bank is BBBP and the 8kB CHR ROM bank is BBBcCC. 

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers triggering on reads](Category_Mappers_triggering_on_reads.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
