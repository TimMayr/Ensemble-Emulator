# INES Mapper 083

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_083) | View [other pages](Special_AllPages.xhtml#INES_Mapper_083)

iNES Mapper 83 is used for games from Cony, also known as Yoko. There are three somewhat incompatible subtypes, denoted by [submappers](NES_2_0_submappers_Proposals.xhtml#083:_Cony "NES 2.0 submappers/Proposals"), although the proper subtype is easy to heuristically discern by looking at the CHR-ROM size: 

  * 256 KiB CHR-ROM => **Submapper 0** (1 KiB CHR-ROM banking, no WRAM) 
    * Street Fighter II Pro/Street Blaster II Pro
    * Street Fighter IV Pro 10/Street Blaster IV Pro 10
    * Street Blaster V Turbo 20
    * Street Fighter X Turbo 40
    * Fatal Fury 2/餓狼伝説 2
    * Fatal Fury 2'/餓狼伝説 2'
  * 512 KiB CHR-ROM => **Submapper 1** (2 KiB CHR-ROM banking, no WRAM) 
    * Super Blaster VII Turbo 28
    * World Heroes 2/快打英雄榜 2
    * World Heroes 2 Pro//快打英雄榜 2 Pro
  * 1024 KiB CHR-ROM => **Submapper 2** (1 KiB CHR-ROM banking with outer bank, 32 KiB banked WRAM) 
    * Dragon Ball Party



A fourth Yoko mapper, functionally similar but with different register addresses, has been assigned to [NES 2.0 Mapper 264](NES_2_0_Mapper_264.xhtml "NES 2.0 Mapper 264"). 

## Contents

  * 1 Registers
    * 1.1 Mode Register ($8100, write)
    * 1.2 PRG-ROM registers 0-3 ($8300-$8303, write)
    * 1.3 PRG-ROM register 4/Outer Bank register ($8000, write)
    * 1.4 CHR-ROM registers ($8310-$8317, write)
    * 1.5 DIP Switch ($5000, read)
    * 1.6 Scratch RAM ($5100-$5FFF, read/write)
    * 1.7 IRQ Acknowledge and Counter Low ($8200)
    * 1.8 IRQ Enable and Counter High ($8201)
  * 2 IRQ Operation



# Registers

## Mode Register ($8100, write)

Mask: $8300 
    
    
    7654 3210
    ---------
    ED6M M.mm
    |||| | ++- Select nametable mirroring type
    |||| |     0: Vertical
    |||| |     1: Horizontal
    |||| |     2: One screen, page 0
    |||| |     3: One screen, page 1
    |||+-+---- Select PRG banking mode
    |||        0: 16 KiB PRG-ROM bank at $8000-$BFFF, determined by PRG register 4 ($8000)
    |||           16 KiB PRG-ROM bank at $C000-$FFFF, fixed to last bank
    |||        1: 32 KiB PRG-ROM bank at $8000-$FFFF, determined by PRG register 4 ($8000) SHR 1
    |||        2: 8 KiB PRG-ROM bank at $8000-$9FFF, determined by PRG register 0 ($8300)
    |||           8 KiB PRG-ROM bank at $A000-$BFFF, determined by PRG register 1 ($8301)
    |||           8 KiB PRG-ROM bank at $C000-$DFFF, determined by PRG register 2 ($8302)
    |||           8 KiB PRG-ROM bank at $E000-$FFFF, fixed to last bank
    |||        3: same as mode 2
    ||+------- **Submappers 0 and 1** : Enable/disable 8 KiB PRG-ROM at $6000-$7FFF
    ||         0: Disabled, open bus at $6000-$7FFF
    ||         1: 8 KiB PRG-ROM bank at $6000-$7FFF, determined by PRG register 3
    ||         **Submapper 2** : no function
    |+-------- IRQ counting mode
    |          0: Increment
    |          1: Decrement
    +--------- IRQ Enable latch
               0: Disabled
               1: Enabled
    

In **Submapper 2** , PRG banking as described above applies to the inner bank within the 256 KiB outer bank selected by the Outer Bank register ($8000) bits 4-5. 

## PRG-ROM registers 0-3 ($8300-$8303, write)

Mask: $8313 
    
    
    $8300: PRG register 0 (8 KiB PRG-ROM bank at $8000-$9FFF in PRG banking mode 2)
    $8301: PRG register 1 (8 KiB PRG-ROM bank at $A000-$BFFF in PRG banking mode 2)
    $8302: PRG register 2 (8 KiB PRG-ROM bank at $C000-$DFFF in PRG banking mode 2)
    $8303: PRG register 3 (8 KiB PRG-ROM bank at $6000-$7FFF if Mode Register ($8100) bit 5 is set
           (**Submappers 0 and 1** only)
    

## PRG-ROM register 4/Outer Bank register ($8000, write)

Mask: $8300 
    
    
    7654 3210
    ---------
    WWOO PPPP
    |||| ++++- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF in PRG mode 0
    |||| +++-- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF in PRG mode 1
    ||++------ **Submapper 2** only: Select 256 KiB Outer PRG/CHR-ROM bank
    ++-------- **Submapper 2** only: Select 8 KiB WRAM bank at CPU $6000-$7FFF
               
    
    

## CHR-ROM registers ($8310-$8317, write)

Mask: $831F 

**Submapper 0 and 2** : 
    
    
    $8310: Select 1 KiB CHR-ROM bank at PPU $0000-$03FF
    $8311: Select 1 KiB CHR-ROM bank at PPU $0400-$07FF
    $8312: Select 1 KiB CHR-ROM bank at PPU $0800-$0BFF
    $8313: Select 1 KiB CHR-ROM bank at PPU $0C00-$0FFF
    $8314: Select 1 KiB CHR-ROM bank at PPU $1000-$13FF
    $8315: Select 1 KiB CHR-ROM bank at PPU $1400-$17FF
    $8316: Select 1 KiB CHR-ROM bank at PPU $1800-$1BFF
    $8317: Select 1 KiB CHR-ROM bank at PPU $1C00-$1FFF
    $8318-$831F: No function
    

In **Submapper 2** , these registers select the inner CHR-ROM bank, while the Outer Bank register ($8000) bits 4-5 select the 256 KiB outer CHR-ROM bank. 

**Submapper 1** : 
    
    
    $8310: Select 2 KiB CHR-ROM bank at PPU $0000-$07FF
    $8311: Select 2 KiB CHR-ROM bank at PPU $0800-$0FFF
    $8312-$8315: No function
    $8316: Select 2 KiB CHR-ROM bank at PPU $1000-$17FF
    $8317: Select 2 KiB CHR-ROM bank at PPU $1800-$1FFF
    $8318-$831F: No function
    

## DIP Switch ($5000, read)

Mask: probably $DF00 
    
    
    7654 3210
    ---------
    .... ..DD
           ++- DIP switch setting
    

Several games display different names (_Street Fighter_ versus _Street Blaster_) depending on the DIP switch setting. Some games will display a garbled title screen at some DIP switch settings, and the "good" setting is not always 0 or 3! 

## Scratch RAM ($5100-$5FFF, read/write)

Mask: probably $DF03 
    
    
    $5xx0: Scratch RAM byte #0
    $5xx1: Scratch RAM byte #1
    $5xx2: Scratch RAM byte #2
    $5xx3: Scratch RAM byte #3
    

## IRQ Acknowledge and Counter Low ($8200)

Mask: $8301 

A write directly changes the low eight bits of the 16-bit IRQ counter and acknowledges the IRQ. 

## IRQ Enable and Counter High ($8201)

Mask: $8301 

A write directly changes the high eight bits of the 16-bit IRQ counter, and copies bit 7 of the Mode register to the actual IRQ Enable register (which is not directly accessible). 

# IRQ Operation

If the IRQ counter is enabled (by writing a value to $8201 after setting bit 7 in the Mode register), and the counter is not zero, the counter is increased (Mode register bit 6 clear) or decreased (Mode register bit 6 set) on every M2 cycle. If it reaches zero, an IRQ is raised, and the IRQ counter is automatically disabled. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with large PRG RAM](Category_Mappers_with_large_PRG_RAM.xhtml)
