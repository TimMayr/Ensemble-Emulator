# File talk:Ntsc timing.png

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File_talk%3ANtsc_timing.png) | View [other pages](Special_AllPages.xhtml#File_talk_Ntsc_timing_png)

the first green ppu cycle 0 on the visible scanline 0 seems to be misleading. according to blargg's odd/even frame test, this skipped cycle should be arranged at the last ppu cycle of the pre-render scanline, and the idle cycle presented in the image should be a read (if odd + rendering enabled). \--~~110.206.2.212~~ 01:34, 10 December 2013 (MST) 

  * One of the comments (at the lower-right) states that the skipped cycle is actually at y=261,x=340 (and circuit analysis confirms that this is when it happens); I'm not sure why it's shown the way it is on the diagram. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 05:23, 11 December 2013 (MST) 
    * Might it have to do with the long-standing off-by-one error in cycle counts? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 06:28, 11 December 2013 (MST)
    * I think I drew it like that to avoid giving the impression that the final NT fetch on the pre-render line is skipped during odd frames. What really happens is that the final cycle of the fetch moves to (0,0), replacing the idle tick. Can't think of a non-convoluted way to express that in the diagram. --[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)"))


