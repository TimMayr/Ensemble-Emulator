# Talk:VRC6

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AVRC6) | View [other pages](Special_AllPages.xhtml#Talk_VRC6)

## Contents

  * 1 Extended VRC6 PPU banking modes
  * 2 Raw data
  * 3 Which emulator support?
  * 4 SRAM at $6000-$7FFF ?
  * 5 VRC6 and $B003 bit 5



## Extended VRC6 PPU banking modes

The VRC6 documentation (currently linked from [here](http://forums.nesdev.org/viewtopic.php?f=2&t=10611)) seems to indicate that register $B003 has quite a lot more bits than we currently know about: 

  * D0/D1: CHR/NT Bank mode 
    * mode 0 - normal
    * mode 1 - regs 0-3 select 2KB CHR banks, regs 4-7 select nametable banks
    * mode 2 - regs 0-3 select 1KB CHR banks at 0000-0FFF, regs 4-5 select 2KB CHR banks at 1000-1FFF, regs 6-7 select nametable banks (controlled by mirroring)
    * mode 3 - pattern tables seemingly also get mapped into the nametables?
  * D2/D3: mirroring control
  * D4: 0 = Use extra 8KB CHR RAM for nametables (in bank modes 1-3); 1 = Use CHR ROM for nametables
  * D5: something related to CHR ROM?
  * D7: SRAM enable (1 = enable)



\--[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 09:07, 23 October 2013 (MDT) 

    That's funny- I noticed something related when I was reformatting up the [VRC6 pinout](VRC6_pinout.xhtml "VRC6 pinout") page, pin 32.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:54, 23 October 2013 (MDT)

## Raw data

Here's the raw data I received from BootGod that I used to determine the exact banking pattern of the VRC6: 
    
    
    b003 0000 0400 0800 0c00 1000 1400 1800 1c00 2000 2400 2800 2c00
    00   81   88   90   99   A0   A9   B1   B8   71   71   78   78
    01   81   81   88   88   90   90   99   99   60   69   71   78
    02   81   88   90   99   A0   A0   A9   A9   71   78   71   78
    03   81   88   90   99   A0   A0   A9   A9   71   78   71   78
    04   81   88   90   99   A0   A9   B1   B8   71   78   71   78
    05   81   81   88   88   90   90   99   99   60   69   71   78
    06   81   88   90   99   A0   A0   A9   A9   71   71   78   78
    07   81   88   90   99   A0   A0   A9   A9   71   71   78   78
    08   81   88   90   99   A0   A9   B1   B8   71   71   78   78
    09   81   81   88   88   90   90   99   99   60   69   71   78
    0A   81   88   90   99   A0   A0   A9   A9   71   78   71   78
    0B   81   88   90   99   A0   A0   A9   A9   71   78   71   78
    0C   81   88   90   99   A0   A9   B1   B8   71   78   71   78
    0D   81   81   88   88   90   90   99   99   60   69   71   78
    0E   81   88   90   99   A0   A0   A9   A9   71   71   78   78
    0F   81   88   90   99   A0   A0   A9   A9   71   71   78   78
    10   81   88   90   99   A0   A9   B1   B8   B1   B1   B8   B8
    11   81   81   88   88   90   90   99   99   A0   A9   B1   B8
    12   81   88   90   99   A0   A0   A9   A9   B1   B8   B1   B8
    13   81   88   90   99   A0   A0   A9   A9   B1   B8   B1   B8
    14   81   88   90   99   A0   A9   B1   B8   B1   B8   B1   B8
    15   81   81   88   88   90   90   99   99   A0   A9   B1   B8
    16   81   88   90   99   A0   A0   A9   A9   B1   B1   B8   B8
    17   81   88   90   99   A0   A0   A9   A9   B1   B1   B8   B8
    18   81   88   90   99   A0   A9   B1   B8   B1   B1   B8   B8
    19   81   81   88   88   90   90   99   99   A0   A9   B1   B8
    1A   81   88   90   99   A0   A0   A9   A9   B1   B8   B1   B8
    1B   81   88   90   99   A0   A0   A9   A9   B1   B8   B1   B8
    1C   81   88   90   99   A0   A9   B1   B8   B1   B8   B1   B8
    1D   81   81   88   88   90   90   99   99   A0   A9   B1   B8
    1E   81   88   90   99   A0   A0   A9   A9   B1   B1   B8   B8
    1F   81   88   90   99   A0   A0   A9   A9   B1   B1   B8   B8
    20   81   88   90   99   A0   A9   B1   B8   70   71   78   79
    21   80   81   88   89   90   91   98   99   60   69   71   78
    22   81   88   90   99   A0   A1   A8   A9   71   78   71   78
    23   81   88   90   99   A0   A1   A8   A9   70   78   71   79
    24   81   88   90   99   A0   A9   B1   B8   70   78   71   79
    25   80   81   88   89   90   91   98   99   60   69   71   78
    26   81   88   90   99   A0   A1   A8   A9   71   71   78   78
    27   81   88   90   99   A0   A1   A8   A9   70   71   78   79
    28   81   88   90   99   A0   A9   B1   B8   70   70   78   78
    29   80   81   88   89   90   91   98   99   60   69   71   78
    2A   81   88   90   99   A0   A1   A8   A9   71   78   71   78
    2B   81   88   90   99   A0   A1   A8   A9   71   79   71   79
    2C   81   88   90   99   A0   A9   B1   B8   71   79   71   79
    2D   80   81   88   89   90   91   98   99   60   69   71   78
    2E   81   88   90   99   A0   A1   A8   A9   71   71   78   78
    2F   81   88   90   99   A0   A1   A8   A9   70   70   78   78
    30   81   88   90   99   A0   A9   B1   B8   B0   B1   B8   B9
    31   80   81   88   89   90   91   98   99   A0   A9   B1   B8
    32   81   88   90   99   A0   A1   A8   A9   B1   B8   B1   B8
    33   81   88   90   99   A0   A1   A8   A9   B0   B8   B1   B9
    34   81   88   90   99   A0   A9   B1   B8   B0   B8   B1   B9
    35   80   81   88   89   90   91   98   99   A0   A9   B1   B8
    36   81   88   90   99   A0   A1   A8   A9   B1   B1   B8   B8
    37   81   88   90   99   A0   A1   A8   A9   B0   B1   B8   B9
    38   81   88   90   99   A0   A9   B1   B8   B0   B0   B8   B8
    39   80   81   88   89   90   91   98   99   A0   A9   B1   B8
    3A   81   88   90   99   A0   A1   A8   A9   B1   B8   B1   B8
    3B   81   88   90   99   A0   A1   A8   A9   B1   B9   B1   B9
    3C   81   88   90   99   A0   A9   B1   B8   B1   B9   B1   B9
    3D   80   81   88   89   90   91   98   99   A0   A9   B1   B8
    3E   81   88   90   99   A0   A1   A8   A9   B1   B1   B8   B8
    3F   81   88   90   99   A0   A1   A8   A9   B0   B0   B8   B8
    

The registers from D000 through E003 contained the pattern 01/08/10/19/20/29/31/38. 

The $40s bit in this table is the CHRROM /OE. The $80s bit in this table is the CIRAM /OE. 

(Thus, values from $80-$BF are from ROM and values from $40-$7F are from RAM. Other values are not present (fortunately) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:57, 23 January 2014 (MST) 

    If this table is correct, "For the pattern tables, when the $20s bit of $B003 is set, 2 KiB banks pass PPU A10 through (limiting the register to seven bits wide by ignoring the LSB)" is wrong; it doesn't ignore the LSB of the register. (I have written a QBASIC program to follow the rules mentioned in the article, and it matches this table only if you ignore that rule.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 03:07, 10 February 2014 (MST)

    

    Please point out which line(s) you feel contradict(s) the assertion of the article—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:13, 10 February 2014 (MST)

The following QBASIC program gives output matching the table above: 
    
    
    DEFINT A-Z
    DATA &H01,&H08,&H10,&H19,&H20,&H29,&H31,&H38
    DIM R(0 TO 7), O(0 TO 11)
    DEF FNHEX$ (X) = RIGHT$("0" + HEX$(X), 2) + "   "
    
    FOR I = 0 TO 7
     READ R(I)
    NEXT I
    
    OPEN "VRC6BANK.OUT" FOR OUTPUT AS #1
    PRINT #1, "b003 0000 0400 0800 0c00 1000 1400 1800 1c00 2000 2400 2800 2c00   "
    FOR V = 0 TO &H3F
     SELECT CASE V AND 7
      CASE 0, 4
       FOR I = 0 TO 7
        O(I) = R(I)
       NEXT I
       IF V AND 4 THEN
        O(8) = R(6)
        O(9) = R(7)
        O(10) = R(6)
        O(11) = R(7)
       ELSE
        O(8) = R(6)
        O(9) = R(6)
        O(10) = R(7)
        O(11) = R(7)
       END IF
      CASE 1, 5
       O(0) = R(0)
       O(1) = R(0)
       O(2) = R(1)
       O(3) = R(1)
       O(4) = R(2)
       O(5) = R(2)
       O(6) = R(3)
       O(7) = R(3)
       O(8) = R(4)
       O(9) = R(5)
       O(10) = R(6)
       O(11) = R(7)
      CASE 2, 3
       O(0) = R(0)
       O(1) = R(1)
       O(2) = R(2)
       O(3) = R(3)
       O(4) = R(4)
       O(5) = R(4)
       O(6) = R(5)
       O(7) = R(5)
       O(8) = R(6)
       O(9) = R(7)
       O(10) = R(6)
       O(11) = R(7)
      CASE 6, 7
       O(0) = R(0)
       O(1) = R(1)
       O(2) = R(2)
       O(3) = R(3)
       O(4) = R(4)
       O(5) = R(4)
       O(6) = R(5)
       O(7) = R(5)
       O(8) = R(6)
       O(9) = R(6)
       O(10) = R(7)
       O(11) = R(7)
     END SELECT
     FOR I = 0 TO 7
      O(I) = O(I) OR &H80
      ' *** The following line is the one I mean ***
      IF V AND &H20 AND ((V AND 3) = 1 OR (I > 3 AND (V AND 2) = 2)) THEN O(I) = (O(I) AND &HFE) OR (I AND 1)
     NEXT I
     FOR I = 8 TO 11
      IF V AND &H10 THEN O(I) = O(I) OR &H80 ELSE O(I) = O(I) OR &H40
     NEXT I
     IF V AND &H20 THEN
      FOR I = 8 TO 11
       SELECT CASE V AND 15
        CASE 0, 7
         O(I) = O(I) AND &HFE
         IF I AND 1 THEN O(I) = O(I) OR 1
        CASE 4, 3
         O(I) = O(I) AND &HFE
         IF I AND 2 THEN O(I) = O(I) OR 1
        CASE 8, 15
         O(I) = O(I) AND &HFE
        CASE 12, 11
         O(I) = O(I) OR 1
       END SELECT
      NEXT I
     END IF
     PRINT #1, FNHEX$(V);
     FOR I = 0 TO 11
      PRINT #1, FNHEX$(O(I));
     NEXT I
     PRINT #1,
    NEXT V
    CLOSE
    

But maybe I have made another mistake in this program, which I haven't noticed. It seems to be the same as meaning, it only passes PPU A10 through if the register is used for multiple 1K banks. Perhaps this is what is actually meant by the article text, although it is a bit unclear (maybe it should say, "2 KiB banks (i.e. if one register is used twice) pass PPU A10 through", but I wait for your advice on this). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:35, 10 February 2014 (MST) 

## Which emulator support?

Can you please tell me which emulator support all of the features of the $B003 register of VRC6? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 17:50, 9 February 2014 (MST) 

    None (as of the time I signed this). Having a test ROM is probably a prerequisite. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:58, 9 February 2014 (MST) 

    Someone who has a VRC6 cartridge could try to do it, perhaps. (I may be able to write a test ROM myself, but it won't be useful (at least at first) since I have no way of testing it, although perhaps the results could be compared with the table above as a kind of partial testing.) Furthermore, what are PPU addresses $3xxx mapped to on VRC6? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 02:10, 10 February 2014 (MST) 

    There's no justifiable reason to care what PPU addresses $3xxx are mapped to, and so we did not measure. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:13, 10 February 2014 (MST) 

    I am one that does care. It would be necessary, if someone tries to write a program to access $3000 in PPU, and then you have to know how to emulate such a thing (maybe it is the same as $2xxx, which is what it is when no extra logic is controlling it, but I don't know). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:35, 10 February 2014 (MST)
    I just the $b003 features to bizhawk, but given how weird they are, and the fact that there are no ROMs that actually used them, I couldn't say that they work for sure without a test ROM. [Natt](User_Natt.xhtml "User:Natt") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Natt&action=edit&redlink=1 "User talk:Natt \(page does not exist\)")) 11:53, 15 February 2014 (MST) 

    The implementation at [[1]](http://code.google.com/p/bizhawk/source/browse/trunk/quicknes/nes_emu/Mapper_Vrc6.cpp) appears to be incomplete. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 13:19, 15 February 2014 (MST) 

    That's for the quicknes core, which we're going to offer as a speedier (but less accurate) alternate. The Bizhawk Original core is the one I was referring to: [[2]](http://code.google.com/p/bizhawk/source/browse/trunk/BizHawk.Emulation.Cores/Consoles/Nintendo/NES/Boards/VRC6.cs) [Natt](User_Natt.xhtml "User:Natt") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Natt&action=edit&redlink=1 "User talk:Natt \(page does not exist\)")) 15:20, 15 February 2014 (MST) 

    Ah, OK, thank you, I didn't know that. It looks OK to me (but I don't know C# programming very well). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:59, 15 February 2014 (MST)
    Here's a test rom. Bizhawk (latest SVN) passes it, FCEUX fails predictably. It shouldn't be considered accurate until it's been run on actual VRC6 hardware. Also, it's the first thing I've ever written for NES, so who knows what's broken on it. [[3]](https://www.mediafire.com/?6hvuj53omv8y3fn) [Natt](User_Natt.xhtml "User:Natt") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Natt&action=edit&redlink=1 "User talk:Natt \(page does not exist\)")) 13:57, 16 February 2014 (MST)

## SRAM at $6000-$7FFF ?

The document makes a reference to a PRG RAM enable flag at bit 7 of register $B003. Perhaps an entry for it should be added in the Banks section. At least one game (Madara) seems to rely on reading/writing to this region to maintain save points, so it would be reasonable to conclude that this region is not volatile PRG RAM but SRAM. It is also unclear what happens with SRAM when PRG RAM flag is not set - are writes to SRAM ignored and reads return open bus? — Preceding unsigned comment added by [Colinvella](https://www.nesdev.org/w/index.php?title=User:Colinvella&action=edit&redlink=1 "User:Colinvella \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Colinvella&action=edit&redlink=1 "User talk:Colinvella \(page does not exist\)") • ~~contribs~~) 

    Plain iNES1 headers cannot specify whether a game has PRG RAM that is _not_ battery backed. And unfortunately NesCartDB is currently down so I can't just say "X% of games have no PRG RAM, Y% have PRG RAM and no battery, Z% have at least one PRG RAM with a battery"...
    PRG RAM that is not battery backed is just treated like the in-NES RAM: stays across warm boots, lost on a power cycle.
    In any case, _for the VRC6_ , the two m26 games have battery-backed PRG RAM, and Akumajou Dracula has none. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:42, 30 July 2016 (MDT)

    

    NEScartDB's contents indicate: Out of 1382 total games, 16.6% have a battery and 4.4% have external PRG RAM with no battery. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:07, 25 October 2016 (MDT)

## VRC6 and $B003 bit 5

In the edit history comments: 

the specific banks used when bit 5 is clear are a function of banks 6 and 7, so you may end up with one-screen or "backwards" vertical instead, also - Lidnariq 

    Oh! I misunderstood when I said vertical was forced, I mistakenly thought PPU A10 was passed, not the internal register LSB. Isn't it arbitrary then, not just vertical/1-screen? - Rainwarrior 

    Not fully arbitrary. If we're just talking about the standard 8 1 KiB banks mode (i.e. ([$B003] & 3) == 0), then you can get 1scA, 1scB, H, V, "backwards" H, and "backwards" V, depending on [$B003] & 4 and the specific lsbits of R6 and R7.
    Full arbitrary mirroring control only comes from using the modes where ([$B003] & 3) == 1 —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:29, 10 March 2018 (MST)

    

    

    So: arbitrary 2-nametable arrangements only in the most wasteful of all modes, where half your CHR is duplicated 1k pages? Would there ever be a good reason to leave bit 5 clear, or is it basically useless? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:45, 10 March 2018 (MST) 

    Yeah, on the PCBs as designed, I honestly cannot figure out when one would ever leave bit 5 clear. I have to assume that the PCB of a game that used "mode $01" would have connected PPU A10 directly to the CHR. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:13, 10 March 2018 (MST)
    Bit 5 is left clear and used with Mode 1 when using 512 KiB CHR-ROM, by connecting VRC6's A10-A17 to CHR-ROM's A11-A18 and PPU's A10 to CHR-ROM's A10. There is no wasting of space for normal pattern data in this arrangement, but you are wasting half of each 2 KiB bank if you are using ROM nametables, as the last page of the Japanese VRC VI data sheet indicates. The current test ROM does not test that way of connecting the chip and thus may lead to false conclusions. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 19:39, 10 March 2018 (MST) 

    Is the datasheet available somewhere? The first reference link on this article is now dead, unfortunately. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:43, 10 March 2018 (MST)
    I try to use [the wayback machine](https://web.archive.org/web/20160706191925/http://assemblergames.com/l/threads/several-famicom-nes-misc-dev-documents-from-nintendo-and-konami.48390/) but while it has the forum thread archived, all of the documents are dead hotfile links. :( - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:46, 10 March 2018 (MST)
