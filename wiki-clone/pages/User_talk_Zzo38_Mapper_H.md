# User talk:Zzo38/Mapper H

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Mapper_H) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Mapper_H)

### Status

This can't be fully finished yet... the following things don't make sense: 

  * Why PPU A8…A11 == b'0000'? That's the first sixteen patterns in the both pattern tables and the top eight rows of the top-left-most nametable.
  * Why latch PPU A8 and A9 when PPU A8…A11 already == b'0000' ?
  * Why latch PPU A12 and A13? They're rather difficult to control in a useful fashion.



—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:15, 13 September 2014 (MDT) 

    That first one is a mistake; it should be only A10 and A11. Thank you for notifying me. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 20:32, 18 September 2014 (MDT)

  
That doesn't really help. Let's try a simple cycle-by-cycle description of what's going on. Assuming for the moment that scrollX=scrollY=0, and MODE=SCROLL=COLOR=0 at the very start. 

  * Fetch $2000, data=VV ; MODE←$80 ; COLOR←V&3
  * Fetch $23C0, data=COLOR*85 ; MODE←$BC
  * Fetch $0VV0, data=PP ; actual fetch comes from ROM at $16VV4 ; SCROLL←0 ; COLOR←P&3
  * Fetch $0VV8, data=QQ ; actual fetch comes from ROM at $16VVC ; SCROLL←0 ; COLOR←Q&3
  * Fetch $2001, data=WW ; MODE←$80 ; COLOR←W&3
  * Fetch $23C0, data=COLOR*85 ; MODE←$BC
  * Fetch $0WW0, data=RR ; actual fetch comes from ROM at $16WW4; SCROLL←0 ; COLOR←R&3



So that pretty clearly shows that since the attribute table fetches interfere with MODE, we can't usefully control MODE when PPU A13 = 1. The value will ALWAYS be between $BC and $BF, producing normal display for the top 64 scanlines, and weird aliasing for the following 176. Furthermore, there is thus NO way to get values from $80 through $BB or $C0 through $FF. 

What if we prevented attribute table fetches from updating MODE? It's both a little better and a bit worse. There's still no way to select values from $C0 through $FF, because the PPU normal fetch order never fetches from $3xxx. On the other hand, every scanline will flip between MODE&0xFE and MODE|1, left and right: fairly hard to compensate for. AND to no real benefit. 

So let's set scrollX=256 and scrollY=240, so that MODE is only ever updated during the pattern table fetches. 

  * For brevity's sake, ignore fetches from $2C00 and $2FC0, it only affects COLOR
  * Fetch $0VV0. If VV<64, then set MODE=VV ... Wait, why didn't we just latch VV on the nametable fetch in the first place ? Also, COLOR = VV&3 = which also = MODE&3\. If the user flipped between left and right pattern tables, flipping PPU A12, flipping SCROLL D6, that effectively ties CHR A12 to CHR A15: the only way to fetch tiles where CHR A12 != CHR A15 is by raster effects, changing the tables mid-screen, after MODE is set.



This is entirely ignoring the dummy nametable fetches during sprite fetch. Those will clobber MODE, although it should usually be 0. It does keep the user from using a quarter of the sprite tables, because the values fetched will cause weird internal scrolling. 

  
Here's something that will provides the same meat, but is far more predictable: 

  * On nametable (but not attribute table) fetches, latch the two LSB of the byte fetched. That's COLOR.
  * On nametable (but not attribute table) fetches, IF the MSB is CLEAR, latch the remaining five bits. That's MODE.
  * CHR-ROM is not connected to PPU A11. i.e. both $45 and $C5 fetch the same tile: the former just has a side effect.
  * The four LSB of MODE and SCROLL have the same effect as before.
  * The MSB and final bit of MODE selects between 8 KiB banks of CHR. There just aren't enough variables to provide any more.



—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:58, 19 September 2014 (MDT) 

    Thank you, although my intention was that you would not use the $2000-$23FF nametable at all, and that it would include tiles to switch the mode, at the beginning of each row and possibly also later in the row. Thank you for writing all of this stuff anyways though (maybe it can still help)! --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 20:28, 20 September 2014 (MDT)

    Perhaps I made mistake, your way may be better. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:00, 26 February 2017 (MST)
