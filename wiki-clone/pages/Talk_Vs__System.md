# Talk:Vs. System

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AVs._System) | View [other pages](Special_AllPages.xhtml#Talk_Vs__System)

Is there a way to detect if the game is running on Vs. System or not (which also works in emulators)? In case I wanted to make up the game which can work coin-operated arcade game but also works without it if you put it in a ordinary cartridge? --Zzo38 

    You can always make a separate version for Vs. boards. It's not like you can plug a Famicom cartridge into a Vs. board or anything. But unless you're using Duck Hunt or Tennis, one problem with Vs. is that the colors are all scrambled, making it much harder to do decent fades. --[Tepples](User_Tepples.xhtml "User:Tepples") 16:17, 7 October 2012 (MDT)

    The page mentions that certain revisions of the PPU (RC2C05*) return a magic number in the lower bits of $2002 read. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 16:47, 7 October 2012 (MDT) 

    But then those are the ones that swap $2000 and $2001, so you pretty much have to make special builds for the 2C05. --[Tepples](User_Tepples.xhtml "User:Tepples") 18:11, 7 October 2012 (MDT) 

    If they swap $2000 and $2001 then you might be able to detect that using sprite 0 hit, somehow. Switches might also be used to configure the palettes in case 2C04 is used. Does 2C04 have any differences from 2C03 other than the colors? I know you cannot put a Famicom cartridge in the Vs. boards, but you could replace the chips manually, or make a clone hardware with the same purpose of $4016,$4017,$4020 registers (I don't know if it might be possible to build such a clone hardware into a cartridge, controller ports, and expansions ports, to wire to the coin slots and so on?). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 18:47, 7 October 2012 (MDT) 

    If you're replacing chips manually, and you know what kind of board you're replacing them into, why not just use a build specifically made for one Vs. board? I don't see the advantage of having one ROM image that runs on several fundamentally incompatible system boards. --[Tepples](User_Tepples.xhtml "User:Tepples") 18:59, 7 October 2012 (MDT) 

    Well I suppose yes it could be made separate ROM images, one for not Vs. System, and other for the Vs. System boards. (Using conditional compiling, perhaps?). I still was interested to know if there is ways to detect it, though. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 19:39, 7 October 2012 (MDT)

    

    

    

    

    Between the part where you'll need 5 additional builds (4 palette scrambles, 1 $2000/$2001 swap) to accommodate all the different Vs hardware, and only have the ability to detect the last, it's not clear to me how to feasible autodetection (as opposed to different builds, DIP switch settings, asking the user, &c)—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 10:36, 8 October 2012 (MDT)

    

    

    

    

    I suppose best would be to simply have one normal ROM image and one arcade ROM image (using macros and conditional compilation), and to use either the switches (three would be needed, unless there is a way to distinguish 2C03 from 2C04 in general, in which case two is sufficient) or a service menu to select the palette? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 15:34, 28 April 2013 (MDT)

## Coin acknowledgement

I finally noticed what seems to be the official Vs System schematic­—<http://nesdev.org/VSSCHEM.pdf> —and noticed that the coin acknowledgement seems to actually only be NAND3($4000-$5FFF,A5,M2) (gate 4D, coordinates E5 or J5). Which means that the register 1- should react to reads as well as writes, and 2- is duplicated all over the place. I don't suppose we can get any kind of verification; should I just update the document? 

Also, there's something funny involving $6000-$7FFF, OUT1, and IRQs in this schematic. It looks like the two CPUs have 2KiB of RAM that can be shared depending on the primary CPU's OUT1 pin. I guess Nocash already figured that out, though: <http://problemkaputt.de/everynes.htm#vssystemcontrollers> —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:20, 18 April 2013 (MDT) 

## Service button

If anyone makes a new game using Vs. System, then instead of simply adding a credit, it can be used when held down, displaying a diagnostics menu (containing such things as number of games played since power on, idle time, palette test, $2002 readout, high score, last score, last game duration), and if other buttons are pushed while it is active, to do other things: 

  * A and B = Change the colors. This can be used if the game is running on a different 2C04 variant than normal (altenatively, three switches can be used, especially if there is no save game file, then using the switches would be better). Sound test could also be done using these buttons.
  * START = Add a credit.
  * SELECT = Discard all credits.
  * Up/Down/Left/Right = Edit the high score.



This is just a suggestion and you can make the game do whatever you want with this; it doesn't have to be the above. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 15:34, 28 April 2013 (MDT) 

## Moniker

The kit manual (see link the Wiki page) refers to the device as the VS. UniSystem, not the Vs. Unisystem. Should we fix the character casing on the Wiki page? 
