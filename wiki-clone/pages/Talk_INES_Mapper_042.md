# Talk:INES Mapper 042

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_042) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_042)

It seems that the mapper should disable IRQ and reset the IRQ counter when it is triggered, otherwise any ROM that uses the IRQ counter will get stuck in a loop. Please verify if this is correct. 

    Because of how the 6502's IRQ input works, IRQs cannot self-acknowledge. The IRQ is level triggered, not edge triggered (unlike the NMI input), and the mapper can only acknowledge it in response to something that the CPU does.
    "Something that the CPU does" could be: Reads the addresses of the IRQ vector, reads some other location in memory, or writes to some location in memory. Chances are very very good that mapper 42 is identical to the other discrete logic FDS ports ([iNES Mapper 040](INES_Mapper_040.xhtml "INES Mapper 040") and [iNES Mapper 050](INES_Mapper_050.xhtml "INES Mapper 050")) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:26, 22 August 2016 (MDT)
    FCEUX implements the IRQ as: a 15 bit up counter where IRQ is asserted while the two MSBs are high. Nestopia implements it as the same, but obfuscated. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:41, 22 August 2016 (MDT)
