# Talk:PPU attribute tables

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APPU_attribute_tables) | View [other pages](Special_AllPages.xhtml#Talk_PPU_attribute_tables)

\- Even if the NES does not display a picture of 256x256 pixels, it's technically possible to do. I studied the Mega Man II gfx engine a few years ago, and my level editor renders a full picture of 256x256 pixels. In other words, even if the NES cuts off 2xF8 by half, it's still technically possible to render it. 

  


## Attribute Equation Order

It may be a little better to have this: 

value = (topleft << 0) | (topright << 2) | (bottomleft << 4) | (bottomright << 6) 

ordered like this: 

value = (bottomright << 6) | (bottomleft << 4) | (topright << 2) | (topleft << 0) 

  
Obviously the result is the same, but the second one keeps the order they'd be in the byte after the actual bit shifts. Bottomright would be in the far left of the byte, etc. It makes it easier to quickly glance and see the order. Just a thought, since I don't know if the current way is the standard way of representing bit shift equations or something. \--[Kasumi](https://www.nesdev.org/w/index.php?title=User:Kasumi&action=edit&redlink=1 "User:Kasumi \(page does not exist\)") 02:24, 27 September 2011 (UTC) 

## Can we get a screenshot?

I think it'd help if we had a picture depicting how the attribute table works, using basically a screenshot of the NES nametable (i.e. a screenshot) and then a grid overlayed/splitting up the sections and depicting what attribute table bit/entry correlates with what part of the palette, and the RAM offsets on X/Y axis as well. I don't think ASCII works well for depicting this clearly. I actually find the way i described it in [nestech.txt](Nestech_txt.xhtml "Nestech.txt"), with words (I think I described it 4 different ways), to be more useful. --[Koitsu](User_Koitsu.xhtml "User:Koitsu") in #nesdev 

    I might use a screenshot of [Thwaite](User_Tepples_Thwaite.xhtml "User:Tepples/Thwaite") for this. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:21, 21 March 2015 (MDT)
