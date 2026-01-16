# Talk:MMC3

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMMC3) | View [other pages](Special_AllPages.xhtml#Talk_MMC3)

The hardware section of this page needs to be tidied up a bit, because right now, it's just a giant infodump with little organization. :P --[Drag](User_Drag.xhtml "User:Drag") 13:09, 4 June 2012 (PDT) 

There's some documentation about the differences between MMC3 revisions here: [[1]](http://forums.nesdev.org/viewtopic.php?t=6467) I'll add them in at some point, if nobody else does. --[Drag](User_Drag.xhtml "User:Drag") 18:00, 19 April 2011 (UTC) 

## Contents

  * 1 PRG RAM bank behavior
  * 2 MMC3 scanline counter
  * 3 Oversize "non-compliance"?
  * 4 PRG RAM protection
  * 5 Poorly organized?
  * 6 Scanline counter
  * 7 Is this a typo?
  * 8 OAM Corruption with PPU_MASK



## PRG RAM bank behavior

When PRG RAM is accessed via $6000-$7FFF, what does the MMC3 put on the upper PRG ROM address lines? --[Tepples](User_Tepples.xhtml "User:Tepples") 23:24, 20 April 2011 (UTC) 

    Ideally, the MMC3 wouldn't put anything on the ROM address lines. :P Either way, if the CPU is accessing $6000-7FFF, I'd imagine whatever chip is selected would see %011xxxxx xxxxxxxx on its address lines. RAM would just see the x part, and whatever mapper-supplemented upper address lines if the RAM is bankswitched. --[Drag](User_Drag.xhtml "User:Drag") 04:05, 21 April 2011 (UTC) 

    When I hear "wouldn't put anything", I think "high impedance". It appears you claim that if $6000-$7FFF is accessed, the PRG A13 through PRG A18 outputs from the mapper are high-Z, not the value in either PRG bank register. Do I misunderstand? --[Tepples](User_Tepples.xhtml "User:Tepples") 22:42, 21 April 2011 (UTC) 

    The simplest thing electrically is to never disable the address drivers. Given how MMC3 works I'd guess it drives the upper address lines high. [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 18:08, 22 April 2011 (UTC)

    

    Actually, I'm not entirely sure I understand what you're saying. When you say PRG ROM address lines, I thought you meant the address lines on the actual ROM. Yes, even if the CPU is accessing $6000-7FFF, the ROM would still see the address bus, but it won't do anything with it, because the mapper won't select the chip. That's what I meant by "wouldn't put anything on the ROM address lines". Even though it's physically putting something there, the ROM is disabled so it never does anything with it.

    

    If you're talking about what happens with WRAM, I imagine the chip wouldn't have the upper three address lines, unless the RAM is bankswitched. --[Drag](User_Drag.xhtml "User:Drag") 07:33, 25 April 2011 (UTC) 

    I was asking whether or not it's feasible to build PRG RAM bankswitching by running the same upper address lines out of the mapper to both PRG ROM and PRG RAM. When PRG RAM is being accessed, what is the voltage on each of the upper address lines PRG A18-A13 (pins 23, 25, 21, 18, 22, 19) coming out of the MMC3? Does it depend on either of the PRG ROM bank values, or is it a constant value? --[Tepples](User_Tepples.xhtml "User:Tepples") 13:25, 25 April 2011 (UTC)

    

    

    

    Ahh, ok, that makes more sense. The MMC3 outputs different things to the most significant rom bits depending on what address is being accessed, so it's entirely possible that the circuitry involved selects between a couple different latches, depending on bits 14 and 13 of the PRG address. If it uses an ordinary decoder, then it's possible that bit 15 goes to the /OE of whatever multiplexer it uses.
    TL;DR: The "banks" for $0000-$7FFF may mirror the bank configuration of $8000-$FFFF, or it may not. Either way, accessing $6000-7FFF is like accessing $E000-FFFF (which is fixed to the last bank), so whether it looks at A15 or not, you'll probably end up with all 1's across A13+. This is just my speculation though. --[Drag](User_Drag.xhtml "User:Drag") 19:56, 25 April 2011 (UTC)

  


    I can't say for sure, because very complex equipment would be needed to verify this, but it's _extremely_ likely that the MMC3's PRG-banking ciruitery completely ignores the state of A15. That is, acessing any adress with A15 low will have the higher adress lines pointed to the same latch as if the corresponding adress with A15 high was used. In particular, when acessing $6000-$7fff, the MMC3 will act like when acessing $e000-$ffff (which means all adress lines will go high, pointing to the "last-hardwired" bank).

    This hypothesis is especially robust when you consider there is no A15 on the cart edge, so you have to invert PRG /CE to get the state of A15. Also, when the PPU fetches nametable area ($2000-$2fff), the MMC3 does bank the CHR-ROM exactly like when acessing the corresponding area with A13 low, that is $0000-$1fff, and the TLSROM and TKSROM boards makes a good use of this.

    This is valid not only for the MMC3 but for 99% of mappers in fact : Anyone would be crazy to add extra circuitry in their chips so that it acts differently when the CPU is adressing space where the ROM will never respond anyways. (Probably only mappers such as the MMC5 or FME7, which can map ROM and RAM anywhere regardless of the state of A15, will not follow this rule). So, no, sorry but no PRG-RAM switching would be possible on MMC3. But any mapper that was not designed to switch PRG-RAM, but that was designed to switch PRG-ROM at $e000-$ffff could be abused to also switch RAM. Bregalad 11:39, 1 May 2011 (UTC)

## MMC3 scanline counter

What happens when both background and sprites are fetched from $1xxx? --[Zepper](User_Zepper.xhtml "User:Zepper") 02:28, 8 November 2011 (UTC) 

    I think that should disable IRQs, because A12 will never be 0 for longer than 4 pixels —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:12, 25 January 2015 (MST)

    

    Even during the dummy fetches at the end of a line (337-340)? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:03, 25 January 2015 (MST)

    

    

    Depends on what the idle cycle's address is. If A12 is 0 during it, then A12 could be low for 9 pixels. And then that gets into the question below… —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:40, 25 January 2015 (MST)

_The MMC3 scanline counter is based entirely on PPU A12, triggered on rising edges (after the line remains low for a sufficiently long period of time)_ Which period of time exactly? —anonymous contributer from ~~94.254.14.226~~

    Without clocking it manually, it's hard to say, but certainly not more than 64 pixels (because it still works if the pattern tables are swapped). See also <https://github.com/christopherpow/nes-test-roms/tree/master/mmc3_irq_tests> ; test 3.4 there also requires that it be no longer than ≈22cy, but that's about the same as 64 pixels. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:01, 25 January 2015 (MST) 

    I performed some tests using my own dumper and real cartridge. Seems like IRQ counter working when there are 2 rising edges of M2 between falling and rising of A12. It's the only one necessary condition. —~~83.149.9.144~~ 07:25, 31 January 2015 (MST) 

    That can't quite be right. In simulation I can get two rising edges of M2 (since that's 3 pixels apart) during the 4 pixels that A12 is low... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:46, 10 December 2016 (MST)

## Oversize "non-compliance"?

I could use a clarification about [this edit](http://wiki.nesdev.org/w/index.php?title=MMC3&curid=89&diff=11058&oldid=11049) by Lidnariq. If the ROM size is <= 512K the top two data bits _have_ to be ignored when writing the bank register. What possible incompatibility could result from implementing an 8-bit register? Why would an emulator choose to present an error to the user instead of simply supporting the oversize ROM? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:59, 30 May 2015 (MDT) 

    For the same reason that NES emulators shouldn't implement a plugin that takes in arbitrary video and quantizes it to the NES output: the hardware that does that doesn't exist. Same problem with ROM hacks that only work in Nesticle. Emulators must implement the restrictions of the hardware, because otherwise you end up with crap like the aforementioned ROM hack where because they don't understand _why_ they're not allowed to do that, they include patches to hack FCEUXDSP and Nintendulator to support their BS oversize ROM, and no-one can ever make a reproduction that uses an MMC3. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:55, 30 May 2015 (MDT)

    

    Ahh, the extension seems extremely logical and straightforward to me, but after checking FCEUX, Nestopia, and Nintendulator, I can see that all of them prevent oversized MMC3. I don't really understand the motivation for this (does it create other software incompatibility?), but the lack of emulator support is good reason to remove oversize support from the infobox. Hadn't read the description of that romhack either (was just part of a merge), but it's kind of interesting that not much supports it. I'll revise. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:15, 30 May 2015 (MDT)

## PRG RAM protection

I just noticed that _Zoda's Revenge_ uses some strange writes to $A001 (RAM protection). 
    
    
       LDA #$30
       STA $A001
    

or 
    
    
       LDA #$B0
       STA $A001
    

So RAM should be protected. But it writes to RAM after this. 

FCE ultra actually ignores state of $A001 for mapper #4 and game is working fine but it crashes on my cartridge (no sound after start, crashes later). Is it game using some other revision of MMC3? Or MMC3 actually using other bits in $A001 too? Or maybe this game using some weird wiring of MMC3? ~~94.253.14.226~~ 01:20, 22 September 2015 (MDT) 

    Startropics and Startropics 2 use the [MMC6](MMC6.xhtml "MMC6"), not the [MMC3](MMC3.xhtml "MMC3"), which is the same except for this write protect behaviour. Unfortunately both are assigned to iNES mapper 4, so emulators that chose to implement the write protection have a compatibility conflict. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:39, 22 September 2015 (MDT)

    

    Thank you very much! ~~94.253.14.226~~ 01:47, 22 September 2015 (MDT)

## Poorly organized?

I hate to critique someone else's work, but this article is extremely hard to parse for someone just learning about how bank switching and memory mappers work. It doesn't use or define terms in a consistent way, and jumps from basic usage to obscure exceptions to discussions of how emulators implement (or don't implement) aspects of the mapper, and back again. 

Perhaps the articles on specific mappers isn't the place for an overview or tutorial, but it doesn't appear that there's anywhere else on the Wiki that provides this information. -- ~~208.71.141.54~~ 13:36, 16 December 2017 (MST) 

    Maybe you're right that we need [Programming MMC3](https://www.nesdev.org/w/index.php?title=Programming_MMC3&action=edit&redlink=1 "Programming MMC3 \(page does not exist\)") alongside [Programming UNROM](Programming_UNROM.xhtml "Programming UNROM") and [Programming MMC1](Programming_MMC1.xhtml "Programming MMC1"). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:20, 20 December 2017 (MST) 

    I didn't know those existed. Yes, I think that's the resource I was envisioning. I'm not qualified to assist in drafting but I think it would be valued by many people, especially given the unique aspects of MMC3! ~~208.71.141.54~~ 23:26, 20 March 2018 (MDT)

## Scanline counter

"Because of this, if BG uses the left pattern table ($0000), and if sprites always use the right pattern table ($1000), A12 will remain low during all nametable and BG pattern fetches, and high during all sprite pattern fetches, causing it to oscillate exactly one time per scanline". That is not correct, A12 does not oscillate exactly one time, it oscillates _eight_ times per scanline. The paragraph above it mentions that the MMC3 filters A12 ("after the line remains low for two rising edges of M2"), but this description is written as if it describes the raw PPU A12 signal, so one does not understand why the MMC3 needs to filter. 

Also, [Krizz has since found](https://forums.nesdev.org/viewtopic.php?f=5&t=10344&start=45#p242427) that the MC-ACC does not just look for falling edges of A12; instead of filtering A12 by requiring that "the line remains low for two rising edges of M2", it looks at the raw A12 signal and prescales by eight. Only when emulating this will _Mickey's Safari in Letterland_ look correctly. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 04:17, 5 October 2019 (MDT) 

## Is this a typo?

R6 and R7 will ignore the top two bits, as the MMC3 has only **6** PRG ROM address lines. 

It's supposed to be 64, isn't it? - anonymous on 07:03, 10 November 2019 (MST) 

    **6** address lines = 6 bits. 26 = 64 banks. Saying that it has only 64 banks would be circular, because it doesn't explain why. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:06, 10 November 2019 (MST)

## OAM Corruption with PPU_MASK

**** ROUGH DRAFT ****

There are certain timings in which turning off background rendering with the scanline counter can corrupt OAM. This can happen in horizontal blank. This cannot be fixed by updating OAM_DMA. There are safe windows when you can turn off the background without corrupting OAM memory. 

There are no emulators that can truly reproduce this affect. Nor does it happen on every NES model, some revision 2 consoles do not suffer these side effects. 

Optimally, you would like your PPU_MASK write to be between PPU dot cycles 320-340. This has the best chance of success. It is recommended that you test with [Fiskbit's Test ROM](https://forums.nesdev.org/download/file.php?id=17351) when choosing an interrupt target for a scanline, as there are slight variances in timing; if you are indeed lucky enough to own affected hardware. 

  


* * *MMC3 Simple Scanline Blanker * *
* * Now with Less OAM Corruption! * *

IRQ: PHA 

; (other things besides spinning in a loop) 

; Wait for the golden window 

LDA #$11 

SEC 

\- 

ADC #$FE 

BNE - 

; Turn off rendering 

STA PPU_ADDR 

STA PPU_ADDR 

STA PPU_MASK 

; Acknowledge Interrupt 

STA $E000 

PLA 

RTI 
