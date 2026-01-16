# Talk:PPU palettes

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APPU_palettes) | View [other pages](Special_AllPages.xhtml#Talk_PPU_palettes)

It says that the background palette hack can be used to display the unused palette data. Can the EXT pins also do this? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:30, 1 July 2013 (MDT) 

    Yes, I think so. While rendering background pixels, the palette index that gets looked up is simply 0<exp_in3><exp_in2><exp_in1><exp_in0>. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 03:58, 2 July 2013 (MDT)
    Interestingly, it looks like the background palette hack wouldn't work for _output_ to the EXT pins though (bit 6 of $2000 set). The logic that causes the VRAM address to be used for the palette index while the background palette hack is active comes after the logic that outputs the index to the EXT pins. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 04:15, 2 July 2013 (MDT) 

    Thank you for information. I suppose if you are using the master/slave, you can connect the EXT pins in reverse (EXT0 to EXT3, EXT1 to EXT2, EXT2 to EXT1, EXT3 to EXT0) so that this can be used to make extra colors. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 23:05, 2 July 2013 (MDT)

## Contents

  * 1 Source of palette
  * 2 Parsing the palette
  * 3 Browser compatibility of Drag's palette generator
  * 4 Palette color format?
  * 5 Platte display
  * 6 Move the palette chart to the top
  * 7 "classic EGA palette"
  * 8 Background Palette Hack Intentionality?
  * 9 RC2C03B possible bad dump?
    * 9.1 RC2C03B



## Source of palette

Where did the palette added by Lidnariq in [this edit](http://wiki.nesdev.org/w/index.php?title=PPU_palettes&diff=7888&oldid=7887#RGB_palette_values) come from? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:56, 25 December 2013 (MST) 

    Screenshot from Nestopia running blargg's full_colors and nes_ntsc. Feel free to bikeshed. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:00, 25 December 2013 (MST)

## Parsing the palette

The palette values from this table are read from left to right, then up -> down. Is this ok? 

00 01 02 03 ... 0F  
10 11 12 13 ... 1F  
20 ...  
([unsigned](https://en.wikipedia.org/wiki/wikipedia:Signatures "wikipedia:wikipedia:Signatures") comment by [Zepper](User_Zepper.xhtml "User:Zepper"))

    Almost. They appear to go from $00 to $0D, then the next line from $10 to $1D, etc. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 06:42, 28 December 2013 (MST)

## Browser compatibility of Drag's palette generator

It appears that Firefox 29 implements built-in validation for `<input type="number">` such that said elements are expected by default to contain integers. This affects the default hue and saturation settings, causing the boxes to be given glowing red borders and explanatory tooltips, and preventing the values from being changed to other reasonable non-integers. The default settings still _work_ (in that they produce the same palette as in older versions of Firefox) as long as they are not changed. 

Relevant documentation can be found on various Mozilla websites (not bothering to link any of it here due to lack of login). --~~150.135.211.242~~ 22:30, 18 May 2014 (MDT) 

    Maybe send this information to Drag? I don't think it does much good here. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:00, 20 May 2014 (MDT) 

    How does Drag prefer to be contacted? I am not active on the forums (and therefore don't qualify for an account on the wiki either). The homepage of the domain on which the generator is hosted mentions that Drag has a Twitter account, but I'm not a registered Twitter user either (and I don't feel like registering just for this one issue).
    I don't see how posting here is useless given that some other user could make a fixed version. For example, Tepples has some webspace which could theoretically be used for this. --~~150.135.211.242~~ 14:46, 20 May 2014 (MDT) 

    You used to need a reputation on the BBS or [IRC](NESdev_IRC_channel.xhtml "IRC") to qualify for a wiki account. You don't anymore; that ended 20 months ago with the transition away from Parodius. Feel free to register and jump right into the talk pages, and so long as your posts there show clue, you'll get [autoconfirmed](Nesdev_wiki_Users.xhtml "Nesdev wiki:Users") in no time. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:35, 20 May 2014 (MDT)
    I'll take this into account if I decide to poke at my palette generator again. :P Thanks for letting me know about it though. --[Drag](User_Drag.xhtml "User:Drag") ([talk](User_talk_Drag.xhtml "User talk:Drag")) 17:27, 26 May 2014 (MDT)
    The number fields should now be fixed, I hope. I just needed to add a "step" property to the input field to specify that I expect decimals to 0.001 precision. --[Drag](User_Drag.xhtml "User:Drag") ([talk](User_talk_Drag.xhtml "User talk:Drag")) 01:09, 28 May 2014 (MDT)

← 

One more problem with the validation: Press the "Get current preset" button with everything else left at defaults. The Wx value of 0.3127 gets flagged as invalid because it has one more decimal place than the current validation allows. It's probably a good idea to allow four decimal places in all the custom colorimetry settings, based on the conventions I've seen. (I suppose I'm fine downloading a copy to run locally if you don't want to bother) 

(Aside: My main reason not for registering at this point is lack of HTTPS support. Even if I were to ignore that, I'm a little confused why some accounts have needed to be confirmed manually recently.) --~~150.135.211.242~~ 14:32, 21 April 2015 (MDT) 

## Palette color format?

What's the color format used in the palette entries? Example: 755,637,700,447... ??? How do I "decode" the 755 value in RGB? --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 13:24, 29 October 2014 (MDT) 

    Those are RGB333 values, so 637 means 6/7 red (0xDB), 3/7 green (0x6D), and 7/7 blue (0xFF). --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 20:36, 29 October 2014 (MDT) 

    The .PAL format should consist of the same color formatting as the [Classic VGA Palette format](http://www.shikadi.net/moddingwiki/VGA_Palette) so if you know how to program back then, you already know this. --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 05:55, 31 October 2014 (MDT)

Why does this page show strange, nonstandard palettes first ? 

## Platte display

One version of the canonical, standard NES palette should appear at the top of the page. Other palettes used by RGB, playchoice or whathever are annecdotical and should only appear below. They should most certainly *not* appear first. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 14:29, 6 January 2016 (MST) 

## Move the palette chart to the top

I don't know about you guys, but most of the time I visit this page is because I want to see a chart which maps NES colors to their corresponding numbers. Strangely, the only chart on the page for this is found at the very bottom! The charts that come before it are for the 2C03, 2C04, 2C05 PPUs, not the 2C02 PPU which is what most people would expect. 

Suggested change: Move [savtool's NES palette](http://wiki.nesdev.org/w/index.php/File:Savtool-swatches.png) to the top of the page, above any mention of the 2C03. 

[Pubby](https://www.nesdev.org/w/index.php?title=User:Pubby&action=edit&redlink=1 "User:Pubby \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Pubby&action=edit&redlink=1 "User talk:Pubby \(page does not exist\)")) 11:20, 11 May 2016 (MDT) 

    Good point. Done. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 13:10, 11 May 2016 (MDT)

## "classic EGA palette"

"These NES colors approximate colors in the classic Windows or EGA palette, though the NES doesn't really have a good yellow" Is this sentence supposed to refer to the chromas prior to or those following it? [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 21:01, 10 January 2017 (MST) 

    The values before the statement you quoted aren't specific colors. They're values like $x2 and $x4 which represent entire "columns" of colors. For example, $x2 = $02, $12, $22, and $32. $x4 = $04, $14, $24, and $34. Also, the statement you quoted ends with a colon, indicating it refers to the values that follow it. --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 19:08, 11 January 2017 (MST)
    It refers to those that follow. I added a section break between the chroma list and the EGA list to clarify. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 12:36, 13 January 2017 (MST)

## Background Palette Hack Intentionality?

I've looked over the palette logic in Visual 2C02, and I don't see any signs that the "background palette hack" was intentional - whenever the VRAM address is within $3F00-$3FFF, the Palette RAM's address input is connected to A0-A4 so that writes to $2007 will go into the appropriate memory cells, but there's no logic to _disable_ the palette's output signals so they still end up driving the luma/chroma generators during the visible portion of the frame. To me, that looks very much like it's a side effect of how rendering works. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 18:14, 2 July 2022 (UTC) 

## RC2C03B possible bad dump?

There seems to be only one known instance of this, and there are suspicions that it is is simply a bad dump. Moved here to talk page waiting for someone to improve the evidence for it. Has another of these ever been found? Is the original still accessible? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:09, 7 April 2023 (UTC) 

### RC2C03B

For no particularly discernible reason, the palette for this specific PPU varies in six colors: 
    
    
    333,014,006,326,403,503,510,420,320,100,031,040,022,000,000,000
    555,016,027,407,507,704,700,630,430,140,040,053,044,000,000,000
    777,357,447,637,707,717,740,750,660,340,070,276,077,000,000,000
    777,547,657,757,747,755,764,772,773,552,473,276,467,000,000,000
    

00  | 01  | 02  | 03  | 04  | 05  | 06  | 07  | 08  | 09  | 0A  | 0B  | 0C  | 0D  | 0E  | 0F   
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
10  | 11  | 12  | 13  | 14  | 15  | 16  | 17  | 18  | 19  | 1A  | 1B  | 1C  | 1D  | 1E  | 1F   
20  | 21  | 22  | 23  | 24  | 25  | 26  | 27  | 28  | 29  | 2A  | 2B  | 2C  | 2D  | 2E  | 2F   
30  | 31  | 32  | 33  | 34  | 35  | 36  | 37  | 38  | 39  | 3A  | 3B  | 3C  | 3D  | 3E  | 3F   
  
The pattern compared to the normal 2C03 is that where color bits 0-1 are 01 (i.e. palette entries $01/$11/$21/$31, $05/$15/$25/$35, and $09/$19/$29/$39), green is ANDed with 5 (bit 1 is 0). No games are known to be designed for this palette. Given the pattern, it was probably a mistake in the mask. 

Later anecdotes imply the specific PPU used to generate the above dump might have been damaged. [[1]](https://forums.nesdev.org/viewtopic.php?p=283412#p283412)
