# INES Mapper 236

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_236) | View [other pages](Special_AllPages.xhtml#INES_Mapper_236)

**iNES Mapper 236** denotes the Realtec **8031** and **8155** circuit boards, used for a number of multicarts with 128 KiB PRG-ROM and 64 KiB CHR-ROM: 

  * _35-in-1 (HM5511)_
  * _56-in-1 (NT-009)_
  * _68-in-1 (HM5511)_
  * _70-in-1_



The Realtec **8099** PCB has twice the ROM size: 

  * _智慧大全 12-in-1_



A variant of this circuit board (Realtec **8106** PCB) with 512 KiB PRG-ROM and 8 KiB of unbanked CHR-RAM [is emulated under the same mapper number](https://github.com/SourMesen/Mesen/blob/master/Core/Bmc70in1.h): 

  * _800-in-1_



Due to the presence of four solder pads that select one of sixteen displayed game counts, the same ROM is bound to exist with different cartridge codes. 

## Lower Address Latch ($8000-$BFFF, write)
    
    
    Mask: $C000
    
    Variant with CHR-ROM:
    A~FEDC BA98 7654 3210
      -------------------
      10.. .... ..M. CCCC
                  |  ++++- CHR A16..A13
                  +------- Nametable mirroring
                            0: Vertical
                            1: Horizontal
    
    Variant with CHR-RAM:
    A~FEDC BA98 7654 3210
      -------------------
      10.. .... ..M. .PPP
                  |   +++- PRG A19..A17
                  +------- Nametable mirroring
                            0: Vertical
                            1: Horizontal
    

## Upper Address Latch ($C000-$FFFF, write)
    
    
    Mask: $C000
    
    Variant with CHR-ROM:
    A~FEDC BA98 7654 3210
      -------------------
      11.. .... ..SS PPPP
                  || ++++- PRG A17..A14
                  ++------ PRG Mode
                            0: UNROM (PRG A14..16=111 when CPU A14=1)
                            1: Read Solder Pad (like SS=0, plus
                               PRG A0..A3=solder pad value)
                            2: NROM-256 (PRG A14=CPU A14)
                            3: NROM-128
    
    Variant with CHR-RAM:
    A~FEDC BA98 7654 3210
      -------------------
      11.. .... ..SS .PPP
                  ||  +++- PRG A16..A14
                  ++------ PRG Mode
                            0: UNROM (PRG A14..16=111 when CPU A14=1)
                            1: Read Solder Pad (like SS=0, plus
                               PRG A0..A3=solder pad value)
                            2: NROM-256 (PRG A14=CPU A14)
                            3: NROM-128
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
