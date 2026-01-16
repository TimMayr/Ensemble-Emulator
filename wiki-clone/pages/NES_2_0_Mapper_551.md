# NES 2.0 Mapper 551

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_551) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_551)

**NES 2.0 Mapper 551** is a variant of [INES Mapper 178](INES_Mapper_178.xhtml "INES Mapper 178") used for several Jncota re-releases of RPG games that were originally published by C&E and Supertone: 

  * _聖火列傳 (Shènghuǒ Lièzhuàn)_ and its title screen hack _侠客传奇 (Xiákè Chuánqí)_ , originally released for [INES Mapper 240](INES_Mapper_240.xhtml "INES Mapper 240")
  * _荊軻新傳 (Jīngkē Xīnzhuàn)_ and its title screen hack _战国风云 (Zhànguó Fēngyún)_ , originally released for [INES Mapper 240](INES_Mapper_240.xhtml "INES Mapper 240")



Compared to [INES Mapper 178](INES_Mapper_178.xhtml "INES Mapper 178"), mirroring is hard-wired, and the chipset's internal CHR-RAM is not used in favor of CHR-ROM. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Mode Register ($4800)
    * 2.2 Low PRG Bank Register ($4801)
    * 2.3 High PRG Bank Register ($4802)
    * 2.4 CHR-ROM Bank register ($4803)



# Banks

  * CPU $6000-$7FFF: 8 KiB unbanked PRG-RAM
  * CPU $8000-$FFFF: 16/32 KiB PRG-ROM bank, switchable
  * PPU $0000-$1FFF: 8 KiB switchable CHR-ROM bank
  * Nametable mirroring: Hard-wired



# Registers

## Mode Register ($4800)
    
    
    7654 3210
    ---------
    .... .SS.
          ||
          ||
          ||
          ++-- PRG banking mode
                0: NROM-256/BNROM (PRG A14=CPU A14)
                1: UNROM (PRG A14..16=111b if CPU A14=1)
                2: NROM-128
                3: UNROM but with bit 0 of "fixed" bank selectable
                   (PRG A15..16=11b if CPU A14=1)
    

## Low PRG Bank Register ($4801)
    
    
    7654 3210
    ---------
    .... .LLL
          +++- PRG A16..A14
    

This can be considered an inner bank register for UNROM mode. 

## High PRG Bank Register ($4802)
    
    
    7654 3210
    ---------
    HHHH HHHH
    ++++-++++- PRG A24?..A17
    

This can be considered an outer bank register for UNROM mode. 

## CHR-ROM Bank register ($4803)
    
    
    7654 3210
    ---------
    BBBB BBBB
    ++++-++++- CHR-ROM A20?..A13
    

Categories: [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml)
