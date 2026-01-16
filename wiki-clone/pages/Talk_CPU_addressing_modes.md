# Talk:CPU addressing modes

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACPU_addressing_modes) | View [other pages](Special_AllPages.xhtml#Talk_CPU_addressing_modes)

## (d),y addressing typo

For (d),y adressing, shouldn't the formula be: 

PEEK(PEEK(arg) + (PEEK((arg + 1) % 256) * 256) + y) 

instead of 

PEEK(PEEK(arg) + PEEK((arg + 1) % 256) + y) 

? 

— Preceding unsigned comment added by [135.23.206.197](https://www.nesdev.org/w/index.php?title=User:135.23.206.197&action=edit&redlink=1 "User:135.23.206.197 \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:135.23.206.197&action=edit&redlink=1 "User talk:135.23.206.197 \(page does not exist\)") • ~~contribs~~) 

    Yes. Amended. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:39, 9 February 2016 (MST)

## Phrasing doesn't match table

The line: 

"The (d**)** ,y mode is used far more often than (d,x)."

Should be modified to match the declaration pattern in the proceeding table, as follows: 

"The (d,y**)** mode is used far more often than (d,x)." \--SirGouki 

    There is no (d,y) mode. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:48, 22 April 2021 (MDT) 

    To clarify, there are two different addressing modes which perform indexing and indirection: one performs indexing first and indirection second (and uses the X register), and the other performs indirection first and indexing second (and uses the Y register). The first one effectively lets you use an array of pointers to individual bytes (which isn't very useful), but the second one is equivalent to having a single pointer to a 256-byte array (which is **extremely** useful). Furthermore, the table doesn't say "(d,y)" as you claim, but says "(d),y" as it should. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 06:58, 23 April 2021 (MDT)
