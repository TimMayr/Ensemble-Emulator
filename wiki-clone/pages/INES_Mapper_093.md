# INES Mapper 093

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_093) | View [other pages](Special_AllPages.xhtml#INES_Mapper_093)

This is part of a family of Sunsoft mappers used by Japanese games: ([iNES Mapper 089](INES_Mapper_089.xhtml "INES Mapper 089"), iNES Mapper 093, [iNES Mapper 184](INES_Mapper_184.xhtml "INES Mapper 184")) 

This mapper represents the [Sunsoft-2 IC](Sunsoft_2_pinout.xhtml "Sunsoft 2 pinout") on the Sunsoft-3R board. It is used by [Shanghai](https://nescartdb.com/profile/view/3142/shanghai), as well as [Fantasy Zone](https://nescartdb.com/profile/view/3858/fantasy-zone) whose program is compatible with both this board and a different one that used the [Sunsoft-1 IC](Sunsoft_1_pinout.xhtml "Sunsoft 1 pinout"). 

[Mapper 89](INES_Mapper_089.xhtml "INES Mapper 089") uses the same Sunsoft-2 IC but on the Sunsoft-3 board and so differs in that it has mapper-controlled one-screen mirroring and CHR ROM banking. Note that the CHR bank lines are still hooked up to the RAM, explaining the RAM enable bit. For homebrew and hacks, games with CHR ROM should use 89's banking behavior with fixed mirroring. 

Despite the mapper existing as a single IC, its functionality [is describable](Sunsoft_2_pinout.xhtml "Sunsoft 2 pinout") using just a [74377](74377.xhtml "74377") and a [7432](7432.xhtml "7432") and should probably be considered discrete logic. 

Here is the documentation in [disch's original style](INES_Mapper_DischDocs.xhtml "INES Mapper DischDocs"): 
    
    
     Registers **BUS CONFLICTS**:
     --------------------------
       
       $8000-FFFF:  [.PPP ...E]
         P = PRG Reg  (16k @ $8000)
         E = CHR RAM enable:
             0 = RAM is disabled; writes are ignored and reads are open bus
                 (like [iNES Mapper 185](CNROM.xhtml "INES Mapper 185") except no games use this)
             1 = normal operation.
     
     
     PRG Setup:
     --------------------------
     
           $8000   $A000   $C000   $E000  
         +---------------+---------------+
         |     $8000     |     { -1}     |
         +---------------+---------------+
    

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
