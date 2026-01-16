# INES Mapper 032

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_032) | View [other pages](Special_AllPages.xhtml#INES_Mapper_032)

iNES Mapper 032 represents games using Irem's G-101 mapper IC. It came in a [52-pin DIP](Irem_G_101_pinout.xhtml "Irem G-101 pinout") package. 
    
    
     These are not Disch's original notes:
     ========================
     =  Mapper 032          =
     ========================
     
     Example Games:
     --------------------------
     Image Fight
     Major League
     Kaiketsu Yanchamaru 2
       
     Registers:
     --------------------------
     
     Range,Mask:   $8000-BFFF, $F007
     
       $8000-$8007:  [...P PPPP]    PRG Reg 0
       $9000-$9007:  [.... ..PM]  ** see footnote
           P = PRG Mode
           M = Mirroring (0=Vert, 1=Horz)
       $A000-$A007:  [...P PPPP]    PRG Reg 1
       $B000-$B007:  [CCCC CCCC]    CHR Regs
     
     
     PRG Setup:
     ---------------------------
     
                    $8000   $A000   $C000   $E000  
                  +-------+-------+-------+-------+
     PRG Mode 0:  | $8000 | $A000 | { -2} | { -1} |
                  +-------+-------+-------+-------+
     PRG Mode 1:  | { -2} | $A000 | $8000 | { -1} |
                  +-------+-------+-------+-------+
     
     
     CHR Setup:
     ---------------------------
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +-------+-------+-------+-------+-------+-------+-------+-------+
         | $B000 | $B001 | $B002 | $B003 | $B004 | $B005 | $B006 | $B007 |
         +-------+-------+-------+-------+-------+-------+-------+-------+
    
     Footnote:
     --------------------------
     Major League wants hardwired 1-screen mirroring. (CIRAM A10 is tied to +5V
     on this game). Additionally, the register at $9000 is entirely disabled: 
     the game can only request "PRG mode 0".
    
     A [NES 2.0](NES_2_0.xhtml "NES 2.0") [submapper](NES_2_0_submappers.xhtml#iNES_Mapper_032_.2F_Irem_G101 "NES 2.0 submappers") has been assigned for this difference.
     Otherwise you'll have to use a hash check.
    

Naruko says that only the game "愛先生の OSHIETE 私の星" uses PRG mode 1, which is why it was documented incorrectly for so long. 

## See also

  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.
  * Naruko's notes: [[1]](http://w.livedoor.jp/famicomcartridge/d/Irem%20G-101)



Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
