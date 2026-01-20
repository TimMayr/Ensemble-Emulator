# VRC2 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC2_pinout) | View [other pages](Special_AllPages.xhtml#VRC2_pinout)

Konami [VRC2 and VRC4](VRC2_and_VRC4.xhtml "VRC2 and VRC4"): 0.6" 40-pin PDIP (iNES Mappers **21** , **22** , **23** , and **25**) 
    
    
     r: connects to ROM
     f: connects to Famicom
    
                         .--\/--.
          (f) CPU A13 -> |01  40| -- +5V
          (f) CPU A14 -> |02  39| -> PRG A17 (r)
          (fr) CPU Ax -> |03  38| -> PRG A15 (r)
          (fr) CPU Ay -> |04  37| <- CPU A12 (f)
          (f) PPU A12 -> |05  36| -> PRG A14 (r)
          (f) PPU A11 -> |06  35| -> PRG A13 (r)
          (f) PPU A10 -> |07  34| -> PRG A16 (r)
          (r) PRG /CE <- |08  33| <- CPU D0 (fr)
          (f) CPU R/W -> |09  32| <- CPU D1 (fr)
       (CHR /CE) OR Y <- |10  31| <- CPU D2 (fr)
       (PPU A13) OR B -> |11  30| <- CPU D4 (fr)
       (PPU /RD) OR A -> |12  29| <- CPU D3 (fr)
        (f) CIRAM A10 <- |13  28| -> CHR A17 (r)
          (f) /ROMSEL -> |14  27| -> CHR A15 (r)
               (f) M2 -> |15  26| -> CHR A12 (r)
     VRC4 (f) CHR A18 <- |16  25| -> CHR A14 (r)
        VRC4 (f) /IRQ <- |17  24| -> CHR A13 (r)
         VRC4 /WR9003 <- |18  23| -> CHR A11 (r)
        VRC4 WRAM /CE <- |19  22| -> CHR A16 (r)
                  GND -- |20  21| -> CHR A10 (r)
                         `------'
     3, 4: see below
    

The VRC2's pins 16-19 seem to have been [intended for a never-seen-used EEPROM](http://forums.nesdev.org/viewtopic.php?t=8569)

Konami was greatly fond of making minor variations from one game to the next, presumably to make life harder on pirates. 

VRC4a, PCB 352398, mapper 21 
    
    
       **(fr) CPU A2** -> |03  38| -> PRG A15 (r)
       **(fr) CPU A1** -> |04  37| <- CPU A12 (f)
    

VRC4b, PCB 351406, mapper 25 
    
    
       **(fr) CPU A0** -> |03  38| -> PRG A15 (r)
       **(fr) CPU A1** -> |04  37| <- CPU A12 (f)
    

VRC2c, PCB 351948, mapper 25 
    
    
       **(fr) CPU A0** -> |03  38| -> PRG A15 (r)
       **(fr) CPU A1** -> |04  37| <- **74'139 /Y2**[[1]](https://forums.nesdev.org/viewtopic.php?t=6584)
    

  * The additional logic evaluates (/ROMSEL OR CPUA12), preventing bus conflicts between the Microwire interface and the PRG RAM on the board by making I/O to $6000-$6FFF instead appear to the VRC2 to be happening to $7000-$7FFF



VRC4c, PCB 352889, mapper 21 
    
    
       **(fr) CPU A7** -> |03  38| -> PRG A15 (r)
       **(fr) CPU A6** -> |04  37| <- CPU A12 (f)
    

VRC4d, PCB 352400, mapper 25 
    
    
       **(fr) CPU A2** -> |03  38| -> PRG A15 (r)
       **(fr) CPU A3** -> |04  37| <- CPU A12 (f)
    

VRC4e, PCB 352396, mapper 23 
    
    
       **(fr) CPU A3** -> |03  38| -> PRG A15 (r)
       **(fr) CPU A2** -> |04  37| <- CPU A12 (f)
    

VRC2b, PCBs 350603, 350636, 350926, and 351179, mapper 23 
    
    
       **(fr) CPU A1** -> |03  38| -> PRG A15 (r)
       **(fr) CPU A0** -> |04  37| <- CPU A12 (f)
    

VRC2a, PCB 351618, mapper 22 (no submapper necessary) 
    
    
       **(fr) CPU A0** -> |03  38| -> PRG A15 (r)
       **(fr) CPU A1** -> |04  37| <- CPU A12 (f)
                      :      :
     (f) CIRAM A10 <- |13  28| -> **CHR A16 (r)**
       (f) /ROMSEL -> |14  27| -> **CHR A14 (r)**
            (f) M2 -> |15  26| -> **CHR A11 (r)**
     VRC2 µWire DO -> |16  25| -> **CHR A13 (r)**
     VRC2 µWire DI <- |17  24| -> **CHR A12 (r)**
     VRC2 µWire SK <- |18  23| -> **CHR A10 (r)**
     VRC2 µWire CS <- |19  22| -> **CHR A15 (r)**
               GND -- |20  21| -> **n/c**
    

The VRC2 µWire interface is thought to be nonfunctional. 

Sources: 

  * <https://web.archive.org/web/20140920211213/http://nintendoallstars.w.interia.pl/romlab/vrcp.htm>
  * <http://forums.nesdev.org/viewtopic.php?t=8569>



  


## Pirate clones

There exist [pirate versions](http://forums.nesdev.org/viewtopic.php?f=9&t=8569&start=15#p240820) of both VRC2 and VRC4. The pinout and operation is the same: 

  * VRC2: 23C3662, AX-40G, 23C269, AX5705, nameless
  * VRC4: AX5208P, AX5208C, V4, PT8155



Note: 

  * PT8155 seems to have CHR-A17 and PRG-A17 (and maybe PRG-A18?) pins either broken, repurposed or not connected at all, because there exists at least one PCB with that chip (P-4073 PP-43KII, game _Wai Wai World 2_) that has additional logic to provide missing CHR-A17 and PRG-A17 lines (see [schematic](File_PT8155_Wai_Wai_World_2_png.xhtml "File:PT8155 Wai Wai World 2.png"))



Categories: [Pinouts](Category_Pinouts.xhtml)
