# INES Mapper 200

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_200) | View [other pages](Special_AllPages.xhtml#INES_Mapper_200)

**iNES Mapper 200** denotes the circuit boards of simple address-latch-based NROM-128 multicarts, i.e. CPU $8000-$BFFF is always a mirror of CPU $C000-$FFFF. 

The mapper number is used for two variants that differ in the bit that selects nametable mirroring. 

PCB that use this mapper : 

  * **MG109** for the _[1993 Super 50 in 1 Game](1993_Super_50_in_1_Game.xhtml "1993 Super 50 in 1 Game")_ multicart



## Contents

  * 1 Address Latch ($8000-$FFFF, write)
    * 1.1 Submapper 0
    * 1.2 Submapper 1
  * 2 Notes



# Address Latch ($8000-$FFFF, write)

## Submapper 0
    
    
    A~[1... .... .... bBBB]
                      |+++- PRG A16..A14, CHR A15..A13
                      +---- PRG A17, CHR A16, Mirroring:
                             0: Vertical
                             1: Horizontal
    

## Submapper 1
    
    
    A~[1... .... .... .bBB]
                       |++- PRG A15..A14, CHR A14..A13
                       +--- PRG A16, CHR A15, Mirroring:
                             0: Vertical
                             1: Horizontal
    

# Notes

  * Some menus select bank numbers beyond the PRG ROM size, expecting to read back a solder pad value to select one of several displayed game counts.
  * [NES 2.0 Mapper 338](NES_2_0_Mapper_338.xhtml "NES 2.0 Mapper 338") flips the meaning of the mirroring bit.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
