# INES Mapper 070

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_070) | View [other pages](Special_AllPages.xhtml#INES_Mapper_070)
    
    
     ========================
     =  Mapper 070          =
     ========================
     
     
     Example Games:
     --------------------------
     Family Trainer - Manhattan Police
     [Family Trainer - Meiro Daisakusen](https://nescartdb.com/profile/view/4091/family-trainer-5-meiro-daisakusen)
     [Kamen Rider Club](https://nescartdb.com/profile/view/1742/kamen-rider-club-gekitotsu-shocker-land)
     Space Shadow
     
     
     Notes:
     ---------------------------
     Many of these games use the [Family Trainer Mat](Power_Pad.xhtml#Family_Trainer_Mat "Power Pad") as an input device.
     
     
     Registers: **BUS CONFLICTS**
     --------------------------
       $8000-FFFF:  [PPPP CCCC]
         P = Selects 16k PRG @ $8000
         C = Selects 8k CHR @ $0000
     
     
     PRG Setup:
     --------------------------
     
           $8000   $A000   $C000   $E000  
         +---------------+---------------+
         |     $8000     |     { -1}     |
         +---------------+---------------+
    

The version of this board with 1-screen mapper-controlled mirroring is [mapper 152](INES_Mapper_152.xhtml "INES Mapper 152"), and the version without a fixed bank at $C000-$FFFF is oversize [GxROM](GxROM.xhtml "GxROM"). Before Mapper 152 was defined, the version with 1-screen mirroring had been denoted by setting the "alternative nametables" bit in the iNES header (according to the FWNES98E.TXT file of the fwNES emulator version 0.302b). 

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
