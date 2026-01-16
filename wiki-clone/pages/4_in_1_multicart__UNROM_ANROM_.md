# 4-in-1 multicart (UNROM/ANROM)

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/4-in-1_multicart_%28UNROM/ANROM%29) | View [other pages](Special_AllPages.xhtml#4_in_1_multicart__UNROM_ANROM_)

[![](../wiki-images/4-in-1_unrom%2Canrom_menu.png)](File_4_in_1_unrom_anrom_menu_png.xhtml)

[](File_4_in_1_unrom_anrom_menu_png.xhtml "Enlarge")

Menu

[![](../wiki-images/4-in-1_unrom%2Canrom_components.jpg)](File_4_in_1_unrom_anrom_components_jpg.xhtml)

[](File_4_in_1_unrom_anrom_components_jpg.xhtml "Enlarge")

Components

PRG: 512 kiB ROM 

CHR: 8 kiB RAM 

Bus conflicts: no 
    
    
    gG | GAME
    ---+----------------
    00 | Trog
    10 | Duck Tales I
    01 | Marble Madness
    11 | Captain Skyhawk
    

## Contents

  * 1 Registers
    * 1.1 $6000-$7FFF: UNROM/ANROM banking type & game number
    * 1.2 $8000-$FFFF: bank select (when in UNROM mode)
    * 1.3 $8000-$FFFF: bank select (when in ANROM mode)



## Registers

### $6000-$7FFF: UNROM/ANROM banking type & game number
    
    
    7  bit  0
    ---- ----
    TgGP
    ||||
    |||+------------ protection:    0 - off, 1 - contents of this register cannot be changed until console reset
    |++------------- selects one of four 128 kB games; g is lower address line: PRG-A18 = G, PRG-A17=g 
    +--------------- banking style: 0 - UNROM: $8000-$bfff 16 kB switchable CBA bank, $c000-$ffff fixed to last bank, PPU vertical mirroring
                                    1 - ANROM: $8000-$ffff 32 kB switchable BA bank, PPU single screen mirroring
    

### $8000-$FFFF: bank select (when in UNROM mode)
    
    
    7  bit  0
    ---- ----
          CBA
          |||
          +++------------ UNROM bank (when in UNROM MODE)
    

### $8000-$FFFF: bank select (when in ANROM mode)
    
    
    7  bit  0
    ---- ----
       M   BA
       |   ||
       |   ++------------ ANROM bank (when in ANROM MODE)
       +----------------- one screen mirroring
      
    

[![](../wiki-images/4-in-1_unrom%2Canrom_top.jpg)](File_4_in_1_unrom_anrom_top_jpg.xhtml)

[](File_4_in_1_unrom_anrom_top_jpg.xhtml "Enlarge")

Top layer

[![](../wiki-images/4-in-1_unrom%2Canrom_bottom.jpg)](File_4_in_1_unrom_anrom_bottom_jpg.xhtml)

[](File_4_in_1_unrom_anrom_bottom_jpg.xhtml "Enlarge")

Bottom layer

[![](../wiki-images/4-in-1_unrom%2Canrom_schematics.png)](File_4_in_1_unrom_anrom_schematics_png.xhtml)

[](File_4_in_1_unrom_anrom_schematics_png.xhtml "Enlarge")

Schematics

Categories: [Mappers](Category_Mappers.xhtml)
