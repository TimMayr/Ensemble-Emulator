# LH2833 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/LH2833_pinout) | View [other pages](Special_AllPages.xhtml#LH2833_pinout)

LH2833: custom DRAM chip 28-pin 0.6" DIP 
    
    
                              .---\/---.
    (tied to CPU M2) +CS1? -> | 01  28 | -- +5V
                      /RAS -> | 02  27 | <- +CS2? (tied to CPU A13)
                    PRG A0 -> | 03  26 | <- +CS3? (tied to CPU A14)
                PRG A3/A10 -> | 04  25 | <- +CS4? (tied to CPU /ROMSEL)
                PRG A2/A9  -> | 05  24 | <- PRG A4/A11
                PRG A1/A8  -> | 06  23 | <- PRG A5/A12
                    CPU A7 -> | 07  22 | <- PRG A6/A13
                    /CAS0  -> | 08  21 | <- R/W 
                    /CAS1  <- | 09  20 | <- /CS? (tied to GND)
                              | 10  19 | <> CPU D4
                    CPU D5 <> | 11  18 | <> CPU D3
                    CPU D6 <> | 12  17 | <> CPU D2
                    CPU D7 <> | 13  16 | <> CPU D1
                       GND -- | 14  15 | <> CPU D0
                              '--------'
    
    Chip is enabled for $6000-$DFFF, so pins 1, 25, 26, 27 must be
    internally used as address inputs to chip enable:
    
    CPU M2 | /ROMSEL | A14 | A13 || Enabled?
       1   |    1    |  1  |  1  ||    Y
       x   |    0    |  0  |  0  ||    Y
       x   |    0    |  0  |  1  ||    Y
       x   |    0    |  1  |  0  ||    Y
    

Categories: [Pinouts](Category_Pinouts.xhtml)
