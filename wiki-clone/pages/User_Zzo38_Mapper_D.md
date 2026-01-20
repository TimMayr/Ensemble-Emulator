# User:Zzo38/Mapper D

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_D) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_D)

This mapper is designed to be possible to write a game that can run on both this mapper and the [MMC5](MMC5.xhtml "MMC5"), even though most of the features of MMC5 aren't needed. 

(The main reason for inventing this is so that a game can be written using 64K RAM and 256K ROM, can be emulated using existing emulators without modifying it, and can be easily made into a cartridge using discrete logic components.) 

## Contents

  * 1 Registers
    * 1.1 $5113: PRG RAM bank
    * 1.2 $5115: PRG ROM bank
  * 2 Using MMC5 as this mapper



## Registers

There are two (write-only) registers. Canonically they're mapped at $5113 and $5115, but they respond to writes at any address matching this pattern: 
    
    
    [..01 .... .... .?..]
    

Where the question mark selects the register. There will be bus conflicts if you write the register using the addresses which are in the ROM ($9000-$9FFF or $D000-$DFFF), and writes to mirrors of internal RAM at $1000-$1FFF also trigger this register (you shouldn't ordinarily do this). 

### $5113: PRG RAM bank

Register 0 is the RAM bank register. All the bits are used to select a 8K RAM bank at $6000-$7FFF. (According to Disch's notes, assume 64K RAM if the [NES 2.0](NES_2_0.xhtml "NES 2.0") header isn't present.) 

### $5115: PRG ROM bank

Register 1 is the ROM bank register. All bits except for the highest and lowest bit select a 16K ROM bank at $8000-$BFFF. 
    
    
    7654 3210
    |||| ||||
    |||| |||+- Always 0
    |+++-+++-- Select 16K PRG ROM bank at $8000-$BFFF
    +--------- Always 1
    

## Using MMC5 as this mapper

If you want to write a program which works both with MMC5 and with the mapper described here, there are a few things to do. 

One is to initialize the MMC5 registers. All initializations must be in the last 8K bank at $E000-$FFFF. The assignments listed below should work (I hope): 

  * $5100: $01
  * $5101: $00
  * $5102: $01
  * $5103: $02
  * $5105: Set according to mirroring bit in header
  * $5200: $00
  * $5204: $00



Now when writing to the registers of this mapper, you must use: 

  * $5113 for accessing register 0
  * $5115 for accessing register 1, and the high bit of the data must be set



In addition, you shouldn't write to $1000-$1FFF since that is both RAM and registers in this mapper, but is RAM only for MMC5; it mirrors the $0000-$07FF RAM internally in the console unit but due to this reason, don't try to write to there. 

  

