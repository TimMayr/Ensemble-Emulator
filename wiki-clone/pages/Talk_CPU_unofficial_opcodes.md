# Talk:CPU unofficial opcodes

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACPU_unofficial_opcodes) | View [other pages](Special_AllPages.xhtml#Talk_CPU_unofficial_opcodes)

## Puzznic on a Z80?

Regarding [this edit](http://wiki.nesdev.org/w/index.php?title=CPU_unofficial_opcodes&diff=12428&oldid=12425): what does the arcade version running on a Z80 have to do with the NES version? I understand the idea that sharing code that may have been written for 65C02 could cause this particular problem (which is why I didn't just remove that bit of conjecture; though I am starting to regret not removing it), but I don't understand why the arcade version being a Z80 would be pertinent to 6502 code having a weird NOP in it. There were 15 or so different ports of this game, and the relevant ones to inspect might include the C64 and PCE versions, as well as Apple II prototype. If you're looking to make some connection, you could research the opcodes used in these versions, but at this point that's a lot of work to chase down a speculative footnote about why there's a hidden bug (?) in the game. 

Is it even a bug? Does the code seem to expect a BIT #imm functionality? Answering these two questions seems like a prerequisite for even making the 65C02 speculation in the first place. Every bad opcode's gonna mean "something" on another variant... Never mind, I think I'm just going to go the rest of the way and trim it entirely. We haven't justified the connection, at face value. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 21:14, 29 May 2016 (MDT) 

    Took a quick look at it and I don't think there's any reason to speculate about BIT #imm. It's actually BIT #0 ($89 $00), and A will always be $01 when it is hit. Follow the subsequent code any flags this would set are overwritten before they are used. Wondered if it might be an accidentally misspelled LDA #0 ($A9 $00) because it's in a block of code that's a bunch of "LDA #imm + STA zp" stuff, but trying this destroys the game, so it's clearly not supposed to be that. In this particular case BIT #imm is just as much of a NOP as the 6502 version. I don't think it makes worthwhile speculation. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:26, 30 May 2016 (MDT)

## _Gaau Hok Gwong Cheung (Ch)_

Given that instruction $8B is [unstable](http://visual6502.org/wiki/index.php?title=6502_Opcode_8B_%28XAA,_ANE%29), what exactly is the game expecting it to do? — [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:02, 31 January 2017 (MST) 

It only runs $8B from a bad bankswitch. Runs another ~10 garbage instructions before hitting a BRK and recovering. Doesn't matter what the opcode does as long as the emulator doesn't crash/quit. 
