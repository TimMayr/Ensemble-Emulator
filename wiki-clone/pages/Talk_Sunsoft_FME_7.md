# Talk:Sunsoft FME-7

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ASunsoft_FME-7) | View [other pages](Special_AllPages.xhtml#Talk_Sunsoft_FME_7)

2048KB of PRG ROM and 512KB of PRG RAM? The FME-7 only has 44 pins (2 of which are unconnected on NES-BTR), and the existing pins only permit up to 512KB of PRG ROM. Just because the data bus is 8 bits wide does **not** mean that the registers themselves are that wide, and there's also no guarantee that such amounts of PRG RAM are also supported - for example, the MMC3 PRG bank registers are only 6 bits wide, leaving the chip physically incapable of addressing more than 512KB of data, and while the MMC5 can address 1024KB of PRG ROM, it cannot support more than 64KB of PRG RAM. I'm only pointing this out because I don't appreciate my emulator being listed with a bunch of potentially erroneous compatibility errors. --[Quietust](User_Quietust.xhtml "User:Quietust") 17:26, 17 April 2011 (UTC) 

I meant no offense Quietust. I suppose I need to make clear that the extended ROM and RAM amounts are only for the NESDEV1 version of FME-7. I will change the page to reflect this. Nintendulator is 100% compatible with the real FME-7. --[qbradq](User_Qbradq.xhtml "User:Qbradq") 19:45, 17 April 2011 (UTC) 

## Contents

  * 1 PRG RAM on the original mapper
  * 2 Sunsoft vs. FDS
  * 3 NESDEV1
  * 4 Disch Notes



## PRG RAM on the original mapper

When PRG RAM is swapped into $6000, what does the original hardware put on the PRG bank lines? --[Tepples](User_Tepples.xhtml "User:Tepples") 23:24, 20 April 2011 (UTC) 

## Sunsoft vs. FDS

This may sound funny, but it looks like FDS IRQs may be possible (with modifications) with the Sunsoft Mapper... 

And Also the thing with ROM at $6000... It seems possible that this was meant for porting FDS using traditional ROM in place of PRG-RAM. 

Can anyone help me test these to verify? --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") 08:20, 18 August 2012 (MDT) 

    FME-7 might be a sensible choice for porting FDS. I doubt that it was designed with this in mind, though. There is ROM at $6000 because it gives you more addressable space. There is a timer IRQ because timers are useful. I don't think you need any more justification than that. There weren't any FDS games ported to FME-7. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 16:51, 18 August 2012 (MDT)

## NESDEV1

Qbradq disappeared from the forums close to a year ago. I'm tempted to remove most of the details about the nesdev1. --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 19:46, 18 August 2012 (MDT) 

    Personally I don't see why there should be any info on the wiki about a proposed cartridge that's never had a public release/production. It benefits nobody and is only a source of confusion. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 00:54, 19 August 2012 (MDT)

## Disch Notes

Any new correct information from the Disch Notes should be merged into the article, and the disch notes should be deleted. There's not much there that I can see, but here are two tidbits that I don't know are accurate, that should either be added to the article, or discarded: 

  * Note about open bus behaviour w.r.t. command $8
  * Are PRG select registers 8 bit? (commands $9-B)



Other than this, I think the IRQ section could use a little more explanation, but otherwise I don't see anything of additional value in the disch notes. \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:41, 1 March 2015 (MST) 

    PRG banks are 5 bits on 5a and 5b, and 6 bits on FME-7. But no games have more than 256 KiB PRG anyway. The open bus behavior is buried in our description of Reg:8 but probably should be made a little more explicit. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:10, 1 March 2015 (MST)

    

    Okay, I think I've amended anything that was left, and trimmed the redundant disch notes. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 08:47, 2 March 2015 (MST)
