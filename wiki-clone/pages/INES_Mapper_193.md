# INES Mapper 193

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_193) | View [other pages](Special_AllPages.xhtml#INES_Mapper_193)

iNES Mapper 193 represents boards that use [NTDEC](https://en.wikipedia.org/wiki/NTDEC "wikipedia:NTDEC")'s [TC-112 mapper IC](NTDEC_TC_112_pinout.xhtml "NTDEC TC-112 pinout"). 
    
    
     Example Game:
     --------------------------
     Fighting Hero (Unl)
     [War in the Gulf](https://nescartdb.com/profile/view/4247) & [(Brazilian release)](https://nescartdb.com/profile/view/4248)
     
     Registers:
     ---------------------------
     Regs at $6000-7FFF = no SRAM
     
     Range,Mask:   $6000-7FFF, $E007
     
       $6000:  [CCCC CC..] CHR Reg 0
       $6001:  [CCCC CCC.] CHR Reg 1
       $6002:  [CCCC CCC.] CHR Reg 2
       $6003:  [.... PPPP] PRG Reg
       $6004:  [.... ...M] Mirroring
         0 = Vert (A10)
         1 = Horz (A11)
    

No games are known to rely on changing mirroring at run-time. At least some games have hard-wired mirroring. 
    
    
     CHR Setup:
     ---------------------------
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +-------------------------------+---------------+---------------+
         |           <<$6000>>           |    <$6001>    |    <$6002>    |
         +-------------------------------+---------------+---------------+
     
     PRG Setup:
     ---------------------------
     
           $8000   $A000   $C000   $E000  
         +-------+-------+-------+-------+
         | $6003 | { -3} | { -2} | { -1} |
         +-------+-------+-------+-------+
    

See also: <https://forums.nesdev.org/viewtopic.php?t=19751>

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
