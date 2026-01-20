# Talk:INES Mapper 219

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_219) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_219)

I've just implemented this mapper in the hardware CPLD (tested on game Toy Story (Unl). I think the description in Wiki is quite inaccurate. Here are missing elements. 

1\. There is no $8000 Select Register 1 / $8002 Select Register 2 with values in the table as you have given. There is only one Select Register, which is mirrored at $8000 and $8002. Implementing it that way works, while implementing in way you put on wiki does not work. Toy Story writes #$26 at $8002, later #$25 at $8002 which does not agree with your definition Here is proof: [url=<http://obrazki.elektroda.pl/1955748400_1492435497.png>][img]<http://obrazki.elektroda.pl/1955748400_1492435497_thumb.jpg>[/img][/url] 

2\. All PRG registers are initialized at startup at $F. Initializing at $0 fails to load the game 

3\. Mirroring register ($A000-$BFFE) is also present and functions the same way like in MMC3. 
