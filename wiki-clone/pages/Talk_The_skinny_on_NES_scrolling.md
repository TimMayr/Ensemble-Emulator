# Talk:The skinny on NES scrolling

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AThe_skinny_on_NES_scrolling) | View [other pages](Special_AllPages.xhtml#Talk_The_skinny_on_NES_scrolling)

## Contents

  * 1 Changes 1/08/2013
  * 2 Temporary X?
  * 3 hblank
  * 4 Short writes
  * 5 Actual timing for V/T updates during rendering
  * 6 Split X/Y scroll simplification



## Changes 1/08/2013

I made the following changes: 

  * I removed the first example, since it appeared to be broken w.r.t. fine Y scroll.


  * I added a note about $2007 having side effects on _v_ , please elaborate or correct it if anything is wrong. I note that nintendulator does a weird Y-1 thing if rendering is enabled when it is used? I'm not sure what is going on there.


  * Bit 14 of _t_ was inconsistently sometimes referred to as bit 15.


  * Added a note about Y increment of _v_ , which appears to occur on pixel 250 in nintendulator? I find this confusing, so I would appreciate if someone with authoritative knowledge could correctly explain the timing for the Y increment of _v_. Complete wrapping logic would also be helpful.


  * The imaginary bit 15 was removed from the examples at the top of the page, for clarity and consistency.


  * Added examples for simple scrolling, where only X, or no split is needed.


  * Revised the $2006, $2005, $2005, $2006 example to be a more practical example, explaining how to set the scroll for a specific X/Y/nametable.


  * Explanation of why the registers work this way, trying to make it easier to remember.



Please check for errors, and make corrections as necessary. We could also use some information on tile/attribute fetching, timings and which/whether can be changed mid-scanline. 

\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 03:31, 8 January 2013 (MST) 

## Temporary X?

Does _x_ also have a corresponding temporary/reload register that gets copied into _x_ at the beginning of each line? 

Correct me if I'm wrong, but can't I write $2005 anywhere in the scanline, and _x_ should be reloaded to the expected value at the start of the next line. If there was no reload value for it, and _x_ is immediately set by $2005, it would be absolutely critical where in the scanline I happen to set it, and I don't remember this ever being the case when trying it on an NES. 

\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 05:01, 8 January 2013 (MST) 

    I guess this one doesn't matter. If it is reloaded we can just refer to the reload value as _x_. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 06:42, 8 January 2013 (MST)

## hblank

Making scroll writes during hblank... Some threads have suggested this, but is it possible, and how? When I've tried to do scrolling in the past, I could never seem to get timings that accurate, so it was always necessary to pull it back from the right edge and accept some amount of glitching on the end of the line. Glitches like this seem to be very normal for games with split scrolling... can it actually be avoided? Is it bad advice to suggest timing it during hblank? 

\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 05:33, 8 January 2013 (MST) 

    Okay, enough information has been gathered on the forum to answer this. The article is updated. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 18:03, 8 January 2013 (MST)

## Short writes

Is it worth describing short sequences for quicker (and sometimes easier to calculate) but incomplete scrolling (i.e. 2005/2006, 2006/2006)? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 19:57, 8 January 2013 (MST) 

    I don't see why not. Maybe put them in an "advanced examples" section? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 23:22, 8 January 2013 (MST)

## Actual timing for V/T updates during rendering

Some of this was mentioned in the Visual 2C02 forum thread, but I've just double-checked it in the simulator: 

  * Pre-render scanline, _all_ dots 280 thru 304 - copy **non** -horizontal bits (yyyN.YYYYY.....) from T to V
  * Every scanline, dots 328/336/8/16/24/32/.../240/248/256 - increment horizontal bits in V
  * Every scanline, dot 256 - increment vertical bits in V
  * Every scanline, dot 257 - copy horizontal bits from T to V



Note that these are using the "revised" cycle timings mentioned [here](http://forums.nesdev.org/viewtopic.php?f=9&t=9440&start=15#p102081). --[Quietust](User_Quietust.xhtml "User:Quietust") 20:49, 10 January 2013 (MST) 

    Wow that's some great information, thanks! I will integrate it into the article when I have some time, if somebody else doesn't get to it first. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 02:49, 11 January 2013 (MST)

## Split X/Y scroll simplification

The example given in the article for mid-frame X/Y scroll updates uses the write sequence $2006 + $2005 + $2005 + $2006, where both of the $2006 write values need to be calculated ahead of time. Since only the last write is necessary to make the Y-scroll update take place, you can replace the first $2006 write with a $2005 write (if the base nametable isn't going to change, otherwise you'd need an additional write to $2000) which can be a bit simpler to actually code since you'd only need 3 distinct values, allowing you to safely fit the entire sequence within HBLANK (STX $2005; STY $2005; STX $2005; STA $2006). --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 14:18, 8 August 2013 (MDT) 

    On the one hand, now all four writes can fit in hblank. On the other hand, now all four writes _have_ to be in hblank to avoid visible glitches. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:05, 8 August 2013 (MDT) 

    Correct. There's a window of about 20 CPU cycles when _v_ can be modified, namely during sprite fetch. A sequence of _n_ consecutive writes takes 4 _n_ \- 3 cycles from the start of the first write to the end of the last: 4 _n_ because STA $200x takes four cycles, and - 3 because the first opcode fetch doesn't count toward the timing. The $2006-$2005-$2005-$2006 sequence requires only the last two writes cycle of to be in this window, resulting in a write sequence that occupies 5 of 20 cycles. The simplification requires the whole thing because of the immediate effect of fine X changes, resulting in 13/20, and 7-cycle jitter is at the very limit of what can be achieved with sprite 0 or NMI alone. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 12:28, 9 August 2013 (MDT)
