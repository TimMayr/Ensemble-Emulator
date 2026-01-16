# Talk:Fixed cycle delay

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AFixed_cycle_delay) | View [other pages](Special_AllPages.xhtml#Talk_Fixed_cycle_delay)

## Contents

  * 1 No requirements missing from most examples?
  * 2 Page size problematic? Maybe the wiki sin't the best place for exhaustive permutations...
  * 3 Formalizing random writes
  * 4 What is the point of this ridiculously huge page?
  * 5 Tool suggestions



## No requirements missing from most examples?

Shouldn't there be a "no requirements" entry for _every_ example? Isn't that the natural maximum byte size / endpoint for each of these? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:14, 16 March 2016 (MDT) 

    It is not possible to do 3-cycle delay without any requirements. The best you can get is `JMP *+3` which requires relocations, but does not have any execution-time side-effects. Additionally, "no requirements" would always involve a long sequence of "NOP"s followed by a "JMP" if the cycle count is odd. To reduce the size of the page, I omitted these for larger delays, and additionally started considering "writes to stack" harmless from 64 cycles onwards, because in most applications it _is_. --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 15:02, 17 March 2016 (MDT)

## Page size problematic? Maybe the wiki sin't the best place for exhaustive permutations...

I'm noticing the wiki has some serious problems trying to diff some of the history on this page. I noticed [this edit](http://wiki.nesdev.org/w/index.php?title=Fixed_cycle_delay&curid=1621&diff=12038&oldid=12037) with the comment "Further tweak code to prefer repeated sequences, because reducing the page size helps fend off MediaWiki crashing". Given the explosive nature of permutations here, maybe it would be better to just implement this as a javascript tool on a webpage, and link it from here? (Would also be nice because you could just dial in constraints, etc.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 21:02, 16 March 2016 (MDT) 

    I'd recommend splitting them by 2-19, 20-100, 100-200, etc. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 09:40, 17 March 2016 (MDT)

    I agree with principle on a Javascript tool, but there is the problem that finding these delay options is rather CPU-time intensive. The running time for my generator program is O(n^2) for the number of cycles to delay. This bodes badly with a tool that would run in the browser. --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 15:18, 17 March 2016 (MDT)

    

    All O(n^2) tells me is that if n gets large enough, execution times would eventually be unbearable, but n isn't unbounded here. Does generating a single case really take a significant amount of time for, say, n=200? How long does the whole table take? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:53, 17 March 2016 (MDT)

    

    

    Just to be clear, I'm not really trying to pressure you into rewriting it as javascript; I don't have any need for such a tool, I'm just asking from a standpoint of outside curiosity. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:56, 17 March 2016 (MDT)

    

    A program in JavaScript need not necessarily run in browser, but could also be able to run on command-line by Node.js. (It would be possible also to write a JavaScript program that can detect it so that it work on a webpage but also it will work on a if you download it and run it on Node.js too; for example `if(this && this.document)` is one way to detect) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 10:47, 22 April 2016 (MDT)

## Formalizing random writes

"it is difficult to formalize the rules under which one could write to such random addresses."

A write to a random address in a 256-byte page is fine if all addresses in the page are decoded to nothing, to a read-only memory, to a read-only port without side effects, or to memory that will be overwritten later. Most NES mappers decode $4100-$41FF to nothing. In addition, many games use $0200-$02FF, $0300-$03FF, or $0700-$07FF to hold a [display list](PPU_OAM.xhtml "PPU OAM") rebuilt from scratch every frame; if the delay occurs before the next rebuild of the display list, there is no conflict. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 09:43, 17 March 2016 (MDT) 

I tried to think of a way to use "INC abs,X" in a partial-instruction context, but without any additional game-specific knowledge, there are only a few addresses where one can write safely. Here's a sample: 
    
    
    A9 FE    LDX #$FE ; hides 'INC $FDD0,X'
    D0 FD    BNE *-1

This would read from $FDCE, read from FECE, and then write to FECE. Writing to $8000-$FFFF is unsafe in several mappers, so I tried a bit different approaches. 
    
    
    A9 FE    LDX #$3E ; hides 'ROL $3FE0,X'
    E0 3F    CPX #$3F
    D0 FB    BNE *-3

This would read from $3F1E (maps to $2006; write-only register, so reading is safe), and then read and write $401E (unmapped, so access is safe). This would be completely safe. Similar combinations were found for each RMW operation (limited by the value of X which doubles as the RMW opcode number). However, there is no guarantee that the code will not loop infinitely, because the branch instruction is not masked on the second iteration, unlike in the previous sample. If the second opcode was 1-byte long, then the first byte of the branch instruction would be masked, but the second byte would still be executed. The only byte that would work for the second byte of the branch instruction would be F8, which means the code would have to be at least 9 bytes long, which further complicates matters. 

If branches and partial-instruction execution are not involved, then it bears down to how to set up X in such manner that a wrap is guaranteed, and whether the setup code + the RMW instruction are short enough for the number of cycles they consume. 

I am yet to think of a way where RMW abs,X can be used. I was able to incorporate INC zp,X in a few situations where X is known to be zero, though. Of course if you can know a certain page is free for tampering, something could be devised. There might also be benefit if it is known some zp address contains a pointer to another address that can be read/written safely. The addresses I considered _unsafe for reading_ were 4015-4017, 4020-5FFF, and (&7 in (2,7)) in 2000-3FFF. The addresses I considered _unsafe for writing_ were _everything except_ 4018-401F, and (&7==2) in 2000-3FFF. --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 15:18, 17 March 2016 (MDT) 

    Why include all 6 variations on INC $0200,X? Isn't this trivial? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:28, 30 March 2016 (MDT)

    

    In this article it is admittedly trivial, but the code comes from a generator that also considers INC $02xx,X instructions in contexts where half of the instruction is hidden in another instruction, which puts more constraints onto which memory addresses can be used. It just happens to be that within the first 160 samples these cases do not manifest. --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 20:32, 30 March 2016 (MDT)

## What is the point of this ridiculously huge page?

This page is certainly competely useless, as anyone who has a notion of how many cycles an instruction takes can write fixed delay code. This page should be IMO removed. There is certainly NO need to provide 10 examples of fixed delay cycles for every single number between 3 and 100. I am too lazy to log in, but it's me Bregalad. ~~193.134.219.72~~ 06:02, 1 April 2016 (MDT) 

    The point is both ROM hacking and trivial interest. You need to create as small code as possible, and there are constraints of what you can clobber. It is not a trivial challenge. But if you so choose as to delete the page, be my guest. I just hoped someone would share my interest. --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 07:56, 1 April 2016 (MDT)

    It's not up to me to decide whether to delete or not, but my opinion is that this information is not meaningful for NES development. A routine which can do a delay in a certain range with a variable is, however, meaningful for NES development. The fact that "pha" is considered as a delay routine of 3 cycles is by itself a huge aberration to me, if the whole list goes on like that I cannot imagine how much of a mess this is. Maybe if it would be stripped of so many versions and have only a single version for each # of cycles that could at least make a little sense. (Bregalad)~~193.134.219.72~~ 07:07, 4 April 2016 (MDT)

    Sorry to Bisqwit, but I also agree that this page is of no practical use. I was following it from curiosity because I think the permutations are bizarre/interesting. I think as instruction it's bad for beginners (i.e. too much information, requires advanced knowledge to really sort out what you want-- and if you have advanced knowledge you don't need this table, really). Like I suggested above, I still think an online generator program would be tons better than a huge dump on a wiki page like this. The wiki is just a poor medium to approach this. Also there's the issue that trying to look at diffs of this page crashes the wiki, which I think might indicate that the wiki itself isn't happy about it? ;) Even just a forum thread with a link to a pastebin of the dump might be a better place to share this information; you could have a dialog with people about the generator, etc. which as a discussion would be more interesting/useful/fruitful than trying to create this pointless reference table on the wiki in relative isolation. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:59, 4 April 2016 (MDT)

  
On a slightly less unilateral note ... in what not-awkwardly-construed situation is it ever useful to have a delay function that trashes S? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:36, 4 April 2016 (MDT) 

    Well I have no personal experience of it, but when making demos you need to squeeze every cycle and every byte, and it may sometimes involve using registers in non-conventional manner. An example that comes to my mind is Blargg's NES tests, where in some of them, interrupt/NMI handlers never return, and/or they clobber registers. The "stomper" NES demo does almost all its processing without stack manipulations. Similarly for the "full palette" demo. While it is indeed farfetched, it is not unconceivable that a demo might not care about clobbering S if it saves space. --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 18:19, 5 April 2016 (MDT)

## Tool suggestions

I like the new webpage tool, but I have some suggestions. 

  1. Decimal clobber should not be on by default. Inconsequential on a real NES, but lots of subtle consequences on emulators/clones, so I think this should be the same category as unofficial opcodes.
  2. It should have option to generate a single delay length, or some other kind of grouping options. 
     * If you're trying to use this for practical code generation, you really only need 1 specific delay length (or maybe a couple, but never hundreds of variations). The sheer size and complexity of the generated include file makes it a little daunting for use. Would you want to add 100,000 lines of code to your program just to delay 15 cycles optimally?
     * If you're trying to use this to investigate how to write delay code, sticking hundreds of code snippets together in an include file like this obfuscates this study, I think. Maybe seeing a range of delays (e.g. 10 to 20) would be interesting to study, seeing how the code changes between them. Similarly, seeing the same delay length with various constrains would be interesting to look at side by side.
     * Could "preview" a single delay case in a text field on the page. That would let you quickly generate and look at a couple of related cases, would be good for the person trying to learn, but also the person who just wants to copy and paste a small code snippet to do the job they need.
  3. Relative branching using *- is also obfuscating for study. Unless jumping to the middle of an instruction, an anonymous label would be better for study. (Even "label+1" would more clearly express a split instruction than something like "*-5".)



\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:34, 20 April 2016 (MDT) 

    

  1. I will remove the default-selection from the decimal mode flag. I kind of wish to see references on the "lots of subtle consequences on emulators/clones" claim though.
  2. I disagree on the 1 specific delay length. I explained the rationale on romhacking.net: “Advantages over handwritten code include that the number of cycles to delay can be any compile-time numeric expression, as is needed in my project, where the number of splits & delays per screen can be large, especially with the map screen that is split into four slices, and the number of cycles to delay depends on various factors: The NTSC/PAL selection, and the mapper, as page-switches in each mapper consume a different number of cycles. Using generated code thus helps keep small not only the binary size, but the source code size as well.” In other words, using a macro is necessary when the number of cycles to delay depends on compile-time options, and can not be known in advance when the source code is written. And even when that is not a factor, it is still much more convenient and better for documentation purposes to say `delay_n 5000` than to copypaste 12 lines of code. :)
  3. Relative branching is done that way because it works well for the generator, but more importantly it helps bring down the source code length, which is a factor with macros this long. It also helps avoid scope problems that easily arise with ca65.


    Previewing would be entirely possible and not too inefficient to implement, I think. Still requires going through a 4-gigabyte unindexed data file on the server side though. I will think of it. Thank you for the feedback! --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 19:39, 20 April 2016 (MDT) 

    Does that mean this "generator" is really just a selector for which .inc file to download, all pre-generated? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:04, 21 April 2016 (MDT) 

    There are three layers of "generators". On the bottom layer, there is the generator that makes the database, and takes about a week or two to run. Done once (or more, if updated). It is a database of all different options for 20000 cycles, with bitmasks for which constraints are required for each option. I found this to be the most efficient solution considering the vast number of combinations from all different constraints (about 30 bits, i.e. thousand million combinations). Now it only needs about 100 choices for each bitmask by average. During generation the constraints may vary. For example, wrapping code in "PHA-PLA" allows to use code that clobbers A without actually clobbering A. It allows the generator to reuse work. On the middle there is a second generator that reads the database and produces a single .inc file, and it is continuously run to produce more of them. It takes about a minute or two to run for a single combination of constraints. For each clock cycle, it selects the nicest line from the database matching the given constraints, and then searches and combines identical or similar subsequences and parametrizes them to reduce the macro file size. On the top is the website which simply selects an .inc file, does sanity checks and either redirects or informs the user. For the user, the whole process is represented as a single "generator". --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 05:14, 21 April 2016 (MDT)
