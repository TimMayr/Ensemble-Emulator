# Talk:Mask ROM pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMask_ROM_pinout) | View [other pages](Special_AllPages.xhtml#Talk_Mask_ROM_pinout)

## Contents

  * 1 SLxROM, TLxROM
  * 2 Nintendo SROM CHR ROM pinout - 8 KBytes (24pin)
  * 3 Higher CHR lines
  * 4 MMC5 pin 31



## SLxROM, TLxROM

Does anyone have the pinout for SLxROM and TLxROM boards ? Such boards were apparently rarely/never used in PAL games so I can't do much about it, unless I pay more than $30 to import USA games. —[User:Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:34, 08 February 2014 (MST) 

    The pictures on NesCartDB show the SL3ROM, SL1ROM, and TL1ROM boards are using ordinary 28-pin 128KiB mask ROMs for CHR, combining PPU A13 and PPU /RD with a 74'32 (*-TL1ROM, NES-SL1ROM, NES-SL3ROM) or a one-transistor A≤B gate (HVC-SL1ROM).
    The pictures on NesCartDB of SLRROM and SL2ROM similarly only indicate parts with typical pinouts: they have a standard 32-pin 128KiB mask ROM for CHR.
    Only one copy of game, without photos, claims to have a TL2ROM board, which I'm going to declare a typo until further evidence arrives. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:57, 8 February 2014 (MST)

    

    When you say "standard", does it mean JEDEC, or standards in regard to Nintendo's habits ?[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 15:07, 8 February 2014 (MST)

    

    

    The 28-pin ROM is the standard 28-pin 128KiB mask ROM pinout (JEDEC 21-C, p3.2.1-6 which specifies mask ROM pinout of 28-pin ROMs from 8KiB to 128KiB). The 32-pin CHRROMs look to be Nintendo's standard 32-pin layout, since at least one IC, µPD23C1013, is used as CHR on both SLRROM and TLROM. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:17, 8 February 2014 (MST)

  


## Nintendo SROM CHR ROM pinout - 8 KBytes (24pin)

The pinout of this 64Kb (8KB) ROM is the same as the Motorola MCM68764/MCM68766 EPROM. Enjoy! - RJ @ arcadecomponents.com 

## Higher CHR lines

Apparently, the unused high address lines of PRG and CHR mask ROMs must be connected to VCC, but this was vital only on PRG ROMs as the CHR-ROM are not required to be enabled for booting, before the first mapper write. So, does games which don't use all their available CHR-space uses '1' for the unused bits ? For instance, a 32kb CHR-ROM MMC1 game should use either CHR bank range 8-15 or 24-31 instead of using 0-7 which would happen naturally, in order to set A15 to '1'. Does commercial games do this ?—[User:Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:34, 08 February 2014 (MST) 

    No, most commercial games more often use the lowest mirror (although sometimes a random mirror) of undersized ROMs. I'm not clear where the "must be" comment came from. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:57, 8 February 2014 (MST)

I think it came from me. It's a known fact Nintendo used different boards for different PRG sizes of compatible pinout (16k, 32k, 64k - respectively 128, 256 and 512 kbits). The only difference between these boards is that higher address lines that goes unused on smaller sizes are hard-wired to VCC, instead of being connected to the mapper. If the pins were unused on the mask ROMs, then it wouldn't matter to connect them on the mapper, so they could just use the biggest sized board for everything (i.e. NROM-256 instead of NROM-128, SBROM instead of SEROM, SAROM instead of SIROM, etc, etc...). CNROM boards also have a solder pad to choose between connecting A14 and forcing it to VCC (resulting in a 16k PRG-ROM). 

It's clear that designing and producing more different boards with no reason makes few sense, so they probably had a reason for forcing unused higher address lines to VCC. My theory is that it could in some cases be either an enable signal, or a higher address lines, as they could use larger mask ROM, and program only the upper half/quarter and tie the line to VCC in order to mirror it to the whole range (effectively making it almost the same as an enable signal, bus conflicts aside). [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 15:08, 8 February 2014 (MST) 

  


## MMC5 pin 31

It is: 
    
    
     A18    /CE        A18 [/CE]      A16 - |02   31| - A18 [PGM]    +5V    /CE    /CE
    

but shouldn't be?: 
    
    
     A18    /CE        A18 [/CE]      A16 - |02   31| - A18 [PGM]    +5V    /CE    /OE
    

\--[Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 19:42, 31 October 2017 (MDT) 
