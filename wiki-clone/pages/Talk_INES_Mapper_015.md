# Talk:INES Mapper 015

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_015) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_015)

On the page it says that in power-up state, the bank mode is 2, which is 8K. While I don't have any experience with testing out hardware mappers, I do have some ROMs that have this mapper and both don't have valid interrupt vectors in the first 8K. It seems to me more likely that the power-up state is bank mode 0 (32K), which both my emulators seem to use for this mapper. If I configure my emulator to use another bank mode at boot, the game won't run.--[Tai Ferret](https://www.nesdev.org/w/index.php?title=User:Tai_Ferret&action=edit&redlink=1 "User:Tai Ferret \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Tai_Ferret&action=edit&redlink=1 "User talk:Tai Ferret \(page does not exist\)")) 10:52, 5 April 2016 (MDT) 

## Bank mode in Powerup State

I can also confirm that the Mapper implementation works (at least for the Contra 16 100-in-1 ROM) only when the starting bank mode is 0, not 2. 
