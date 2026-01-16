# Talk:INES Mapper 227

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_227) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_227)

puNES uses (addr >> 3) for the PRG bank. The info says (addr >> 2). 
    
    
      $8000-FFFF:  A~[.... ..LP  OPPP PPMS]
    

What's the correct info after all? 

## /* update */

I can confirm that Disch' information is correct. 

  


* * *

O=1, S=1: | < P > | 

Should be P >> 1, not P (tested on real hardware). 

    `<P>` is Disch's convention for "P÷2" —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:39, 19 April 2017 (MDT)
