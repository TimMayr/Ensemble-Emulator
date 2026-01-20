# VRC3 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC3_pinout) | View [other pages](Special_AllPages.xhtml#VRC3_pinout)

[VRC3](VRC3.xhtml "VRC3"): 18-pin PDIP (**mapper 73**) 
    
    
    r: connects to PRG ROM
    f: connects to Famicom
    w: connects to PRG RAM
                     .-\_/-.
      (r) PRG A16 <- |01 18| -- +5V
      (r) PRG /CE <- |02 17| -> WRAM /CE (w)
      (r) PRG A14 <- |03 16| <- R/W      (wfr)
      (r) PRG A15 <- |04 15| <- CPU D3   (wfr)
      (f)      M2 -> |05 14| <- CPU D0   (wfr)
    (wfr) CPU A12 -> |06 13| <- CPU D1   (wfr)
     (rf) CPU A13 -> |07 12| <- CPU D2   (wfr)
      (f) CPU A14 -> |08 11| <- /ROMSEL  (f)
              Gnd -- |09 10| -> /IRQ     (f)
                     '-----'
    

Kaiser seems to have made a subtle upgrade of the VRC3: 

National Semiconductor KS202: 20-pin PDIP (canonically [iNES Mapper 142](INES_Mapper_142.xhtml "INES Mapper 142")) 
    
    
                     .-\_/-.
    (wfr) CPU A12 -> |01 20| -> WRAM /CE (w)
      (f) CPU A13 -> |02 19| <- CPU D3   (wfr)
      (f) CPU A14 -> |03 18| <- R/W      (wfr)
              +5V -- |04 17| <- CPU D0   (wfr)
      (f)      M2 -> |05 16| <- CPU D1   (wfr)
      (r) PRG A14 <- |06 15| <- CPU D2   (wfr)
      **(r) PRG A13** <- |07 14| -- GND
      (r) PRG A15 <- |08 13| <- **RESET**
      (r) PRG A16 <- |09 12| <- /ROMSEL  (f)
      (r) PRG /CE <- |10 11| -> /IRQ     (f)
                     '-----'
    

Source: [[1]](https://forums.nesdev.org/viewtopic.php?f=9&t=19314#p242678)

Categories: [Pinouts](Category_Pinouts.xhtml)
