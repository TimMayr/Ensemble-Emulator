# User talk:Jeffythedragonslayer

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AJeffythedragonslayer) | View [other pages](Special_AllPages.xhtml#User_talk_Jeffythedragonslayer)

## Four player adapters - signature byte

When talking about the four player adapters (Hori 4 Players Adapter, Four Score, or Satellite), the "signature byte" means: after you read 8 bits for controller 1 or 2, then read 8 more bits for controller 3 or 4, you read 8 more bits and make sure they match a known pattern. If those last 8 bits match the known pattern, that confirms the four player adapter is connected. 
    
    
     Hori 4 Players Adapter
                                                                       |--signature--|
       Reads from $4016 bit 1: ¹A,B,SE,ST,U,D,L,R, ³A,B,SE,ST,U,D,L,R, 0,0,1,0,0,0,0,0
       Reads from $4017 bit 1: ²A,B,SE,ST,U,D,L,R, ⁴A,B,SE,ST,U,D,L,R, 0,0,0,1,0,0,0,0
     
     Four Score (and Satellite?)
                                                                       |--signature--|
       Reads from $4016 bit 0: ¹A,B,SE,ST,U,D,L,R, ³A,B,SE,ST,U,D,L,R, 0,0,0,1,0,0,0,0
       Reads from $4017 bit 0: ²A,B,SE,ST,U,D,L,R, ⁴A,B,SE,ST,U,D,L,R, 0,0,1,0,0,0,0,0
    

\--[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 00:36, 21 February 2023 (UTC) 
