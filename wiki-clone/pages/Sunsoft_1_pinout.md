# Sunsoft 1 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sunsoft_1_pinout) | View [other pages](Special_AllPages.xhtml#Sunsoft_1_pinout)

Sunsoft-1: 16 pin 0.3" PDIP (canonically [mapper 184](INES_Mapper_184.xhtml "INES Mapper 184")) 
    
    
                    .---\/---.       
     (f) PPU A12 -> |01    16| -- +5V
          (f) M2 -> |02    15| -> CHR A12 (r)
    (fr) /ROMSEL -> |03    14| -> CHR A14 (r)
    (fr) CPU A13 -> |04    13| -> CHR A13 (r)
    (fr) CPU A14 -> |05    12| <- CPU RnW (f)
     (fr) CPU D5 -> |06    11| <- CPU D0 (fr)
     (fr) CPU D4 -> |07    10| <- CPU D1 (fr)
             GND -- |08    09| <- CPU D2 (fr)
                    `--------'
    

NOTE: D6 is NOT connected. Games simply always write to the mapper with D6 set. The wiring for _Fantasy Zone_ implies the virtual D6 is treated as though it were connected to +5V always. 

Alternative wiring for _Fantasy Zone_ : 
    
    
                    .---\/---.       
     (f) **CPU A14** -> |01    16| -- +5V
          (f) M2 -> |02    15| -> **PRG A14** (r)
    (fr) /ROMSEL -> |03    14| -> **PRG A16** (r)
    (fr) CPU A13 -> |04    13| -> **PRG A15** (r)
     (f) CPU A14 -> |05    12| <- CPU RnW (f)
             **+5V** -> |06    11| <- CPU D0 (fr)
             **+5V** -> |07    10| <- CPU D1 (fr)
             GND -- |08    09| <- CPU D2 (fr)
                    `--------'
    

Categories: [Pinouts](Category_Pinouts.xhtml)
