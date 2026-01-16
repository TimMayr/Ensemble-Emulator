# Talk:Action 53

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAction_53) | View [other pages](Special_AllPages.xhtml#Talk_Action_53)

Will you release the ROM file? Will you release the source codes of the files for the Action 53 engine? Do you have 72-pins and 60-pins cartridges? Should you give examples about quality of Action 52 since not everyone would know that? Why do you hate unwinnable situations by ratchet scrolling (there are some situations in which you can make a game having this as part of a puzzle)? Which input devices can be used? Is NROM with CHR RAM for pattern tables acceptable? If you have 53 games and someone has more which is eligible, can you also make the second volume? Are there requirements for initialization which must be performed? Can you make the main menu perform TV mode detection and store the result in the X register (like NSF does)? Another idea would be allow use of Famicom keyboard on main menu (but not required): Arrow keys to move cursor, space bar to display description, RETURN to start game, CLR HOME to reset, letters and digits select a game quickly (assign a code consisting of a letter and digit to each game) and displays the description (pushing RETURN from there starts the game). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 14:30, 26 September 2012 (MDT) 

    Let's break this down:
    The ROM file and the menu source code are available: [Action 53 0.03 release announcement](http://forums.nesdev.org/viewtopic.php?p=95846#p95846).
    As for 72- and 60-pin carts, that depends on whom I eventually choose as the manufacturer.
    As for ratchet scrolling, I want to avoid the frustration of having to press Reset and lose all progress. Super Mario Bros. handles it well: apart from hardcore glitch abuse, the player can't really get stuck in a situation where the only option is to wait for time to run out. I was thinking more of situations like R-Type, where the path forks, and the player can't tell which fork is correct without choosing the wrong one and losing a life at a dead end.
    As for what not to do (A52 style), I've described that in more detail in the page on my own wiki; plenty of the [subjective requirements](http://pineight.com/mw/index.php?title=Action_53#Subjective_requirements) link to pages on TV Tropes that list examples of egregiously violating games.
    NROM modded with CHR RAM is perfectly fine; Munchie Attack uses that configuration. In fact, if there's demand, I plan to allow for including BNROM games in the collection by allowing the builder to patch a bank lookup table.
    The [TV system detection code](Detect_TV_system.xhtml "Detect TV system") is just 32 bytes and, like the gamepad reading code, could just as easily be duplicated in all banks. Most games won't see a value in the X register anyway, as they LDX #$FF TXS very early on.
    As for the Family BASIC keyboard, I'm sort of loath to support any peripherals that I can't easily obtain to test with. I added controller, mouse, and Zapper support because that's what I happened to have lying around. --[Tepples](User_Tepples.xhtml "User:Tepples") 17:10, 26 September 2012 (MDT) 

    I agree about avoiding that frustration with ratchet scrolling; but depending on the game there may be solutions: 

  * Have it switchable by a mode on the title screen (possibly a secret mode).
  * Push SELECT and START button at the same time to commit suicide.
  * Have infinite lives so you can retry the level as often as you want (but you should absolutely not put a checkpoint in the wrong path!!).


    I happen to like games with a lot of traps like these as long as you can recover (such as save files or whatever; of course in this case there are no save files). But if you don't like it, it is OK; if I make anything like that you can choose not to put it on Action 53.
    For your other things I understand OK; yes I did read your wiki already. However: Is it required that each game can run standalone once extracted from the Action 53 ROM?
    Another question: Is there requirement for input devices used by the games? My suggestion (just my opinion; of course the way you decide to do it may differ) is: Any controllers may be used, but it must work with only two standard controllers, without requiring the use of SELECT/START/microphone on the second controller (but they can be used as an optional feature if the game supports them, I suppose, as with any other input devices such as mouse and keyboard). This way is more compatiblity.
    \--[Zzo38](User_Zzo38.xhtml "User:Zzo38") 18:55, 26 September 2012 (MDT) 

    If there's some means of escape, such as suicide with infinite lives to go back to a position that's no longer unwinnable, I guess that goes back to [how I define "SHOULD NOT" when written in all caps](http://tools.ietf.org/html/rfc2119): it means there is an occasional good reason to do something, but think hard first.
    I test each ROM standalone on a PowerPak before putting it in. I encourage standalone operation because it allows taking advantage of the "aggregate" exception in the GNU General Public License. In cases where multiple activities fit in one PRG ROM bank, one should be able to access them all by changing the reset vector and possibly inserting a CNROM mapper write to bring in the CHR ROM bank.
    As for your controller suggestion, there is already one game that's unreasonably difficult without a Zapper and CRT, and two "Toys" are completely unusable. But the idea of not requiring the use of Select and Start on controller 2 or the Famicom microphone is a good idea; I'll see what I can draw from it.
    \--[Tepples](User_Tepples.xhtml "User:Tepples") 06:41, 27 September 2012 (MDT)
    Someone I'm considering for the manufacturing told me this: "As for 60-pin cartridges, I'm willing to support the idea but there are a few issues around the fact that would need to be addressed. Namely what cases are these going to fit in... Secondly there would have to be an understanding by the 60-pin purchasers that they're going to pay a little more because I can't leverage quantity for them like I can with the 72-pin world." \--[Tepples](User_Tepples.xhtml "User:Tepples") 14:49, 2 October 2012 (MDT)
    Well, OK, so it means both 60-pins and 72-pins are available but 60-pins cost more? How much more? Also, can PRG RAM be used by any games in Action 53? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 14:30, 5 October 2012 (MDT) 

    I don't yet know exactly how much replication will cost. As for PRG RAM, what were you planning on making? --[Tepples](User_Tepples.xhtml "User:Tepples") 16:48, 5 October 2012 (MDT) 

    I have no specific plan at this time, which is related to the PRG RAM, I am just wondering if it can be used. So, if I ever do make something with PRG RAM, or someone else does, they would know whether or not it is eligible for Action 53. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 17:36, 5 October 2012 (MDT)

## Crediting

Why's tepples credited for the SFX editor? Wasn't that Novasquirrel's? Or was it reworked significantly?[Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 08:36, 20 May 2016 (MDT) 

    Tepples wrote an entirely different SFX editor that was included on the cart --[NovaSquirrel](User_NovaSquirrel.xhtml "User:NovaSquirrel") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:NovaSquirrel&action=edit&redlink=1 "User talk:NovaSquirrel \(page does not exist\)")) 13:21, 20 May 2016 (MDT) 

    [Mine](http://forums.nesdev.org/viewtopic.php?p=133723#p133723) is the one with 4 channels, mouse support, and a longer sequence. The full version has save support; its .sav file can be renamed to .txt to reveal sound effect definitions that can be copied and pasted into a Pently 0.03 or 0.04 project. (Pently 0.05 uses a different input format.) The version on DA53 has the save support removed. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 13:59, 20 May 2016 (MDT)
