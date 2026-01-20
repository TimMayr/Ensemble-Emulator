# User:Ben Boldt

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ABen_Boldt) | View [other pages](Special_AllPages.xhtml#User_Ben_Boldt)

Winning isn't fun. It is game over. 

See my [Wikipedia User Page](http://en.wikipedia.org/wiki/User:Ben_Boldt). 

See my [contributions](http://datacrystal.romhacking.net/wiki/Special:Contributions/Ben_Boldt) at DataCrystal. 

See my [college website](https://benboldtumd.github.io/index.html) (archived). 

  

    
    
                                 _____
                                /     \
                     CPU A0 -> / 1  44 \ -- CN09.10
                    CPU A1 -> / 2    43 \ -- CN09.11
                   CPU A2 -> / 3      42 \ -- CN09.12
                  CPU A3 -> / 4        41 \ -- CN09.13
                 CPU A4 -> / 5          40 \ -- CN09.14 to C09 to 5V
                CPU A5 -> / 6            39 \ -- CN05.16
               CPU A6 -> / 7              38 \ -- CN09.15, IC10.8 (IC10.9, 1C10.10 = /IRQ, so this should be IRQ)
              CPU A7 -> / 8                37 \ -- CN05.12, SW04.3
         A8 NAND A9 -> / 9                  36 \ -- CN05.13 (via R84 -> L18)
      A10 NAND A11 -> / 10                   35 \ -- CN05.15 (via R85 -> L20)
          CPU A12 -> / 11    Ricoh RF5GH05    34 \ -- GND
                    /        Package QFP-44       \
                    \         0.8mm pitch         /
          CPU A13 -> \ 12                     33 / -- CN06.9
           CPU A14 -> \ 13                   32 / -- CN06.8
            CPU A15 -> \ 14                 31 / -- CN06.7
              CPU D0 <> \ 15               30 / -- CN06.6
               CPU D1 <> \ 16             29 / -- CN06.5
                    M2 -> \ 17           28 / -- CN06.4
                CPU R/W -> \ 18         27 / -- CN06.3
                 /ROMSEL <- \ 19       26 / -- CN06.2
              CPU RAM /CE <- \ 20     25 / -- CN06.1
                   PPU /CE <- \ 21   24 / -- +5V
                        +5V -- \ 22 23 / -- CN06.11 via R28
                                \     /
                                 \   /
                                  \ /
                                   V
    Note that CN09 is unpopulated.
    
