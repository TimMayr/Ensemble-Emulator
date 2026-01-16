# Talk:INES Mapper 218

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_218) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_218)

## Alternate PPU memory maps

I can make a suggestion of the variant of mapper 218 using [NES 2.0](NES_2_0.xhtml "NES 2.0") header. If the header specifies neither CHR ROM nor CHR RAM, then it acts as described in this document. If it does specify either CHR ROM or CHR RAM (only one or the other is allowed, not both; RAM can optionally be battery-backed (but you can't combine battery with non-battery)), then it is mirrored across 16K, and the mirroring flags control how many positions to shift the high bit of the address (if vertical mirroring is specified, this is the ordinary memory mirroring; this also means specifying 2K non-battery RAM is the same as using CIRAM, regardless of mirroring setting, and that specifying 1K non-battery RAM and vertical mirroring acts like using CIRAM but connecting CIRAM A10 to ground). This is no longer a single chip cartridge, but it does seem to be what would be a variant of this iNES mapper nevertheless. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:55, 30 December 2013 (MST) 

    I cannot fathom the use of variants that map any size of ROM over the entire PPU address space. The background would never be anything more than a static 512x480 (or smaller) picture.
    Otherwise, I think you're describing the following six new modes: 

  1. 4KiB RAM, RAM A10,11 = PPU A10,A11 : Nametables and both pattern tables all from the same RAM. Technically allows four screen layout, but not in a particularly useful way
  2. 4KiB RAM, RAM A10,11 = PPU A11,A12 : Each pattern table gets 2KiB, but the left pattern table is shared with the nametables (which are equivalently in horizontal mirroring)
  3. 4KiB RAM, RAM A10,11 = PPU A12,A13 : 1-screen mirroring, 1KiB for each pattern table
  4. 8KiB RAM, RAM A10,11,12 = PPU A10,11,12 : Four-screen layout, but left pattern table is more or less unusable
  5. 8KiB RAM, RAM A10,11,12 = PPU A11,12,13 : Each pattern table gets 2KiB and the nametables are in horizontal mirroring
  6. 16KiB RAM, all connected : traditional 8KiB CHRRAM + 4 screen layout.


    The reason, as I understand it, that m218 is interesting is that it lets you save on populating one IC, so that you only need PRG ROM and maybe the CIC. One you have to add an extra IC, you may as well add it in a useful layout; therefore an 8KiB RAM for patterns with the NES's CIRAM for nametables is strictly more useful than variants 2, 3, and 5. Variants 1 and 4 could have some merit, but only because there's no established variant that uses 10KiB of RAM for anything other than (8KiB pattern + 2KiB NT) or [mapper 77](INES_Mapper_077.xhtml "INES Mapper 077"). Variant 6 is already the default behavior for all mappers than have CHRRAM and 4-screen layout specified. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:55, 30 December 2013 (MST)

    

    You are correct I agree that mapper 218 is interesting since PRG ROM is the only IC needed (other than CIC, if necessary); however, there is a potential interest for the variant I mentioned, which seems like it would be a reasonable thing for mapper 218 to do if the NES 2.0 header specifies the existence of CHR-ROM or CHR-RAM (regardless of how useful it might be). However, full area CHR-ROM might not be entirely useless; maybe it can be useful for some kind of very simple game that uses only a scrolling static background and sprites (one example of such a game might be Pong). The 16KiB RAM can of course also be useful (if you don't want mirroring); doing this also gives you some extra RAM (usable only when not rendering), and if it is battery-backed you might use it for save game data. Of course what is useful depends much on the program. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 19:29, 31 December 2013 (MST)
