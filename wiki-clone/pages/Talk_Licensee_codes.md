# Talk:Licensee codes

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ALicensee_codes) | View [other pages](Special_AllPages.xhtml#Talk_Licensee_codes)

## Uncertain codes

Some of the licensee codes are a bit difficult to nail down because they only appear once, the code is plausibly not filled in, the header validation byte is wrong, and/or the game has multiple names on it that the code could plausibly belong to. Game Boy documentation helps in the cases where it matches up with the possibility we see on the NES, and cartridge IDs can provide a hint, but otherwise, there's some uncertainly, sometimes enough that I'm not yet comfortable adding some codes to the list. Of what's already on the list, here are the ones in which I have lower confidence: 

  * $0B - Coconuts Japan: Coconuts is apparently a developer and is pretty consistently paired up with C-Dream as the publisher. The cartridge ID for these games is usually CDS-xx, which suggests C-Dream, but Game Boy documentation has this as Coconuts. On GB, Pachio-kun Game Gallery and Pachio-kun Castle, which appear to be Coconuts-only games, use 0B (2-char) and $0B (8-bit) codes, respectively. Coconuts thus seems more likely. I can't find much information about C-Dream and don't know if it's another branch of a company that owns Coconuts or what.
  * $9E - Face: This only appears in Chiyonofuji no Ooichou (sometimes marked as Sendai no Tomio no Daiginnan), which has both Face and ARC listed. Arc is documented on Game Boy as having code $99, and the cartridge ID is FAC-EJ, so Face seems most likely.
  * $EF - Fujimic: This only appears in Shoukoushi Ceddie, which has Nippon Animation, Fuji Television, and Fujimic listed on the title screen, and was allegedly developed by Graphic Research. Fuji Television is placed prominently on the box, but the cartridge ID is FMI-CD and フジミック (Fujimic) can be seen in the bottom left corner on the back of the box. Fujimic seems the most likely, but with so many names, it's hard to say.



There were several things I left off the list. I'm considering adding some of these in and am open to feedback. These are as follows: 

  * $83: This appears in Tokyo Pachi Slot Adventure. This is another Coconuts / C-Dream game with a CDS cartridge ID. The header is valid, so this is presumably a real code, but I have no idea why it's not $0B and what it could mean.
  * $86: This appears in Puyo Puyo, which lists Tokuma Shoten Intermedia and Compile. This got FDS and Famicom releases, with IDs GTS-PYO and TIM-PY, respectively. Tokuma Shoten games are almost all GTS-xxx. Perhaps this is distinguishing Tokuma Shoten ($C4) and Tokuma Shoten Intermedia ($86), or maybe this is Tokuma Shoten / Compile.
  * $AA: This code is used in many games to mean Victor Musical Industries, and I don't think there's any doubt that that's correct. Mini-Putt, however, uses $AA and appears to have nothing to do with VMI. For Mini Putt, which has cartridge ID WAV-Q6, this likely means A Wave.
  * $B4: This appears in Just Breed. This is an Enix game, apparently developed by Random House. Its cartridge ID is EFC-I5, which is standard for Enix. Random House does not appear on the cartridge, box, or title screen. Maybe this is Random House, or maybe Enix / Random House, but I don't know.
  * $D8: This appears in Mizushima Shinji no Dai Koushien and Street Fighter 2010 (J). The header validation byte is wrong in both of these, but the header data is otherwise filled in correctly (Street Fighter 2010 has no title, but correctly specifies no title). Both of these are Capcom / Status games, and while Capcom games normally have a cartridge ID of CAP-xx, these two are STE-xx (the only two STE games in the library). ステイタス (Status) is featured first before Capcom on the Street Fighter 2010 box under the STE-20 ID. I suspect this code should be either Status or Capcom / Status.
  * $FE: This appears in The Flintstones: The Rescue of Dino & Hoppy (J). This is a Taito game, apparently developed by Sol. Its cartridge ID is TFC-FS, standard for Taito. Sol does not appear on the cartridge, box, or title screen. Maybe this is Sol, or maybe Taito / Sol, I don't know.
  * $FF: This appears in Tashiro Masashi no Princess ga Ippai and The Uncanny X-Men. The former has correct header info (though the title is EFC-TP instead of a real title) and is apparently by Enix / Sony Records. The latter also has correct header info (though it has no title) and is by LJN. The Game Boy documentation claims $FF is LJN, but also has LJN on 8-bit codes $56 and $DB and 2-char code 56. $FF might just be a placeholder.



[Fiskbit](User_Fiskbit.xhtml "User:Fiskbit") ([talk](User_talk_Fiskbit.xhtml "User talk:Fiskbit")) 15:39, 28 March 2022 (UTC) 
