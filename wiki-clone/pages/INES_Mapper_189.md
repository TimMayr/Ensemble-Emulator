# INES Mapper 189

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_189) | View [other pages](Special_AllPages.xhtml#INES_Mapper_189)
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 189          =
     ========================
     
     
     Example Game:
     --------------------------
     Thunder Warrior
     
     
     Notes:
     ---------------------------
     This mapper is a modified [MMC3](MMC3.xhtml "MMC3").  Everything operates just as it does on the MMC3, only the normal PRG regs
     (R:6,R:7) are ignored, and a new PRG Reg is used instead.
     
     For details on MMC3, see [mapper 004](MMC3.xhtml "INES Mapper 004")
     
     
     Registers:
     ---------------------------
     Regs at $6000-7FFF means no SRAM
     
     
       $4120-7FFF:  [AAAA BBBB]
          A,B:  PRG Reg
     
     
       $8000-FFFF:  Same as on MMC3
     
     
     PRG Setup:
     --------------------------
     
     'A' and 'B' bits of the $4120 reg seem to be effectively OR'd.
     That is... $30, $03, and $21  will all select page 3
     
          $8000   $A000   $C000   $E000  
         +-------------------------------+
         |             $4120             |
         +-------------------------------+
    

This mapper seems to be for HKOs and pirate backports. 

FCEUX's source comments that there are actually two different PRG mappers here; one pays attention to only the upper nybble while the other pays attention to only the lower nybble. Fortunately, all(?) games that use this mapper write 0 in the unused bits, so the OR behavior described above actually works. If a game ever writes something where the upper and lower nybbles are both nonzero and nonidentical, and the game is 512 KiB or smaller, it's not actually clear what to do. 

[One game](http://bootgod.dyndns.org:7777/profile.php?id=4266) (a romhack of an HKO) is present in NesCartDB; it shows a PT8154 ([Namco 108](INES_Mapper_206.xhtml "INES Mapper 206") clone), a 74175 (which only provides 4 bits of PRG banking), and a [74138](74138.xhtml "74138") (to decode the PRG register). At least /ROMSEL, A14, M2, R/W, and one additional address line must connect to the '138, but which exact lines are used vary from game to game. All four games found in GoodNES do reliably write with A8 high and A6, A7, A9, A10, A11, and A12 low. 

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
