# INES Mapper 250

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_250) | View [other pages](Special_AllPages.xhtml#INES_Mapper_250)

INES Mapper 250 is a mapper by Nitra, used for pirate games such as [Time Diver Avenger](http://community.fandom.com/wiki/c:bootleggames:Time_Diver:_Avenger "wikia:c:bootleggames:Time Diver: Avenger") and the Queen Bee V. 

Hardware consists of regular MMC3 chip connected in different way: 

  * A10 instead of A0 is used to determine which register to write to (i.e. the registers are at $8000, $8400, $A000, $A400, $C000, $C400, $E000, $E400 and their associated
  * A7..A0 is used instead of D7..D0: the value written to each register is the lower byte of the address (i.e. address AND $00FF), data value is discarded


  * Queen Bee V contains also 74153 mux which controls PRG-A13 resulting in the following change:


    
    
    below
    $8000 $8000 $a000 $c000 $e000
       0   MMC3  MMC3   0     1
    

Source: [[1]](http://krzysiobal.com/carts/?action=search&mpr=250)

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
