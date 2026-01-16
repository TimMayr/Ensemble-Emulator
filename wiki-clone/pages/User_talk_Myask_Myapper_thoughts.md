# User talk:Myask/Myapper thoughts

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AMyask/Myapper_thoughts) | View [other pages](Special_AllPages.xhtml#User_talk_Myask_Myapper_thoughts)

## Textbox Bank

I'm confused by what you mean when you say "might need to use sprites to mark the corners" ... I assumed you mean visually, except that "otherwise you need extra viable start- and end-tile designations" is incongruous? After all, the NES fetches 34 background tiles, then 8 sprite tiles, and the address bus doesn't give any hint as to the sprite X coordinate, and so I don't see how using using sprites to mark corners helps. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:54, 14 October 2016 (MDT) 

    Just visually, yes. Supposing you had simple rectangular[ish] boxes like in Dragon Warrior, you'd need top corner, side edge, bottom corner for start and stop tiles; one **could** allow there to be three different tiles each of "select bank X" to allow for this (in the mapper design; as this hasn't passed the stage where the mapper is incorrigible). I think it's a bad idea, but one could do it. Alternately, if one leaves the top and bottom edges in a/the not-inside-box bank, so you don't have to switch those lines in, then you don't get the 'free' breaking in half of the 16-pixel attribute, be it for opening/closing of the box or just so you don't have to align on those boundaries for top/bottom.
    I'd think to go for four to eight options, with one being "what the rest of the mapper thinks".
    Though, now you mention it, the sprite fetches could trigger it, too, [just at scanline bounds] which leads to the handy fallout that is fire-and-forget, no-interrupt, no-spinning on sprite0, no cycle timing scanline-triggered banking. (I suppose you still have to set scroll registers for, say, a status bar, so it's not a miracle.)
    Of course, in the Omniscient Mapper case where we've spied on and saved a copy of OAMDMA to cartside, you could use that sort of thing, but you might as well just allow arbitrary X/Y-specified banking then, and take out the sprite middleman. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 19:29, 14 October 2016 (MDT)
