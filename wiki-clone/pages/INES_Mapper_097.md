# INES Mapper 097

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_097) | View [other pages](Special_AllPages.xhtml#INES_Mapper_097)

iNES Mapper 097 represents the single game that uses [Irem's TAM-S1](Irem_TAM_S1_pinout.xhtml "Irem TAM-S1 pinout") IC, [Kaiketsu Yanchamaru](https://nescartdb.com/profile/view/3801). 
    
    
     Registers:
     --------------------------
     This mapper does not have bus conflicts.
     
       $8000-BFFF:  [M..P PPPP]
         P = PRG Reg  (16k @ $C000)
         M = Mirroring:
            %0 = Horz (PPU A11)
            %1 = Vert (PPU A10)
     
     
     PRG Setup:
     --------------------------
     
           $8000   $A000   $C000   $E000  
         +---------------+---------------+
         |     { -1}     |     $8000     |
         +---------------+---------------+
    

The underlying hardware is a little more capable, but is wired in a way such that that is inaccessible. 

[Mapper 180](INES_Mapper_180.xhtml "INES Mapper 180") is basically this mapper without mirroring control. (But the fixed bank is 0, not last) 

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
