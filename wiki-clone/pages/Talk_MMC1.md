# Talk:MMC1

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMMC1) | View [other pages](Special_AllPages.xhtml#Talk_MMC1)

## Contents

  * 1 SOROM, SUROM, and SXROM
  * 2 SXROM Heuristics
  * 3 Resetting the mapper on hardware
  * 4 Battery and resistor question



## SOROM, SUROM, and SXROM

I'm a little fuzzy about why these are considered different [NES 2.0 submappers](NES_2_0_submappers.xhtml "NES 2.0 submappers"). The functionality of all three of these looks the same to me if the extra banking bits become useless/mirror when the ROM/RAM addressed is a smaller size (just like in any other mapper supporting various sizes). 
    
    
    43210
    +---+
    PSSxC
    
    
    
    4:P - A18 of 512k PRG ROM, disconnected if smaller
    3:S - A13 of 16k PRG RAM, disconnected if smaller
    2:S - A14 of 32k PRG RAM, disconnected if smaller
    1:x - unused
    0:C - 4k CHR banking control
    

What am I missing? I can't spot the incompatibility that necessitated three submappers. Isn't this just one mapper together? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:58, 8 August 2015 (MDT) 

Just my $2, but I fully agree with Rainwarrior.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:28, 9 August 2015 (MDT) 

    Yeah, tepples confirmed it in a discussion elsewhere (and lidnariq seemed to confirm it in an oblique way?). I've already put this information to use in the description at [NES 2.0 submappers#001: MMC1](NES_2_0_submappers.xhtml#001:_MMC1 "NES 2.0 submappers") anyway. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:58, 9 August 2015 (MDT)

    The only issue I see is that people seem to like thinking of the two PRG-RAM banking bits in order, which would pose a problem if exchanging SXROM save RAM images is desired. On the other hand, this is a much more elegant way to handle it, so it sgtm. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:09, 10 August 2015 (MDT)

    

    Actually, thinking about this for a moment, this is a far better explanation of why I think we don't need a submapper for SUROM/SXROM/SOROM, by parsing the bits as follows:
    
    
    43210
    +---+
    EDCBA
    
    4:E - CHR A16, if extant; PRG ROM A18, if extant
    3:D - CHR A15, if extant; PRG RAM A13, if extant
    2:C - CHR A14, if extant; PRG RAM A14, if extant
    1:B - CHR A13, if extant
    0:A - CHR A12
    

—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:39, 11 August 2015 (MDT) 

## SXROM Heuristics

Should the section on SOROM/SUROM/SXROM be modified because the amount of PRG-ROM in SXROM need not be "large", as shown with Dezaemon (128K)? [Great Hierophant](https://www.nesdev.org/w/index.php?title=User:Great_Hierophant&action=edit&redlink=1 "User:Great Hierophant \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Great_Hierophant&action=edit&redlink=1 "User talk:Great Hierophant \(page does not exist\)")) 19:41, 1 Oct 2018 (MDT) 

    Yes, it should. Actually this entire paragraph is wrong, because determining SUROM from other MMC1-based configuration is not problematic; just using the good old iNES header marked with 32 "16kb PRG-ROM banks" is enough to detect SUROM; there is nothing problematic. The only problematic thing is accounting for larger RAM (16k or 32k; SOROM or SXROM) because in the original iNES header there's no way to indicate a larger RAM. NES 2.0 header solves this, therefore submapper is unnecessary but a NES 2.0 header is necessary. For games using the old iNES header, the only choice is to either detect games based on CRC (bad approach) or to assume there's always 32kb if no save file is already present; and write only the banks that were actually used to the save file.

[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:42, 2 October 2018 (MDT) 

    The only place the distinction matters is the recommendation of emulating SNROM-style PRG RAM protection with small PRG and CHR. The only other place large PRG matters is for allowing the same bit to control selecting between 256 KiB outer banks, which is a no-op if there aren't multiple 256 KiB outer banks. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:54, 2 October 2018 (MDT)

    I completely forgot about SNROM-style PRG RAM protection. I still think it should be handled individually, i.e. if PRG-ROM < 256k and CHR-RAM is used then this protection is activated, this does not require heuristics nor a NES 2.0 header.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:20, 2 October 2018 (MDT)

    

    I think that whole "heuristic" segment should be axed. The only iNES 2 thing is PRG-RAM size, and even that's not much of an issue. The organization got muddled, ending up in an iNES 2 category because the succinct combined implementation was a reaction to Kevtris' redundant submapper allocations. All this stuff about "8k of hard drive space" etc. seems like extra confusion of the issue, TBH. -[Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 14:18, 2 October 2018 (MDT)

    

    I more or less just removed that whole section, and removed the reference to iNES 2 except for PRG-RAM size. I tried to look over the heuristic formula to see if anything was worth keeping in it, but I didn't find anything in there that didn't seem redundant or unimportant to the more succinct combined register description. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:00, 2 October 2018 (MDT)

    

    You did exactly the right thing IMO.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 07:03, 3 October 2018 (MDT)

## Resetting the mapper on hardware

Hi, 

When using a cartridge that uses multiple banks, you must reset the mapper on console reset (by storing 0x80 into 0x8000). Without doing this, even if manually configuring the banks, the system does not know where to look for the reset vector (since the last bank is NOT fixed to 0xC000). Emulators don't care about this but hardware definitely does. 

Can we get this added to this page? 

  


## Battery and resistor question

_Boards using an MMC1 may contain a battery connected to the PRG RAM's power line to preserve the data. Boards doing so will allow extra circuitry to be used, with 2 diodes and 2 resistors. A diode is needed from both voltage sources: The battery and the NES 5V, so that one cannot supply current to the other, and**there is a resistor in series with the battery so that no current is drained from the battery when 5V is present**._

I always thought the resistor (~1k) is put to measure the uA current consumption of battery for test purposes. Can you explain how it could prevent from using battery when 5V is present? [Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 14:08, 4 June 2019 (MDT) 
