# Talk:INES Mapper 078

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_078) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_078)

The problem about different mirroring control could be solved in both [NES 2.0](NES_2_0.xhtml "NES 2.0") format (using submapper numbers; perhaps 1=Uchuusen, 2=Holy Diver, 0=treat it as iNES and use checksums or whatever) as well as [DotFami](User_Zzo38_DotFami.xhtml "User:Zzo38/DotFami") format (where the mapper definition is included in the same file as the ROM). You should probably work on defining the submapper numbers for NES 2.0. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 20:50, 3 August 2012 (PDT) 

    I'm not clear why defining one of them as being a different mapper isn't better. I guess "fights with other refiling tools", but seeing as GoodNES hasn't been updated for years... The same is true for mapper 34 (BNROM v NINA-001) --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 10:28, 4 August 2012 (PDT)

    

    Due to existing already using the one mapper number. Because of this, I would think it might make sense to use submapper zero to tell the emulator to treat it the same as iNES and nonzero submapper numbers to tell more specifically what difference, to have compatibility (and if some emulator does not support submapper numbers it can treat it as iNES and hopefully work anyways), since mapper 078 already corresponds to both of these things so you can keep the mapper 078. (Of course if you do not like this, whoever documents and/or implement NES 2.0 format will do what they do like.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 10:46, 4 August 2012 (PDT) 

    Then ought we to collect these proposals for [NES 2.0 submappers](NES_2_0_submappers.xhtml "NES 2.0 submappers") somewhere? --[Tepples](User_Tepples.xhtml "User:Tepples") 08:52, 23 September 2012 (MDT)

    I suppose it also doesn't matter if you are making a game with this mapper which uses only one nametable. (Another question: What happens if CIRAM A10 is not connected?) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 23:48, 22 September 2012 (MDT) 

    What happens is [undefined](https://en.wikipedia.org/wiki/Undefined_behavior "wikipedia:Undefined behavior"). CIRAM A10 goes directly to an input of one of the LH5116 (CMOS 2Kx8 SRAM) ICs on the NES motherboard. Input pins should never be left floating unless you know the device has an internal pull-up or pull-down resistor, and a CMOS IC is less likely to have a pull-up than ICs made with some other processes that have pull-ups as the default for all pins. --[Tepples](User_Tepples.xhtml "User:Tepples") 08:52, 23 September 2012 (MDT)
