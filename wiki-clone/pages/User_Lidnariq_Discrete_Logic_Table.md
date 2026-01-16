# User:Lidnariq/Discrete Logic Table

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ALidnariq/Discrete_Logic_Table) | View [other pages](Special_AllPages.xhtml#User_Lidnariq_Discrete_Logic_Table)

It appears that all discrete logic mappers either switch 32kB at a time with no fixed bank ("GxROM-like"), or have a 16kB fixed bank and can switch the other ("UxROM-like"). The tables below illustrate the tradeoffs between CHR, PRG, and banking style. 

GxROM-like | 8kB CHR bank bits   
---|---  
0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8   
32kB PRG bank bits | 0  | [NROM](NROM.xhtml "NROM"), [143](INES_Mapper_143.xhtml "INES Mapper 143"), [185](CNROM.xhtml "INES Mapper 185") | [Vs. System](INES_Mapper_099.xhtml "INES Mapper 099"), [145](INES_Mapper_145.xhtml "INES Mapper 145"), [149](INES_Mapper_149.xhtml "INES Mapper 149") | [CNROM](CNROM.xhtml "CNROM"), [87](INES_Mapper_087.xhtml "INES Mapper 087")=[101](INES_Mapper_101.xhtml "INES Mapper 101"), [CPROM](CPROM.xhtml "CPROM")†, [173](INES_Mapper_173.xhtml "INES Mapper 173") |  |  |  |  |  | oversize [CNROM](CNROM.xhtml "CNROM")  
1  | [AN1ROM](AxROM.xhtml "AxROM")¹ |  | [MHROM](GxROM.xhtml "GxROM"), [132](INES_Mapper_132.xhtml "INES Mapper 132"), [133](INES_Mapper_133.xhtml "INES Mapper 133"), [203](INES_Mapper_203.xhtml "INES Mapper 203") | [NINA-03/06](NINA_003_006.xhtml "NINA-003-006")=[146](NINA_003_006.xhtml "INES Mapper 146"), [148](INES_Mapper_148.xhtml "INES Mapper 148") |  |  |  |  |   
2  | [ANROM](AxROM.xhtml "AxROM")¹, [BNROM](INES_Mapper_034.xhtml "BNROM") |  | [GNROM](GxROM.xhtml "GxROM"), [38](INES_Mapper_038.xhtml "INES Mapper 038"), [299](NES_2_0_Mapper_299.xhtml "NES 2.0 Mapper 299")ʰ, [379](NES_2_0_Mapper_379.xhtml "NES 2.0 Mapper 379") | [58](INES_Mapper_058.xhtml "INES Mapper 058")ʰ=[213](INES_Mapper_213.xhtml "INES Mapper 213"), [59](INES_Mapper_059.xhtml "INES Mapper 059")ʰ, [86](INES_Mapper_086.xhtml "INES Mapper 086"), [96](INES_Mapper_096.xhtml "INES Mapper 096")†, [174](INES_Mapper_174.xhtml "INES Mapper 174")ʰ, [288](NES_2_0_Mapper_288.xhtml "NES 2.0 Mapper 288") | [Color Dreams](Color_Dreams.xhtml "Color Dreams"), [36](INES_Mapper_036.xhtml "INES Mapper 036"), [57](INES_Mapper_057.xhtml "INES Mapper 057")ʰ, [140](INES_Mapper_140.xhtml "INES Mapper 140"), [147](INES_Mapper_147.xhtml "INES Mapper 147"), [319](NES_2_0_Mapper_319.xhtml "NES 2.0 Mapper 319")ʰ, [332](NES_2_0_Mapper_332.xhtml "NES 2.0 Mapper 332")ʰ |  | oversize [38](INES_Mapper_038.xhtml "INES Mapper 038") |  |   
3  | [AOROM](AxROM.xhtml "AxROM")¹, [283](NES_2_0_Mapper_283.xhtml "NES 2.0 Mapper 283") |  |  |  | [41](INES_Mapper_041.xhtml "INES Mapper 041")ʰ, [113](INES_Mapper_113.xhtml "INES Mapper 113")ʰ, [261](NES_2_0_Mapper_261.xhtml "NES 2.0 Mapper 261")ʰ, [335](NES_2_0_Mapper_335.xhtml "NES 2.0 Mapper 335")ʰ |  |  |  |   
4  | oversize [AxROM](AxROM.xhtml "AxROM")¹, [231](INES_Mapper_231.xhtml "INES Mapper 231")ʰ, [285](NES_2_0_Mapper_285.xhtml "NES 2.0 Mapper 285")ʰ¹, [337](NES_2_0_Mapper_337.xhtml "NES 2.0 Mapper 337")ʰ | [GTROM](GTROM.xhtml "GTROM")⁴ |  |  | oversize [GNROM](GxROM.xhtml "GxROM") | [290](NES_2_0_Mapper_290.xhtml "NES 2.0 Mapper 290")ʰ, [389](NES_2_0_Mapper_389.xhtml "NES 2.0 Mapper 389")ʰ |  |  |   
5  | [15](INES_Mapper_015.xhtml "INES Mapper 015")ʰ, [227](INES_Mapper_227.xhtml "INES Mapper 227")ʰ, [349](NES_2_0_Mapper_349.xhtml "NES 2.0 Mapper 349")ʰ, [541](NES_2_0_Mapper_541.xhtml "NES 2.0 Mapper 541")ʰ |  |  |  |  |  | [225](INES_Mapper_225.xhtml "INES Mapper 225")ʰ=[255](INES_Mapper_255.xhtml "INES Mapper 255")ʰ | [46](INES_Mapper_046.xhtml "INES Mapper 046") |   
6  | [226](INES_Mapper_226.xhtml "INES Mapper 226")ʰ |  |  |  |  |  | [228](INES_Mapper_228.xhtml "INES Mapper 228")ʰ, [314](NES_2_0_Mapper_314.xhtml "NES 2.0 Mapper 314")ʰ | [62](INES_Mapper_062.xhtml "INES Mapper 062")ʰ, [519](NES_2_0_Mapper_519.xhtml "NES 2.0 Mapper 519")ʰ |   
7  | [63](INES_Mapper_063.xhtml "INES Mapper 063")ʰ, [235](INES_Mapper_235.xhtml "INES Mapper 235")ʰ¹ |  |  |  |  |  |  |  |   
8  | oversize [BNROM](INES_Mapper_034.xhtml "BNROM") |  |  |  |  |  |  |  |   
  
  


UxROM-like | 8kB CHR bank bits   
---|---  
0 | 1 | 2 | 3 | 4 | 5 | 6   
16kB PRG bank bits | 2  |  |  |  |  | [168](INES_Mapper_168.xhtml "INES Mapper 168")† |  |   
3  | [UNROM](UxROM.xhtml "UxROM"), [94](INES_Mapper_094.xhtml "INES Mapper 094"), [180](INES_Mapper_180.xhtml "INES Mapper 180") |  | [29](INES_Mapper_029.xhtml "INES Mapper 029") |  | [72](INES_Mapper_072.xhtml "INES Mapper 072"), [78](INES_Mapper_078.xhtml "INES Mapper 078")¹­ʰ, [89](INES_Mapper_089.xhtml "INES Mapper 089")¹, [93](INES_Mapper_093.xhtml "INES Mapper 093")*, [152](INES_Mapper_152.xhtml "INES Mapper 152")¹ |   
4  | [UOROM](UxROM.xhtml "UxROM") |  |  |  | [70](INES_Mapper_070.xhtml "INES Mapper 070"), [92](INES_Mapper_092.xhtml "INES Mapper 092")  
5  |  |  | [Sealie UNROM 512](UNROM_512.xhtml "UNROM 512")¹ |   
6  | oversize [94](INES_Mapper_094.xhtml "INES Mapper 094") |  |   
7  |  |   
8  | oversize [UxROM](UxROM.xhtml "UxROM"), oversize [180](INES_Mapper_180.xhtml "INES Mapper 180")  
  
  
† 4F+4 or 4+4F CHR-RAM banking, not 8 CHR-ROM banking  
¹ has mapper-controlled single-screen mirroring  
ʰ has mapper-controlled H/V mirroring  
* Emulators commonly implement mapper 93 as a plain UNROM variant, not supporting CHR banking. But the hardware does support it. 

Non-standard CHR banking: 

  * [NINA-001](INES_Mapper_034.xhtml "NINA-001") has 1 bit for 32 PRG and 8 bits for 4+4 CHR banking
  * [77](INES_Mapper_077.xhtml "INES Mapper 077") has 4 bits for 2+6RAM CHR banking (plus 4 bits for 32 PRG banking)
  * [60](INES_Mapper_060.xhtml "INES Mapper 060"), [107](INES_Mapper_107.xhtml "INES Mapper 107"), and [201](INES_Mapper_201.xhtml "INES Mapper 201") use the same bits to control both PRG and CHR banks
  * [184](INES_Mapper_184.xhtml "INES Mapper 184") has 5 bits for 4+4 CHR banking



Exceptions: 

  * [40](INES_Mapper_040.xhtml "INES Mapper 040") and [50](INES_Mapper_050.xhtml "INES Mapper 050") have 24F+8+8F banking.


