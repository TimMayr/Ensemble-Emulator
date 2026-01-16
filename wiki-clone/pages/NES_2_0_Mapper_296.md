# NES 2.0 Mapper 296

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_296) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_296)

NES 2.0 Mapper is used for the _FC Pocket RS-20_ and _dreamGEAR My Arcade Gamer V_ handheld consoles. They use VT3x hardware, which is mostly compatible hardware to the earlier [VT0x and VT1x](VTxx.xhtml "VTxx") consoles, but add an outer bank register and extended support for legacy mappers. While the original OneBus hardware only has MMC3-compatible registers, this type can directly emulate CNROM, UNROM and MMC1 behavior as well. 

## Contents

  * 1 Banking registers
    * 1.1 Mapper Configuration Register ($411D, write)
    * 1.2 Encryption Control ($411E, write)
    * 1.3 Outer Bank Register ($412C)
    * 1.4 Outer Bank Register 2 ($412E)
  * 2 Two-channel PCM expansion sound
    * 2.1 Select channel 0 and set volume ($4031, write), Select channel 1 and set volume ($4032, write)
    * 2.2 Expansion sound and channel enable ($4033, write)
    * 2.3 Select playback address bits 6-13 ($4012), bits 14-20 ($4035), bits 21-28 ($4036)
    * 2.4 Channel status ($4014, read)
  * 3 Palette



# Banking registers

## Mapper Configuration Register ($411D, write)
    
    
    7654 3210
    ---------
    .... .CMM
          |++- Select legacy mapper to emulate in the $8000-$FFFF range
          |     0: MMC3
          |     1: MMC1
          |     2: UNROM
          |     3: CNROM
          +--- Select CHR memory type
                0: CHR-ROM (banked)
                1: CHR-RAM (unbanked)
    

## Encryption Control ($411E, write)
    
    
    7654 3210
    ---------
    cc.. .C.C
    ||    +-+- 00: Disable CPU opcode encryption after the next JMP instruction
    ||         11: Enable CPU opcode encryption after the next JMP instruction
    ++-------- 00: Disable CHR data encryption
               11: Enable CHR data encryption
    

  * When CPU encryption is active, CPU opcode bytes only are XORed with $A1.
  * When CHR data encryption is active, CHR data bytes' bits 0 and 4, 1 and 2, 3 and 7, and 5 and 6 are swapped.



## Outer Bank Register ($412C)
    
    
    7654 3210
    ---------
    .... cpCP
         |||+- PRG A25 (32 MiB)
         ||+-- CHR A25 (32 MiB)
         |+--- PRG A26 (64 MiB)
         +---- CHR A26 (64 MiB)
    

## Outer Bank Register 2 ($412E)
    
    
    7654 3210
    ---------
    .... ...B
            +- PRG/CHR A27 (128 MiB)
    

# Two-channel PCM expansion sound

Expansion sound involves two simultaneous linear 8 bit unsigned PCM channels which just keep playing from their current address until byte $FF is encountered. Note that the address (excluding bits 0-5) is programmed directly and not relative to other bankswitching registers except the 32 MiB outer bank. The playback rate is set using the normal [$4010 register](APU_DMC.xhtml "APU DMC") bits 0-3. 

## Select channel 0 and set volume ($4031, write), Select channel 1 and set volume ($4032, write)

Selects the channel to which the next writes to $4012, $4035 and $4036 will apply, and sets its linear volume (0-255). 

## Expansion sound and channel enable ($4033, write)
    
    
    7654 3210
    ---------
    E..0 1...
    |  | +---- 1=Enable channel 1
    |  +------ 1=Enable channel 0
    +--------- 1=Enable expansion sound
    

Note that the order of the channel bits is reversed from $4014 (see below). 

## Select playback address bits 6-13 ($4012), bits 14-20 ($4035), bits 21-28 ($4036)

These registers can only be accessed if expansion sound is enabled (via $4033 bit 7, otherwise it $4012 will go to the regular DPCM channel register), and if the selected channel has been _disabled_ via the respective bit in $4033, otherwise the write will be ignored. 

## Channel status ($4014, read)
    
    
    7654 3210
    ---------
    .... ..10
           |+- 1=Channel 0 still playing
           +-- 1=Channel 1 still playing
    

These bits can be used to detect when a sample has finished playing (having encountered byte $FF) in order to restart it. Once a sample has finished playing, these bits return zero, and the channels are no longer enabled. Note that the _FC Pocket RS-20_ menu apparently confuses the two channel bits and continuously writes to restart the background music, which however does not actually occur because the hardware blocks accesss to the playback address registers while the sample is still playing. 

# Palette

While the original [VT03+ Enhanced Palette](VT03__Enhanced_Palette.xhtml "VT03+ Enhanced Palette") uses four bits each for hue, saturation and luminance, the VT3x 12-bit palette simply directly encodes 4-bit RGB values: 
    
    
    BA98 7654 3210
    --------------
    BBBB GGGG RRRR
    |||| |||| ++++- Red level (0-15)
    |||| ++++------ Green level (0-15)
    ++++----------- Blue level (0-15)
    

The _dreamGEAR My Arcade Gamer V_ multicart contains a few games that were previously published on VT03 multicarts. When converting these games to VT3x, the developers apparently forgot to change a few colors to the new format, resulting in some elements using strange color combinations even on real hardware (e.g. the waterfall in Curly Monkey). 

Categories: [CNROM-like mappers](Category_CNROM_like_mappers.xhtml), [Expansion audio](Category_Expansion_audio.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multi-ASIC mappers](Category_Multi_ASIC_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
