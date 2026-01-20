# Talk:PPU rendering

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APPU_rendering) | View [other pages](Special_AllPages.xhtml#Talk_PPU_rendering)

## Contents

  * 1 0=blank vs. 0=active
  * 2 Garbage nametable byte
  * 3 Out of date
  * 4 Delay between shifters shifting and drawing
  * 5 Does it fetch everything?
  * 6 Pre-render scanline dummy fetches...where?



## 0=blank vs. 0=active

There are two different conventions: 

0=active | 0=blanking | Description   
---|---|---  
261 or -1 | 20 | Pre-render scanline   
0-239 | 21-260 | Active picture   
240 | 261 | Post-render scanline   
241-260 | 0-19 | Vertical blanking   
  
You're using the 0=blanking convention. But more often on the BBS and elsewhere on this wiki, I've seen 0=active. The [PPU power up state](PPU_power_up_state.xhtml "PPU power up state") appears to agree with 0=active, as do the (X, Y) coordinate displays in the debugger of FCEUX and Nintendulator and the VCOUNT register on the Game Boy ($FF44) and Game Boy Advance ($04000006). The 0=active convention makes it easier to consolidate discussion of NTSC, PAL, and PAL clone timing as well. --[Tepples](User_Tepples.xhtml "User:Tepples") 20:15, 3 April 2011 (UTC) 

    Ahh, ok. I was just simply going off of what's in Brad Taylor's docs. The 0=blank method didn't make a lot of sense to me either, as does some of the other stuff in this doc (The shift registers make more sense if they shift _left_ rather than right). I'll adjust this. --[Drag](User_Drag.xhtml "User:Drag") 20:47, 3 April 2011 (UTC)

I think at some point, it would be worthwhile to document how the PPU outputs the vertical/horizontal sync signals. If for nothing else than to satisfy my own curiosity. --[Drag](User_Drag.xhtml "User:Drag") 05:30, 6 July 2011 (UTC) 

## Garbage nametable byte

Before each sprite's patterns are loaded, at x=257, 259, 265, 267, ..., 313, 315, there is a garbage nametable fetch. Has anyone discovered a pattern to these fetches' addresses that, say, a mapper could use? --[Tepples](User_Tepples.xhtml "User:Tepples") 16:07, 6 July 2011 (UTC) 

    I was going to ask the same question, what are the addresses of the garbage nametable bytes? I would like to know, too. It would be useful for this purpose, as well as if you are making a hardware clone or whatever and want it accurate. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 13:37, 17 March 2013 (MDT)

## Out of date

Some of the timing information is out of date now (see forums.nesdev.org/viewtopic.php?f=3&t=9901). I'll update it once I get my account activated. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 18:38, 15 March 2013 (MDT) 

## Delay between shifters shifting and drawing

As the notes in the diagram mention, there's actually a delay of two ticks from when the shifters shift to when the pixel is output. Looks like this is due to palette lookup. Here's a Visual 2C02 trace around the beginning of a scanline with the high and low pattern bits = 0x80 (so that the palette index is 3 for the first pixel and 0 for the rest): 
    
    
    hpos	vid_	tile_l	tile_h	pal_ptr
    000	080	fefe	0101	00
    000	080	fefe	0101	00
    000	080	fefe	0101	00
    000	080	fefe	0101	00
    000	080	fefe	0101	00
    000	080	fefe	0101	00
    000	080	fefe	0101	00
    000	080	fefe	0101	00
    001	080	fefe	0101	00
    001	080	fefe	0101	00
    001	080	fefe	0101	00
    001	080	fefe	0101	00
    001	080	fefe	0101	00
    001	080	fefe	0101	00
    001	080	fefe	0101	00
    001	080	fefe	0101	00
    002	080	ff7f	8080	00
    002	080	ff7f	8080	00
    002	080	ff7f	8080	00
    002	080	ff7f	8080	00
    002	080	ff7f	8080	00
    002	080	ff7f	8080	00
    002	080	ff7f	8080	00
    002	080	ff7f	8080	00
    003	080	ffbf	c040	03
    003	080	ffbf	c040	03
    003	080	ffbf	c040	03
    003	080	ffbf	c040	03
    003	080	ffbf	c040	03
    003	080	ffbf	c040	03
    003	080	ffbf	c040	03
    003	080	ffbf	c040	03
    004	080	ffdf	e020	00
    004	080	ffdf	e020	00
    004	080	ffdf	e020	00
    004	080	ffdf	e020	00
    004	400	ffdf	e020	00
    004	400	ffdf	e020	00
    004	400	ffdf	e020	00
    004	400	ffdf	e020	00
    005	400	ffef	f010	00
    005	400	ffef	f010	00
    005	400	ffef	f010	00
    005	400	ffef	f010	00
    005	080	ffef	f010	00
    005	080	ffef	f010	00
    005	080	ffef	f010	00
    005	080	ffef	f010	00
    006	080	fff7	f808	00
    006	080	fff7	f808	00
    006	080	fff7	f808	00
    006	080	fff7	f808	00
    006	080	fff7	f808	00
    006	080	fff7	f808	00
    006	080	fff7	f808	00
    006	080	fff7	f808	00
    007	080	fffb	fc04	00
    007	080	fffb	fc04	00
    007	080	fffb	fc04	00
    007	080	fffb	fc04	00
    007	080	fffb	fc04	00
    007	080	fffb	fc04	00
    007	080	fffb	fc04	00
    007	080	fffb	fc04	00
    008	080	fffd	fe02	00
    008	080	fffd	fe02	00
    008	080	fffd	fe02	00
    008	080	fffd	fe02	00
    008	080	fffd	fe02	00
    008	080	fffd	fe02	00
    008	080	fffd	fe02	00
    008	080	fffd	fe02	00
    009	080	fefe	0101	00
    009	080	fefe	0101	00
    009	080	fefe	0101	00
    009	080	fefe	0101	00
    009	080	fefe	0101	00
    009	080	fefe	0101	00
    009	080	fefe	0101	00
    009	080	fefe	0101	00
    00a	080	ff7f	8080	00
    00a	080	ff7f	8080	00
    

The shifters shift at h=2, the palette address changes at h=3 for the palette lookup, and the pixel is drawn during h=4 (as seen by vid_ changing). 

It also looks like the shift registers are actually reloaded at h=9,17,25,... instead of at h=8,16,24,..., so that should be fixed. :/ -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 17:14, 5 April 2013 (MDT) 

## Does it fetch everything?

If at least one of the background or sprites is enabled, will it still fetch both even though only one is used? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 01:25, 18 June 2013 (MDT) 

    During rendering, the PPU's address bus is _never idle_. I don't know what addresses it reads if background is disabled, but I think the sprite reads from tile #$FF. (Visual2C02 can tell you the right answer) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 02:44, 18 June 2013 (MDT)
    IIRC, disabling only one of them doesn't affect sprite evaluation and fetches at all, but only the final drawing stage (where sprite zero hit detection happens as well). I think you can re-enable sprite rendering in the middle of a sprite and have half of it draw fine. --[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 08:56, 18 June 2013 (MDT)

## Pre-render scanline dummy fetches...where?

"Although no pixels are rendered for this scanline, the PPU still makes the same memory accesses it would for a regular scanline." But *which* memory accesses? Which line is it looking at for the dummy fetches? Same as line 0? Something else? [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 23:46, 20 April 2015 (MDT) 

    I know this is 8 years later, but my best guess is that the fetches come from whatever value is in the PPU's [v](PPU_scrolling.xhtml#PPU_internal_registers "PPU scrolling") register by the time the pre-render scanline starts. So for example, if the vblank routine finishes by setting the PPU scrolling, then scanline -1 would make the same fetches as scanline 0 will. If the PPU wasn't touched during vblank at all, then scanline -1 would make fetches as though it were rendering a hypothetical "scanline 240". Having confirmation on this would be great. [Drag](User_Drag.xhtml "User:Drag") ([talk](User_talk_Drag.xhtml "User talk:Drag")) 20:19, 23 April 2023 (UTC) 

    Yes. It starts rendering from whatever random contents are in V before it fixes things by reloading from T. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:52, 23 April 2023 (UTC)
