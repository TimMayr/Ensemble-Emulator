# User talk:Fiskbit

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AFiskbit) | View [other pages](Special_AllPages.xhtml#User_talk_Fiskbit)

## Underlines in plain text

I have to say, I had no idea that you could do underlines within plain text areas like you did in this table, very cool! 
    
    
    Mode| BG bit depth  |Offsets |     Priorities (front -> back)       |                     Notes                      _|BG1 BG2 BG3 BG4|per tile|                                      |                                                
     0  | 2   2   2   2 |   No   |   S3 1H 2H S2 1L 2L S1 3H 4H S0 3L 4L|_
     1  | 4   4   2     |   No   |   S3 1H 2H S2 1L 2L S1 3H    S0 3L   |BG3 priority = 0                                _|               |        |3H S3 1H 2H S2 1L 2L S1       S0 3L   |BG3 priority = 1                                
     2  | 4   4         |  Yes   |   S3 1H    S2 2H    S1 1L    S0 2L   |                                                
     3  | 8   4         |   No   |   S3 1H    S2 2H    S1 1L    S0 2L   |                                                
     4  | 8   2         |  Yes   |   S3 1H    S2 2H    S1 1L    S0 2L   |                                                
     5  | 4   2         |   No   |   S3 1H    S2 2H    S1 1L    S0 2L   |Fixed 16 pixel char width. Forced high-res mode.
     6  | 4             |  Yes   |   S3 1H    S2       S1 1L    S0      |Fixed 16 pixel char width. Forced high-res mode.
     7  | 8             |   No   |   S3       S2       S1 1L    S0      |Fixed 8x8 char size._
    7EXT| 8   7         |   No   |   S3       S2 2H    S1 1L    S0 2L   |Fixed 8x8 char size. BG2 bit 7 acts as priority.
    

\- [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 17:10, 4 May 2022 (UTC) 
