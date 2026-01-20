# Talk:INES Mapper 150

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_150) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_150)

## Physical vs. logical ordering

Is this like the difference between [iNES Mapper 087](INES_Mapper_087.xhtml "INES Mapper 087") and [iNES Mapper 101](INES_Mapper_101.xhtml "INES Mapper 101")/[140](INES_Mapper_140.xhtml "INES Mapper 140"), where one is ordered as in the mask ROM and the other as it is presented logically to the program? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 21:07, 17 May 2014 (MDT) 

    Sort of. Unlike m101, where its order is sensible (even if in disagreement with the ROM), both orders are pretty illogical for m150/243. Someone should check with Санчез, though. This mapper really feels like Sachen specifically designed it to be maximally annoying for cloners to implement using discrete logic (Two 74'74s, two 74'75s, two 74'138s, one 74'368, one 74'00), even though it only provides [Color Dreams](Color_Dreams.xhtml "Color Dreams") plus mirroring control. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 22:59, 17 May 2014 (MDT)

## And what about Jovial Race?

from 133's page: "GoodNES allocated iNES Mapper 133 for the Sachen unlicensed game "Jovial Race". It's almost identical to NINA-06. Санчез, however, allocated Jovial Race to iNES Mapper 150." [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 19:04, 4 August 2016 (MDT) 
