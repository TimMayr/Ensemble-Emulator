# Talk:Tile compression

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ATile_compression) | View [other pages](Special_AllPages.xhtml#Talk_Tile_compression)

## Contents

  * 1 Citing external sources for format details
  * 2 Non-NES compression algorithms
  * 3 NES Stripe Image RLE
  * 4 Dr.Halo and Psycopathicteen Tile Compressor



## Citing external sources for format details

On 2016-05-02, Bisqwit made [this edit](http://wiki.nesdev.org/w/index.php?title=Tile_compression&diff=12294&oldid=12293) to _" Spell out the format. Depending on external links for textual content is bad wiki practice."_ But I seem to remember about a year earlier, when I was making [these edits](http://wiki.nesdev.org/w/index.php?title=Tile_compression&diff=10971&oldid=10970) about LZSS as used in GBA/DS BIOS, that someone was confused about the scope of this page, whether I was making a complete format reference or not, especially for formats not used on NES. This "someone" may have been on forums.nesdev.org or in #nesdev on EFnet; I don't remember exactly. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 07:24, 4 May 2016 (MDT) 

    I think it was me, and I believe my confusion was that comprehending the stuff on the wiki page depended on information behind the link (which already fully described it), which I felt made the wiki material either useless or redundant (depending on whether you'd read the linked page). I believe I was in favour of just linking it with a brief summary instead, but revising it to have fully formed ideas was an acceptable alternative. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 12:21, 4 May 2016 (MDT)

## Non-NES compression algorithms

Details about compression used in commercial games for another platform other than NES has nothing to do there, in my opinion. Only designs which are useful for either homebrew development or romhacking existing games or using them as inspiration for homebrew is useful. ~~178.196.73.155~~ 14:25, 20 August 2016 (MDT) 

    Why would explaining how a compression format for another CPU of comparable computational ability _not_ belong here? We're not describing LZMA. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:12, 20 August 2016 (MDT)

    Compression formats are separated from the CPU encoding or decoding them, so there is no problem describing any format. However, explaining the details how a particular game compress its data do, in my opinion, not belong here, but to the data crystal romhacking wiki.

[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 13:31, 22 August 2016 (MDT) 

    I should think that the "caution" note about GBA/DS video memory doesn't really belong here, because it is not applicable to NES/Famicom, even if the same format may be used. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 16:22, 22 August 2016 (MDT)

## NES Stripe Image RLE

Do games use NES Stripe Image RLE to update CHR RAM? If so, how do they distinguish $0000-$00FF (tiles 0-15 of the first pattern table) from an end marker? Do they use the mirror at $4000-$40FF? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 15:07, 4 September 2016 (MDT) 

The End Marker in the NES version is zero because that is the simplest approach to do it in 6502, using $FF would require a gigantic hack, and even then there is no DMA except for sprites on NES, so you may have to rewrite the Stripe Image Uploader to use the $FF value 

The SNES version uses the 65816, and it is more complex than the 6502, but comes from the same family, just 16-bit. 

The Stripe Image Uploader only goes to VRAM $0000-$3FFF on NES, unless you want glitching. 

The only time CHR-RAM is rendered from the Stripe Image Format is during the ending of Doki Doki Panic (FDS). 

Super Mario Bros 2 (USA) replaces that with OAM animation instead since it is using CHR-ROM, And is easier to implent with CHR-RAM being not an option. 

\--[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 04:08, 5 September 2016 (MDT) 

Or simply put, Make a NES test ROM for CA65, CHR contents can be... 
    
    
    .dbyt $1000 ;Address of CHR RAM 
    .byte $10   ;Length is 16, or 1 tile
    .word %0011110000000000
    .word %0101101000100100
    .word %1111111100100100
    .word %1111111100000000
    .word %1111111100000000
    .word %1100001100111100
    .word %0111111000011000
    .word %0011110000000000
    .byte $00 ;end of tile
    

\--[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 04:18, 5 September 2016 (MDT) 

    Given that VRAM addresses range from $0000 to $3FFF, I don't see how it'd be a "gigantic hack" for a decoder to use `bpl` instead of `bne` to tell the difference between the high byte of a VRAM address and the $FF terminator. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 09:25, 5 September 2016 (MDT)

It was only to make things simple for the developers at the time, Using $00 from opcode BNE was a part of saving ROM space, and that is common place back then! --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 14:53, 5 September 2016 (MDT) 

Actually, Once you think about it, Using only opcode BPL would set it to use $FF, I totally forgot about that, I will try to make an example hack to see if it is verified. --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 14:59, 5 September 2016 (MDT) 

I have tried replacing all end terminators with $FF in a copy of SMBDIS, as well as replacing just BNE to BPL, and first it loops 256 times to write PPU data slowishly, Writes a redundant tile afterwards, then freezes, Is there a reason why things went wrong? 

I replaced all data including the code-based writes like: 
    
    
    LDA #$00             ;To turn each of these #$00s into #$FFs and change
    STA VRAM_Buffer1+N,x ;the BNE to BPL would do something bad(TM)
    

where N equals where the terminator is position, and it still isn't working, Do you want the current hack sent to you for correction? 

\--[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 19:24, 6 September 2016 (MDT) 

    No thanks. I was mostly developing my own library to use this format in its VRAM buffer, and I wondered about the rationale. I guess the next step is building a list of games that use it; NESdev BBS or RHDN might be more helpful toward this. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 07:15, 7 September 2016 (MDT)

Is "skip 32 bytes" referring to the increment mode (%00000100 of $2000), just to be perfectly clear? "Skip 32 bytes" isn't really correct, if so… [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 17:01, 22 January 2017 (MST) 

You are correct, That was referring to a 32-byte increment mode that PPUCTRL has, I will correct that! --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 05:34, 23 January 2017 (MST) 

## Dr.Halo and Psycopathicteen Tile Compressor

Dr.Halo compression is similar to PackBits but worse. However, sometimes the compression can be slightly improved compare to the other program. See the do_run() subroutine in [ffcut.c](http://zzo38computer.org/fossil/farbfeld.ui/raw/ffcut.c?name=d85a3a77316354d96948c8226467d25f0b4b837d) for details about this algorithm. Another compression is Psycopathicteen Tile Compressor, which is meant for Super Nintendo though, but I have implemented it based on the other people's idea in the SNES area of NESdev forum. These might not be notable in the actual article, but in this talk page it may be notable. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 21:38, 11 November 2017 (MST) 
