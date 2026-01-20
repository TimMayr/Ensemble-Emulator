# MMC1 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC1_pinout) | View [other pages](Special_AllPages.xhtml#MMC1_pinout)

[MMC1](MMC1.xhtml "MMC1"): 24 pin shrink-DIP (most common mapper **1** ; variants as mappers [105](NES_EVENT.xhtml "INES Mapper 105") and [155](MMC1.xhtml "INES Mapper 155")) 

Comes in several varieties: 'MMC1', 'MMC1A', and 'MMC1B2' 
    
    
                    .--\/--.
     PRG A14 (r) <- |01  24| -- +5V
     PRG A15 (r) <- |02  23| <- M2 (n)
     PRG A16 (r) <- |03  22| <- CPU A13 (nr)
     PRG A17 (r) <- |04  21| <- CPU A14 (n)
     PRG /CE (r) <- |05  20| <- /ROMSEL (n)
    WRAM +CE (w) <- |06  19| <- CPU D7 (nrw)
     CHR A12 (r) <- |07  18| <- CPU D0 (nrw)
     CHR A13 (r) <- |08  17| <- CPU R/W (nw)
     CHR A14 (r) <- |09  16| -> CIRAM A10 (n)
     CHR A15 (r) <- |10  15| <- PPU A12 (nr)
     CHR A16 (r) <- |11  14| <- PPU A11 (nr)
             GND -- |12  13| <- PPU A10 (nr)
                    `------'
    

**Pinout legend**
    
    
        -- |  power supply   | --
        <- |     output      | ->
        -> |      input      | <-
        <> |  bidirectional  | <>
        ?? |   unknown use   | ??
    
    n or f - connects to NES or Famicom
     r - connects to ROMs (PRG ROM, CHR ROM, CHR RAM)
     w - connects to WRAM (PRG RAM)
    ?? - could be an input, no connection, or a supply line
    

## Contents

  * 1 Functional variations
    * 1.1 SEROM, SHROM, SH1ROM
    * 1.2 SNROM
    * 1.3 SOROM
    * 1.4 SUROM
    * 1.5 SXROM
    * 1.6 EVENT
    * 1.7 SZROM
    * 1.8 2ME
  * 2 Pirate clones



## Functional variations

As with several other ASIC mappers, parts of the pinout are often repurposed: 

### SEROM, SHROM, SH1ROM

Doesn't support PRG banking 
    
    
                    .--\/--.
             **n/c** <- |01  24| -- +5V
             **n/c** <- |02  23| <- M2 (n)
             **n/c** <- |03  22| <- CPU A13 (nr)
             **n/c** <- |04  21| <- CPU A14 (n)
    
           **CPU A14 (n) - > PRG A14 (r)**
    

### SNROM

Uses CHR A13-A16 as a PRG-RAM disable 
    
    
              **n/c** <- |08  17| <- CPU R/W (nw)
              **n/c** <- |09  16| -> CIRAM A10 (n)
              **n/c** <- |10  15| <- PPU A12 (n)
     **WRAM /CE (w)** <- |11  14| <- PPU A11 (nr)
              GND -- |12  13| <- PPU A10 (nr)
                     `------'
    

### SOROM

Uses CHR A13-A16 as PRG-RAM banking 
    
    
              **n/c** <- |08  17| <- CPU R/W (nw)
              **n/c** <- |09  16| -> CIRAM A10 (n)
     **WRAM A13 (w)** <- |10  15| <- PPU A12 (n)
              **n/c** <- |11  14| <- PPU A11 (nr)
              GND -- |12  13| <- PPU A10 (nr)
                     `------'
    

SOROM is actually implemented using the WRAMs' /CE inputs and an inverter to select only one RAM at a time. 

### SUROM

Uses CHR A13-A16 as PRG-ROM banking 
    
    
              **n/c** <- |08  17| <- CPU R/W (nw)
              **n/c** <- |09  16| -> CIRAM A10 (n)
              **n/c** <- |10  15| <- PPU A12 (n)
      **PRG A18 (r)** <- |11  14| <- PPU A11 (nr)
              GND -- |12  13| <- PPU A10 (nr)
                     `------'
    

### SXROM

Uses CHR A13-A16 as PRG-ROM and PRG-RAM banking 
    
    
              **n/c** <- |08  17| <- CPU R/W (nw)
     **WRAM A13 (w)** <- |09  16| -> CIRAM A10 (n)
     **WRAM A14 (w)** <- |10  15| <- PPU A12 (n)
      **PRG A18 (r)** <- |11  14| <- PPU A11 (nr)
              GND -- |12  13| <- PPU A10 (nr)
                     `------'
    

SXROM uses an external inverter to convert the MMC1's **WRAM +CE** to the 62256's **-CE** input 

### [EVENT](NES_EVENT.xhtml "NES-EVENT")

Uses CHR A13-A16 as more complicated PRG-ROM banking and timer control, also disables CHR RAM banking 
    
    
             **n/c** <- |07  18| <- CPU D0 (nrw)
        **PRG2 A15** <- |08  17| <- CPU R/W (nw)
        **PRG2 A16** <- |09  16| -> CIRAM A10 (n)
         **PRG SEL** <- |10  15| <- PPU A12 (n)
     **TIMER RESET** <- |11  14| <- PPU A11 (nr)
             GND -- |12  13| <- PPU A10 (nr)
                    `------'
    

Since the PPU A12 input's only purpose is to switch the CHR A12 .. A16 outputs, it's not clear why Nintendo didn't tie the MMC1's PPU A12 input low and connect CHR A12 directly to PPU A12. Doing so would have cost nothing (the ability to swap the two nametables is already granted through [PPUCTRL](PPU_registers.xhtml "PPUCTRL")), would have prevented mistakes (unless the same value is in both CHR registers, 4KB mode causes erratic switching of bank during rendering), and would have freed up another bit of control. 

### SZROM

Uses CHR A16 as PRG-RAM banking 
    
    
          CHR A15 <- |10  15| <- PPU A12 (n)
     **WRAM A13 (w)** <- |11  14| <- PPU A11 (nr)
              GND -- |12  13| <- PPU A10 (nr)
                     `------'
    

SZROM permits both CHR ROM banking and 16KB of WRAM at the same time. Like SOROM, it's implemented using the WRAMs' /CE inputs and an inverter to select only one RAM at a time. 

### 2ME

Uses CIRAM A10 and CHR A12, A13, A16 for EEPROM access and CHR A14, A15, and A16 for PRG-RAM access 
    
    
                **$6000-7FFF +EN (we)** <- |06  19| <- **Card D7 (nrw)**
                      **EEPROM DI (e)** <- |07  18| <- **Card D0 (nrwe)**
                     **EEPROM CLK (e)** <- |08  17| <- **Card R/W (nw)**
                    **PRG-RAM A13 (w)** <- |09  16| -> **EEPROM CS (e)**
                    **PRG-RAM A14 (w)** <- |10  15| <- **GND (PPU A12)**
    **EEPROM DO +OE, PRG-RAM /CE (we)** <- |11  14| <- **GND (PPU A11)**
                                GND -- |12  13| <- **GND (PPU A10)**
                                       `------'
    

The data bus here is the Famicom Network System's card data bus, not the CPU data bus. Because the board is not on the PPU bus, the PPU address inputs are grounded, preventing the CHR bank 1 register from being used. 

## Pirate clones

There exist pirate clones: 

  * AX5904 - pinout is same as official MMC1
  * KS5361 - pinout unknown [[1]](http://forums.nesdev.org/viewtopic.php?p=83737#p83737)
  * [KS 203](https://forums.nesdev.org/viewtopic.php?f=9&t=16819#p209968) \- Kaiser clone:


    
    
                  .--\/--.
       PRG A17 <- |01  24| <- PPU A10
       PRG A16 <- |02  23| <- PPU A11
       PRG A15 <- |03  22| <- PPU A12
       PRG A14 <- |04  21| <- CPU A13
           VCC -- |05  20| <- CPU A14
       PRG /CE <- |06  19| <- /ROMSEL
       CHR A12 <- |07  18| <- RESET
       CHR A13 <- |08  17| -- GND
        CPU D7 -> |09  16| <- M2
        CPU D0 -> |10  15| <- CPU R/W
       CHR A16 <- |11  14| -> CHR A15
       CIR A10 <- |12  13| -> CHR A14
                  `------'
                   KS 203
    

Categories: [Pinouts](Category_Pinouts.xhtml)
