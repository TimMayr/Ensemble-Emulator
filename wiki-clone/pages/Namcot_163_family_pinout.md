# Namcot 163 family pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Namcot_163_family_pinout) | View [other pages](Special_AllPages.xhtml#Namcot_163_family_pinout)

The Namcot 129, 160, and 163 seem to have identical pinning. The Namcot 175 and 340 have minor but vital differences. 

[Namcot 129, 160, and 163](INES_Mapper_019.xhtml "Namco 163"): 48-pin QFP (canonically [iNES Mapper 019](INES_Mapper_019.xhtml "INES Mapper 019")) 
    
    
                                 _____
                                /     \
               (f) SOUND   <-  / 1  48 \ -> CHR A14 (r)
              (r) CHR A13 <-  / 2    47 \ -> CHR A15 (r)
             (r) CHR A12 <-  / 3   O  46 \ -> CHR A16 (r)
            (r) CHR A11 <-  / 4        45 \ -> CHR A17 (r)
          (fr)*CHR A10 <-  / 5          44 \ -> $F000.6,7
                  GND --  / 6            43 \ -- +5Vcc
         (r) PRG A18 <-  / 7              42 \ -> CIRAM /CE (f)
        (r) PRG A17 <-  / 8                41 \ -> CHR ROM /CE (r)
       (r) PRG A16 <-  / 9                  40 \ -> WRAM /CE (w)
      (r) PRG A15 <-  / 10                   39 \ -> /IRQ (f)
     (r) PRG A14 <-  / 11        NAMCO        38 \ <- PPU A10 (f)
    (r) PRG A13 <-  / 12    129/163/175/340    37 \ <- PPU A11 (f)
                   /                               \
                   \        Package QFP-48,        /
    (fr)  CPU D7 <> \ 13      0.8mm pitch      36 / <- PPU A12 (f)
     (fr)  CPU D6 <> \ 14                     35 / <- PPU A13 (f)
      (fr)  CPU D5 <> \ 15                   34 / <- 1: apply pin 33 to pin 42 as well
       (fr)  CPU D4 <> \ 16                 33 / <- PPU /RD (f) or GND (see below)
        (fr)  CPU D3 <> \ 17               32 / -- +5Vcc
         (fr)  CPU D2 <> \ 18             31 / -- GND
                 +5Vcc -- \ 19           30 / <- M2 (f)
           (fr)  CPU D1 <> \ 20         29 / <- R/W (fw)
            (fr)  CPU D0 <> \ 21       28 / <- /ROMSEL (f)
                 /$E000.7 <- \ 22     27 / <- CPU A14 (f)
              (r) PRG /CE  <- \ 23   26 / <- CPU A13 (f)
              (fr) CPU A11  -> \ 24 25 / <- CPU A12 (fr)
                                \     /
                                 \   /
                                  \ /
                                   V
    
    01 Some Namcot 163 games use expansion audio, which uses a common mixing circuit:
               From N163------------------+
               (N163.1)                   |
                            R1            |
               From 2A03---/\/\/---+------+--------To RF
               (Cart.45)           |               (Cart.46)
                                  --- C1
                                  ---
                                   |
                                  GND
       R1 appears to be a set of values which may vary in each board (4.7k, 10k, 15k, 22k, 33k).
       C1 may or may not be present in each board. The boards that do have C1 present seem to be a set of values which may vary (1n, 1.6n).
       Note that caps in other games are only noted to be present, with its value not yet measured.
    
    05 also connects to CIRAM A10.
    19,32,43 Some boards connect a battery (through a standard diode switch) to this pin to make the waveform memory nonvolatile:
                                                VCC
                                                |E
                               +--330k--------|< PNP
                               |                |C
               GND---3.3VBAT---+--10k---|>|-+   +-- N163 pin 32
                                            |
               VCC----------------------|>|-+--VBB (N163 pins 19 & 43, WRAM pin 28)
       The role of transistor is to cut off the pin 32 from VCC when voltage is below battery level.
    
    22 This pin is a mysterious open-collector output, reflecting the inverse of bit 7 in register $E000.
    30 Pull-down resistor present on boards with battery for low-power mode.
    33 Ground this pin if CHR's /OE and /CS lines are separate.
    34 If this pin is high, pin 33 is applied to the CIRAM /CE output also. Unclear when this would be useful; if pin 33 is PPU /RD it would write-protect CIRAM.
    44 PPU A12, A13, and bits 6 and 7 in register $F000 control this pin. Unclear how to use this.
    
    (r) - this pin connects to the ROM chips
    (f) - this pin connects to the Famicom connector
    (w) - this pin connects to the WRAM
    
    Internal Vcc Connections:
    19 ------- 43
    32 --|>|-- 19/43 (0.7V diode drop)
    Powering only 19/43 does not allow any digital operation.
    

Namcot 175: 48-pin QFP (canonically [iNES Mapper 210](INES_Mapper_210.xhtml "INES Mapper 210")) 
    
    
    01 sound was removed
    05 CIRAM A10 is hardwired to PPU A10 or A11 as appropriate
    39 does not provide IRQs
    42 does not allow for ROM nametables
    

Namcot 340: 48-pin QFP (also [iNES Mapper 210](INES_Mapper_210.xhtml "INES Mapper 210")) 
    
    
     42\ -> **?**
      41\ -> **?**
       40\ -> **CHR /CE (r)**
        39\ -> **CIRAM A10 (f)**
    01 sound was also removed.
    

## References

  * [plgDavid's N163 cartridge details related to audio](https://docs.google.com/spreadsheets/d/12qGAqMeQFleSPEIZFlxrPzH3MTRKXEa1LIQPXSFD4tk) \- a table which lists values of expansion audio circuit components in most N163 games.



Categories: [Pinouts](Category_Pinouts.xhtml)
