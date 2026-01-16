# Talk:The frame and NMIs

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AThe_frame_and_NMIs) | View [other pages](Special_AllPages.xhtml#Talk_The_frame_and_NMIs)

quote "You can't keep the music updated during slowdown, or during bulk drawing transitions, because NMIs will be disabled"

Sure you can (while keeping NMIs enabled, I mean). I'm not sure if I should edit it in, I don't want to turning a newbie-friendly article into a technical dorkfest, but I was thinking of this: first instruction in the NMI would be 'inc nmi_nest_level', and last NMI instruction (before RTI) would be 'dec nmi_nest_level' the NMI routine would check the nest_level variable, if it is higher than 1, then you could could the play music, then exit. To avoid corrupting temporary variables in the music play routine, you would need to guarantee that the music routine can never be interrupted. [Memblers](https://www.nesdev.org/w/index.php?title=User:Memblers&action=edit&redlink=1 "User:Memblers \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Memblers&action=edit&redlink=1 "User talk:Memblers \(page does not exist\)")) 23:17, 6 October 2014 (MDT) 

  * _Pokémon Red and Blue_ for Game Boy Color appear to do exactly this. You can hear the music lag and then catch up. If that effect is desirable, you could edit it into the article. (_Klax_ for NES and _Tetris_ for Game Boy lag and don't catch up.) --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 08:03, 8 October 2014 (MDT)



I think I mis-explained, I mean the point is the music wouldn't need to lag at all - during a frame overflow, the 'nested' NMI would do nothing but play the music. Similarly, during a bulk VRAM transfer, NMI can be left enabled as long the NMI routine is following a path/mode that doesn't touch the PPU registers. But yeah I'm still not too eager to add it to the article, I think it's better to encourage people to use the mainloop and NMI together, though I suppose the bulk VRAM transfer would still apply to that case. [Memblers](https://www.nesdev.org/w/index.php?title=User:Memblers&action=edit&redlink=1 "User:Memblers \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Memblers&action=edit&redlink=1 "User talk:Memblers \(page does not exist\)")) 21:23, 26 October 2014 (MDT) 
