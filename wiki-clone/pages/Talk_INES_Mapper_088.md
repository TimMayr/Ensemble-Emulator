# Talk:INES Mapper 088

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_088) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_088)

In the edit log for [iNES Mapper 154](INES_Mapper_154.xhtml "INES Mapper 154"), [Tepples](User_Tepples.xhtml "User:Tepples") said: Isn't 206, the ordinary MIMIC-1/108/118 board, theoretically just an undersize 88 anyway? 

    I'm betting that m88 was allocated before m206, and m88's difference from MMC3 does need its own mapper. It does seem to imply that allocating m206 was superfluous, since games with less than 64kB of CHR ROM don't have an CHR A16 line to connect to the PPU's A12.
    However, m88 could be also interpreted as "the MSB line of the address going into the CHR ROM is connected to PPU A12", meaning a 64kB CHR game would have the left and right pattern tables restricted to the first and last 32kB of CHR ROM. Since no m88 game exists with anything other than 128kB of CHR ROM, it's not really clear what the least surprising choice would be. — [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 00:54, 16 July 2012 (PDT) 

    I see three different approaches, depending on how you want to handle "oversize" 206 games: 

  1. Your suggestion for 88 = always run PA12 out to the highest existing CHR ROM address line as determined by the CHR ROM size in the header. When combined with an "oversize" variant of the Namco 108 family IC, this would allow up to 512K CHR.
  2. Treat these mappers as synonymous and have the CHR ROM size determine whether to emulate with A16. This is how the various [Bandai FCG boards](Bandai_FCG_board.xhtml "Bandai FCG board") are handled in [NES 2.0](NES_2_0.xhtml "NES 2.0"): 128 bytes mean 159 (24C01), 256 bytes mean 16 (24C02), and more means 153 (6264).
  3. Treat 88 as the actual board's behavior, with CHR ROM A16 == PA12 installed on those games that use 128K CHR, and have 206 refer to an oversize variant with CHR bank registers as wide as those of MMC3.


    Which is most logical? You might have to consult with the ROM dumping communities to distinguish these. --[Tepples](User_Tepples.xhtml "User:Tepples") 05:43, 16 July 2012 (PDT)
