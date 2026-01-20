# Talk:Four player adapters

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AFour_player_adapters) | View [other pages](Special_AllPages.xhtml#Talk_Four_player_adapters)

Is it possible to use the NES Four Score connected to each NES controller port and to the Famicom four-player adapter to make eight players? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 13:39, 20 March 2013 (MDT) 

    That depends on photos of the controller sockets on the Famicom four-player adapter. If they're identical to NES controller sockets, or an extension cable would make them that way, I don't see how it wouldn't work. Just make sure at least one of them is actually a (wired) Four Score; two (infrared) NES Satellite accessories would probably just interfere with each other. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 14:45, 20 March 2013 (MDT)

    The wired controllers ([commons:File:Wikipedia_nes_fourscore.jpg](https://commons.wikimedia.org/wiki/File:Wikipedia_nes_fourscore.jpg "commons:File:Wikipedia nes fourscore.jpg")) plug into both ports on the front on the NES or AV Famicom. Player 1 and 2 are permanently attached to the plain famicom. You _could_ cut apart a NES four score and solder on a DA15 connector to get 6 players on a conventional famicom.
    The Famicom expansion port doesn't connect to $4016.0, so there's a bunch of discussion/confusion on the internet about how Hori's four player adapter could possibly work. Nonetheless, games do expect the four player adapter to work as outlined on [Standard controller](Standard_controller.xhtml "Standard controller") —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:55, 21 March 2013 (MDT)

## Contents

  * 1 Rackets & Rivals
  * 2 New entries added
  * 3 Hori 4-player adapter mode 4 in japanese games
  * 4 Rename to "Multiplayer adapters"?



## Rackets & Rivals

How do I get edit permissions? I've confirmed on Nestopia on RetroArch that 4 separate inputs are supported for Rackets & Rivals, a European PAL exclusive officially released game. That game is not currently in the NesCartDB so I believe that's why it was missed.—[VideogameScrapbook](https://www.nesdev.org/w/index.php?title=User:VideogameScrapbook&action=edit&redlink=1 "User:VideogameScrapbook \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:VideogameScrapbook&action=edit&redlink=1 "User talk:VideogameScrapbook \(page does not exist\)")) 11:22, 13 August 2019 (MDT) 

    I am told that edit permissions for non-talk pages are automatically granted after a couple talk page edits. Meanwhile, I've added Rackets & Rivals to the page. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:58, 13 August 2019 (MDT)

## New entries added

I now have edit permissions, so I listed other 4 player games I've confirmed work with an authentic Famicom: 

Famicom 4 Players Adapter: 

  * Bomber Man II (Japan)



Homebrew games: 

  * Super PakPak (NES and Famicom)



Hacks: 

  * Battle City - 4 Players v1.3 Ti (NES and Famicom)
  * Battle City Mario - 4 players v1.0 NesDraug (NES and Famicom)



\--[VideogameScrapbook](https://www.nesdev.org/w/index.php?title=User:VideogameScrapbook&action=edit&redlink=1 "User:VideogameScrapbook \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:VideogameScrapbook&action=edit&redlink=1 "User talk:VideogameScrapbook \(page does not exist\)")) 17:57, 25 August 2019 (MDT) 

  


## Hori 4-player adapter mode 4 in japanese games

Why my edit is reverted? Can you list at least one japanese game that checks Hori's signature and uses all four Hori's controller ports at once? Afaik all 3-4 player japanese games using $4016.1 and $4017.1 four third and fourth controllers, without any extra logic. [Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 03:09, 5 October 2020 (MDT) 

    There's a section at the bottom of the page that explicitly enumerates games that use Hori's 4PA. Of those, I've [disassembled Wit's controller reading code](https://forums.nesdev.org/viewtopic.php?p=237997#p237997) and verified that it will work in four player mode with either a 4PA or a Four Score, but will not work by reading four independent controllers _without_ the 4PA or Four Score signature. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:15, 5 October 2020 (MDT) 

    Okay, I'm wrong. So I made more research:

  * Bomberman II (J) - Hori Adapter **not** supported
  * Downtown Nekketsu Koushinkyoku: Soreyuke Daiundoukai - Hori Adapter **supported**
  * Ike Ike! Nekketsu Hockey Bu: Subette Koronde Dai Rantou - Hori Adapter **not** supported
  * Moero TwinBee: Cinnamon-hakase o Sukue! - Hori Adapter **not** supported
  * Nekketsu Kakutou Densetsu - Hori Adapter **not** supported
  * Nekketsu Koukou Dodge Ball Bu - Hori Adapter **not** supported
  * Nekketsu Street Basket: Ganbare Dunk Heroes - Hori Adapter **not** supported
  * Kunio-kun no Nekketsu Soccer League - Hori Adapter **not** supported
  * U.S. Championship V'Ball - Hori Adapter **supported**
  * Wit's - Hori Adapter **supported**



So only 3 of 10 Famicom games support it. Let's check NES Four Score compatible games with Hori adapter (on Famicom via cartridge adapter): 

  * Bomberman II (U) - Hori Adapter still **not** supported
  * Gauntlet II - Hori Adapter **not** supported
  * Greg Norman's Golf Power - Hori Adapter **supported**
  * Harlem Globetrotters - Hori Adapter **not** supported
  * Indy Heat - Hori Adapter **supported**
  * Kings of the Beach - Hori Adapter **supported**
  * M.U.L.E. - Hori Adapter **not** supported
  * Magic Johnson's Fast Break - Hori Adapter **supported**
  * Monster Truck Rally - Hori Adapter **not** supported
  * NES Play Action Football - Hori Adapter **supported**
  * A Nightmare on Elm Street - Hori Adapter **supported**
  * Nintendo World Cup - Hori Adapter **supported** (while it not supported by japanese version!)
  * Rackets and Rivals - Hori Adapter **not** supported
  * R.C. Pro-Am II - Hori Adapter **supported**
  * Roundball: 2-on-2 Challenge - Hori Adapter **not** supported
  * Smash T.V. - Hori Adapter **not** supported
  * Spot - Hori Adapter **supported**
  * Super Jeopardy! - Hori Adapter **supported**
  * Super Off Road - Hori Adapter **supported**
  * Super Spike V'Ball - Hori Adapter **supported**
  * Swords and Serpents - Hori Adapter **not** supported
  * Top Players' Tennis - Hori Adapter **supported**



13 of 22. It's most 4-player NES games. E.g. these games check for the Hori's signature in the $4016.1 and $4017.1, not only Four Score's signature in the $4016.0 and $4017.0. And it's weird because you can't connect Hori's adapter to NES. 

Also I made fceux fork with Hori adapter emulation: <https://github.com/ClusterM/fceux/tree/hori> [Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 01:53, 6 October 2020 (MDT) 

    Hm. We should reformat this section of the page with your data, and to explicitly mention which multiplayer protocols (4PA / FS / "SNES-like") are supported by each game. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 22:56, 6 October 2020 (MDT)
    Add Sachen's "Millionaire" to the list of games that take four independent players' input using the Japanese protocol. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 08:39, 7 October 2020 (MDT)

Uh, how can Wit's support the "simple" 3/4 player mode? I know it reads both the two LSbits for the first eight reads, but it always folds them togeher. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:13, 8 October 2020 (MDT) 

    I have not analyzed the code, I just found it working when I tried it.[NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 12:40, 8 October 2020 (MDT)
    The routine at $FE29 is not the only routine for controller reading. $FE29 is actually called by $C131, and will return ZR if the Four Player signature is found, and NZ when not. If the the Four Player signature has not been found, the game executes a _second_ controller reading routine at $C136 that stores the 1P result in $1E, the 3P "simple mode" result in $20, the 2P result in $1F and the 4P "simple mode" result in $21. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 12:53, 8 October 2020 (MDT)

## Rename to "Multiplayer adapters"?

I find it odd that this page is called "Four player adapters" when the contents are about adapters that range from 2 to 8 players. This really should just be named "Multiplayer adapters" or something along those lines. --[Bro3256](User_Bro3256.xhtml "User:Bro3256") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bro3256&action=edit&redlink=1 "User talk:Bro3256 \(page does not exist\)")) 06:30, 14 April 2025 (UTC) 

    No the adapters don't. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 07:30, 14 April 2025 (UTC)
