# NES 2.0 Mapper 416

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_416) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_416)

**NES 2.0 Mapper 416** denotes the PCB used for a 4-in-1 multicart that includes the [N-32 cartridge conversion](INES_Mapper_050.xhtml "INES Mapper 050") of SMB2J. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Data Latch
    * 2.2 PRG Bank Select (SMB2J mode only)
    * 2.3 IRQ Control (SMB2J mode only)



# Banks

  * CPU $6000-$7FFF: Fixed 8 KiB PRG-ROM bank #7
  * CPU $8000-$FFFF: Switchable 8/16/32 KiB PRG-ROM bank(s)
  * PPU $0000-$1FFF: 8 KiB switchable CHR-ROM bank



# Registers

## Data Latch
    
    
    Write to $8000 (mask $8000):
    D~[BPA. Mcc.]
       |||  |++--- CHR A14..A13
       |||  |+---- 1=Horizontal, 0=Vertical mirroring
       |||  +----- PRG A16, also select SMB2J (0)/Normal (1) mode
       ||+-------- PRG A14 in Normal mode
       +|--------- PRG A15 in Normal mode
       ++--------- 00: NROM-064 banking (PRG A13=0, PRG A14=A)
                   01: NROM-128 banking (PRG A13=CPU A13, PRG A14=A)
                   1x: NROM-256 banking (PRG A13=CPU A13, PGR A14=CPU A14)
    

## PRG Bank Select (SMB2J mode only)
    
    
    Write to $4020 (mask unknown)
    D~[.... .BAC]
             ||+- PRG A15
             ++-- PRG A14..13
    

## IRQ Control (SMB2J mode only)
    
    
    Write to $4120 (mask unknown)
    D~[.... ...E]
               +- 1=Counting, generate IRQ on 12-bit counter overflow
                  0=Counter reset, acknowledge/disable IRQ
    

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
