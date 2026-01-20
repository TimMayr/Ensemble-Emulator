# Talk:NES 2.0 Mapper 319

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0_Mapper_319) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0_Mapper_319)

## typo?

FCEUX's source in boards/hp898f.cpp swaps the location of the two registers relative to what's documented here. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:32, 15 May 2018 (MDT) 

Additionally, the description here (modifying overriding PRG A16 with CPU A14, instead of PRG A14 with CPU A14) really sounds like this description is a misdump. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:13, 15 May 2018 (MDT) 

    I have corrected the relative register location. Not sure what you mean by "misdumped description " in the second part. If the 16 KiB PRG-ROM bank bit 2 (4s) is modified, it's going to affect PRG A16, not A14. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 23:52, 15 May 2018 (MDT)
    Or do you mean the _ROM_ (and not the description) is misdumped, having the banks in a supposedly implausible order? I have not dumped the cart myself, and I am not going to assert bad dump status without having seen the board topology. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 00:10, 16 May 2018 (MDT) 

    That is exactly what I mean. There are a wealth of other boards which support both NROM-256 and NROM-128 by relaying or overriding CPU A14; this cart is unique in interpreting that signal as connecting to PRG A16 instead of A14. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:09, 16 May 2018 (MDT) 

    Here are the values that the menu is writing and the resulting 16 KiB bank numbers at 8000 and C000 according to the A16 description that runs all games on the multicart:
    
    
                   Written 16K Banks# @
                   Value   8000    C000
    Menu		38	07	07
    Super Mario    D0	02	06 *
    Star Force	C8	01	05 *
    Tennis		98	03	03
    F-1 Race	80	00	00
    Lode Runner	A0	04	04
    Duck Hunt	88	01	01
    Galaxian	38	07	07
    

    

    

    (The cart has Inventor's renamed hacks of the games; I used the original names.) I'm struggling to think how to rearrange the banks in such a way that Super Mario could work with an A14 override (yielding in banks 02,03 instead of 02,06) without breaking Tennis. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 14:29, 16 May 2018 (MDT) 

    There are two options; 1- rotate the 16KiB segments: 0 1 2 3 4 5 6 7 → 0 2 4 6 1 3 5 7. The register then becomes [MDpP P...], where PRGA16,15=PP, PRGA14=D?CPUA14:p
    2- swap A16 and A14, 0 1 2 3 4 5 6 7 → 0 4 2 6 1 5 3 7. The register is then the same as above but PP is bit-reversed. Either way, it makes it clear it's actually 32K banking. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:05, 16 May 2018 (MDT) 

    I have added a disclaimer. If CaH4e3 still has the cartridge, maybe he can provide a PCB image to clear this up. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 16:58, 16 May 2018 (MDT)

Having dumped additional cartridges using this mapper, I have redefined the PRG register to expect a more sensible bank order. That makes the existing UNIF ROM image of _Prima Soft 9999999-in-1_ obsolete at least when directly converted to NES 2.0. I could not reproduce what FCEUX does with the low CHR bank bits, so I have removed that for now, and none of the dumped files seem to use it. I have PCB images from Consolethinks, but all there is to see is a single glob and a solder pad. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 11:54, 21 September 2019 (MDT) 
