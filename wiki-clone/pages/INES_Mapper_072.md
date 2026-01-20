# INES Mapper 072

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_072) | View [other pages](Special_AllPages.xhtml#INES_Mapper_072)

iNES Mapper 072 describes games released on Jaleco's JF-17 board. 

## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 PRG Bank select ($8000-$FFFF, 128s bit)
    * 2.2 CHR Bank select ($8000-$FFFF, 64s bit)
    * 2.3 Audio playback control ($8000-$FFFF, 32s and 16s bits)
  * 3 Hardware
  * 4 Notes
  * 5 In Disch's style



## Overview

  * PRG ROM size: 128 KiB (256KiB for [mapper 92](INES_Mapper_092.xhtml "INES Mapper 092"))
  * PRG ROM bank size: 16 KiB
  * PRG RAM: None
  * CHR capacity: 128 KiB ROM
  * CHR bank size: 8 KiB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Cuttable trace selects vertical or horizontal mirroring
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): Yes



## Registers

### PRG Bank select ($8000-$FFFF, 128s bit)
    
    
     7  bit  0
     Pxxx DDDD
     |    ||||
     |    |+++- 16 KB PRG ROM bank for CPU $8000-$BFFF
     |    +---- Required by [mapper 92](INES_Mapper_092.xhtml "INES Mapper 092") games only
     +--------- When a 1 is written after a 0 was previously written, load the bank specified by D
    

### CHR Bank select ($8000-$FFFF, 64s bit)
    
    
     7  bit  0
     xCxx DDDD
      |   ||||
      |   ++++- 8 KB CHR ROM bank for PPU $0000-$1FFF
      +-------- When a 1 is written after a 0 was previously written, load the bank specified by D
    

### Audio playback control ($8000-$FFFF, 32s and 16s bits)
    
    
     15   11 address   0  7 data  0
     ---- ---- ---- ----  ---- ----
     1... .... ...A AAAA  xxRS ....
                  | ||||    ||
                  | ||||    |+------ While 0 and no previous sound is playing, start playing
                  | ||||    |        sound specified by A.
                  | ||||    +------- While 0, the µPD7756C sound IC is held in reset and unable
                  | ||||             to make sound.  Hold this low for at least 34 cycles.
                  +-++++------------ The sound to play when triggered by S bit
    

## Hardware

  * A [74161](74161.xhtml "74161") is attached to the top 4 bits of the data bus, in the exact same way as in all of Nintendo's discrete logic mappers.
  * The top two output bits of the '161 connect to the CLOCK pins of two different [74174s](74174.xhtml "74174").
  * The '174 latches the contents of the bottom 4 bits of the data bus when its CLOCK pin goes from 0 to 1.
  * Three of a [7432](7432.xhtml "7432")'s OR gates are used to make the PRG act like [UNROM](UxROM.xhtml "UxROM"). (The last is used to allow for a 28-pin 128kB CHR ROM)
  * The bottom two output bits of the '161 connect to the /RESET and /START pins of the µPD7756C sound IC.



A design that replaced the '161 with a [74139](74139.xhtml "74139") could have the side effect of preventing bus conflicts. However, the [µPD7756](INES_Mapper_086.xhtml#References "INES Mapper 086") requires that its /RESET signal be low for at least 19µs and its /START timing requirements may also be too strict for the NES, so Jaleco used a latch instead of a demultiplexer. 

## Notes

  * Three games are known to use this mapper: 
    * Moero!! Juudou Warriors
    * Moero!! Pro Tennis — with expansion sound
    * Pinball Quest — occasionally induces bus conflicts on bits that end up not mattering



## In Disch's style
    
    
     ========================
     =  Mapper 072          =
     ========================
     
     Example Games:
     --------------------------
     Pinball Quest (J)
     Moero!! Pro Tennis
     Moero!! Juudou Warriors
     
     
     Registers (**BUS CONFLICTS**):
     ---------------------------
     
       $8000-FFFF:  [PCRS DDDD]
         P = When a 1 is written after a 0 was previously written,
             the bottom three bits of the data bus are copied to the PRG bank select
         C = When a 1 is written after a 0 was previously written,
             the bottom four bits of the data bus are copied to the CHR bank select
         R = For games that have add-on sound, while 0,
             the ADPCM playback IC is held in reset and unable to make sound
         S = For games that have add-on sound, when the value written here changes
              (direction unknown because the datasheet contradicts itself), 
             the sound specified by the bottom 5 bits of the address bus is played.
             Leaving the value at 0 will probably result in erratic audio playback.
         D = the three- or four- bit bank number to switch to, as appropriate.
     
     
     Notes:
     ---------------------------
     
     Commands pass through a latch.  Rather than writing to the regs directly, you write the
     desired page number and command to the latch, then send another command that readies it for the next time.
     
        Commands (PC bits together):
          %00 = Do nothing (prepare for next write)
          %01 = Set CHR Page
          %10 = Set PRG page
          %11 = Set both simultaneously
     
        Example:
          If a game wanted to select CHR page 3, it would first write $43, then $03.  The $43 fills the latch with command 
          bits $4, which instruct bank $3 to be used for CHR; then the write of $03 prepares for the next write by
          resetting the command bits to $0. The $03 should be able to be any value from $00 to $0F, because the command
          bits are what is crucial.
     
     No current theory explains why games go to any effort to put the bank's nybble in the second byte, although perhaps
     it has to do with not disturbing the bank registers while the logic propagates.
     
     CHR Setup:
     ---------------------------
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +---------------------------------------------------------------+
         |                            CHR Reg                            |
         +---------------------------------------------------------------+
     
     
     PRG Setup:
     ---------------------------
     
           $8000   $A000   $C000   $E000  
         +---------------+---------------+
         |    PRG Reg    |     { -1}     |
         +---------------+---------------+
    

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [Expansion audio](Category_Expansion_audio.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
