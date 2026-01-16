# INES Mapper 033

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_033) | View [other pages](Special_AllPages.xhtml#INES_Mapper_033)

iNES Mapper 033 represents [Taito's TC0190](Taito_TC0190_pinout.xhtml "Taito TC0190 pinout"). It also represents a subset of [Taito's TC0350](Taito_TC0350_pinout.xhtml "Taito TC0350 pinout"), but the only game we currently know to have used that IC ( [Don Doko Don](http://bootgod.dyndns.org:7777/profile.php?id=4025) ) didn't use its interrupt abilities (which are identical to those described on [iNES Mapper 048](INES_Mapper_048.xhtml "INES Mapper 048")). 
    
    
     Here are Disch's original notes:
     ========================
     =  Mapper 033          =
     ========================
     
     
     Example Games:
     --------------------------
     Akira
     Bakushou!! Jinsei Gekijou
     Don Doko Don
     Insector X
     
     
     Note:
     --------------------------
     Most dumps of [mapper 048](INES_Mapper_048.xhtml "INES Mapper 048") games floating around are erroneously labelled as mapper 033.  Mapper 033 does not
     have IRQs, mapper 048 does, and mirroring on each is handled a bit differently.  Apart from that, the two
     are very similar.
     
     
     Registers:
     --------------------------
     
     Range,Mask:   $8000-BFFF, $A003
     
       $8000 [.MPP PPPP]
         M = Mirroring (0=Vert, 1=Horz)
         P = PRG Reg 0 (8k @ $8000)
     
       $8001 [..PP PPPP]   PRG Reg 1 (8k @ $A000)
       $8002 [CCCC CCCC]   CHR Reg 0 (2k @ $0000)
       $8003 [CCCC CCCC]   CHR Reg 1 (2k @ $0800)
       $A000 [CCCC CCCC]   CHR Reg 2 (1k @ $1000)
       $A001 [CCCC CCCC]   CHR Reg 3 (1k @ $1400)
       $A002 [CCCC CCCC]   CHR Reg 4 (1k @ $1800)
       $A003 [CCCC CCCC]   CHR Reg 5 (1k @ $1C00)
     
     
     PRG Setup:
     ---------------------------
     
           $8000   $A000   $C000   $E000  
         +-------+-------+-------+-------+
         | $8000 | $8001 | { -2} | { -1} |
         +-------+-------+-------+-------+
     
     
     CHR Setup:
     ---------------------------
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +---------------+---------------+-------+-------+-------+-------+
         |     $8002     |     $8003     | $A000 | $A001 | $A002 | $A003 |
         +---------------+---------------+-------+-------+-------+-------+
    

Note that unlike the MMC3, the value written for the two 2 KiB CHR banks do not drop the LSB; the number written specifies the offset into CHR as a multiple of 2 KiB. Furthermore, it turns out the MSB of the register is implemented (even if no games ever used it) and the two 2 KiB CHR banks can address a full 512 KiB of CHR. (The four 1 KiB CHR banks are limited to the first 256 KiB of CHR) 

  
The TC0190 also showed up in a few arcade games using [Taito's F2 hardware](https://github.com/mamedev/mame/blob/master/src/mame/taito/taito_f2.cpp#L57). 

## See also

  * [Taito mapper](http://nesdev.org/taito-j.txt) by goroh (japanese)
  * [NES mapper list](http://www.romhacking.net/documents/362/) by Disch



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
