# INES Mapper 086

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_086) | View [other pages](Special_AllPages.xhtml#INES_Mapper_086)

**iNES Mapper 086** designates Jaleco's JF-13 board used for the Red and Black releases of _Moero!! Pro Yakyuu_ , the Japanese version of Jaleco's _Bases Loaded_. 

## Contents

  * 1 Hardware
  * 2 Registers
    * 2.1 PRG and CHR bank ($6000-$6FFF)
    * 2.2 Audio control ($7000-$7FFF)
  * 3 References



## Hardware

This board has 128 KiB PRG ROM, 64 KiB CHR ROM, no PRG RAM (the mapper ports are mapped in $6000-$7FFF instead, but that means no bus conflicts either), a [74HC139](74139.xhtml "74139"), two [74HC174s](74174.xhtml "74174"), and an additional ASIC "D7756C 148" by NEC that plays recorded speech compressed with ADPCM. An additional set of sample files is needed to emulate the sound chip. 

This board is subject to the same design flaw as [iNES Mapper 038](INES_Mapper_038.xhtml "INES Mapper 038"): accidental copies of the registers are present at $E000-$FFFF because /ROMSEL is high when M2 is low, not just when A15 is low. 

Half the 74139 decodes when R/W is low and A14 and /ROMSEL are high. That then chains into the other half, which decodes writes to $6xxx (and $Exxx) and $7xxx (and $Fxxx). 

## Registers

### PRG and CHR bank ($6000-$6FFF)

The PRG and CHR banking for this mapper is similar to [GNROM](GxROM.xhtml "GNROM") or [Color Dreams](Color_Dreams.xhtml "Color Dreams"). 
    
    
    7  bit  0
    .CPP ..CC
     |||   ||
     |++------ Select 32 KiB PRG bank at $8000
     +-----++- Select 8 KiB CHR bank at $0000
    

### Audio control ($7000-$7FFF)
    
    
    7  bit  0
    ..RP TTTT
      || ||||
      || ++++- Track number
      |+------ While 0 and no sound is currently playing, start (or restart) sound specified in lower 4 bits
      +------- If 0 for at least 19µs=34cycles, the µPD7756C is reset and sound playback stops
    

Track numbers used in the game: 

0 | Strike   
---|---  
1 | Ball   
2 | Time   
3 | Out   
4 | Safe   
5 | Foul ball   
6 | Fair ball   
7 | Batter out   
8 | Play ball   
9 | Ball four   
10 | Home run   
11 | New pitcher   
12 | Ouch (hit by pitch)   
13 | Fool   
14 | blow sound?   
15 | shout of joy?   
  
## References

  * D7756 [description](http://www.cpu-world.com/Support/7/77.html) and [datasheet](http://bootgod.dyndns.org:7777/downloads/UPD7755_UPD7756_UPD7757_UPD7758.pdf)
  * BBS topics [762](http://forums.nesdev.org/viewtopic.php?t=762) and [4030](http://forums.nesdev.org/viewtopic.php?p=32572#p32572)



Categories: [Expansion audio](Category_Expansion_audio.xhtml), [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
