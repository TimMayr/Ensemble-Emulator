# NES 2.0 Mapper 266

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_266) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_266)

**NES 2.0 Mapper 266** is used by _City Fighter IV_ , a hack of _Master Fighter II_ that adds (very grungy) PCM speech output. Given that its resolution (four bits) is worse than the NES/Famicom's own DAC at $4011 (seven bits), it's not immediately obvious what the purpose of including its own DAC is, other than perhaps the possibility of low-pass filtering the 3.5 kHz-sampled PCM data on the circuit board. Its UNIF board name is **UNL-CITYFIGHT**. 

The circuit board mounts a [VRC4](VRC2_and_VRC4.xhtml "VRC4") clone (A2/A3) with two of its higher CPU address lines mixed up: 
    
    
    CPU A3  -> VRC4 CPU Ax (pin 03)
    CPU A2  -> VRC4 CPU Ay (pin 04)
    CPU A13 -> VRC4 CPU A14 (pin 02)
    CPU A14 -> VRC4 CPU A13 (pin 01)
    

... and two additional registers that are selected via the VRC4 /WR9003 pin, e.g. by writing to $900C, and distinguished via CPU A11 ($0800). This results in the following effective registers: 

## Contents

  * 1 Mirroring Control ($9000)
  * 2 PRG Select ($900C)
  * 3 DAC Output ($980C)
  * 4 CHR Select ($A00x/$B00x/$D00x/$E00x)
  * 5 IRQ Control ($F00x)
  * 6 Notes



## Mirroring Control ($9000)

Mask: $F00C, see [VRC4 description](VRC2_and_VRC4.xhtml#Mirroring_Control_.28.249000.2C_.249001.2C_.249002.2C_.249003.29 "VRC2 and VRC4"). 

## PRG Select ($900C)
    
    
    Mask: $F80C
    
    D~7654 3210
      ---------
      .... PP..
           ++--- PRG A16..A15
    

Similar to [INES Mapper 189](INES_Mapper_189.xhtml "INES Mapper 189"), VRC4's fine-grained PRG banking is replaced with a single 32 KiB bank switch. 

## DAC Output ($980C)
    
    
    Mask: $F80C
    
    D~7654 3210
      ---------
      .... DDDD
           ++++- 4 bit unsigned PCM data
    

## CHR Select ($A00x/$B00x/$D00x/$E00x)
    
    
    Mask: $F00C
    
    $D000/$D004 (LSB/MSB): Select 1 KiB CHR-ROM bank at PPU $0000-$03FF
    $D008/$D00C (LSB/MSB): Select 1 KiB CHR-ROM bank at PPU $0400-$07FF
    $A000/$A004 (LSB/MSB): Select 1 KiB CHR-ROM bank at PPU $0800-$0BFF
    $A008/$A00C (LSB/MSB): Select 1 KiB CHR-ROM bank at PPU $0C00-$0FFF
    $B000/$B004 (LSB/MSB): Select 1 KiB CHR-ROM bank at PPU $1000-$13FF
    $B008/$B00C (LSB/MSB): Select 1 KiB CHR-ROM bank at PPU $1400-$17FF
    $E000/$E004 (LSB/MSB): Select 1 KiB CHR-ROM bank at PPU $1800-$1BFF
    $E008/$E00C (LSB/MSB): Select 1 KiB CHR-ROM bank at PPU $1C00-$1FFF
    

## IRQ Control ($F00x)

Mask: $F00C, see [VRC4 description](VRC2_and_VRC4.xhtml#IRQ_Control_.28.24F00x.29_VRC4 "VRC2 and VRC4"). 

# Notes

[![](../wiki-images/City_fighter_4_dac_audio.png)](File_City_fighter_4_dac_audio_png.xhtml)

[](File_City_fighter_4_dac_audio_png.xhtml "Enlarge")

Captured waveform of the audio speech when using R-2R as DAC

The game uses the VRC4's pseudo-scanline mode during speech output. FCEUX only emulates a downward-counting CPU cycle counter, counting on every other cycle, causing speech to be played at too low a pitch compared to hardware. 

Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
