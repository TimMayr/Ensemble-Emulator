# Bandai LZ93D50 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Bandai_LZ93D50_pinout) | View [other pages](Special_AllPages.xhtml#Bandai_LZ93D50_pinout)

  

    
    
               .---\/---.                      
            ?? |  1  52 | -- +5V                                       PPU A10 -> /  1  52 \ -> CHR A12? (=EXT PRG P31)
            ?? |  2  51 | -> PRG RAM /CS                              PPU A11 -> /  2    51 \ -> CHR A15?
    PRG /CE <- |  3  50 | ?? ?                               PPU A12? (=GND) -> /  3      50 \ <- PPU /RD? (=GND)
        +5V ?? |  4  49 | ?? +5V                            PPU A13? (=GND) -> /  4        49 \ <- CPU R/W
         M2 -> |  5  48 | ?? ?                                           ? ?? /  5          48 \ <- CPU D2
    CPU A13 -> |  6  47 | -> PRG A17                                  GND -- /  6            47 \ <- CPU D1
    CPU A14 -> |  7  46 | -> PRG A15                                 SDA ?? /  7              46 \ <- CPU D0
     CPU A3 -> |  8  45 | -> PRG A14                            SCL INT <- /  8                45 \ <- CPU /ROMSEL
     CPU A2 -> |  9  44 | -> PRG A16                          ? (=GND) -> /  9                  44 \ <- CPU A0
     CPU A1 -> | 10  43 | <- CPU D7                          CHR /CE? <- / 10                    43 \ <- CPU A1
     CPU A0 -> | 11  42 | <- CPU D6                         CHR A10? <- / 11                      42 \ <- CPU A2
    /ROMSEL -> | 12  41 | <- CPU D5                        CHR A16? <- / 12                        41 \ <- CPU A3
     CPU D0 -> | 13  40 | <> CPU D4            CHR A11? (=6264 P1) <- / 13                          40 \ ?? ?
     CPU D1 -> | 14  39 | <- CPU D3            CHR_A13? (=SCL_EXT) <- \ 14                          39 / <- CPU A14
     CPU D2 -> | 15  38 | -> /IRQ                          CHR A14? <- \ 15                        38 / <- CPU A13
        R/W -> | 16  37 | -> CIRAM A10                      CHR A17? <- \ 16                      37 / <- M2
    PPU /RD -> | 17  36 | -> CHR A17                        CIRAM A10 <- \ 17                    36 / ?? ?
    CHR A15 <- | 18  35 | -> CHR A14                                 ? ?? \ 18                  35 / -> PRG /CE (=EXT ROM /RD)
    CHR A12 <- | 19  34 | -> CHR A13                               /IRQ <- \ 19                34 / ?? ?
    PPU A10 -> | 20  33 | -> CHR A11                              CPU D3 -> \ 20              33 / -- GND
    PPU A11 -> | 21  32 | -> CHR A16                               CPU D4 -> \ 21            32 / -- VCC
    PPU A12 -> | 22  31 | -> CHR A10                                CPU D5 -> \ 22          31 / -> /$6000-$7FFF (=74368's/OE)
    PPU A13 -> | 23  30 | -> CHR /CE                                 CPU D6 -> \ 23        30 / ?? ?
        GND -- | 24  29 | <- **                                       CPU D7 -> \ 24      29 / ?? ?
          ? ?? | 25  28 | -> I²C SCL                                  PRG A16 <- \ 25    28 / -> PRG A17
        GND -- | 26  27 | <> I²C SDA                                   PRG A14 <- \ 26  27 / -> PRG A15
               '--------'                         
    Bandai LZ93D50: 52-pin shrink PDIP                                 Bandai LZ93D50P: QFP52 0.65mm pitch
    (Canonically [iNES Mapper 159](INES_Mapper_159.xhtml "INES Mapper 159"))                                   (Canonically [iNES Mapper 157](INES_Mapper_157.xhtml "INES Mapper 157"))                                      
                                                             
    ** Grounded on BA-JUMP2. Floating on other games.         * This chip is used only in Datach Joint System Adapter with
       Seems likely to be an enable for reading I²C from        non-canonical connections (marked as =...)
       $6000-$7fff (internal pullup, normally floating)       * Behaviour of pins ending with "?" is not confirmed, but it
                                                                is suggested by their location identical to PDIP version 
    

Bandai's variants of the LZ93D50 are almost as bad as [MMC1](MMC1_pinout.xhtml "MMC1 pinout"). 

[BA-JUMP2](INES_Mapper_153.xhtml "INES Mapper 153"): supports PRG RAM instead of I²C, replaces banked CHR-ROM with unbanked CHR-RAM, and increases PRG max to 512KiB: 
    
    
    PPU /RD -> | 17  36 | -> **n/c**
        **n/c** <- | 18  35 | -> **n/c**
        **n/c** <- | 19  34 | -> **n/c**
    PPU A10 -> | 20  33 | -> **n/c**
    PPU A11 -> | 21  32 | -> **n/c**
        **GND** -> | 22  31 | -> **PRG A18**
        **GND** -> | 23  30 | -> **n/c**
        GND -- | 24  29 | <- **GND**
          ? ?? | 25  28 | -> **PRG RAM +CS**
        GND -- | 26  27 | <> **n/c**
               '--------'
    

[Datach Joint ROM System](INES_Mapper_157.xhtml "INES Mapper 157"): Two separate I²C clocks, replaces banked CHR-ROM with unbanked CHR-RAM, and external barcode reader. 

This IC is actually a 52-pin QFP, with a slightly different pin order from the PDIP instantiation. The pertinent changes: 

  * I²C SCL is only for the internal EEPROM
  * CHR A13 is External I²C SCL
  * WRAM /CS drives a tristateable buffer that connects barcode data to CPU D3.



See also: 

  * <http://seesaawiki.jp/famicomcartridge/d/Bandai%20Datach>
  * <http://seesaawiki.jp/famicomcartridge/d/Bandai%20LZ93D50%20standard>



Categories: [Pinouts](Category_Pinouts.xhtml)
