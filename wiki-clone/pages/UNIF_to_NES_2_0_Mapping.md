# UNIF to NES 2.0 Mapping

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UNIF_to_NES_2.0_Mapping) | View [other pages](Special_AllPages.xhtml#UNIF_to_NES_2_0_Mapping)

This page aims to list the [NES 2.0](NES_2_0.xhtml "NES 2.0") equivalent header values of all known [UNIF](UNIF.xhtml "UNIF") board names. 

Notes: 

  * Empty cells mean mapper-controlled mirroring and zero amounts of PRG-RAM, respectively. An empty CHR-RAM cell means zero amount of CHR-RAM unless the cart has no CHR-ROM, in which case it has 8K of CHR-RAM.
  * Unless mentioned otherwise, PRG-RAM can be battery-backed if an UNIF **BATT** chunk is present. Boards using both battery-backed and non-backed RAM are denoted using the non-backed amount first, followed by a plus sign and the battery-backed amount.
  * Common prefixes such as BMC-, BTL-, HVC-, NES-, UNL are omitted and replaced with *-.
  * Compatible boards with the same stem but different prefixes or additional suffixes are not listed separately. For example, ***-NROM** includes **NES-NROM-128-02** as well as **IREM-NROM-128** as well.
  * Board names are shown capitalized and with leading and trailing space characters removed, even as some UNIF image files do include MAPR chunks with leading or trailing space characters and lowercase letters.
  * Because the tables do not list the maximum PRG- and CHR-ROM sizes for each board type for brevity, a reverse lookup of the correct UNIF MAPR for a given NES 2.0 header is not directly possible.



## Contents

  * 1 Nintendo-made boards
  * 2 Boards made by third-party licensees
  * 3 Boards made by unlicensed and bootleg publishers
  * 4 Homebrew boards
  * 5 So-far unassigned UNIF board names



# Nintendo-made boards

UNIF MAPR | Mapper.Submapper | Mirroring | PRG-RAM | CHR-RAM | Notes   
---|---|---|---|---|---  
***-B4** | 4.0 |  |  |   
***-AMROM** | 7.2 |  |  | 8K |   
***-ANROM** | 7.1 |  |  | 8K |   
***-AN1ROM** | 7.1 |  |  | 8K |   
***-AOROM** | 7._x_ |  |  | 8K | _x_ =1..2 depending on +CE wiring, or 0 if unknown   
***-BNROM** | 34.2 | H/V |  | 8K |   
***-CNROM** | 3.2 | H/V |  |  |   
***-CNROM+SECURITY** | 185._x_ | H/V |  |  | _x_ =4..7 depending on diode configuration, or 0 if unknown   
***-CPROM** | 13.0 |  |  | 16K |   
***-FAMILYBASIC** | 0.0 | V | 2K |  |   
***-EKROM** | 5.0 |  | 8K |  |   
***-ELROM** | 5.0 |  |  |  |   
***-ETROM** | 5.0 |  | 8K+8K |  |   
***-EWROM** | 5.0 |  | 32K |  |   
***-FJROM** | 10.0 |  | 8K |  |   
***-FKROM** | 10.0 |  | 8K |  |   
***-HROM** | 0.0 | V |  |  |   
***-HKROM** | 4.1 |  | 1K |   
***-NROM** | 0.0 | H/V |  |  |   
***-PEEOROM** | 9.0 |  |  |  |   
***-PNROM** | 9.0 |  |  |  |   
***-RROM** | 0.0 | H/V |  |  |   
***-RTROM** | 0.0 | H/V |  |   
***-SROM** | 0.0 | H/V |  |  |   
***-SAROM** | 1.0 |  | 8K |  |   
***-SBROM** | 1.0 |  |  |  |   
***-SCROM** | 1.0 |  |  |  |   
***-SC1ROM** | 1.0 |  |  |  |   
***-SEROM** | 1.5 |  |  |  |   
***-SFROM** | 1.0 |  |  |  |   
***-SF1ROM** | 1.0 |  |  |  |   
***-SFEXPROM** | 1.0 |  |  |  |   
***-SGROM** | 1.0 |  |  | 8K |   
***-SHROM** | 1.5 |  |  |  |   
***-SH1ROM** | 1.5 |  |  |  |   
***-SIROM** | 1.0 |  | 8K |  |   
***-SJROM** | 1.0 |  | 8K |  |   
***-SKROM** | 1.0 |  | 8K |  |   
***-SLROM** | 1.0 |  |  |  |   
***-SL1ROM** | 1.0 |  |  |  |   
***-SL2ROM** | 1.0 |  |  |  |   
***-SL3ROM** | 1.0 |  |  |  |   
***-SLRROM** | 1.0 |  |  |  |   
***-SMROM** | 1.0 |  |  | 8K |   
***-SNROM** | 1.0 |  | 8K | 8K |   
***-SNWEPROM** | 1.0 |  | 8K | 8K |   
***-SOROM** | 1.0 |  | 8K+8K | 8K |   
***-SUROM** | 1.0 |  | 8K | 8K |   
***-SXROM** | 1.0 |  | 32K | 8K |   
***-TBROM** | 4.0 |  |  |  |   
***-TEROM** | 4.0 |  |  |  | Jumpers CL1/CL2 connected   
***-TEROM** | 4.2 | H/V |  |  | Jumpers CL1/CL2 disconnected   
***-TFROM** | 4.0 |  |  |  | Jumpers CL1/CL2 connected   
***-TFROM** | 4.2 | H/V |  |  | Jumpers CL1/CL2 disconnected   
***-TGROM** | 4.0 |  |  | 8K |   
***-TKROM** | 4.0 |  | 8K |  |   
***-TK1ROM** | 4.0 |  | 8K |  |   
***-TKEPROM** | 4.0 |  | 8K |  |   
***-TKSROM** | 118.0 |  | 8K |  |   
***-TLROM** | 4.0 |  |  |  |   
***-TL1ROM** | 4.0 |  |  |  |   
***-TL2ROM** | 4.0 |  |  |  |   
***-TLSROM** | 118.0 |  |  |  |   
***-TNROM** | 4.0 |  | 8K | 8K |   
***-TQROM** | 119.0 |  |  | 8K |   
***-TR1ROM** | 4.0 | 4 |  |  |   
***-TSROM** | 4.0 |  | 8K |  |   
***-TVROM** | 4.0 | 4 |  |  |   
***-STROM** | 0.0 | H/V |  |  |   
***-UNROM** | 2.2 | H/V |  | 8K |   
***-UOROM** | 2.2 | H/V |  | 8K |   
  
# Boards made by third-party licensees

UNIF MAPR | Mapper.Submapper | Mirroring | PRG-RAM | CHR-RAM | Notes   
---|---|---|---|---|---  
**ACCLAIM-MC-ACC** | 4.3 |  |  |  |   
**BANDAI-FCG-1** | 16.4 |  |  |  |   
**BANDAI-FCG-2** | 16.4 |  |  |  |   
**BANDAI-LZ93D50** | 16.5 |  |  |  |   
**BANDAI-LZ93D50+24C01** | 159.0 |  | 128 |  |   
**BANDAI-LZ93D50+24C02** | 16.5 |  | 256 |  |   
**BANDAI-PT-554** | 3.2 | H |  |  | 1x Misc. ROM for M50805   
**IREM-FCG-1** | 16.4 |  |  |  |   
**JALECO-JF01** | 0.0 | H |  |  |   
**JALECO-JF02** | 0.0 | H |  |  |   
**JALECO-JF03** | 0.0 | H |  |  |   
**JALECO-JF04** | 0.0 | H |  |  |   
**JALECO-JF15** | 2.2 | V |  | 8K |   
**JALECO-JF18** | 2.2 | V |  | 8K |   
**JALECO-JF23** | 18.0 |  |  |  |   
**JALECO-JF24** | 18.0 |  |  |  |   
**JALECO-JF25** | 18.0 |  |  |  |   
**JALECO-JF27** | 18.0 |  | 8K |  |   
**JALECO-JF29** | 18.0 |  |  |  |   
**JALECO-JF37** | 18.0 |  |  |  |   
**JALECO-JF40** | 18.0 |  | 8K |  |   
**NAMCOT-129** | 19.0 |  |  |  |   
**NAMCOT-163** | 19.0 |  | 0 or 8K |  |   
**NAMCOT-3301** | 0.0 | H |  |  |   
**NAMCOT-3302** | 0.0 | V |  |  |   
**NAMCOT-3303** | 0.0 | H |  |  |   
**NAMCOT-3304** | 0.0 | H |  |  |   
**NAMCOT-3305** | 0.0 | V |  |  |   
**NAMCOT-3311** | 0.0 | H |  |  |   
**NAMCOT-3312** | 0.0 | V |  |  |   
**NAMCOT-CNROM+WRAM** | 3.2 | H | 2K |  |   
**NES-NTBROM** | 68.1 |  | 8K |  |   
**SUNSOFT_UNROM** | 93.0 |  |  | 8K |   
**KONAMI-QTAI** | 547.0 |  | 8K+8K | 8K | UNIF uses 256 KiB 2bpp, NES 2.0 uses 128 KiB 1bpp CHR-ROM   
  
# Boards made by unlicensed and bootleg publishers

UNIF MAPR | Mapper.Submapper | Mirroring | PRG-RAM | CHR-RAM | Notes   
---|---|---|---|---|---  
***-SL1632** | 14.0 |  |  |  |   
***-AC-08** | 42.0 |  |  | 8K |   
***-LH-09** | 42.0 |  |  |  |   
***-SUPERVISION16IN1** | 53.0 |  |  | 8K | Final 8K moved to front   
***-SUPERHIK8IN1** | 45.0 |  |  |  |   
***-STREETFIGTER-GAME4IN1** | 49.? |  |  |  | Sic. $6000 set to $41 rather than $00 on power-up.   
***-MARIO1-MALEE2** | 42.0 |  | 2K |  |   
***-D1038** | 59.0 |  |  |  |   
***-T3H53** | 59.0 |  |  |  |   
***-SA-016-1M** | 79.0 | H/V |  |  |   
***-VRC7** | 85.0 |  |  |  |   
***-SC-127** | 90.0 |  | 8K |  |   
***-BB** | 108.0 |  |  |  |   
***-SL12** | 108.0 |  |  |  |   
***-H2288** | 123.0 |  |  |  |   
***-22211** | 132.0 | H/V |  |  |   
***-SA-72008** | 133.0 | H/V |  |  |   
***-T4A54A** | 134.0 |  |  |  |   
***-SACHEN-8259D** | 137.0 |  |  |  |   
***-SACHEN-8259B** | 138.0 |  |  |  |   
***-SACHEN-8259C** | 139.0 |  |  |  |   
***-SACHEN-8259A** | 141.0 |  |  |  |   
***-KS7032** | 142.0 |  |  |  |   
***-SA-NROM** | 143.0 |  |  |  |   
***-SA-72007** | 145.0 |  |  |  |   
***-TC-U01-1.5M** | 147.0 |  |  |  |   
***-SA-0037** | 148.0 |  |  |  |   
***-SA-0036** | 149.0 |  |  |  |   
***-SACHEN-74LS374N** | 150.0 |  |  |  |   
***-FS304** | 162.0 |  | 8K | 8K |   
***-SUPER24IN1SC03** | 176.0 |  |  | 8K |   
***-FK23C** | 176.0 |  |  | 256K* | Up to 256K of CHR-RAM in the absence of CHR-ROM   
***-FK23CA** | 176.0 |  |  | 256K* | Up to 256K of CHR-RAM in the absence of CHR-ROM   
**WAIXING-FS005** | 176.0 |  | 32K | 8K |   
***-NOVELDIAMOND9999999IN1** | 201.0 |  |  |  |   
***-JC-016-2** | 205.0 |  |  |  |   
***-8237** | 215.0 |  |  |  |   
***-8237A** | 215.1 |  |  |  |   
***-N625092** | 221.0 |  |  |  |   
***-GHOSTBUSTERS63IN1** | 226.0 |  |  |  |   
***-42IN1RESETSWITCH** | 233.0 |  |  |  |   
***-150IN1A** | 235.0 |  |  |  |   
***-212-HONG-KONG** | 235.0 |  |  |  |   
***-70IN1** | 236.0 |  |  |  |   
***-70IN1B** | 236.0 |  |  |  |   
***-603-5052** | 238.0 |  |  |  |   
**WAIXING-FW01** | 227.0 |  | 8K |  |   
***-43272** | 227.0 |  | 8K |  |   
***-ONEBUS** | 256.0 |  | 8K |  |   
***-DANCE** | 256.0 |  |  |  |   
**PEC-586** | 257.0 |  | 8K |  |   
***-158B** | 258.0 |  |  |  |   
***-F-15** | 259.0 |  |  |  |   
***-HPXX** | 260.0 |  | 8K |  |   
***-HP2018-A** | 260.0 |  | 8K |  |   
***-810544-C-A1** | 261.0 |  |  |  |   
***-SHERO** | 262.0 | 4 |  | 8K |   
***-KOF97** | 263.0 |  |  |  |   
***-YOKO** | 264.0 |  |  |  |   
***-T-262** | 265.0 |  |  | 8K |   
***-CITYFIGHT** | 266.0 |  |  |  |   
**COOLBOY** | 268.0 |  | 8K | 256K |   
**MINDKIDS** | 268.1 |  | 8K | 256K |   
***-22026** | 271.0 |  |  |  |   
***-80013-B** | 274.0 |  |  | 8K |   
***-GKCXIN1** | 288.0 |  |  |  |   
***-GS-2004** | 283.0 |  |  | 8K |   
***-GS-2013** | 283.0 |  |  | 8K |   
***-A65AS** | 285.0 |  |  | 8K |   
***-BS-5** | 286.0 |  |  |  |   
***-411120-C** | 287.0 |  |  |  |   
***-K-3088** | 287.0 |  |  |  |   
***-60311C** | 289.0 |  |  | 8K |   
***-NTD-03** | 290.0 |  |  |  |   
***-DRAGONFIGHTER** | 292.0 |  |  |  |   
***-13IN1JY110** | 295.0 |  |  | 8K |   
***-TF1201** | 298.0 |  |  |  |   
***-11160** | 299.0 |  |  |  |   
***-190in1** | 300.0 |  |  |  |   
***-8157** | 301.0 |  |  | 8K |   
***-KS7057** | 302.0 |  |  | 8K |   
***-KS7017** | 303.0 |  | 8K | 8K |   
***-SMB2J** | 304.0 |  |  |  |   
***-KS7031** | 305.0 |  |  | 8K |   
***-KS7016** | 306.0 |  |  | 8K |   
***-KS7037** | 307.0 |  | 8K | 8K |   
***-TH2131-1** | 308.0 |  |  |  |   
***-LH51** | 309.0 |  | 8K | 8K |   
***-LH32** | 125.0 |  | 8K | 8K |   
***-KS7013B** | 312.0 |  |  | 8K |   
***-RESET-TXROM** | 313.0 |  |  |  |   
***-64IN1NOREPEAT** | 314.0 |  |  |  |   
***-830134C** | 315.0 |  |  |  |   
***-HP898F** | 319.0 |  |  |  |   
***-830425C-4391T** | 320.0 |  |  | 8K |   
***-K-3033** | 322.0 |  |  |  |   
***-MALISB** | 325.0 |  |  |  |   
***-10-24-C-A1** | 327.0 |  | 8K | 8K |   
***-RT-01** | 328.0 |  |  |  |   
***-EDU2000** | 329.0 |  | 32K | 8K |   
***-12-IN-1** | 331.0 |  |  |  |   
***-WS** | 332.0 |  |  |  |   
***-NEWSTAR-GRM070-8IN1** | 333.0 |  |  |  |   
***-8-IN-1** | 333.0 |  |  |  |   
***-CTC-09** | 335.0 |  |  |  |   
***-K-3046** | 336.0 |  |  | 8K |   
***-CTC-12IN1** | 337.0 |  |  | 8K |   
***-SA005-A** | 338.0 |  |  |  |   
***-K-3006** | 339.0 |  |  |  |   
***-K-3036** | 340.0 |  |  | 8K |   
***-TJ-03** | 341.0 |  |  |  |   
***-GN-26** | 344.0 |  |  |  |   
***-L6IN1** | 345.0 |  |  |  |   
***-KS7012 "** | 346.0 |  | 8K | 8K |   
***-KS7030** | 347.0 |  | 8K | 8K |   
***-830118C** | 348.0 |  |  |  |   
***-G-146** | 349.0 |  |  | 8K |   
***-891227** | 350.0 |  |  | 8K |   
**3D-BLOCK** | 355.0 |  |  | 8K | Requires 1x Misc. ROM for microcontroller   
***-SA-9602B** | 513.0 |  |  | 32K |   
***-DANCE2000** | 518.0 |  | 8K | 8K |   
***-EH8813A** | 519.0 |  |  |  |   
**DREAMTECH01** | 521.0 |  |  | 8K |   
***-LH10** | 522.0 |  | 8K | 8K |   
***-900218** | 524.0 |  |  |  |   
***-KS7021A** | 525.0 |  |  |  |   
***-BJ-56** | 526.0 |  | 8K |  |   
***-AX-40G** | 527.0 |  |  |  |   
***-831128C** | 528.0 |  | 8K |  |   
***-T-230** | 529.0 |  |  |  |   
***-AX5705** | 530.0 |  |  |  |   
  
# Homebrew boards

UNIF MAPR | Mapper.Submapper | Mirroring | PRG-RAM | CHR-RAM | Notes   
---|---|---|---|---|---  
**COOLGIRL** | 342.0 |  | 32K | 512K |   
***-DRIPGAME** | 284.0 |  | 8K |  |   
**FARID_SLROM_8-IN-1** | 323.0 |  |  |  |   
**FARID_UNROM_8-IN-1** | 324.0 |  |  | 8K |   
**RET-CUFROM** | 29.0 |  |  | 32K |   
  
# So-far unassigned UNIF board names

  * 81-01-31-C
  * *-RESETNROM-XIN1
  * CHINA_ER_SAN2
  * *-GENERIC15IN1
  * *-KS106C
  * *-SB-2000
  * SSS-NROM-256 (Famicom Box)
  * *-SV01


