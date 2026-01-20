# Talk:Emulator tests

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AEmulator_tests) | View [other pages](Special_AllPages.xhtml#Talk_Emulator_tests)

The <http://blargg.parodius.com> links are broken (e.g., instr_test-v3.zip). --[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") 21:30, 17 January 2013 (MST) 

    They'll [remain broken until slack.net comes back](http://forums.nesdev.org/viewtopic.php?p=106293#p106293). --[Tepples](User_Tepples.xhtml "User:Tepples") 06:38, 18 January 2013 (MST)

Breaking out the "Hard-to-emulate games" section into its own page might be nice. While they make good "emulator tests" in a sense, there's no indication that you'll find them here. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 14:20, 1 April 2013 (MDT) 

    I went ahead. Hopefully not too controversial. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 07:55, 2 April 2013 (MDT)

The Nintendulator log for the nestest.nes file appears to contain inaccuracies. For example, line 103 (opcode 0x28: PLP) should pull 0xFF from the stack and set the processor status to 0xFF, but as you can see in the next line, the processor status is 0xEF. This is wrong according to sources such as [visual6502](http://visual6502.org), [e-tradition](http://www.e-tradition.net/bytes/6502/index.html), and [obelisk's 6502 reference](http://www.obelisk.demon.co.uk/6502/reference.html#PLP) \-- a random contributor at ~~129.120.54.184~~ 15:33, 24 July 2013 (MDT) 

    The BRK bit _does not exist_. P is a _six-bit_ register, and the other two bits only have defined values when in the stack. In this case, any of 0xCF, 0xDF, 0xEF, and 0xFF are valid values for the fictionalized eight-bit version of P. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:06, 24 July 2013 (MDT)

## Mouse test link is broken

The mouse test at <http://rdev.g-pw.org/makimura/homebrew> cannot be found. The page (and site) is down.~~184.97.70.230~~ 12:50, 31 January 2015 (MST) 

## ROM loading is broken

The ROM loading process is broken on JSNES.org and it needs to be fixed 
