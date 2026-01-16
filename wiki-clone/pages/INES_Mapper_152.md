# INES Mapper 152

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_152) | View [other pages](Special_AllPages.xhtml#INES_Mapper_152)
    
    
     ========================
     =  Mapper 152          =
     ========================
     
     
     Example Games:
     --------------------------
     [Arkanoid 2 (J)](https://nescartdb.com/profile/view/2253/arkanoid-ii)
     [Gegege no Kitarou 2](https://nescartdb.com/profile/view/1539/gegege-no-kitarou-2-youkai-gundan-no-chousen)
     
     
     
     Registers: (** BUS CONFLICTS **)
     --------------------------
     
       $8000-FFFF:  [MPPP CCCC]
         M = Mirroring:
             0 = 1ScA
             1 = 1ScB
     
         P = PRG Reg (16k @ $8000)
         C = CHR Reg (8k @ $0000)
     
     
     PRG Setup:
     --------------------------
     
           $8000   $A000   $C000   $E000  
         +---------------+---------------+
         |     $8000     |     { -1}     |
         +---------------+---------------+
    

The version of this board without mirroring control is [mapper 70](INES_Mapper_070.xhtml "INES Mapper 070"). One variant of [mapper 78](INES_Mapper_078.xhtml "INES Mapper 078") is the same thing with the nibbles swapped. 

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml)
