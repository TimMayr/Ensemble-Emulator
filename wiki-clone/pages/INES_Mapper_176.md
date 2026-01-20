# INES Mapper 176

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_176) | View [other pages](Special_AllPages.xhtml#INES_Mapper_176)

**iNES Mapper 176** denotes the **8025** enhanced [MMC3](MMC3.xhtml "MMC3") chipset. It used by many multicarts, Chinese single-game and educational computer cartridges, and Techno Source's _Intellivision X2_ Plug-and-Play console. Incompatible variations exist that are denoted via NES 2.0 Submapper. 

Submappers   
---  
# | PCB codes | [UNIF](UNIF.xhtml "UNIF") MAPR | MMC3 | Outer bank registers | Example   
Ext. mode | PRG bits | $46/47 | [1SM](Mirroring.xhtml#Single-Screen "Mirroring") | Extra WRAM | Address mask | PRG A21+ | CHR A21+ | CNROM latch   
0 | LP-8002KB, SFC-12B | **BMC-Super24in1SC03** | - | 6 | normal | - | - | $Fxx3 | - | - | - | _YH-xxx_ multicarts, _Rockman 6-in-1_  
1 | FK-xxx/BS-xxx | **BMC-FK23C** /**BMC-FK23CA** | $5FF3.1 | 8 | normal | - | - | $Fxx3 | - | - | yes | _FK-xxxx_ multicarts   
2 | FS005/FS006 | **WAIXING-FS005** | $5FF3.1 | 6 | swapped | yes | $A001.0-1 | $Fxx3 | $5xx0.3/7, $5xx2.6-7/5 | - | - | Waixing 2005+ re-releases, _245-in-1 Real Game_  
3 | JX9003B | - | - | 8 | normal | - | - | $Fxx7 | $5xx5 | $5xx6 | - | _Super Mario 160-in-1 Funny Time_  
4 | ? | - | - | 6 | normal | - | - | $Fxx3 | $5xx2.7 | - | - | _GameStar Smart Genius Deluxe_  
5 | HST-162 | - | - | 6 | normal | - | - | $Fxx3 | $4800 | - | - | _Game 500-in-1_  
  
  * The **SFC-12B** PCB (submapper 0) mounts both CHR-ROM and CHR-RAM that are selected via $5xx0.5.
  * Eight MMC3 PRG bits mean that bit 6 and 7 of $8000.6/7 are applied, and the fixed banks are $FE/$FF rather than $3E/$3F. This implies that the multicart's reset vector lies at address PRG-ROM address $1FFFFC rather than $7FFFC if the PRG-ROM is that large.



## Contents

  * 1 Registers
    * 1.1 Mode Register ($5xx0)
    * 1.2 PRG Base Register LSB ($5xx1)
    * 1.3 PRG Base Register MSB ($5xx5), Submapper 3 only
    * 1.4 PRG Base Register MSB ($4800), Submapper 5 only
    * 1.5 CHR Base Register LSB ($5xx2)
    * 1.6 CHR Base Register MSB ($5xx6), Submapper 3 only
    * 1.7 Extended Mode Register ($5xx3), Submappers 1-2 only
    * 1.8 Mirroring Register ($A000)
    * 1.9 RAM Configuration Register ($A001), Submapper 2 only
    * 1.10 UNROM latch ($8000-$FFFF)
    * 1.11 CNROM latch ($8000-$FFFF), Submapper 1 only
    * 1.12 MMC3-compatible registers, Extended MMC3 Mode ($8000/$8001, $C000/$C001, $E000/$E001)
  * 2 Solder Pad
  * 3 Protection (Submapper 2 only)
  * 4 See also



# Registers

**FS005 (Submapper 2)** can disable the registers in the $5000-$5FFF range using the RAM Configuration Register ($A001). The same register also enables a mixed CHR-ROM/CHR-RAM mode that is similar to [INES Mapper 195](INES_Mapper_195.xhtml "INES Mapper 195"). 

## Mode Register ($5xx0)

Mask: $F _xx_ 7 (**Submapper 3**), $5 _xx_ 3 (all others), _x_ determined by solder pad setting
    
    
    7654 3210
    ---- ----
    PCTm PMMM
    |||| ||||
    |||| |+++- Select PRG Banking Mode (ignored in Extended MMC3 Mode)
    |||| |      0: MMC3 PRG Mode, 2 MiB/512 KiB Outer PRG Bank Size
    |||| |      1: MMC3 PRG Mode, 256 KiB Outer PRG Bank Size
    |||| |      2: MMC3 PRG Mode, 128 KiB Outer PRG Bank Size
    |||| |      3: NROM-128 PRG Mode, 16 KiB PRG at $8000-$BFFF mirrored at $C000-$FFFF
    |||| |      4: NROM-256 PRG Mode, 32 KiB PRG at $8000-$FFFF
    |||| |      5: UNROM PRG Mode, 16 KiB PRG at $8000-$BFFF, inner bank #7 at $C000-$FFFF
    |||| |      6-7: Never used
    |||| +---- PRG A21 (**Submapper 2** only)
    |||+------ Select Outer CHR Bank Size
    |||         0: In MMC3 CHR Mode: 256 KiB
    |||            In CNROM CHR Mode: 32 KiB (**Submapper 1** only)
    |||         1: In MMC3 CHR Mode: 128 KiB
    |||            In CNROM CHR Mode: 16 KiB (**Submapper 1** only)
    ||+------- **Submappers 0/1** with both CHR-ROM and CHR-RAM (e.g. **SFC-12B** PCB):
    ||          0: Select CHR-ROM
    ||          1: Select CHR-RAM
    ||         **Submapper 1** with $5xx0.6=1:
    ||          0: Enable CNROM latch (CNROM mode)
    ||          1: Disable CNROM latch (NROM mode)
    |+-------- CHR A10-12 Mode
    |           0: from MMC3
    |           1: from PPU
    +--------- PRG A22 (**Submapper 2** only)
    
    Power-on value: $00
    

## PRG Base Register LSB ($5xx1)

Mask: $F _xx_ 7 (**Submapper 3**), $F _xx_ 3 (all others), _x_ determined by solder pad setting
    
    
    7654 3210
    ---- ----
    .PPP PPPP
     ||| ||||
     +++-++++- PRG A20..A14
    
    Power-on value: $00
    

Depending on the PRG banking mode set via $5xx0, only the higher bits of this register are applied: 
    
    
    PRG A..
    21111111
    09876543
    --------
    MMMMMMMM  Submappers 1/3 in PRG Mode 0 (MMC3 2 MiB) or Extended MMC3 Mode
    BBMMMMMM  Submappers 0/2/4 in PRG Mode 0 (MMC3 512 KiB)
    BBBMMMMM  PRG Mode 1 (MMC3 256 KiB)
    BBBBMMMM  PRG Mode 2 (MMC3 128 KiB)
    BBBBBBBC  PRG Mode 3 (NROM-128)
    BBBBBBCC  PRG Mode 4 (NROM-256)
    BBBBLLLC  PRG Mode 5 (UNROM)
    
    M: Bit comes from MMC3 ($8000.6/7 or fixed bank)
    B: Bit comes from PRG Base Register ($5xx1)
    L: Bit comes from UNROM Latch ($8000-$FFFF or fixed bank)
    C: Bit comes from CPU
    

**Submapper 5** only selects PRG A14-A18 (i.e. 512 KiB) via this register, selecting upper PRG bits via register $4800. 

## PRG Base Register MSB ($5xx5), Submapper 3 only

Mask: $F _xx_ 7 (**Submapper 3**), _x_ determined by solder pad setting
    
    
    7654 3210
    ---- ----
    .... PPPP
         ||||
         ++++- PRG Base A24..A21
    
    Power-on value: $00
    

## PRG Base Register MSB ($4800), Submapper 5 only

Mask: $F800 
    
    
    7654 3210
    ---- ----
    ..PP PPPP
      || ||||
      ++-++++- PRG Base A24..A19
    
    Power-on value: $00
    

## CHR Base Register LSB ($5xx2)

Mask: $F _xx_ 7 (**Submapper 3**), $5 _xx_ 3 (all others), _x_ determined by solder pad setting
    
    
    7654 3210
    ---- ----
    ccdC CCCC
    |||| ||||
    ++++-++++- CHR A20..A13
    ||+------- PRG A25 (**Submapper 2** only)
    ++-------- PRG A24..A23 (**Submapper 2** only)
    +--------- PRG A21 (**Submapper 4**)
    
    Power-on value: $00
    

Depending on the CHR banking mode set via $5xx0, only the higher bits of this register are applied: 
    
    
    CHR A..
    21111111111
    09876543210
    -----------
    BBBMMMMMMMM  MMC3 CHR Mode ($5xx0.6=0), 256 KiB ($5xx0.4=0)
    BBBBMMMMMMM  MMC3 CHR Mode ($5xx0.6=0), 128 KiB ($5xx0.4=1)
    BBBBBBLLPPP  CNROM CHR Mode ($5xx0.6=1, $5xx0.5=0), 32 KiB ($5xx0.4=0), **Submapper 1** only
    BBBBBBBLPPP  CNROM CHR Mode ($5xx0.6=1, $5xx0.5=0), 16 KiB ($5xx0.4=1), **Submapper 1** only
    BBBBBBBBPPP  NROM CHR Mode ($5xx0.6=1, $5xx0.5=1 or **Submapper** other than **1**)
    
    M: Bit comes from MMC3 ($8000.0-5)
    B: Bit comes from CHR Base Register ($5xx2)
    L: Bit comes from CNROM Latch ($8000-$FFFF)
    P: Bit comes from PPU
    

## CHR Base Register MSB ($5xx6), Submapper 3 only

Mask: $F _xx_ 7 (**Submapper 3**), _x_ determined by solder pad setting
    
    
    7654 3210
    ---- ----
    .... PPPP
         ||||
         ++++- CHR Base A24..A21
    
    Power-on value: $00
    

## Extended Mode Register ($5xx3), Submappers 1-2 only

Mask: $F _xx_ 3, _x_ determined by solder pad setting
    
    
    7654 3210
    ---- ----
    .?.. .?E.
           | 
           +- Extended MMC3 Mode
               0: disable
               1: enable
    
    Power-on value: $00
    

Most multicarts write value $44 rather than $00 to disable Extended MMC3 Mode. Hardware tests do not indicate any difference in behavior between writing $00 and $44. 

## Mirroring Register ($A000)
    
    
    Mask: $E003
    
    7654 3210
    ---- ----
    .... ..MM
           ++- Select nametable mirroring 
               0: Vertical
               1: Horizontal
               2: Single-screen, page 0 (**Submapper 2** only)
               3: Single-screen, page 1 (**Submapper 2** only)
    Power-on value: $00
    

Single-screen mirroring is only available when the RAM Configuration Register is enabled ($A001.5). 

## RAM Configuration Register ($A001), Submapper 2 only

Mask: $E003 

This register functions like MMC3 register $A001 until bit 5 is set, which turns it into the RAM Configuration Register. 
    
    
    7654 3210
    ---- ----
    RFE. SCWW
    |||  ||||
    |||  ||++- Select 8 KiB PRG-RAM bank at $6000-$7FFF. Ignored if Bit 5 is clear.
    |||  |+--- Select the memory type in the first 8 KiB of CHR space. Ignored if Bit 5 is clear.
    |||  |      0: First 8 KiB are CHR-ROM
    |||  |      1: First 8 KiB are CHR-RAM
    |||  +---- Unknown
    ||+------- RAM Configuration Register Enable
    ||          0: RAM Configuration Register disabled, $A001 functions as on MMC3, 8 KiB of WRAM
    ||          1: RAM Configuration Register enabled, 32 KiB of WRAM
    |+-------- Outer Bank Registers Enable. Ignored if Bit 5 is clear.
    |           0: Outer Bank Registers disabled, $5000-$5FFF maps to the second 4 KiB of the 8 KiB WRAM bank 2
    |           1: Outer Bank Registers enabled in the $5000-$5FFF range
    +--------- PRG RAM enable (0: disable, 1: enable)
    
    Power-on value: $00
    

## UNROM latch ($8000-$FFFF)

In UNROM Mode (PRG Mode 5), writing to this address range changes the 16 KiB inner PRG bank at $8000-$BFFF. 

## CNROM latch ($8000-$FFFF), Submapper 1 only

In CNROM Mode, writing to this address range changes the inner CHR bank. 

## MMC3-compatible registers, Extended MMC3 Mode ($8000/$8001, $C000/$C001, $E000/$E001)

Mask: $E003 (verified on real hardware) 

Some multicart games depend on writes to $9FFF not doing anything as a result of the MMC3 address mask being $E003 rather than the standard $E001. 

If the "Extended MMC3 Mode" bit in register $5xx3 is clear, then these registers function identically to the [MMC3](MMC3.xhtml "MMC3"). On **Submappers 1 and 2** , if the "Extended MMC3 Mode" bit is set, four more bank registers become available at [$8000/$8001](MMC3.xhtml#Bank_select_.28.248000-.249FFE.2C_even.29 "MMC3"), so that the original two 2 KiB CHR banks become four 1 KiB CHR banks, and the two fixed 8 KiB PRG banks become selectable, similar to the [RAMBO-1](RAMBO_1.xhtml "RAMBO-1"). Register $8000 if $5xx3 bit 1 is set: 
    
    
    7  bit  0
    ---- ----
    CP.. RRRR
    ||   ||||
    ||   ++++- Specify which bank register to update on next write to Bank Data register
    ||         $0: Select 1 KB CHR bank at PPU $0000-$03FF (or $1000-$13FF)
    ||         $1: Select 1 KB CHR bank at PPU $0800-$0BFF (or $1800-$1BFF)
    ||         $2: Select 1 KB CHR bank at PPU $1000-$13FF (or $0000-$03FF)
    ||         $3: Select 1 KB CHR bank at PPU $1400-$17FF (or $0400-$07FF)
    ||         $4: Select 1 KB CHR bank at PPU $1800-$1BFF (or $0800-$0BFF)
    ||         $5: Select 1 KB CHR bank at PPU $1C00-$1FFF (or $0C00-$0FFF)
    ||         $6: Select 8 KB PRG ROM bank at $8000-$9FFF (or $C000-$DFFF)
    ||         $7: Select 8 KB PRG ROM bank at $A000-$BFFF
    ||         $8: Select 8 KB PRG ROM bank at $C000-$DFFF (or $8000-$9FFF)
    ||         $9: Select 8 KB PRG ROM bank at $E000-$FFFF
    ||         $A: Select 1 KB CHR bank at PPU $0400-$07FF (or $1400-$17FF)
    ||         $B: Select 1 KB CHR bank at PPU $0C00-$0FFF (or $1C00-$1FFF)
    |+-------- Invert PRG A14
    +--------- Invert CHR A12
    
    Power-on values:
    * Standard MMC3 Registers $0-$7: $00, $02, $04, $05, $06, $07, $00, $01
    * Extended MMC3 Registers $8-$B: $FE, $FF, $FF, $FF
    

# Solder Pad

The address mask in the $5000-$5FFF range is determined by the solder pad setting: 
    
    
    Pad setting  Address mask
    -----------  ------------
    0            $5013
    1            $5023
    2            $5043
    3            $5083
    4            $5103
    5            $5203
    6            $5403
    7            $5803
    

  * A solder pad setting of zero (address mask $5013) will produce a usable result for any ROM image.
  * Some multicarts only display their menu at settings other than 0.



# Protection (Submapper 2 only)

Later Waixing games (and re-releases of earlier games) use the RAM Configuration Register for copy-protection purposes: 

  * Write $A1 to $A001: Address range $5000-$5FFF to second half of 8 KiB WRAM bank 2, mapper registers there are disabled.
  * Write three values to $5000, $5010 and $5013.
  * Do further initialization.
  * Write $E2 to $A001. Mapper registers in address range $5000-$5FFF; WRAM at CPU $6000-$7FFF points to 8 KiB WRAM bank 2.
  * Copy 20 bytes from $7000 to $6000.
  * Copy and XOR bytes from $6000, $6010 and $6013 to $0100-$0102.
  * Execute code at CPU $0100.



Hacked ROMs can be detected by them writing to $5000/$5010/$5013 but then no longer jumping to $0100. 

# See also

  * [NES 2.0 Mapper 523](NES_2_0_Mapper_523.xhtml "NES 2.0 Mapper 523") is a variant of this mapper with hard-wired mirroring that connects CHR-ROM differently to produce 4/2 KiB instead of 2/1 KiB banks.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with large PRG RAM](Category_Mappers_with_large_PRG_RAM.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
