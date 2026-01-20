# VRC5 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC5_pinout) | View [other pages](Special_AllPages.xhtml#VRC5_pinout)

Konami VRCV: 80-pin QFP (canonically [mapper 547](NES_2_0_Mapper_547.xhtml "NES 2.0 Mapper 547")) 
    
    
                                      _____
                           CPU A8 -> /01 80\ ? (goes to QTA 39, maybe PRG RAM A14?)
                             VCC -- /02   79\ <> CPU D0
                         CPU A9 -> /03 (o) 78\ <> CPU D1
                       CPU A10 -> /04       77\ <> CPU D2
                      CPU A11 -> /05         76\ <> CPU D3
                     CPU A12 -> /06           75\ <> CPU D4
                    CPU A13 -> /07             74\ <> CPU D5
                   CPU A14 -> /08               73\ -- VCC
                  CPU R/W -> /09                 72\ <> CPU D6
               CPU /RMSL -> /10                   71\ <> CPU D7
                     M2 -> /11                     70\ <- PPU /WE
                   GND -- /12                       69\ <- PPU /RD
                 /IRQ <- /13                         68\ ? (goes to QTA 18, maybe PRG RAM A13?)
             CIR A10 <- /14                           67\ -> PRG RAM A12
        CHR RAM A12 <- /15                             66\ -> EXT PRG ROM0 /CE
            PPU A3 -> /16                               65\ -> EXT PRG ROM1 /CE
           PPU D0 <> /17                                64/ -> EXT PRG ROM2 /CE
          PPU D1 <> /18                                63/ -- GND
         PPU D2 <> /19         VRC 5                  62/ -> INT PRG ROM /CE
        PPU D3 <> /20                                61/ -> PRG A17
       PPU D4 <> /21                                60/ -> PRG A16
      PPU D5 <> /22                                59/ -> PRG A15
        GND -- /23                                58/ -> PRG A14
    PPU D6 <> /24                                57/ -> PRG A13
    PPU D7 <> \25                               56/ -> QTRAM /WE
     PPU A7 -> \26                             55/ -> INT PRG RAM /CE
      PPU A8 -> \27                           54/ -> EXT PRG RAM /CE
       PPU A9 -> \28                         53/ -> CHR ROM A16
       PPU A10 -> \29                       52/ -- GND
        PPU A11 -> \30                     51/ -> CHR ROM A15
         PPU A12 -> \31                   50/ -> CHR ROM A14
          PPU A13 -> \32                 49/ -> CHR ROM A13
               VCC -- \33               48/ -> CHR ROM A12
          CIRAM /CE <- \34             47/ -> CHR ROM A11
           QTRAM +CE <- \35           46/ <> QTRAM D7
          CHR ROM /CS <- \36         45/ <> QTRAM D6
           CHR RAM /CS <- \37       44/ <> QTRAM D5
               QTRAM D0 <> \38     43/ <> QTRAM D4
                QTRAM D1 <> \39   42/ -- GND
                 QTRAM D2 <> \40 41/ <> QTRAM D3
                              \   /
                               \ /
    

**Notes:**

  * CPU R/W (pin 9) and CPU /RMSL (pin 10) might be reversed



**Source:**

  * [BBS](http://forums.nesdev.org/viewtopic.php?f=2&t=19237&start=15#p242297)



Categories: [Pinouts](Category_Pinouts.xhtml)
