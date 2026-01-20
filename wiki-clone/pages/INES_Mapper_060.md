# INES Mapper 060

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_060) | View [other pages](Special_AllPages.xhtml#INES_Mapper_060)

iNES Mapper 60 is used for a _Reset-based NROM-128 4-in-1_ multicart. 
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 060          =
     ========================
     
     Example Game:
     --------------------------
     Reset Based 4-in-1
     
     
     Notes:
     ---------------------------
     This mapper is very, very unique.
     
     It's a multicart that consists of four NROM games, each with 16k PRG (put at $8000 and $C000) and 8k CHR.
     The current block that is selected is determined by an internal register that can only be incremented by a
     soft reset!
     
     I would assume the register is 2 bits wide?  Don't know for sure.
    

## Errata

NESten's Mapper DLL v1.0 contained an incorrect implementation of T3H53 as iNES mapper 059 and UNIF mapper BMC-T3H53. The UNIF implementation was improved in NESten's Mapper DLL v1.2 while the iNES implementation was not, causing confusion about what kind of cartridge the unimproved mapper 59 was supposed to be. As Nintendulator's mapper DLL is based on NESten's mapper DLL, this was carried over to Nintendulator. Presumably as a result of this confusion, FCEUX and others put the correct implementation of T3H53 at INES Mapper 060, displacing the "Reset-based NROM-128 4-in-1" in the process. Therefore, a .NES ROM file denoting mapper 60 that is not a reset-based multicart is actually for [mapper 59](INES_Mapper_059.xhtml "INES Mapper 059"). 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
