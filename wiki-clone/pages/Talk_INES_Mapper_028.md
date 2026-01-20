# Talk:INES Mapper 028

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_028) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_028)

## Miscellany

Does this mapper use RAM at $6000-$7FFF? PRG RAM? Whatever? --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 06:15, 18 September 2013 (MDT) 

    It supports the same [PRG RAM at $6000-$7FFF](PRG_RAM_circuit.xhtml "PRG RAM circuit") that the iNES format implies for most discrete mappers. There's no disable register, if that's what you meant. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 12:47, 18 September 2013 (MDT)

Are both the inner and outer PRG regs always used in PRG swaps? --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 18:44, 21 September 2013 (MDT) 

    Both registers are used to form the final 16K bank number unless you're in P=G=0 mode, in which case only the outer bank is used. But if you're just switching banks within one game, you're probably writing only to the inner PRG register. Let me make an analogy to a possibly more familiar mapper, [MMC1](MMC1.xhtml "MMC1"): Consider the inner PRG register like $E000 and the outer like bit 4 of $A000 on SUROM/SXROM. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 12:57, 22 September 2013 (MDT)

Yes, but there's something weird for me. While the PRG diagram shows $8000-$BFFF to use R:01, _and_ $C000-$FFFF to use R:81, I think such PRG bank selection would be _separated_. Later, there's an info regarding **ORing both PRG registers** $01 and $81 (and making shift and ANDs) _to compose_ the desired PRG page. Could someone clarify it? --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 18:56, 24 September 2013 (MDT) 

## DRY failure

Apropos of the edit log for the page itself, I'm pretty certain this page's existence is a DRY failure. Why do we have documentation for the same thing in two places? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:50, 2 April 2015 (MDT) 

    Quite. Why do we *still* have this documentation for the same thing in two places? Should probably redirect to [Action 53 mapper](Action_53_mapper.xhtml "Action 53 mapper")… [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 05:40, 11 February 2017 (MST) 

    I have now redirected it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 07:55, 11 February 2017 (MST)
