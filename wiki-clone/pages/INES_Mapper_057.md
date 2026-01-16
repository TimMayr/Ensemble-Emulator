# INES Mapper 057

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_057) | View [other pages](Special_AllPages.xhtml#INES_Mapper_057)
    
    
     Based on Disch's original notes:  
     ========================
     =  Mapper 057          =
     ========================
     
     Example Games:
     --------------------------
     GK 47-in-1
     6-in-1 (SuperGK)
     
     
     Registers:
     ---------------------------
     
     Range,Mask:   $8000-FFFF, $8800
     
       $8000:  [CH.. ..AA]
          C = CHR Mode, 0=CNROM, 1=NROM
          H = CHR A16
          A = CHR A13-A14 in CNROM mode (only used if C=0)
     
       $8800:  [PPPO MBbb]
          P = PRG Reg
          O = PRG Mode
          M = Mirroring (0=Vert, 1=Horz)
          B = CHR A15
          b = CHR A13-14 in NROM mode (only used if C=1)
     
     
     CHR Setup:
     ---------------------------
     If C=0, the 8 KiB CHR-ROM bank is HBAA.
     If C=1, the 8 KiB CHR-ROM bank is HBbb.
     
     PRG Setup:
     ---------------------------
     
                    $8000   $A000   $C000   $E000  
                  +---------------+---------------+
     PRG Mode 0:  |     $8800     |     $8800     |
                  +-------------------------------+
     PRG Mode 1:  |            <$8800>            |
                  +-------------------------------+
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml)
