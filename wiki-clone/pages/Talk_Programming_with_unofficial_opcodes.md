# Talk:Programming with unofficial opcodes

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AProgramming_with_unofficial_opcodes) | View [other pages](Special_AllPages.xhtml#Talk_Programming_with_unofficial_opcodes)

## Contents

  * 1 Stability of ATX
  * 2 NOPs that discard reads
  * 3 Bus conflicts serious here?
  * 4 ARR and AXS instructions
  * 5 Lot check for unofficial opcodes?



## Stability of ATX

[The reference I'm using](http://anyplatform.net/media/guides/cpus/65xx%20Processor%20Data.txt) lists ATX as "unstable"; I made a point of leaving "unstable" instructions out of a reference for programmers. Furthermore, it states that its behavior might differ between machines. It cites two references agreeing with you but also cites [Adam Vardy's document](http://nesdev.org/extra_instructions.txt), which calls the instruction "OAL", claims that it includes ORA #$EE as one of the steps, and further claims that different machines use different values instead of $EE. It might actually be ORA <line noise> AND #i TAX, and predictable only if i = 0 (in which case it's no different from LAX #0). --[Tepples](User_Tepples.xhtml "User:Tepples") 16:25, 8 February 2011 (UTC) 

    On IRC, kevtris reported that he has seen a constant $FF as the value in the ORA step, which would put ATX with the LAXs. The difference between $EE and $FF might arise from the lack of decimal mode in the 2A03's ALU. But I'd still recommend against using this instruction because different NES consoles may still interpret it differently. --[Tepples](User_Tepples.xhtml "User:Tepples") 04:05, 11 April 2011 (UTC)

<http://visual6502.org/wiki/index.php?title=6502_Opcode_8B_%28XAA,_ANE%29> strongly implies weirdness with the $10 and $01 bits on all unstable instructions are due to a different period of analog feedback inside the 6502. [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 18:59, 11 April 2011 (UTC) 

## NOPs that discard reads

[This was a thread in the forum](http://forums.nesdev.org/viewtopic.php?p=60734#p60734) in the past, but maybe we might mention the side-effect reads (There, the monikers DLD (dummy load) and LDN (load no-op) were suggested, other random possibilities include "LoaD and Discard"). Could one use 'LDD $20FF,X' where X is some multiple of eight to discard two bytes from CHR-ROM, or would that tight of timing screw up the PPU's fetch FSM? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:41, 24 August 2013 (MDT) 

LDD a ($0C aa aa; 4 cycles)
LDD a,X ($1C aa aa, $3C aa aa, $5C aa aa, $7C aa aa, $DC aa aa, $FC aa aa; 4 or 5 cycles)
LDD d ($04 dd, $44 dd, $64 dd; 3 cycles)
LDD d,X ($14 dd, $34 dd, $54 dd, $74 dd, $D4 dd, $F4 dd; 4 cycles)
    Reads from memory at the specified address. Discards the result. Affects no register nor flags. Only useful for side effects (e.g. PPUADDR increment) so on the NES the zero-page versions are only useful for timing.
    LDD d,X reads from both `d` and `(d+X)&255`. LDD a,X additionally reads from `a+X-256` it crosses a page boundary (i.e. if `((a & 255) + X) > 255`)
    Sometimes called TOP (triple-byte no-op), SKW (skip word), DOP (double-byte no-op), or SKB (skip byte).

Figured I'd format it. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:35, 24 August 2013 (MDT) 

    I'd call it IGN (ignore). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 21:04, 24 August 2013 (MDT)

## Bus conflicts serious here?

Some distinction should be made between "bad" bus conflicts, e.g. CMOS high versus anything else's low, and benign ones, like a pull-up resistor versus anything. Here it talks of bus conflicts in the unofficial opcodes, but from the sounds of it it's no different stress-wise than how a normal NMOS output works internally when outputting low, where its in "conflict" with the pull-up current source. I think people are (rightly) concerned about intentionally causing bus conflicts, and might be wary of using techniques that cause what they think are the same thing. Maybe calling them stressful bus conflicts and benign bus conflicts would make it clearer. [Blargg](User_Blargg.xhtml "User:Blargg") ([talk](User_talk_Blargg.xhtml "User talk:Blargg")) 21:36, 6 January 2014 (MST) 

    Yes, I agree that it should be made clear about how potentially damaging it can be or not (but I don't know how it works with unofficial opcodes). If there are resistors then it isn't really a bus conflict but is a default logic. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:22, 6 January 2014 (MST) 

    I have been told that the internal bus conflicts in the CPU aren't dangerous.--[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:58, 27 January 2014 (MST)

## ARR and AXS instructions

Are these descriptions for ARR and AXS instructions correct? VirtuaNES doesn't emulate them exactly this way nor does any other documentation I can find specify that they work quite this way either. One file says this: "ARR: part of this command are some ADC mechanisms. following effects appear after AND but before ROR: the V-Flag is set according to (A and #{imm})+#{imm}, bit 0 does NOT go into carry, but bit 7 is exchanged with the carry." and "AXS: performs CMP and DEX at the same time, so that the MINUS sets the flag like CMP, not SBC." \--[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:58, 27 January 2014 (MST) 

    The (only) reliable reference is Visual6502 and tests done on a physical NMOS 6502. With that it mind, I tested:
    $6B(ARR): As far as I can tell, Visual6502 seems to be omitting the AND effect. The rest of the document here agrees with what Visual6502 is doing.
    $CB(AXS): The document here agrees with what Visual6502 is doing.
    For both, the writeups you've found are of dubious applicability. The former (ARR) is another way of phrasing the same claims on this page. The latter (AXS) doesn't actually describe the behavior at all, only what lines in the PAL are fired. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:44, 27 January 2014 (MST) 

    Of course, that is correct, that the physical NMOS 6502 is the best way to know what it is doing. Why is Visual6502 omitting the AND effect of the $6B instruction (is it a bug in Visual6502)? And you are correct, but I didn't realize it at first. I may have made some mistakes myself in trying it, but it is also possible that some emulators are also defective, or probably both. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 00:09, 28 January 2014 (MST) 

    A lot of the AND behavior in unofficial instructions arises from bus conflicts, which in NMOS act as AND (lower voltage wins). Perhaps some simulators are less likely to handle bus-conflict ANDs correctly than AND gates made out of transistors. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:43, 28 January 2014 (MST)

## Lot check for unofficial opcodes?

"Using the CPU's unofficial opcodes could get your game rejected from Nintendo lot check" What reason do we have to believe that this is true? 

I'm trying to imagine how this test would have been performed. Did they build a custom 2A03 CPU that generates an external signal on these opcodes? (Seems impractical.) Did they run your ROM on an emulator? (I doubt emulators at the time were suitable for testing an interactive game like this.) Did they require full source code disclosure? (That would be a highly unorthodox requirement.) I'm straining to think of a plausible method Nintendo would have tested for this. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 08:14, 21 August 2014 (MDT) 

If you want to explain the dearth of unofficial opcodes in licensed NES games, I don't think you need any extra reason beyond them being largely unknown at the time, and officially discouraged. Even today lots of us avoid using them for the sake of emulator compatibility. The same concern existed then for future hardware revisions. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 08:21, 21 August 2014 (MDT) 

    I have no official evidence, but I do remember having read in _Nintendo Power_ that Nintendo originally intended the Super Famicom to be backward compatible with Family Computer software. I was guessing that back when Nintendo still thought the Super Famicom was going to be backward compatible, it could have tested games with a 65802 (a 65816 in a 6502 pin-compatible package) instead of a 6502. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 12:58, 21 August 2014 (MDT)
