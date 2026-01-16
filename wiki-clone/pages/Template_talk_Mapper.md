# Template talk:Mapper

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template_talk%3AMapper) | View [other pages](Special_AllPages.xhtml#Template_talk_Mapper)

## Uses

Duplicating the information from the Mapper Table (the icons) to the relevant mapper pages seems like an admirable goal, but it also makes me wonder how to maintain a Single Version of the Truth, so that they cannot get out of sync. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 00:57, 25 October 2016 (MDT) 

    Out of sync with what? This template tries to include [INES Mapper XXX/Icon](https://www.nesdev.org/w/index.php?title=INES_Mapper_XXX/Icon&action=edit&redlink=1 "INES Mapper XXX/Icon \(page does not exist\)") page, and includes [![Mfr icon Missing.png](../wiki-images/Mfr_icon_Missing.png)](File_Mfr_icon_Missing_png.xhtml) as default when it doesn't exist. I propose moving icons from [Mapper](Mapper.xhtml "Mapper") to corresponding mapper subpages, then using this template everywhere. From what I got by googling, it might be easily done with [this](https://www.mediawiki.org/wiki/MediaWiki_Bulk_Page_Creator) tool --[PostNES](User_PostNES.xhtml "User:PostNES") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:PostNES&action=edit&redlink=1 "User talk:PostNES \(page does not exist\)")) 19:18, 26 October 2016 (MDT)

I'm not altogether certain what we gain from changing everything over to use the template. I don't think the icons would look good outside of the grid, so I'm not certain what's the point of moving the data out of the wikitable and into a large set of articles. 

e.g. if we take the copy from [195](INES_Mapper_195.xhtml "INES Mapper 195"): 

> [iNES Mapper 195](INES_Mapper_195.xhtml "INES Mapper 195"), like mappers [![Template Mapper Noicon.png](../wiki-images/Template_Mapper_Noicon.png)](INES_Mapper_074_Icon.xhtml "INES Mapper 074/Icon") [074](INES_Mapper_074.xhtml "INES Mapper 074"), [![Template Mapper Noicon.png](../wiki-images/Template_Mapper_Noicon.png)](INES_Mapper_119_Icon.xhtml "INES Mapper 119/Icon") [119](INES_Mapper_119.xhtml "INES Mapper 119"), [![Template Mapper Noicon.png](../wiki-images/Template_Mapper_Noicon.png)](INES_Mapper_191_Icon.xhtml "INES Mapper 191/Icon") [191](INES_Mapper_191.xhtml "INES Mapper 191"), [![Template Mapper Noicon.png](../wiki-images/Template_Mapper_Noicon.png)](INES_Mapper_192_Icon.xhtml "INES Mapper 192/Icon") [192](INES_Mapper_192.xhtml "INES Mapper 192"), and [![Template Mapper Noicon.png](../wiki-images/Template_Mapper_Noicon.png)](INES_Mapper_194_Icon.xhtml "INES Mapper 194/Icon") [194](INES_Mapper_194.xhtml "INES Mapper 194"), replaces some pages of CHR in this MMC3-like mapper with RAM. 

it just doesn't look good to me... even if those icons were the Waixing logo instead. 

Also also, is there any chance of hitting the transclusion limit if we do do this replacement? It'd be nice to know before any moves happen. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:48, 26 October 2016 (MDT) 

    Actually, I made this template in first place because I didn't like typing a lot of `[[INES Mapper XXX|XXX]]`-type links... then, icons looked like a nice feature --[PostNES](User_PostNES.xhtml "User:PostNES") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:PostNES&action=edit&redlink=1 "User talk:PostNES \(page does not exist\)")) 04:11, 27 October 2016 (MDT)

    I don't think the icons should be used outside of the mapper grid. In the middle of text they will be very obtrusive. I think the template without icons is nice to have, though. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:58, 28 October 2016 (MDT)

OK, I modified the template, the icon is now optional. Also added examples --[PostNES](User_PostNES.xhtml "User:PostNES") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:PostNES&action=edit&redlink=1 "User talk:PostNES \(page does not exist\)")) 05:29, 2 November 2016 (MDT) 
