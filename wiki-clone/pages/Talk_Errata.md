# Talk:Errata

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AErrata) | View [other pages](Special_AllPages.xhtml#Talk_Errata)

## To-do list

Some things to add by people who know what these refer to: 

PPU: must wait a couple of vblanks before chip is fully active 

PPU: can't reliably write to ports more often than several CPU cycles apart 

Page-crossing pointer quirks: 

JMP ($nnnn) 

($nn),Y 

($nn,X) -- [Blargg](User_Blargg.xhtml "User:Blargg") ([talk](User_talk_Blargg.xhtml "User talk:Blargg")) 18:03, 21 October 2013 (MST) 

    That's something personal, but I don't think zero-page overflowing is a caveat at all... after all it's called zero-page adressing, so it's suppose to address bytes in zero page. If it would overflow into page 1 THEN it'd be a caveat, but apparently it's not the case.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 15:09, 23 May 2014 (MDT)

    

    A terminology issue... I think it's a caveat, but not an errata. It's by design, not a flaw in the implementation (errata), but it is something the programmer needs to be aware of (caveat). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:54, 23 May 2014 (MDT)

## I don't understand this caveat
    
    
     In horizontal or 4-screen mirroring, writing $2000 at the exact start of horizontal blanking may cause the PPU to start reading from the first pattern table instead of the second.
     (Workaround: Don't disable NMI during active picture. Instead, use a variable to lock out reentrant NMI.)
    

  
Can someone rewrite it with a better wording so that non-native english speakers understand it ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 05:12, 4 November 2013 (MST) 

    Errata in the errata? Chalk it up to [Muphry's law](https://en.wikipedia.org/wiki/Muphry%27s_law "wikipedia:Muphry's law"). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 15:54, 4 November 2013 (MST)

    This was the SMB bug where on 1/3 power-ons, the PPU alignment allows the mid-screen write to $2000 to interfere with the nametable selection on the next line if perfectly (unfortunately) timed. It results in one line of the screen being rendered with the wrong nametable. What do you think would help the wording, Bregalad? [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 09:52, 5 November 2013 (MST) 

    Sorry to confuse... I already edit-ninja'd the thing he was confused by. Look at [my change](http://wiki.nesdev.org/w/index.php?title=Errata&diff=7703&oldid=7688) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:40, 5 November 2013 (MST)

OK now I think I understand the problem, but I don't see how the "workarround" proposed is related to the problem in any way - which leads me to think I might have misunderstanded the problem in the 1st place. 

So my understanding is that, writing to $2000, in any scanline, at a particular PPU clock cycle, will force the nametable bit to '0', even if D0 of the data written to $2000 is 1. Is my understanding correct ? In this case the workarround would be sure to make write to $2000 early enough so they happen before HBlank, right ? I don't see why this is related to NMIs in any way.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:05, 23 May 2014 (MDT) 

    It's related because disabling/enabling NMIs is in the same register as the upper nametable selection bits. Also, "just have to write to $2000 before hblank" isn't useful advice without some way to know when in the frame you are (whether timed code, irq, polling: all of which are more complicated than "use a semaphore to prevent reentry") —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 03:12, 23 May 2014 (MDT)

Ok so did I understand the caveat correctly ? I still see no point between it and NMIs, in fact with your post I noted that yes, the NMI enable bit is in the same register as upper scrolling bits, but if the user is writing to $2000 outside of VBlank for adjusting scrolling horizontally, and has control of the timing, so it's always possible to adjust the timing to prevent scrolling glitches caused by it. 

    The problem observed was using $2000 to disable NMI on entry to your NMI routine to prevent re-entry, and then using $2000 when the routine is finished to re-enable the NMI (timing for this is not consistent). If you don't know about this conflict, it seems like a valid solution to prevent re-entrant NMI. Since it was used in Super Mario Bros. it would be easy to assume this is a problem free technique, but it is not, as we have determined. The other place this conflict will come up would be for some sort of scanline split of a rendering state, which, yes, you should be able to easily time to avoid writing $2000 on the wrong cycle. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:11, 23 May 2014 (MDT)

Oh, OK, I got it now. Not that it was clearly worded in the 1st place...[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 14:41, 23 May 2014 (MDT) 

## Unofficial instructions

I just want to point out that more games rely on not simply connecting CIRAM/CE to PPU/A13 than using unofficial instructions, and there are plenty of clones that disable that control. Furthermore, if they use a non-NMOS implementation of the CPU, many of the unofficial opcodes would require extra effort to implement. 

On the other hand, the A53 cart contains two games that use the MUSE replay engine, which uses AXS and LAX (zz),y and I don't _think_ we've had anyone unable to run it as a result. — [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:18, 12 December 2013 (MST) 
