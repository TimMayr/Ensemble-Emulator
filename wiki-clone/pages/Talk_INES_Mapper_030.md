# Talk:INES Mapper 030

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_030) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_030)

## Board allocation

Study Hall definitely needs its own mapper, but why does BK:MoT need to be here? It's handled by the standard m2 emulator oversize logic... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:54, 8 April 2014 (MDT) 

    While it is true that BK:MoT can function with just the Mapper 2 oversize, the game was made on the board that Mapper 30 is defined for. Given that it exists on the UNROM 512 board, I do have some plans to make a convenience hack, where the games continue points write something to the flash, so that the game can be saved, resumed later, without needing to write down passwords. Modding that to real cart will just be a matter of desoldering the 4 flash locked jumnpers, soldering the flash enabled jumper, mounting the 74*139, and flashing the modified rom to the cart. Such a hack would definitely need the mapper 30 definition. [Caitsith2](https://www.nesdev.org/w/index.php?title=User:Caitsith2&action=edit&redlink=1 "User:Caitsith2 \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Caitsith2&action=edit&redlink=1 "User talk:Caitsith2 \(page does not exist\)")) 12:15, 8 April 2014 (MDT)

    

    So... you want to allocate mapper 30 for a BK2 hack you plan to make in the future? I don't think it's good practice to allocate mappers before there is something to be emulated that needs one. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior"))

    

    

    Oh wait, I missed the comment about Study Hall. What are Study Hall's special requirements? Bankable CHR-RAM? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:09, 8 April 2014 (MDT)
    Ah, OK, I see from the [UNROM 512 Talk Page](Talk_UNROM_512.xhtml "Talk:UNROM 512") this mapper would support several RetroUSB games? I misunderstood the discussion. Are there other such games that don't already fall into existing mappers, or is it just Study Hall? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior"))

    

    

    

    Study Hall explicitly requires self-flashing support. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:41, 8 April 2014 (MDT)
