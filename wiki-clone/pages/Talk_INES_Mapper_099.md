# Talk:INES Mapper 099

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_099) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_099)

## Minimal discrete logic clone

A [74138](74138.xhtml "74138"), selecting on M2, A14, A4, and A1 high; R/W and A0 low, and a 7474 (or any larger latch). Maps the write-only register at every address [4-7C-F].[13579BDF][26AE]. Incorrectly decodes writes to $4012 and DualSystem RAM. Using a [74161](74161.xhtml "74161") or any other latch with an additional active-low enable would remove the collision with the DualSystem RAM; however, there's no obvious place to get an extra active-high enable to prevent the collision with $4012. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:15, 31 October 2013 (MDT) 

  * Half of the 7474 can be configured to [act as an inverter](http://forums.nesdev.org/viewtopic.php?p=125760#p125760). In this case, instead of requiring A0 low, we could instead require A2 high, mapping the register at [4-7C-F].[13579BDF][67EF]. Now it incorrectly decodes writes to $4017, but few games will write to that register more than once. (It still collides with writes to the RAM at $6xxx.)
  * For games that require the extra RAM, using a 74138 or 74139 has the advantage that it already produces a /($4000-$5fff) signal. This could then be combined with another '138 or '139 to require A4, A2, and A1 high; and R/W (and A0) low.



Regardless, open bus behavior will cause the game to perceive one of the coin acceptors as always true. The last configuration above can produce /RD4016or4017, which can be combined with a tristateable buffer (and a button? or infrared receiver? or oscillator?) to pull D6 and D5 high or low as appropriate to fake the coin acceptors. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:37, 4 May 2014 (MDT) 
