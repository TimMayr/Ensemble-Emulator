# INES Mapper 143

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_143) | View [other pages](Special_AllPages.xhtml#INES_Mapper_143)

iNES Mapper 143 denotes an NROM circuit board with a simple form of protection: 

  * _Dancing Blocks_
  * _Magical Mathematics_ (NTSC version only)



The functional description is consistent with a [74138](74138.xhtml "74138") and a 74368. 

Reads from these addresses produce this output: 
    
    
    A:[010. ...1 ..DD DDDD] -> [xxii iiii]
                   || ||||      |||| ||||
                   ++-++++------||++-++++-- reads provide the logical inversion of DDDDDD in the bits iiiiii
                                ++--------- open bus
    

Nestopia additionally implies that at least _Dancing Blocks_ relies extensively on the values of uninitialized memory, and so manually sets the emulated console's 2K of internal RAM to a different pattern than Nestopia's default. 

  * <http://cah4e3.shedevr.org.ru/%5Blst%5D-sachen-mappers.txt>


