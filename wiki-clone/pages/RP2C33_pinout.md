# RP2C33 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/RP2C33_pinout) | View [other pages](Special_AllPages.xhtml#RP2C33_pinout)

[Family Computer Disk System](Family_Computer_Disk_System.xhtml "Family Computer Disk System") ASIC RP2C33 or RP2C33A: 64-pin shrink DIP (FDS files) 
    
    
                     .---\/---.
     (Rf) /ROMSEL -> | 01  64 | -- +5V
     (Rf) CPU A14 -> | 02  63 | <- XTAL2
     (Rf) CPU A13 -> | 03  62 | -> XTAL1
      (f) CPU A12 -> | 04  61 | <- /EnableRAS (gnd)
      (f) CPU A11 -> | 05  60 | -> /RAS (r)
      (f) CPU A10 -> | 06  59 | -> /CAS1 (r)
       (f) CPU A9 -> | 07  58 | -> /CAS0 (r)
       (f) CPU A8 -> | 08  57 | <- R/W (rf)
    (r) PRG A6/13 <- | 09  56 | <- M2 (Rf)
    (r) PRG A5/12 <- | 10  55 | +> /IRQ (f)
    (r) PRG A4/11 <- | 11  54 | -> Audio (f)
    (r) PRG A3/10 <- | 12  53 | <- +TestMode (gnd)
     (r) PRG A2/9 <- | 13  52 | +> SER OUT
     (r) PRG A1/8 <- | 14  51 | <- SER IN
       (r) PRG A0 <- | 15  50 | +> $4025W.2 (Disk 0=Write, 1=Read)
      (rf) CPU A7 -> | 16  49 | +> $4025W.1 (Motor 0=Start, 1=Stop)
       (f) CPU A6 -> | 17  48 | +> $4025W.0 (1=Reset transfer timing)
       (f) CPU A5 -> | 18  47 | <- $4032R.2 (1=Write protected)
       (f) CPU A4 -> | 19  46 | <- $4032R.1 (1=Disk not ready)
       (f) CPU A3 -> | 20  45 | <- $4032R.0 (1=Disk missing)
       (f) CPU A2 -> | 21  44 | <+> EXT0
       (f) CPU A1 -> | 22  43 | <+> EXT1
       (f) CPU A0 -> | 23  42 | <+> EXT2
          /CE4100 <- | 24  41 | <+> EXT3
      (rf) CPU D0 <> | 25  40 | <+> EXT4
      (rf) CPU D1 <> | 26  39 | <+> EXT5
      (rf) CPU D2 <> | 27  38 | <+> EXT6
      (rf) CPU D3 <> | 28  37 | <+> EXT7/BATT
      (rf) CPU D4 <> | 29  36 | <- PPU A10 (f)
      (rf) CPU D5 <> | 30  35 | <- PPU A11 (f)
      (rf) CPU D6 <> | 31  34 | -> CIRAM A10 (f)
              GND -- | 32  33 | <> CPU D7 (rf)
                     '--------'
    R - connects to LH2833 DRAM in later PCB revisions
    r - connects to DRAM (all revisions)
    f - connects to Famicom
    + - output is open-drain, requires an external pull-up resistor
    
    Expansion audio mixing circuit schematic:
                                 1.2M
                             +---/\/---+
                    +        |         |       56K
    From RP2C33---||---/\/---+---|>o---+---+---/\/---+
    (RP2C33.54)   1u    2M                 |         |
                                     0.1u ---        |
                                          ---        |
                                           |         |            100K
                                          GND        |        +---/\/---+
                                                     |     +  |         |
    From 2A03----------------------------------/\/---+---||---+---|>o---+---To RF
    (Cart.45)                                  56K      1u              (Cart.46)
    

Notes: 

  * PRG address bus is multiplexed, as is typical for DRAMs.
  * EXT port can be written via $4026 and read via $4033



Originally transcribed from <https://web.archive.org/web/20140920224500/green.ap.teacup.com/junker/119.html> ([PDF rehosted on forum](https://forums.nesdev.org/viewtopic.php?t=25219)) 

Categories: [Pinouts](Category_Pinouts.xhtml)
