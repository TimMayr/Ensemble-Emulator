# UNIF/UNL-DripGame

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UNIF/UNL-DripGame) | View [other pages](Special_AllPages.xhtml#UNIF_UNL_DripGame)

UNL-DripGame is an FPGA mapper comparable in capability to ASIC mappers, invented by Quietust for a [port of a game](https://www.qmtpro.com/~nes/drip/). 

[NES 2.0 Mapper 284](NES_2_0_Mapper_284.xhtml "NES 2.0 Mapper 284") is assigned to this game, as was [iNES Mapper 102](INES_Mapper_102.xhtml "INES Mapper 102"). The UNIF name is "UNL-DripGame". 

## Contents

  * 1 Memory map
  * 2 Readable registers
    * 2.1 $4800-$4FFF (Status)
    * 2.2 $5000-$57FF (Sound channel 0 status)
    * 2.3 $5800-$5FFF (Sound channel 1 status)
  * 3 Writable registers
    * 3.1 $8000 (Sound channel 0 silence register)
    * 3.2 $8001 (Sound channel 0 data)
    * 3.3 $8002 (Sound channel 0 low period)
    * 3.4 $8003 (Sound channel 0 high period)
    * 3.5 $8004 (Sound channel 1 silence register)
    * 3.6 $8005 (Sound channel 1 data)
    * 3.7 $8006 (Sound channel 1 low period)
    * 3.8 $8007 (Sound channel 1 high period)
    * 3.9 $8008 (Low IRQ register)
    * 3.10 $8009 (High IRQ register)
    * 3.11 $800A (Control)
    * 3.12 $800B (PRG bank)
    * 3.13 $800C (CHR bank 0)
    * 3.14 $800D (CHR bank 1)
    * 3.15 $800E (CHR bank 2)
    * 3.16 $800F (CHR bank 3)
  * 4 Extended attribute tables
  * 5 IRQ
  * 6 Audio
  * 7 See also



## Memory map

CPU: 

  * $4800-$5FFF = Readable registers
  * $6000-$7FFF = PRG RAM
  * $8000-$BFFF = Switchable bank
  * $C000-$FFFF = Fixed to the last bank



## Readable registers

### $4800-$4FFF (Status)
    
    
    [DSSS SSSS]
     |||| ||||
     |+++-++++---- Ready flag; $64 if ready
     +------------ DIP switch
    

Other registers must not be read or written until the mapper is ready. The reference implementation of the mapper is an FPGA that loads its fusemap from another on-board memory, and until the FPGA signals that it is ready, its behavior is undefined other than that the fixed PRG bank is fixed. 

### $5000-$57FF (Sound channel 0 status)
    
    
    [FE.. ....]
     ||
     |+----------- Buffer is empty
     +------------ Buffer is full
    

### $5800-$5FFF (Sound channel 1 status)

Similar to $5000 but for the second channel. 

## Writable registers

All of these registers are mirrored at $8000-$BFFF; the low four bits select the register number. 

### $8000 (Sound channel 0 silence register)

Writing anything silences channel 0 and clears the buffer. 

### $8001 (Sound channel 0 data)

Writing data inserts it into the buffer. 

### $8002 (Sound channel 0 low period)
    
    
    [PPPP PPPP]
    

Specify low 8-bits of period of channel 0. 

### $8003 (Sound channel 0 high period)
    
    
    [VVVV PPPP]
     |||| ||||
     |||| ++++---- High 4-bits of period
     ++++--------- Volume control
    

Specify high 4-bits of period of channel 0, and volume control. 

### $8004 (Sound channel 1 silence register)

Writing anything silences channel 1 and clears the buffer. 

### $8005 (Sound channel 1 data)

Writing data inserts it into the buffer. 

### $8006 (Sound channel 1 low period)
    
    
    [PPPP PPPP]
    

Specify low 8-bits of period of channel 1. 

### $8007 (Sound channel 1 high period)
    
    
    [VVVV PPPP]
     |||| ||||
     |||| ++++---- High 4-bits of period
     ++++--------- Volume control
    

Specify high 4-bits of period of channel 1, and volume control. 

### $8008 (Low IRQ register)
    
    
    [XXXX XXXX]
     |||| ||||
     ++++-++++---- Low 8-bits of IRQ value
    

### $8009 (High IRQ register)
    
    
    [EXXX XXXX]
     |||| ||||
     |+++-++++---- High 7-bits of IRQ value
     +------------ Enable flag
    

IRQ is automatically disabled when it is overflows. Writes to this register additionally acknowledge the interrupt. 

### $800A (Control)
    
    
    [.... SAMM]
          ||||
          ||++---- Nametable mirroring (00=V, 01=H, 10=1sc-A, 11=1sc-B)
          |+------ Enable extended attribute table
          +------- Enable writing to SRAM
    

### $800B (PRG bank)
    
    
    [.... PPPP]
          ||||
          ++++---- Select 16K PRG bank at $8000-$BFFF
    

### $800C (CHR bank 0)
    
    
    [.... CCCC]
          ||||
          ++++---- Select 2K CHR bank at $0000-$07FF
    

### $800D (CHR bank 1)
    
    
    [.... CCCC]
          ||||
          ++++---- Select 2K CHR bank at $0800-$0FFF
    

### $800E (CHR bank 2)
    
    
    [.... CCCC]
          ||||
          ++++---- Select 2K CHR bank at $1000-$17FF
    

### $800F (CHR bank 3)
    
    
    [.... CCCC]
          ||||
          ++++---- Select 2K CHR bank at $1800-$1FFF
    

## Extended attribute tables

This memory is write-only and is mapped at $C000-$C3FF for first nametable, and $C400-$C7FF for last nametable. It is mirrored across $C000-$FFFF. 

Only the two LSB of data are used, and they control the palette of the corresponding tile position in a nametable. (i.e. the bits written to CPU $C523 correspond to the tile at CIRAM $523, which if mirrored horizontally would correspond to the tile at PPU $2923 and $2D23) 

## IRQ

Data written to the low IRQ register is stored in a buffer but won't reload the counter until the high IRQ register is programmed. 

IRQ is count down per cycle, and then trigger the IRQ and stop the timer when it expires. 

## Audio

There are two channels, both of which function in the same way. 

The buffers are "first in first out" buffers which store unsigned sample data with a bias of -$80. 

Each buffer stores 256 bytes which can be written in sequence. If a channel is inactive, filling the buffer with even one byte activates that channel immediately. 

Changes to the period are delayed until the sample is finished playing. However, the volume control can be changed at any time. 

The channel becomes inactive as soon as the buffer becomes empty. 

## See also

  * [Quietust's mapper document](https://www.qmtpro.com/~nes/drip/dripmap.txt)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
