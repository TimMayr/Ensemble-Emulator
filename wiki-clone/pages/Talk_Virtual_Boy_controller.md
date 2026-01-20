# Talk:Virtual Boy controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AVirtual_Boy_controller) | View [other pages](Special_AllPages.xhtml#Talk_Virtual_Boy_controller)

## Endianness

The bits appear to be in the same order as on the Super NES controller, except numbered 15 to 0, with the right Control Pad in place of the Super NES right face buttons, and with the VB's B and A buttons placed in the "signature" area. This got me wondering about the actual endianness. Section 4.1 of David Tucker's manual implies that bit 15 is transmitted _first._ If I'm reading it right, this would allow use of nearly the same code as a Super NES controller in a twin-stick situation like _Smash TV_ or _Spook-o'-tron_ , but detection between a mouse and a VB controller might be a bit trickier. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:03, 20 April 2019 (MDT) 

    Yeah, it was backwards. I was waiting on confirmation from Roth. Also took a look at my dump of Spook-O-Tron to be sure. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:30, 20 April 2019 (MDT)
