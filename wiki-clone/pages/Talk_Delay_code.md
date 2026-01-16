# Talk:Delay code

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ADelay_code) | View [other pages](Special_AllPages.xhtml#Talk_Delay_code)

## Jxx pseudo-instructions?

I'm noticing a bunch of branching pseudo-instructions starting with J instead of B. I presume this refers to an inverted branch across a JMP instruction, but given that this is tutorial material, shouldn't only real instructions be used? This code seems to be dependent on custom macros? ca65 does come with ".macpack longbranch", but they don't use a capital J as far as I understand, and ASM6 doesn't have anything of the sort packaged with it, does it? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:43, 16 March 2016 (MDT) 

    Okay, I just found the definitions at [Fixed cycle delay](Fixed_cycle_delay.xhtml "Fixed cycle delay") (left there by mistake?), which makes me understand that they're really just the straight branch with a page check. I think this is really bad nomenclature because of the already existing longbranch conventions which have the same names but different function. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:13, 16 March 2016 (MDT)

    

    Sorry about the confusion. I'm not sure where I adopted these macros from. But that's what they do, branch with page-wrap check. The longbranch functions are named similarly, but with a lowercase j, not a capital J. --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 22:22, 17 March 2016 (MDT) 

    Would bccnw, beqnw, etc. describe the intent better? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:55, 20 March 2016 (MDT) 

    Maybe. Should I change it? --[Bisqwit](User_Bisqwit.xhtml "User:Bisqwit") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Bisqwit&action=edit&redlink=1 "User talk:Bisqwit \(page does not exist\)")) 12:35, 21 March 2016 (MDT) 

    I think it would be a good idea to change them, yes. I don't have much opinion about what to use ("nw" seems as good as many), just I don't like the current similarity to the longbranch macros. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 12:23, 24 March 2016 (MDT)
