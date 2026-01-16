# PEGASUS 5 IN 1

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PEGASUS_5_IN_1) | View [other pages](Special_AllPages.xhtml#PEGASUS_5_IN_1)

[![5-in-1 menu.png](../wiki-images/5-in-1_menu.png)](File_5_in_1_menu_png.xhtml)

[](File_5_in_1_menu_png.xhtml "Enlarge")

[![](../wiki-images/5in1_2xROM_components.jpg)](File_5in1_2xROM_components_jpg.xhtml)

[](File_5in1_2xROM_components_jpg.xhtml "Enlarge")

2xROM version components

[![](../wiki-images/5in1_2xROM_bottom.jpg)](File_5in1_2xROM_bottom_jpg.xhtml)

[](File_5in1_2xROM_bottom_jpg.xhtml "Enlarge")

2xROM version bottom side

[![](../wiki-images/5in1_2xROM_sch.png)](File_5in1_2xROM_sch_png.xhtml)

[](File_5in1_2xROM_sch_png.xhtml "Enlarge")

2xROM version schematics

[![](../wiki-images/5in1_4xROM_components.jpg)](File_5in1_4xROM_components_jpg.xhtml)

[](File_5in1_4xROM_components_jpg.xhtml "Enlarge")

4xROM version components side

[![](../wiki-images/5in1_4xROM_top.jpg)](File_5in1_4xROM_top_jpg.xhtml)

[](File_5in1_4xROM_top_jpg.xhtml "Enlarge")

4xROM version top side

[![](../wiki-images/5in1_4xROM_bottom.jpg)](File_5in1_4xROM_bottom_jpg.xhtml)

[](File_5in1_4xROM_bottom_jpg.xhtml "Enlarge")

4xROM version bottom side

[![](../wiki-images/5in1_4xROM_sch.png)](File_5in1_4xROM_sch_png.xhtml)

[](File_5in1_4xROM_sch_png.xhtml "Enlarge")

4xROM version schematics

PEGASUS 5 IN 1 (known in Poland as Złota Piątka - Golden Five) is an unlicenced Famiclone cartridge (sold in box with manual), which consists of the following 5 Codemasters' games (each of them was also separately available as licensed game): 

  * Big Nose Freaks Out,
  * Micro Machines,
  * Fantastic Adventures of Dizzy (blue version),
  * Ultimate Stuntman,
  * Big Nose The Caveman.



Cartridge menu was encoded into Dizzy and Ultimate Stuntman was slightly modified (Codemaster's title screen was cut off and game was adapted to be playable on Dendy's clones, as the original ROM hangs due to bug in video system detection routine). Other games are the same like licensed version. 

The cartridge PCB was available in two versions: 

  * 4xROM (3x 2 Mbit mask ROM + 4 Mbit mask KROM with additional 74138 decoder)
  * 2xROM (2 Mbit mask ROM + 8 Mbit mask ROM)



First version is probably older due to earlier datamarks on chips. PAL chip is slightly different in both versions, although the ROM's content (after combining) and mapper description is the same 

# Registers

OUTER & INNER registers are cleared on startup but not software reset 
    
    
    $8000-$bfff: OUTER REGISTER
    [.... wPPP]
          ||||
          |+++- select 256KB outer PRG bank (one out of 5 games: 0=Dizzy, 1=BNFO, 2=US, 3=BNTC, 4=MM)
          +---- 1: protect OUTER register from further writes
    
    
    
    $c000-$ffff: INNER REGISTER
    [.... pppp]
          ||||
          ++++- select 16KB inner PRG bank 
    

# Memory map

  * $8000-$bfff - switchable 16 kB PRG-ROM bank
  * $c000-$ffff - fixed 16 kB PRG-ROM bank


    
    
    $8000-$bfff | $c000-$ffff
    ------------+------------
    PPPpppp     |  PPP1111
    

  


# Trivia

  * PAL's marking on every PCB was removed to make rev-en harder,


  * The reset R-C circuit (which clears both registers at startup but does not affect them after console reset) is problematic, causing the cartridge to not start properly on every power-up or make this is a side-effect of the copy-protection descibed bellow,


  * This multicart uses something whis acts like copy protection: 
    * When game is power up, all regs are cleared by R-C circuit, so the menu routine from Dizzy's ROM is started,
    * At some moment game check for magic value in RAM if it was run one time or not. At this time it was not but it marks that the first time hast just occured.
    * At CPU cycle 101077 (0.06sec after powerup) game writes $0C (0b1100) to $8927, which would normally result in switching to Micro Machines bank and protect outer register from further writes. However, 100n capacitor in RC circuit is charged by very high 39k resistance and at this time it hasn't charged yet so the register is still hold in reset and ignores writes. Then game compares $0C with $08 and because comparison fails, it jumps to reset ($FFFC).
    * Now checking for first run fails so the game jumps to display menu routine.


