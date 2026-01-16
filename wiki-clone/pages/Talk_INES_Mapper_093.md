# Talk:INES Mapper 093

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_093) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_093)

## Banking CHRRAM

If, for some reason, you were to remove the 8KiB CHR RAM IC on the Sunsoft-3R PCB and replace it specifically with a 62256 (the largest compatible 28-pin RAM), the functionality of that PCB would then become: 
    
    
     $8000-FFFF:  [.PPP .C.C]
        P = PRG Reg  (16k @ $8000)
        C = 8KiB CHR RAM bank
    

No-one has ever done this.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:12, 12 May 2013 (MDT) 

    Are you capable of running a test ROM for bankable CHR RAM? It could be used as part of the test suite for this mapper's NES 2.0 representation. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 15:24, 12 May 2013 (MDT) 

    No, I don't have the parts on hand. I think I'd rather define all discrete logic mappers as "can be used to bank CHR-RAM in place of CHR-ROM if the NES2.0 header specifies no CHR ROM and more than 8KiB CHR RAM" rather than formalizing the above idiosyncrasy of noncontiguous bits; e.g. m70 with CHR-RAM is strictly superior for homebrew because it supports 4 bits for each of CHR and PRG instead of 2 and 3, respectively. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:33, 12 May 2013 (MDT)

## Deprecated for development ?
    
    
    This mapper is deprecated for new development. Homebrew projects other than mapper tests should use UxROM (iNES Mapper 002) instead. 
    

Who is this wiki to dictate to readers which mappers to use for their new development ? The NES itself is anyways "deprecated for new development". If someone wants to use mapper 93 instead of mapper 2, why shouldn't they be allowed ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 03:41, 11 April 2017 (MDT) 

    Perhaps [Tepples](http://wiki.nesdev.org/w/index.php?title=INES_Mapper_093&diff=5154&oldid=4052) meant to say that its use is **discouraged** due to the rarity of actual cartridges that use it? --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 06:03, 11 April 2017 (MDT)
