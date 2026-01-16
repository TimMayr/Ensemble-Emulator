# Bandai M60001-0115P pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Bandai_M60001-0115P_pinout) | View [other pages](Special_AllPages.xhtml#Bandai_M60001_0115P_pinout)

(Bandai) Mitsubishi M60001-0115P: 28-pin 0.6" PDIP ([iNES Mapper 188](INES_Mapper_188.xhtml "INES Mapper 188")) 
    
    
                .--\/--.
        blue -> |01  28| -- +5V
      yellow -> |02  27| -> internal PRG /CE
      orange -> |03  26| -> external PRG /CE
     PPU A11 -> |04  25| -> VRAM A10
     PPU A10 -> |05  24| -> LatchedD6
     ROM R/W -> |06  23| -> PRG A14
     /ROMSEL -> |07  22| -> PRG A15
     CPU A14 -> |08  21| -> PRG A16
     CPU A13 -> |09  20| -> PRG A17
     CPU A12 -> |10  19| ??
     CPU D0  <> |11  18| <- CPU D6
     CPU D1  <> |12  17| <- CPU D5
     CPU D2  <> |13  16| <- CPU D4
         GND -- |14  15| <- CPU D3
                '------'
    

The color names were silkscreened on the PCB. They are the wires from the microphone. 

This mapper has the dubious distinction of failing to prevent bus conflicts but also being too complex to comfortably say "it's just a couple silicon dice in a single package", unlike the [Sunsoft-1](Sunsoft_1_pinout.xhtml "Sunsoft 1 pinout") and [-2](Sunsoft_2_pinout.xhtml "Sunsoft 2 pinout"). The described functionality is equivalent to _five_ 74xx ICs. (74377, ½ 74139, ½ 7420, ½ 74244, 7432) 

The M60001 prefix was used by Mitsubishi for custom orders: the M60001-0103P was used in the Joyball for the Famicom: [[1]](http://www.raphnet.net/electronique/famicom_to_nes/index_en.php)

See also: [naruko's writeup](http://seesaawiki.jp/famicomcartridge/d/%a5%ab%a5%e9%a5%aa%a5%b1%a5%b9%a5%bf%a5%b8%a5%aa)

Categories: [Pinouts](Category_Pinouts.xhtml)
