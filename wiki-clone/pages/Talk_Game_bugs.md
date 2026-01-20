# Talk:Game bugs

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AGame_bugs) | View [other pages](Special_AllPages.xhtml#Talk_Game_bugs)

## Contents

  * 1 "Impossible" inputs on emulators
  * 2 Predator
  * 3 Recent changes by PGCX864
    * 3.1 Ghosts 'n Goblins
    * 3.2 Kirby's Adventure
  * 4 smooth sprite flickering in smb3 world 1-4
  * 5 Reliance on RAM values
  * 6 Commando
  * 7 Metroid bugs
  * 8 Final Fantasy
  * 9 Chip 'n Dale Rescue Rangers



## "Impossible" inputs on emulators

Actually, there should probably be an option in the emulator to enable "impossible" input if it is wanted (such as to figure out what strange things are happening, or if you want to pretend to be different kind of input devices where it is possible, etc). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 18:58, 25 March 2013 (MDT) 

    Yeah, impossible controller input can be pretty fun, plus TASers love it. I've rephrased it a bit :). -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 22:41, 25 March 2013 (MDT) 

    Lots of emulators have an option to allow L+R/U+D but it really needs to be prevented by default. Considering that preventing it from happening requires more code than not preventing it (i.e. doing nothing), I think it the phrasing should be more like: "Emulators should prevent this kind of input by default, but may offer an option to turn off this protection for users that want to experiment." \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 09:02, 27 March 2013 (MDT) 

    Done! Are you unable to edit the wiki yourself by the way? I don't guard the stuff I write religiously, and this is pretty sensible stuff with low risk of being controversial. :) --[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 09:08, 27 March 2013 (MDT) 

    I can edit it, I just didn't feel like it for some reason I don't recall. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:36, 28 March 2013 (MDT)

## Predator

If I'm not wrong, the game Predator has a secret based on impossible inputs. Just press UP + DOWN + JUMP or FIRE to skip the current level. --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 18:25, 27 March 2013 (MDT) 

    That could just as easily be a bug. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:36, 28 March 2013 (MDT)

    

    I just had a look at the code and there is an explicit check for L+R (I assume there's a check for U+D before big mode levels as well). The game merges reads from the controller and the expansion port, so this is definitely intentional. --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 12:33, 21 August 2023 (UTC)

## Recent changes by PGCX864

I don't understand the bugs description added recently by PGCX864. They are badly phrased and are not understandable. A more precise description of what looks wrong (and possibly where in the game) is just what is needed.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:06, 29 April 2015 (MDT) 

And then he does that again. Most of his stuff is incorrect English and is total nonsense.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:48, 12 May 2015 (MDT) 

    Assume good faith, and _ayez de la patience, s'il vous plait_. English does not seem to be his first language. (It has an East Asian ring to it.) Of the ones I can puzzle out and check/knew about before, all of them exist. The Megaman ladder trick he refers to is [| this](http://tasvideos.org/GameResources/NES/Rockman.html#GrabbingTheTopOfTheLadderTooHighMm1Mm2), I believe. _Zelda 2_ having the simultaneous left+right input granting enormous speed is used well in one of the TASes, and likewise in _Tiny Toon Adventures_. The screen does indeed shake one pixel when a door closes in _Castlevania III/Akumajou Densetsu_ , but I believe this to be an intentional effect from the door slamming shut. I'll continue to correct what I can. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 03:47, 12 May 2015 (MDT)

Thanks. As for the Akumajo Densetsu door, I belive it's not intentional, because 

1\. It also shift one pixel when you first touch the door, before it is slamming 2\. Castlevania III does not have this bug. If it wasn't a bug it wouldn't have been fixed. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:42, 12 May 2015 (MDT) 

    It is as you say. I suppose if they'd wanted a serious shake effect, it might be for more than once (compare: shaking after each move of the tower in block 9-03, but obviously the tower moving makes for more shaking than a door would.) Or perhaps they just decided it wasn't nice to look at and cut it out. Though, they were changing mappers, and there were a fair number of other, less-desirable changes made. (Also, as an aside, I checked _Panic Restaurant'_ s bug myself.) [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 04:56, 12 May 2015 (MDT)

### Ghosts 'n Goblins

I'm struggling to understand what was meant by "Sometimes it will hit the pixels that are invisible.". Something got lost in translation... --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 07:05, 22 August 2023 (UTC) 

### Kirby's Adventure

"When Kirby copies a new ability, the status bar icon may flicker or display incorrect attributes"

This appears to be a rephrase of what PGCX864 wrote. Has anyone encountered this on real hardware? --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 06:41, 27 August 2023 (UTC) 

## smooth sprite flickering in smb3 world 1-4

World 1-4 in Super Mario Bros 3, the moving wooden lifts flicker smoothy \--~~46.130.11.158~~ 03:32, 5 October 2015 (MDT) 

    Sprite flickering occurs in most NES games. What is notable about this example in SMB3 that would make someone think it's a bug? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:13, 5 October 2015 (MDT)

## Reliance on RAM values

Several entries in this section claim that it is erroneous to use uninitialized RAM for a random number seed. Why would that be considered a bug? --[NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 04:53, 3 January 2018 (MST) 

## Commando

"There are many bugs that may look like emulator glitches but aren't."

Can we possibly come with a worse and less useful description of the glitch? I don't think so. 

    I have no clue who left this remark, but you can relax now. Changed the description to be more... descriptive. --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 12:04, 21 August 2023 (UTC)

## Metroid bugs

What about MEtroid bugs, like the one where sometimes some tile changes colours or styles when entering a room (fixed by leaving)? 

## Final Fantasy

"_Final Fantasy_ || Uses uninitialized RAM ($F4, $F5, $F6) to determine the first battle encountered after continuing the game. The bytes are usually FF on real hardware, leading to a trick where you get a guaranteed Kyzoku encounter in the ocean, which helps you get money early on." Why is this supposed to be a bug, as opposed to using uninitialized RAM as a random number seed? [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 07:22, 13 May 2023 (UTC) 

## Chip 'n Dale Rescue Rangers

I believe the previous version that said "After reading the intro to the last" probably meant "After watching the complete intro" (not "After reading the intro to the last stage"). As the intro sequence is fading out, the game displays sprites with incorrect tiles in the positions for the player initials and life indicators at the top of the screen. When the screen fades back in, the same positions have the correct graphics. It happens if you watch the entire intro sequence or if you press START to bypass the intro sequence, so it doesn't really matter if you watch the complete intro. --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 06:32, 21 August 2023 (UTC) 

    I just looked up a gameplay video and indeed, the sprite HUD tiles are incorrect during the fade-out. Good catch! --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 11:29, 21 August 2023 (UTC)
