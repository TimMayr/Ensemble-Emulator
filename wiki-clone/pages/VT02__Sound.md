# VT02+ Sound

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VT02%2B_Sound) | View [other pages](Special_AllPages.xhtml#VT02__Sound)

The [VT02+ Famiclone consoles](VTxx.xhtml "VTxx") include a second [APU](APU.xhtml "APU"), doubling the number of pulse, triangle and noise channels. Furthermore the single PCM/DPCM channel can use DMA for raw PCM as well, and fetch data from the entire CPU address range. 

## Contents

  * 1 Address Map
  * 2 Registers
    * 2.1 DPCM/Raw PCM selection, output control ($4030)
    * 2.2 Write 8 bit raw PCM data ($4031)
    * 2.3 Channel enable and length counter status for second APU channels ($4035)
    * 2.4 Links



# Address Map
    
    
    $4000-$4003   Pulse 1
    $4004-$4007   Pulse 2
    $4008-$400B   Triangle 1
    $400C-$400F   Noise 1
    $4010-$4013   DMC
    $4015         Channel enable and length counter status for $4000-$4013
    $4020-$4023   Pulse 3
    $4024-$4027   Pulse 4
    $4028-$402B   Triangle 2
    $402C-$402F   Noise 2
    $4030         DPCM/Raw PCM selection, output control
    $4031         Write 8 bit raw PCM data
    $4035         Channel enable and length counter status for $4020-$402F
    

# Registers

The meaning of registers $4020-$402F resembles that of registers $4000-$400F. Note that all V.R. Technology Famiclones reverse the 25% and 50% duty cycles, even as V.R. Technology's official emulator EmuVT does not. 

## DPCM/Raw PCM selection, output control ($4030)
    
    
    7654 3210
    ---- ----
    LlMR EeAA
    |||| |||+- APU DMA address bit 14 (inverted)
    |||| ||+-- APU DMA address bit 15 (inverted)
    |||| |+--- 0: Enable output from first APU ($4000-$400F, DPCM or raw PCM written to $4011)
    |||| |     1: Disable output from first APU ($4000-$400F, DPCM or raw PCM written to $4011)
    |||| +---- 0: Disable output from second APU ($4020-$402F, raw PCM written to $4031 or via DMA)
    ||||       1: Enable output from second APU ($4020-$402F, raw PCM written to $4031 or via DMA)
    |||+------ 0: APU DMA uses DPCM
    |||        1: APU DMA uses raw PCM
    ||+------- 0: Maximum DMA transfer length is 4,081 bytes
    ||         1: Maximum DMA transfer length is 4,096 bytes
    |+-------- 0: Enable loudness of first APU ($4000-$400F, DPCM or raw PCM written to $4011)
    |          1: Disable loudness of first APU ($4000-$400F, DPCM or raw PCM written to $4011)
    +--------- 0: Enable loudness of second APU ($4020-$402F, raw PCM written to $4031 or via DMA)
               1: Disable loudness of second APU ($4020-$402F, raw PCM written to $4031 or via DMA)
    

Note that the meaning of bit 2 is the exact opposite of bit 3. The direction of the bits' effect is chosen so that writing $00 will result in perfect NES/Famicom-compatible behavior: DMA address A14/A15 set so that transfers are limited to $C000-$FFFF, output enabled from the first and disabled from the second APU, DMA uses DPCM, maximum length is 4,081 bytes, and both APUs loud. 

Playing raw PCM via DMA uses the same sampling rates as DPCM. Note that when playing raw PCM data via DMA, the output of the second APU must be enabled first. 

## Write 8 bit raw PCM data ($4031)

Identical to register $4011, except that the resolution is eight rather than seven bits. 

## Channel enable and length counter status for second APU channels ($4035)

This register is the equivalent of $4015, only that it applies to the second APU's channels. 

Write: 
    
    
    7654 3210
    ---- ----
    .... NT43
         |||+- Enable Pulse channel 3
         ||+-- Enable Pulse channel 4
         |+--- Enable Triangle channel 2
         +---- Enable Noise channel 2
    

Read: 
    
    
    7654 3210
    ---- ----
    .... NT43
         |||+- 1=Pulse channel 3's length counter >0
         ||+-- 1=Pulse channel 4's length counter >0
         |+--- 1=Triangle channel 2's length counter >0
         +---- 1=Noise channel 2's length counter >0
    

## Links

  * [Santa Claus](https://forums.nesdev.org/viewtopic.php?p=225447#p225447) NSF rip
  * [100 In 1 (101 In 1) Arcade Action II (AT-103)](https://forums.nesdev.org/viewtopic.php?p=242753#p242753) NSF rip


