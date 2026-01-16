# NES 2.0 Mapper 528

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_528) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_528)

NES 2.0 Mapper 528 is used for the _1995 New Series Super 2-in-1_ multicart (title screen: Rugby 2-in-1), containing both of Sunsoft's _Dodge Danpei_ games. Its UNIF board name is **UNL-831128C**. Although the ROM image's common name is "1995 New Series Super 2-in-1 (Unl,Vrc7p)", only the IRQ counter functions like the [VRC7](VRC7.xhtml "VRC7")'s; the other registers are more like Sunsoft's [FME-7](Sunsoft_FME_7.xhtml "FME-7"). 

## Contents

  * 1 Registers
    * 1.1 Game select ($A00x/C00x)
    * 1.2 FME-7-compatible registers ($A000-A00C/$C000-$C00C)
    * 1.3 PRG $6000-$7FFF control ($A008/$C008)
    * 1.4 VRC-compatible IRQ ($A00D-$A00F/$C00D-$C00F)
  * 2 Notes



# Registers

## Game select ($A00x/C00x)
    
    
    A~FEDC BA98 7654 3210
      -------------------
      1G.. .... .... ....
       +------------------ Select game
                            0: Dodge Danpei
                            1: Dodge Danpei 2
    

## FME-7-compatible registers ($A000-A00C/$C000-$C00C)

These registers function like the Sunsoft [FME-7](Sunsoft_FME_7.xhtml "FME-7")'s registers $0-$C (excluding $8 and $B), except that A0-A3 select the register directly; there is no index register. Register $B is not functional, as the CPU $C000-$DFFF is hard-wired to the second-to-last bank (unlike the FME-7), and CPU $E000-$FFFF hard-wired to the last bank (as on the FME-7). 
    
    
    $A000: Set 1 KiB CHR-ROM bank at PPU $0000-$03FF
    $A001: Set 1 KiB CHR-ROM bank at PPU $0400-$07FF
    $A002: Set 1 KiB CHR-ROM bank at PPU $0800-$0BFF
    $A003: Set 1 KiB CHR-ROM bank at PPU $0C00-$0FFF
    $A004: Set 1 KiB CHR-ROM bank at PPU $1000-$13FF
    $A005: Set 1 KiB CHR-ROM bank at PPU $1400-$17FF
    $A006: Set 1 KiB CHR-ROM bank at PPU $1800-$1BFF
    $A007: Set 1 KiB CHR-ROM bank at PPU $1C00-$1FFF
    $A008: See below.
    $A009: Set 8 KiB PRG-ROM bank at CPU $8000-$9FFF
    $A00A: Set 8 KiB PRG-ROM bank at CPU $A000-$BFFF
    $A00B: No function
    $A00C: Select nametable mirroring type (V/H/S0/S1).
    

## PRG $6000-$7FFF control ($A008/$C008)

This register seems to be a simplified version of the FME-7's register $8. Value $01 selects PRG-RAM, all other values select an 8 KiB PRG-ROM bank at CPU $6000-$7FFF. 

## VRC-compatible IRQ ($A00D-$A00F/$C00D-$C00F)
    
    
    $A00D/$C00D: IRQ Control
    $A00E/$C00E: IRQ Acknowledge
    $A00F/$C00F: IRQ Latch.
    

See [VRC IRQ](VRC_IRQ.xhtml "VRC IRQ") for details. 

# Notes

Note that the first Dodge Danpei game only has 128 KiB of PRG-ROM while the second has 256 KiB, requiring careful application of PRG-ROM base offsets and masks. Although the commonly-available UNIF ROM image only has one PRG chunk, it is highly likely that the cartridge used two PRG-ROM chips. 

Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
