# User:Zzo38/Mapper B

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_B) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_B)

  
This mapper was originally described by tepples as "Mapper 670" after its use of a 74HC670 prior to the division of the [NES 2.0 expanded mapper space](NES_2_0.xhtml#Byte_8_\(Mapper_variant\) "NES 2.0") into [planes](Mapper.xhtml#Supplementary_Multilingual_Plane "Mapper"). 

CPU address space: 

  * $8000-$9FFF: Switchable bank 0
  * $A000-$BFFF: Switchable bank 1
  * $C000-$DFFF: Switchable bank 2
  * $E000-$EFFF: Switchable bank 3
  * $F000-$FFFF: Fixed to last bank



Banks are switched by writing the 8K bank number into the low four bits of registers in the same address space where the banks are at. There is no bus conflicts. 

Sometimes there may be PRG RAM at $6000-$7FFF. Usually there is 8K CHR RAM, although it would also work with 8K CHR ROM. 

## References

  * [Original forum post](http://forums.nesdev.org/viewtopic.php?t=2669) (November 2006)


