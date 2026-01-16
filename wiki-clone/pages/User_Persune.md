# User:Persune

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3APersune) | View [other pages](Special_AllPages.xhtml#User_Persune)

Hello! I usually [write music](https://sattnstar.bandcamp.com/), or [work on hardware/software projects](https://github.com/Gumball2415/) for the Famicom. From time to time, I like to investigate certain aspects of the Famicom, be it audio, video, or otherwise. 
    
    
    TODO:
      - learn wiki markup
      - write and publish personal research here:
    
          - Famicom cartridge PCB measurements, dimensions and footprints
            https://www.nesdev.org/wiki/Famicom_cartridge_dimensions
              Completed so far:
                - FC HVC-TGROM-01
                - FC HVC-CNROM-256K-01
                - TODO: publish HVC-ETROM-01 measurements
    
          - NES cartridge PCB measurements, dimensions and footprints
            https://www.nesdev.org/wiki/NES_cartridge_dimensions
              Completed so far:
                - NES-EWROM-01
                - TODO: measure unlicensed cartridge PCBs
    
          - NES/Famicom equivalent opamp amplifier gain
                - TODO: RF Famicom HVC-CPU-07, RF Famicom HVC-GPN rev, AV Famicom HVCN-CPU rev, Sharp Twin Famicom rev
    
    

Some bits i'm drafting: 

## Contents

  * 1 Famicom EPSM access
  * 2 Famicom / NES cartridge measurements
    * 2.1 Label measurements
      * 2.1.1 Front label measurements
      * 2.1.2 Back label measurements



## Famicom EPSM access

Due to the lack of EXP pins on the Famicom's cartridge pinout, an equivalent version does not have direct access to any of the methods above. 

For the universal $4016 access, one may directly connect OUT1 from pin 11 of the Famicom's expansion port to the EPSM, or replicate OUT1 by latching $4016.D1 writes with external circuitry. 

For the memory mapped access, the default circuit is the $401C-F addressing. 

If another mode of mapper access is desired, a 2x15 pin header may be provided with the bare signals required for use with an external daughterboard. 
    
    
       EPSM    | Daughterboard  |    EPSM
               +----------------\
        GND -- |01            30| -- +5V
    CPU A11 -> |02            29| <- M2
    CPU A10 -> |03            28| <- CPU A12
     CPU A9 -> |04            27| <- CPU A13
     CPU A8 -> |05            26| <- CPU A14
     CPU A7 -> |06            25| -- NC 
     CPU A6 -> |07            24| -> EPSM A1
     CPU A5 -> |08            23| -> EPSM A0
     CPU A4 -> |09            22| -> EPSM CE3
     CPU A3 -> |10            21| -> EPSM /CE2
     CPU A2 -> |11            20| -> EPSM /CE1
     CPU A1 -> |12            19| -- NC
     CPU A0 -> |13            18| -- NC
    CPU R/W -> |14            17| <- /ROMSEL
         NC -- |15            16| -- NC
               +----------------/
    

For advanced data bus latching and controller access, an auxiliary pin header is available to be used, in conjunction with the one above 
    
    
        EPSM     | Daughterboard  |     EPSM
                 +----------------\
       CPU D7 -> |01            24| -> YMF288 D7
       CPU D6 -> |02            23| -> YMF288 D6
       CPU D5 -> |03            22| -> YMF288 D5
       CPU D4 -> |04            21| -> YMF288 D4
       CPU D3 -> |05            20| -> YMF288 D3
       CPU D2 -> |06            19| -> YMF288 D2
       CPU D1 -> |07            18| -> YMF288 D1
       CPU D0 -> |08            17| -> YMF288 D0
      EPSM A1 -> |09            16| -> YMF288 /CS
      EPSM A0 -> |10            15| -> YMF288 A1
     EPSM CE3 -> |11            14| -> YMF288 A0
    EPSM /CE2 -> |12            13| <- EPSM /CE1
                 +----------------/
    

## Famicom / NES cartridge measurements

### Label measurements

Note that these measure the dimensions of the label stickers themselves, not the allocated area on the plastic shell. 

#### Front label measurements

Famicom 1st party short shells (SMB3 JP HVC-UM) \- 90mm x 45.5mm 

Famicom 1st party tall shells (Nobunaga no Yabou: Sengoku Gunyuuden KOE-NU) \- 95mm x 53.5mm 

NES shell (Romance of the Three Kingdoms II NES-XL-USA) \- 50mm x 96.5mm 

#### Back label measurements

Famicom 1st party short shells (SMB3 JP HVC-UM) \- 99.5mm x 28.5mm 

Famicom 1st party tall shells (Nobunaga no Yabou: Sengoku Gunyuuden KOE-NU) \- 99.5mm x 28.5mm 

NES shell (Romance of the Three Kingdoms II NES-XL-USA) \- 78.5mm x 31mm 
