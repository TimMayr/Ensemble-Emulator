# Talk:INES Mapper 043

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_043) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_043)

## Register mask

How confident are you about the masks? Involving 12 bits is a surprisingly large number. And A15 has to be involved—as is, writes to NES internal RAM ($0122, $0322, &c) would control IRQs. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:09, 20 April 2018 (MDT) 

    It's from the original Nintendulator source code. FCEUX implements this mapper somewhat differently, but also checks for a very specific mask. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 15:01, 20 April 2018 (MDT)
    And it turns out the reason FCEUX implements it differently is that it actually puts yet another SMB2J conversion (Mr. Mary 2) under the same mapper 43, which (of course) uses different hardware than Whirlwind Manu's LF36; for one thing, LF36 uses CHR-ROM, while Mr. Mary 2 uses CHR-RAM. I think I'm going to go nuts trying to document all the different SMB2J conversions. :P [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 15:16, 20 April 2018 (MDT) 

    Nintendulator explicitly has a range of $4000-$FFFF – `for (int i = 0x4; i < 0x10; i++) EMU->SetCPUWriteHandler(i, Write);` – so A15 is at least somehow involved...
    Given that this sounds like a discrete logic/PAL-based mapper (e.g. the IRQ is pretty clearly a 4040 or 4020), I'm going to bet that the mask is $F122 (or possibly $E122 or $C122), but without at least a photo of the hardware there's no real point in musing any further. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:22, 20 April 2018 (MDT)
