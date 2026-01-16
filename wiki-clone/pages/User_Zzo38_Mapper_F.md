# User:Zzo38/Mapper F

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_F) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_F)

  
This is a experimental kind of ASIC mapper, with a few complicated features. 

## Contents

  * 1 Overview
  * 2 Memory map
    * 2.1 PRG setup
    * 2.2 CHR setup
  * 3 Registers
    * 3.1 $5800 (W) / CIRAM controls / mute
    * 3.2 $5800 (R) / Audio input
    * 3.3 $5801 (W) / Low PRG ROM bank / video mode
    * 3.4 $5801 (R) / PPU address
    * 3.5 $5802 (W) / IRQ setting / high PRG ROM bank
    * 3.6 $5803 (W) / ExRAM mode / PRG RAM bank
    * 3.7 $5804 / Channel 1 controls
    * 3.8 $5805 / Channel 1 low period
    * 3.9 $5806 / Channel 1 high period
    * 3.10 $5807 (W) / Pattern table CHR banks
    * 3.11 $5808 / Channel 2 controls
    * 3.12 $5809 / Channel 2 low period
    * 3.13 $580A / Channel 2 high period
    * 3.14 $580B (W) / ExRAM mode
    * 3.15 $580C (W) / Channel 3 controls
    * 3.16 $580C (R) / Phase output
    * 3.17 $580D (W) / Channel 3 low period
    * 3.18 $580E (W) / Channel 3 high period
    * 3.19 $580F (W) / Nametable CHR banks
  * 4 ExRAM
  * 5 Power on state



## Overview

  * PRG ROM size: Up to 512 KB
  * PRG ROM bank size: 16 KB
  * PRG RAM: Up to 64 KB
  * CHR capacity: Up to 64 KB ROM, or 64 KB RAM, or 32 KB ROM + 32 KB RAM
  * CHR bank size: 4 KB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): Yes, but irrelevant



A number of advanced features are supported, including: 

  * Three extra sound channels, one of which can be used for interrupts and/or PPU access counting
  * Ability to mute the 2A03 + microphone audio
  * Register to read the audio output from 2A03 + microphone
  * Capability for 8x8 attribute areas
  * ROM nametable support
  * 256 additional bytes of RAM, usable for a number of strange things
  * Ability to write to the NES/Famicom's internal PRG RAM and to the mapper registers/ExRAM simultaneously



## Memory map

### PRG setup

  * $0800-$0FFF: Reading or writing here also writes to (but never reads from) mapper registers
  * $1000-$17FF: ExRAM (write-only)
  * $1800-$1FFF: Registers (write-only)
  * $3000-$3FFF: Mirror of $7000-$7FFF (write-only)
  * $4800-$4FFF: Mirror of $0800-$0FFF, but there is no data to read from, so it can't know what to write
  * $5000-$57FF: ExRAM
  * $5800-$5FFF: Registers
  * $6000-$7FFF: 8K RAM bank
  * $8000-$BFFF: 16K switchable ROM bank
  * $C000-$DFFF: Bottom half of 16K switchable ROM bank
  * $E000-$FFFF: Fixed to the last ROM bank



Some areas are specified "write-only", meaning that if you try to read it, it will have bus-conflicts. 

### CHR setup

  * $0000-$0FFF: 4K switchable ROM or RAM bank, or CIRAM
  * $1000-$1FFF: 4K switchable ROM or RAM bank, or CIRAM
  * $2000-$2FFF: 4K switchable ROM or RAM bank, or CIRAM
  * $3000-$3EFF: 4K switchable ROM or RAM bank, or CIRAM



## Registers

Registers are mapped at $5800-$5FFF, using the low four bits as a register number. There may be different registers for reading as for writing. The same registers are also mapped at $1800-$1FFF, however reading from those address may result in bus conflicts. Writing there is OK, though. 

Therefore, the address mask is: 
    
    
    [0.01 1... .... xxxx]
    

### $5800 (W) / CIRAM controls / mute
    
    
    [ESMM CCCC]
     |||| ||||
     |||| ++++--- CIRAM enable
     ||++-------- Nametable mirroring
     |+---------- CIRAM swap
     +----------- Enable console audio
    

If the "E" bit is cleared, then only the expansion audio is played, and the audio from 2A03 + microphone is disabled. 

Nametable mirroring decides the connection of CIRAM A10, depending on "MM", and then that address line is XORed by "S", as follows: 

  * 00 = PA10
  * 01 = PA11
  * 10 = PA12
  * 11 = PA13



The "CCCC" decides which sections of PPU address space are accessing CIRAM (if the bit is clear, CIRAM is used; if set, the CHR ROM/RAM in the cartridge is used): 

  * bit0 = $0000-$0FFF
  * bit1 = $1000-$1FFF
  * bit2 = $2000-$2FFF
  * bit3 = $3000-$3EFF



### $5800 (R) / Audio input
    
    
    [.... XXXX]
          ||||
          ++++--- 4-bit ADC
    

This register is a 4-bit ADC for the 2A03+microphone audio. Mute and expansion audio are not applied; it is the same audio signal as cartridge is receiving at first. 

### $5801 (W) / Low PRG ROM bank / video mode
    
    
    [VV.P PPPP]
     || | ||||
     || +-++++--- 16K PRG ROM bank at $8000-$BFFF
     ++---------- Video mode
    

The video modes can be: 

  * 00 = No special modes.
  * 01 = When reading the nametables (not counting the attribute tables): Copy PPU address bit0 to bit0 of register $580F. Copy PPU address bit5 to bit1 of register $580F and to bit6 of register $5800. Copy PPU address bit10 XOR PPU address bit11 to bit0 of register $5807.
  * 10 = When reading addresses $1FDx or $1FEx of pattern tables: Copy PPU address bit4 to bit4 of register $5800 and to bit0 of register $5807.
  * 11 = When reading the nametables (not counting the attribute tables): Copy PPU address bit0 to bit0 of register $5807. Copy PPU address bit5 to bit1 of register $5807. Copy PPU address bit10 XOR PPU address bit11 to bit1 of register $5800.



### $5801 (R) / PPU address
    
    
    [AAAA AAAA]
     |||| ||||
     ++++-++++--- PPU address
    

This data will read bit11-bit4 of the PPU address according to what it was during the most recent nametables (not attribute tables) read. 

### $5802 (W) / IRQ setting / high PRG ROM bank
    
    
    [IILP PPPP]
     |||| ||||
     |||+-++++--- 8K PRG ROM bank at $C000-$DFFF
     ||+--------- IRQ latch
     ++---------- IRQ mode
    

This is actually the low 8K of a 16K bank, so the high 8K of a 16K bank cannot be mapped into $C000-$DFFF. 

The IRQ latch is just output to the IRQ pin, and can be manually changed (lowering it will trigger IRQ if enabled). Depending on IRQ mode, it may also automatically change it too. (Note: This can also be used as output to the Famicom expansion port.) 

IRQ mode: 

  * 00 = Manual only.
  * 01 = When reading nametables (not counting attributes), copy inverted bit11 of PPU address to IRQ latch.
  * 10 = Clear IRQ latch when reading PPU addresses $1FD0 or $1FE0.
  * 11 = Clear IRQ latch when writing to PPU addresses $1FD0 or $1FE0.



### $5803 (W) / ExRAM mode / PRG RAM bank
    
    
    [HHHH .BBB]
     ||||  |||
     ||||  +++--- 8K PRG RAM bank at $6000-$7FFF
     ++++-------- High 4-bits of ExRAM mode
    

### $5804 / Channel 1 controls

Same as register $580C, but for channel 1. However, there are two additional bits which you can write, which are only existing in channel 1: 
    
    
    [PCIW AAAA]
     |||| ||||
     |||| ++++--- AND mask
     |||+-------- Waveform (0: saw, 1: square)
     ||+--------- If set, clear IRQ latch after one full period
     |+---------- Clock source (0: CPU, 1: PPU)
     +----------- Phase reset if bit is cleared
    

If the clock source is set to PPU, then this channel is clocked when the PPU reads data. 

### $5805 / Channel 1 low period

Same as register $580D, but for channel 1. 

### $5806 / Channel 1 high period

Same as register $580E, but for channel 1. 

### $5807 (W) / Pattern table CHR banks
    
    
    [HHHH LLLL]
     |||| ||||
     |||| ++++--- 4K CHR bank at $0000-$0FFF
     ++++-------- 4K CHR bank at $1000-$1FFF
    

If there is both CHR ROM and CHR RAM, then the high bit (bit3) of the bank number selects between ROM and RAM (ROM if cleared, RAM if set). 

If CHR RAM is enabled but that address range is also set to CIRAM, then reading will read CIRAM, but writing will write both the CIRAM and the cartridge CHR RAM. 

### $5808 / Channel 2 controls

Same as register $580C, but for channel 2. 

### $5809 / Channel 2 low period

Same as register $580D, but for channel 2. 

### $580A / Channel 2 high period

Same as register $580E, but for channel 2. 

### $580B (W) / ExRAM mode
    
    
    [LLLL LLLL]
     |||| ||||
     ++++-++++--- Low 8-bits of ExRAM mode
    

### $580C (W) / Channel 3 controls
    
    
    [P..W AAAA]
     |  | ||||
     |  | ++++--- AND mask
     |  +-------- Waveform (0: saw, 1: square)
     +----------- Phase reset if bit is cleared
    

The waveform is used to determine the output values: 

  * Saw: 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
  * Square: 0 0 30 30 0 0 30 30 0 0 30 30 0 0 30 30 0 0 30 30 0 0 30 30 0 0 30 30 0 0 30 30



These values from the waveform are then taken, the AND mask is left shift by one and add one, and it is bitwise AND by the result, to determine the output level. 

### $580C (R) / Phase output
    
    
    [...P PPPP]
        | ||||
        +-++++--- Phase output
    

This register outputs the phase value from 0 to 31 which is the index into the waveform. 

### $580D (W) / Channel 3 low period
    
    
    [LLLL LLLL]
     |||| ||||
     ++++-++++--- Low bits of period of channel 3
    

### $580E (W) / Channel 3 high period
    
    
    [HHHH HHHH]
     |||| ||||
     ++++-++++--- High bits of period of channel 3
    

### $580F (W) / Nametable CHR banks
    
    
    [HHHH LLLL]
     |||| ||||
     |||| ++++--- 4K CHR bank at $2000-$2FFF
     ++++-------- 4K CHR bank at $3000-$3EFF
    

If there is both CHR ROM and CHR RAM, then the high bit (bit3) of the bank number selects between ROM and RAM (ROM if cleared, RAM if set). 

If CHR RAM is enabled but that address range is also set to CIRAM, then reading will read CIRAM, but writing will write both the CIRAM and the cartridge CHR RAM. 

## ExRAM

ExRAM is at $5000-$57FF, but there are only 256 bytes, mirrored across this range. It is also writable at $1000-$17FF, but there is bus conflicts if you try to read it from this address. 

Reading/writing it can have various effects on the data and address, depending on the ExRAM mode setting: 

  * bit0: When reading (not when writing), toggle bit7 of the address.
  * bit1: When writing (not when reading), address is shifted left by two, and those two high bits are instead put in the low bits of the address.
  * bit2: When writing, always write $0 as the low nybble.
  * bit3: When writing, always write $F as the low nybble.
  * bit4: When writing, always write $0 as the high nybble.
  * bit5: When writing, always write $F as the high nybble.
  * bit6: After reading, write $00 to that cell in ExRAM.
  * bit7: After reading, write $FF to that cell in ExRAM.
  * bit8: After reading, write the address of the cell into the ExRAM.
  * bit9: After reading, write the value of the internal last-read register to the cell just read, before updating the last-read register. (This last-read register is always updated after reading from ExRAM, regardless if this bit is set or not.)
  * bit10: Clear the IRQ latch when the CPU writes $00 to anywhere in ExRAM.
  * bit11: Suppress updating the internal last-read register.



In some cases multiple bits set are conflicting, and should not be used. 

## Power on state

The power on state is undefined. All relevant registers (including audio) must be initialized. For normal usage, the registers that have to be initialized includes: $0 $1 $2 $4 $7 $8 $C 
