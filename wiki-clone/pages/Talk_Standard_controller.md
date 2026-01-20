# Talk:Standard controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AStandard_controller) | View [other pages](Special_AllPages.xhtml#Talk_Standard_controller)

## Contents

  * 1 Microphone
  * 2 SNES-in-NES reads
  * 3 Bit deletion detection?
  * 4 Open bus
  * 5 Famicom 2nd controller missing Select and Start
  * 6 Bit deletion in other input devices
  * 7 Distinguishing hardware variant
  * 8 A controller MUST NOT toggle the button states on each strobe pulse; that'll break any game that rereads the controller.
  * 9 74165 compatibility
  * 10 Reorganization needed with Controller Reading
  * 11 Report - numbered bits



## Microphone

How to do microphone? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 19:18, 16 September 2012 (MDT) 

    Nevermind I found it on the other page it says $4016 bit2 is microphone. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 19:19, 16 September 2012 (MDT)

## SNES-in-NES reads

It says "all subsequent reads will return D=1 on an authentic controller but may return D=0 on third party controllers", and what happen on Super Nintendo controllers? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 17:50, 27 September 2012 (MDT) 

    After the first eight bits, the Super NES controller returns A, X, L, R, four zero bits, then all 1. --~~98.226.71.46~~ 18:22, 27 September 2012 (MDT)

## Bit deletion detection?

Official controllers have the correct 8 bits, then all 1s. Couldn't you detect the absence of a bit deletion by seeing if the 8th bit is 0 and the 9th bit is 1? This only works while right is not pressed, but ... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:12, 8 February 2013 (MST) 

    Reading 9 bits to skip rereading if Right is not held could work provided that you're sure that the user isn't using a [Four Score](Four_player_adapters.xhtml "Four Score"), an official Super NES controller, or an unlicensed controller that has all 0s after its report. The Four Score and Super NES controller have longer reports (24-bit and 16-bit respectively), but these reports likewise end with a transition from 0 to 1. In any case, in games that don't use players 3 and 4, it'd be quicker just to re-read than to look for the Four Score signature. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 13:34, 8 February 2013 (MST)

## Open bus

Lidnariq: Some more details on the difference between 'x' and '0' in the $4016/$4017 diagrams would be nice, and on what M/F read as on NES. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 01:22, 19 March 2013 (MDT) 

    Done, but my presentation is lousy. I'm not certain if it's even possible to reunify the diagrams anymore.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 02:05, 19 March 2013 (MDT) 

    Much clearer now at least - thanks! -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 02:13, 19 March 2013 (MDT)

## Famicom 2nd controller missing Select and Start

The article says 

> Button status for each controller is returned as an 8-bit report in the following order: A, B, Select, Start, Up, Down, Left, Right. The Select and Start buttons are completely missing on the second controller of the plain Famicom.

So reading $4017 bit 0 on a Famicom returns which one of the following?: 

  * a 6-bit report: A, B, Up, Down, Left, Right
  * an 8-bit report: A, B, 0, 0, Up, Down, Left, Right
  * an 8-bit report: A, B, 1, 1, Up, Down, Left, Right
  * something else?



\--[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 17:23, 19 March 2013 (MDT) 

    The 2nd of those three. I've updated the article to state this explicitly.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:58, 19 March 2013 (MDT)

## Bit deletion in other input devices

Does bit deletion still do in controllers that don't use the shift registers (such as the light gun and keyboard)? From the instructions, it seems to me it is safe to use DPCM if using the light gun or keyboard, but I am unable to test this, and the article should probably be made clear. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:35, 25 August 2013 (MDT) 

    The article does say that the CLK line is what causes bit deletions; devices that don't use it (such as the [zapper](Zapper.xhtml "Zapper"), [keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard"), [data recorder](Family_BASIC_Data_Recorder.xhtml "Family BASIC Data Recorder"), [Oeka Kids tablet](Oeka_Kids_tablet.xhtml "Oeka Kids tablet"), &c) won't be affected. I'm uncertain how to edit this to make that more explicit. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:53, 25 August 2013 (MDT)

## Distinguishing hardware variant

It says bit3 and bit4 of $4016 is "Open bus on traditional Famicom, all 0s on AV Famicom". Does this mean that a code like this might be able to distinguish the traditional Famicom from other hardware? 
    
    
       LDX #$17
       LDA $3FFF,X
       AND #$18
    

I don't know exactly know how open bus works though, but maybe someone is able to test this. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:54, 7 January 2014 (MST) 

    Yes, that is what it means. Well, not exactly that, because the page wrapping of the LDA abs,X should cause a read from $3F16 before the useful read, which will pre-seed open bus to whatever the PPU's internal open bus is. So I think something like
    
    
       LDX #$FF
       STX $2002
       LDA $3F17,X
       AND #$18
    

    should DTRT.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:33, 7 January 2014 (MST) 

    OK, thanks. I forgot about that. (I don't know what "DTRT" means, but I think I can mainly understand your message nevertheless.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 18:07, 7 January 2014 (MST) 

    Google says "Do the right thing". --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:36, 8 January 2014 (MST)

## A controller MUST NOT toggle the button states on each strobe pulse; that'll break any game that rereads the controller.

Is this wiki targeted at controller manufacturers? Or is it for emulators that simulate turbo controllers? In all cases this sentence comes out weird in this context, especially with the strong emphasis on the all caps 'must not'. - [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 07:37, 26 May 2015 

    I agree, and have revised it. "MUST NOT" seems to imply enforcement of policy, of which there is none here. NesDev users can do what they like, the Wiki's job is just to keep them aware of the problems. I added an explanation of exactly why it can cause a problem, which is probably a lot more helpful than telling people IT IS FORBIDDEN. It's probably a perfectly sensible approach if you're building a turbo controller circuit for specific games. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:37, 26 May 2015 (MDT)

    It's for controller manufacturers. I have seen hobbyist controller manufacturers in both NESdev BBS and #nesdev on EFnet. As used in documents that cite RFC 2119, these strong words are "used where it is actually required for interoperation", and a homemade controller that toggles buttons every strobe will not interoperate with games that misbehave when the controller toggles buttons every strobe. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:57, 26 May 2015 (MDT)

## 74165 compatibility

<http://www.raphnet.net/electronique/arcade_control/arcade_control_en.php> "74hc165 wont work because they dont appreciate the clock being high when the latch is low."   
  
I'd have to look closely at datasheets and test actual hardware to be sure, but it seems that the HC/HCT versions don't work for NES. Anyone actually tested this with custom hardware? If it indeed works on the 74HC(T)165 variants, we can revert this edit and put a note on compatibility. [alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 01:28, 15 August 2015 (MDT)  
  
I have tried using a HC165 as a controller for a NES clone (an old one not based in a NOAC) and it worked good for most of the games. However, in some ones like Pac Man it seems to malfunction. The load signal needs to be inverted, for example using a transistor.   
  
As a curiosity, HP inkjet multifunction printers like PSC1215 and many other models with the same shape use a button pad with a HC165 wired like a NES controller. From the track face of the PCB, with the connector at top, and from left to right, this is the pinout to use it with a NES:   
  
LATCH(LOAD) NC GND CLK VCC DATA   
  
Just remember to add the inverter!   
  
<http://privatewww.essex.ac.uk/~nbb/snes-technical.html> Seems the 74HC works fine? "I have used two 74HC165 Shift registers to make a joypad adaptation, although I had to buy a cheap joypad to get the connector. " [alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 02:13, 15 August 2015 (MDT) 

## Reorganization needed with [Controller Reading](Controller_reading.xhtml "Controller Reading")

We have this article, [Standard controller](Standard_controller.xhtml "Standard controller") and also [Controller Reading](Controller_reading.xhtml "Controller Reading"). 

This article talks about too many different types of controllers. It's mostly about the NES' internal controller interface, and only a little bit about the actual NES controller. This needs to be separated. 

Similarly, [Controller Reading](Controller_reading.xhtml "Controller Reading") (which should really be [Controller reading](Controller_reading.xhtml "Controller reading") for consistent naming convention) is more general than being about the NES controller, and really is mostly about code techniques that apply to using the NES' internal interface, and only a little bit about the specific controller. 

I think between these two articles, we should have instead 2 or maybe 3 different articles: 

  1. [Standard controller](Standard_controller.xhtml "Standard controller") \- should only be about the NES controller, and should really just be about the 8 bits it reports, a mention that it will normally spit out 1s after the 8 bits, and maybe some info about where it can be connected (e.g. Four Score, expansion, etc.)
  2. [Controller reading](Controller_reading.xhtml "Controller reading") \- should be about the NES' internal interface and how it applies to all controllers. DPCM deletion is part of this etc. It should not really require code examples they should go into...
  3. [Controller reading code](Controller_reading_code.xhtml "Controller reading code") \- or something suitably named, so we can keep all these code dumps separate from the reference information about the interface.



\-- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 21:23, 5 January 2019 (MST) 

    Well, I did that change. 

  * [Controller reading](Controller_reading.xhtml "Controller reading") \- Generic controller reading issues
  * [Standard controller](Standard_controller.xhtml "Standard controller") \- Only the standard controller is described here, not the generic stuff so much
  * [Controller reading code](Controller_reading_code.xhtml "Controller reading code") \- Code examples go here.


    \-- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:45, 20 April 2019 (MDT)

## Report - numbered bits

The Report section says 

> The standard NES controller will report 8 bits on its data line: 
>     
>     
>     0 - A
>     1 - B
>     2 - Select
>     3 - Start
>     4 - Up
>     5 - Down
>     6 - Left
>     7 - Right

The numbers in this list look like bit numbers, but it's really just telling you the order the bits come in. The exact bit numbers they end up at will depend on which way the code rotates or shifts the bits in. For example, in the Discord server #learning-assistance channel, someone was using values of $10, $20, $40, $80 for the D-pad, but should have been using $08, $04, $02, $01. 

I initially made an edit that removed those numbers 0 to 7. But Fiskbit pointed out that pages for other serial controllers, such as [TV-NET controller](TV_NET_controller.xhtml#Protocol "TV-NET controller"), use a similar format, so I undid my change...--[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 03:28, 13 April 2025 (UTC) 
