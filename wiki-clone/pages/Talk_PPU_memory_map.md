# Talk:PPU memory map

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APPU_memory_map) | View [other pages](Special_AllPages.xhtml#Talk_PPU_memory_map)

\- I believe that _memory range_ illustrates better than _offset / size_. Example: $3F00-$3F0F - background palette; $3F10-$3F1F - sprites palette. 

\- Additionally, it shouldn't use the term _palette_ , since it's _not_ palette, but [indexes to the palette signals](https://www.nesdev.org/w/index.php?title=Indexes_to_the_palette_signals&action=edit&redlink=1 "Indexes to the palette signals \(page does not exist\)"), since there's no real RGB palette.--[Zepper](User_Zepper.xhtml "User:Zepper") 00:59, 30 March 2010 (UTC) 

  


* * *

  
Like mentioned at the bottom of the page, this is a place holder for the PPU memory map since there was none in the first place. It's based on a similar structure to the CPU one and from nestech document as a source. It will require improvement. [Banshaku](User_Banshaku.xhtml "User:Banshaku") 03:09, 30 March 2010 (UTC) 

  


* * *

  
\- Unfortunately, I can't go ahead and create a custom PPU memory table of my wishes... ^_^;; Perhaps tepples can?--[Zepper](User_Zepper.xhtml "User:Zepper") 21:23, 30 March 2010 (UTC) 

  


* * *

  
What do you mean? Anybody has the right to update the wiki. Unless you mean you don't know how to use wiki table format? I don't really know either, just used the format for the CPU one. I would still prefer ascii art actually. [Banshaku](User_Banshaku.xhtml "User:Banshaku") 22:58, 30 March 2010 (UTC) 

* * *

There is a set of CSS styles to make tables look prettier. But it's not installed by default in installations of MediaWiki, and only a user in the 'sysop' group can edit the site CSS to install it. I was a sysop on Atarimike's nesdevwiki.org, but the group was never reassigned upon the server move to Parodius. What is the procedure to request rights on this wiki? --[Tepples](User_Tepples.xhtml "User:Tepples") 13:58, 31 March 2010 (UTC) 

As far as I can gather from what I've been told on the forum, [User:Ndwiki](User_Ndwiki.xhtml "User:Ndwiki") is a [role account](https://en.wikipedia.org/wiki/meta:role_account "wikipedia:meta:role account"), and I'm supposed to e-mail Parodius staff to get the password. --[Tepples](User_Tepples.xhtml "User:Tepples") 01:52, 1 April 2010 (UTC) 

  


* * *

Well, I did the changes. :) Anyway, here's something to discuss: it says "Either CHR ROM or CHR RAM depending on the board". For my best, the PPU memory 0000-1FFF is **CHR RAM** : the hardware chip is RAM, not ROM. The CHR ROM data is always in the cartridge. In that case, CHR ROM _appears_ there. Could someone clarify if it's copied, mirrored or whatever..? --[Zepper](User_Zepper.xhtml "User:Zepper") 13:46, 3 April 2010 (UTC) 

* * *

Are the nametable mirrors at $3xxx ever accessed by the internal logic of the PPU during rendering? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 23:11, 8 June 2013 (MDT) 
