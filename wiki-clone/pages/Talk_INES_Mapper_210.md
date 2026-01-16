# Talk:INES Mapper 210

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_210) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_210)

Does this mapper actually exist? The boards Disch mentions as #210 are #19 in Bootgod's DB and don't seem to have hardwired mirroring when I trace the board. [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 23:11, 15 June 2012 (PDT) 

    I checked out a bunch of roms. They all came to me as mapper 19. I removed the 210 special logic from this mapper, and they all seemed to work, as long as the default nametable registers were $FE, $FF, $FE, $FF (vertical mirroring). Wagyan 2 depended on this. Wagyan 2 is NAMCOT-175. Splatterhouse 2 is NAMCOT-175 as well, and it needs vertical mirroring as well, but it sets its registers. So Wagyan 2 must be on a board that uses the nametable regs and is truly depending on the default values. Therefore, I agree that there is no difference between 019 and 210.

    Even if there is no functional difference between 019 and 210, if there are roms in the wild that are 210, we should keep the mapper reserved for it, at the very least with notes similar to this. Itd be a good candidate for recovery, but there are better. Ill have to scour my roms and see if there are any 210 left.
    [Zeromus](User_Zeromus.xhtml "User:Zeromus") 08:49, 16 June 2012 (PDT)

    

    I wonder if m210 is 'supposed' to represent the Namco175/340, with its simpler mirroring control (as described [by naruko](http://d.hatena.ne.jp/na6ko/20110501)). It's just close enough, and few enough games were released, that I could see how "fixed mirroring" or "correct default" might be good enough to make the games work. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 12:04, 18 June 2012 (PDT)

  
Ok, so having looked over Naruko's notes and added them to the [Namco 163](INES_Mapper_019.xhtml "Namco 163") description, my take is: 

  * Mapper 19 is Namcot 129/163, or an unseen variant of it without sound.
  * Mapper 210 was originally for Namcot 175, which has hardwired mirroring, but has only one functionality conflict (register at $c000 becomes WRAM protect, but Naruko only mentions one game that has it), and so almost any game that used the Namcot 175 can probably actually be emulated safely using Mapper 19.
  * No mapper was allocated for the Namcot 340, which uses the top bits of $e000 to select 1ScA/H/V/1ScB mirroring.



I think I can interpret Naruko's documentation to recommend that m210 be reallocated to the n340, and that the one n175-with-wram game be emulated as n340. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 12:53, 24 June 2012 (PDT) 
