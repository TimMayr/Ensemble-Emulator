# INES Mapper 226

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_226) | View [other pages](Special_AllPages.xhtml#INES_Mapper_226)
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 226          =
     ========================
     
     Example Games:
     --------------------------
     76-in-1
     Super 42-in-1
     
     
     Registers:
     ---------------------------
     
     Range, Mask:  $8000-FFFF, $8001
     
       $8000:  [PMOP PPPP]
          P = Low 6 bits of PRG Reg
          M = Mirroring (0=Horz, 1=Vert)
          O = PRG Mode
     
       $8001:  [.... ...H]
          H = high bit of PRG
     
     
     PRG Setup:
     ---------------------------
     
     Low 6 bits of the PRG Reg come from $8000, high bit comes from $8001
     
     
                    $8000   $A000   $C000   $E000  
                  +-------------------------------+
     PRG Mode 0:  |             <Reg>             |
                  +-------------------------------+
     PRG Mode 1:  |      Reg      |      Reg      |
                  +---------------+---------------+
    

The [original describing document](http://nesdev.org/226.txt) states the 2s bit of the register at $8001 disables CHRRAM writes. No games seem to rely on it, though. It also says that the mapper hardware is made of a 74LS74A, 74LS273, [74LS139](74139.xhtml "74139"), [74LS02](7402.xhtml "7402"), and [74LS153](74153.xhtml "74153"). 

The multicart [reportedly relies](http://web.archive.org/web/20030602133825/http://www.nesplayer.com/pirates/76in1.htm) on the value of uninitialized memory to determine exactly how many games are available. 

The multicart clears both registers on soft reset. 

See also: [Repair of one such cartridge](http://forums.nesdev.org/viewtopic.php?t=5642)

  
At least three different games using this mapper exists: 

  * _42 in 1 (1 MB)_
  * _63 in 1 (1.5 MB)_
  * _76 in 1 (2 MB)_



  
They appear in different ROM package and count configurations: 

  * [![42in1 1x1MB chip](../wiki-images/42in1_1x1MB_chip.jpg)](File_42in1_1x1MB_chip_jpg.xhtml "42in1 1x1MB chip")

42in1 1x1MB chip 

  * [![42in1 2x512kB chip v1](../wiki-images/42in1_2x512kB_chip_v1.jpg)](File_42in1_2x512kB_chip_v1_jpg.xhtml "42in1 2x512kB chip v1")

42in1 2x512kB chip v1 

  * [![42in1 2x512kB chip v2](../wiki-images/42in1_2x512kB_chip_v2.jpg)](File_42in1_2x512kB_chip_v2_jpg.xhtml "42in1 2x512kB chip v2")

42in1 2x512kB chip v2 

  * [![42in1 2x512kB chip v3](../wiki-images/42in1_2x512kB_chip_v3.jpg)](File_42in1_2x512kB_chip_v3_jpg.xhtml "42in1 2x512kB chip v3")

42in1 2x512kB chip v3 

  * [![63in1 3x512kb chip](../wiki-images/63in1_3x512kb_chip.jpg)](File_63in1_3x512kb_chip_jpg.xhtml "63in1 3x512kb chip")

63in1 3x512kb chip 

  * [![76in1 1x2MB chip](../wiki-images/76in1_1x2MB_chip.jpg)](File_76in1_1x2MB_chip_jpg.xhtml "76in1 1x2MB chip")

76in1 1x2MB chip 

  * [![76in1 1x2MB epoxy or chip](../wiki-images/76in1_1x2MB_epoxy_or_chip.jpg)](File_76in1_1x2MB_epoxy_or_chip_jpg.xhtml "76in1 1x2MB epoxy or chip")

76in1 1x2MB epoxy or chip 

  * [![76in1 1x2MB epoxy](../wiki-images/76in1_1x2MB_epoxy.jpg)](File_76in1_1x2MB_epoxy_jpg.xhtml "76in1 1x2MB epoxy")

76in1 1x2MB epoxy 

  * [![76in1 2x1MB chip v1](../wiki-images/76in1_2x1MB_chip_v1.jpg)](File_76in1_2x1MB_chip_v1_jpg.xhtml "76in1 2x1MB chip v1")

76in1 2x1MB chip v1 

  * [![76in1 2x1MB chip v2](../wiki-images/76in1_2x1MB_chip_v2.jpg)](File_76in1_2x1MB_chip_v2_jpg.xhtml "76in1 2x1MB chip v2")

76in1 2x1MB chip v2 

  * [![76in1 2x1MB epoxy](../wiki-images/76in1_2x1MB_epoxy.jpg)](File_76in1_2x1MB_epoxy_jpg.xhtml "76in1 2x1MB epoxy")

76in1 2x1MB epoxy 

  * [![76in1 4x512kB chip](../wiki-images/76in1_4x512kB_chip.jpg)](File_76in1_4x512kB_chip_jpg.xhtml "76in1 4x512kB chip")

76in1 4x512kB chip 




Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
