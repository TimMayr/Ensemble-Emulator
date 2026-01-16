# User talk:Zzo38/Mapper D

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Mapper_D) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Mapper_D)

## Common init code

I like your idea to make an MMC5 subset: it's more or less UNROM with banked PRG RAM but can be tested in any emulator with rudimentary support for MMC5 and NES 2.0. You may want to add two things: a canonical set of init code to get the MMC5 into a state where it acts like this mapper, and a caveat about writes to the RAM mirror at $1000-$1FFF. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 08:32, 11 August 2013 (MDT) 

    Thank you; I think you are correct, and I will add this information. (I did, in fact, create it specifically because emulators do not support the mappers that I just made up by myself!) (Also, if you don't need more than 64K RAM, then you don't need an emulator supporting NES 2.0, either.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 09:47, 11 August 2013 (MDT) 

    If it's marked as MMC5, the NES 2.0 header is needed to get the emulator to provide more than 8K of memory in the first place. And do you intend to hardwire 8 KiB of CHR ROM/RAM, or do you intend to make it respond to $5xxB as if it were CNROM? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 15:19, 11 August 2013 (MDT) 

    Hardwire 8K CHR ROM or CHR RAM. (Also, when I read it, it seems to me that it means the default amount of PRG RAM for MMC5 is 64K, isn't it? It is 8K for most mappers but for some the default amount is supposed to even be zero? Disch's notes specify the default amount of PRG RAM for MMC5 is 64K because existing games do not depend on the mirrors of the RAM, I think.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 15:49, 11 August 2013 (MDT)

## Hardware required

  * 74'138, wired to select _M2=high_ , _R/W=low_ , _A13=low_ , _A12=high_ , and _A4=address input_
    * There is one spare input, which is probably most usefully connected to A14=high.
  * 6 bit latch for ROM banking, such as 74'174
  * 3 bit latch for RAM banking, such as 74'161
  * 1 or 2 74'32s, for 256KiB PRG ROM or more, respectively



—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:21, 4 January 2014 (MST) 
