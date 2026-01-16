# INES Mapper 212

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_212) | View [other pages](Special_AllPages.xhtml#INES_Mapper_212)

According to FCEUX's source, iNES Mapper 212 is yet another simple discrete logic pirate multicart mapper. 

Nestopia calls it "BMC Super HiK 300-in-1". 
    
    
    Write $8000-$FFFF:
     A~[1o.. .... .... MBBb]
         |             ||||
         |             |+++-- Combined PRG and CHR bank number
         |             +----- Nametable mirroring (0:V/A10, 1:H/A11)
         +------------------- Banking style
    

When Banking style is 0, BBb specifies a 16 KiB PRG bank at both CPU $8000 and $C000. When it's 1, BB is 32 KiB PRG bank at CPU $8000. 

Regardless of the value of the "o" bit, BBb is 8 KiB CHR bank at PPU $0000. 

Equivalently, the latched copy of A2 through A0 goes to PRG A16 through A14 and CHR A15 through A13. 

  

    
    
    Mask: $E010
    Read: $6000: [1... ....]
     reads from these addresses return a value with the most significant bit set, while all other addresses and bits are open bus.
    

This is reminiscent of [iNES Mapper 107](INES_Mapper_107.xhtml "INES Mapper 107"). 

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with overlapping PRG and CHR registers](Category_Mappers_with_overlapping_PRG_and_CHR_registers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
