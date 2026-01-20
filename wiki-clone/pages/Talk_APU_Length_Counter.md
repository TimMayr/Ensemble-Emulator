# Talk:APU Length Counter

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAPU_Length_Counter) | View [other pages](Special_AllPages.xhtml#Talk_APU_Length_Counter)

Food for thought: the values in the length counter table [on the die](http://uxul.org/~noname/chips/cpu-2/no-metal/stitched/final/) (bottom of the chip, just left of center), **once incremented by 1** , match the ones on this page. --[Quietust](User_Quietust.xhtml "User:Quietust") 03:30, 23 January 2011 (UTC) 

\- As small note, on $4015 reads in my emulator, I have to put a flag there because the LC *should* be +1. Probably unrelated... \--[Zepper](User_Zepper.xhtml "User:Zepper") 01:28, 28 January 2011 (UTC) 

* * *

There's some structure to how these values are laid out, which is reinforced when you look at the order in which they appear in the PLA on the chip. Here's an index (in binary) to length map: 
    
    
    11111 -> 30
    11101 -> 28
    11011 -> 26
    11001 -> 24
    10111 -> 22
    10101 -> 20
    10011 -> 18
    10001 -> 16
    01111 -> 14
    01101 -> 12
    01011 -> 10
    01001 -> 8
    00111 -> 6
    00101 -> 4
    00011 -> 2
    00001 -> 254
    
    11110 -> 32  (24 times 1 1/3)
    11100 -> 16  (12 times 1 1/3)
    11010 -> 72  (12 times 6)
    11000 -> 192
    10110 -> 96
    10100 -> 48
    10010 -> 24
    10000 -> 12
    
    01110 -> 26  (20 times 1 1/3)
    01100 -> 14  (10 times 1.4)
    01010 -> 60  (10 times 6)
    01000 -> 160
    00110 -> 80
    00100 -> 40
    00010 -> 20
    00000 -> 10
    

The least significant bit seems to be a "use linear length" flag, which makes the length the value of the remaining bits (with 0 being special). When not set, you get doubling note lengths based on 10 and 12, with some other values in there too (maybe handy for slight length alterations and triplets and the like - I'm not a professional musician :P). -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 18:34, 29 May 2013 (MDT) 

    The interpretation I put on the page is probably even more likely... -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 20:51, 29 May 2013 (MDT)
