# UM6561 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UM6561_pinout) | View [other pages](Special_AllPages.xhtml#UM6561_pinout)

80-pin QFP 
    
    
                                        _____
                             PPU D0 <> /01 80\ -> PPU A13
                            PPU D7 <> /02   79\ -> PPU A0
                           PPU D1 <> /03 (o) 78\ -> PPU A12
                          PPU D6 <> /04       77\ -> PPU A1 
                         PPU D2 <> /05         76\ -> PPU A11
                        PPU D5 <> /06           75\ -> PPU A2
                       PPU D3 <> /07             74\ -> PPU A10
                      PPU D4 <> /08               73\ -- GND
                      XTAL1 -> /09                 72\ -> PPU A3
                     XTAL2 -> /10                   71\ -> PPU A9
                      +5V -- /11                     70\ -> PPU A4
               VIDEO OUT <- /12                       69\ -> PPU A8	
          AUDIO AMP OUT <- /13                         68\ -> PPU A5
          AUDIO AMP IN -> /14                           67\ -> PPU A7
            AUDIO OUT <- /15                             66\ -> PPU A6
                 GND -- /16                               65\ -> PPU /A13
             /RESET -> /17                                64/ <- CIRAM A10
                ?? -> /18                                63/ <- CIRAM /CS
         $4016 D2 -> /19         UM6561                 62/ -> PPU /RD
        $4016 D0 -> /20                                61/ -> PPU /WE
       $4017 D0 -> /21                                60/ -- GND
      $4017 D1 -> /22                                59/ <- /IRQ
    $4016 CLK <- /23                                58/ -> CPU /ROMSEL
    $4017 D2 -> /24                                57/ -> CPU R/W
    $4016 D1 -> \25                               56/ <> CPU D0
     $4017 D3 -> \26                             55/ -> CPU A0
          OUT0 <- \27                           54/ <> CPU D1
       $4017 D4 -> \28                         53/ -> CPU A1
            OUT1 <- \29                       52/ <> CPU D2
             OUT2 <- \30                     51/ -> CPU A2
         $4017 CLK -> \31                   50/ <> CPU D3
                +5V -- \32                 49/ -> CPU A3
             CPU A11 <- \33               48/ <> CPU D4
               CPU M2 <- \34             47/ -> CPU A4
               CPU A10 <- \35           46/ <> CPU D5  
                CPU A12 <- \36         45/ -> CPU A5
                  CPU A9 <- \37       44/ <> CPU D6   
                  CPU A13 <- \38     43/ -> CPU A6
                    CPU A8 <- \39   42/ <> CPU D7
                    CPU A14 <- \40 41/ -> CPU A7  
                                \   /
                                 \ /
    

Notes: 

  * UM6561AF-2: Dendy (50 Hz)
  * UM6561F-1: NTSC [[1]](https://forums.nesdev.org/viewtopic.php?f=9&t=18984)
  * UM6561F-2: Dendy (50Hz) [[2]](https://forums.nesdev.org/viewtopic.php?f=9&t=15342#p187941)



Categories: [Pinouts](Category_Pinouts.xhtml)
