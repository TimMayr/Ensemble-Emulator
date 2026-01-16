# Talk:APU Sweep

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAPU_Sweep) | View [other pages](Special_AllPages.xhtml#Talk_APU_Sweep)

Interesting fact: the 2 square channels are almost perfect mirror images of each other on the silicon, with one single difference: where one part of the 2nd channel's sweep unit takes as an input the (inverted) state of the Negate flag, the equivalent input in the 1st channel's sweep unit is hardwired to +5V. Said input is almost definitely the Carry Input flag, and it explains the difference in behavior. --[Quietust](User_Quietust.xhtml "User:Quietust") 00:45, 10 May 2011 (UTC) 

The RP2A03 may have originally been planned to permit periods as low as 4 - the current "silence on period < 8" behavior is accomplished by feeding bits 3-10 into a large NOR gate (if they're all 0, the channel is silenced), but there's another **unconnected** input for bit 2. It might be enlightening to test earlier versions of the chip. --[Quietust](User_Quietust.xhtml "User:Quietust") 17:16, 14 May 2011 (UTC) 

Does the sweep unit disable the channel when the target period is greater than 3FF or greater than 7FF? --[Drag](User_Drag.xhtml "User:Drag") 04:00, 17 May 2011 (UTC) 

    When the target is greater than $7FF. If a sweep unit's value is $00, and the sweep value is in $400-$7FF, then as I understand it, the target period is in $400 + ($400 >> 0) through $7FF + ($7FF >> 0), which equals $800-$FFE, and the channel is silenced. 

    So the next target period is generated right after the channel's period gets updated? --[Drag](User_Drag.xhtml "User:Drag") 02:00, 18 May 2011 (UTC) 

    There's something you should understand about silicon. As soon as a value is clocked into a hardware latch, all the combinational logic feeding off that latch begins to update immediately. So yes, the next target period starts to be generated right after the previous one gets written back, and this new target period is available by the next CPU cycle. --[Tepples](User_Tepples.xhtml "User:Tepples") 02:13, 18 May 2011 (UTC) 

    In this case, though, almost everything in the APU (with the sole exception of the Triangle channel) is synchronized to a pair of clock signals which seem to be _half_ the CPU clock (I haven't traced out the entire circuit - if you want to try, take a look at [this](http://www.qmtpro.com/~nes/misc/apu_clocks.png)), which actually explains a few other observations. --[Quietust](User_Quietust.xhtml "User:Quietust") 03:49, 18 May 2011 (UTC) 

    An attempt at simulating that circuit (by pulsing M1 and M2 the way a real 6502 does, which is "both low -> M1 high -> both low -> M2 high -> both low -> ...") yielded the following result: the signal labeled "OUT2" is only high during **odd** CPU M1 pulses (and is low otherwise), and the signal labeled "OUT1" is only low during _even_ CPU M1 pulses (and is high otherwise - most parts of the APU that use this clock invert it beforehand, effectively making it a 2nd clock phase). I'm not certain that I was simulating it correctly, though. --[Quietust](User_Quietust.xhtml "User:Quietust") 17:15, 18 May 2011 (UTC)

* * *

Just to make sure that I'm getting this right: Is the behavior described by "when clocked by the frame counter, the divider is first clocked and then if the reload flag is set, it is cleared and the divider is reloaded" only visible if the divider's counter happens to be zero, in which case you would get a clock to the sweep unit before the counter is reloaded? -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 21:40, 29 April 2013 (MDT) 

## Clocking behavior

If I'm reading the clocking logic right, the 'enabled' flag controls whether the divider reloads its counter? This means that the divider counts down even if sweep is disabled, and when it gets to 0, only if sweep is enabled it gets reloaded? \--[Daroou](https://www.nesdev.org/w/index.php?title=User:Daroou&action=edit&redlink=1 "User:Daroou \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Daroou&action=edit&redlink=1 "User talk:Daroou \(page does not exist\)")) 22:23, 20 February 2015 (MST) 
