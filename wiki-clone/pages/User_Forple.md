# User:Forple

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AForple) | View [other pages](Special_AllPages.xhtml#User_Forple)

I'm documenting small bits of info and data that doesn't really warrant its own page. 

## Contents

  * 1 Ricoh Chip Date Format (Pre-1990 approximate)
    * 1.1 Format
    * 1.2 Example dates (decoded)
  * 2 WDL Date Code Format
    * 2.1 Example dates (decoded)
      * 2.1.1 Non-Famicom chip decodings (for further examples)
  * 3 Famicom Titler Composite
    * 3.1 RC2C05-99
    * 3.2 RP2C04-0001
  * 4 Observed Logic Chip Manufacturers
  * 5 Ricoh Package Types
  * 6 Hardware Observations
    * 6.1 Mapper 106 Behavior
      * 6.1.1 Table of Results
      * 6.1.2 Testing Method
      * 6.1.3 Possible Cause
      * 6.1.4 Todo
    * 6.2 Columbus Circle Cartridge Behavior



## Ricoh Chip Date Format (Pre-1990 approximate)

Please note that this format has been primarily tested with the following ASICs: 

  * Ricoh 2A03 CPU (RP2A03x series) CPU for Famicom/NES
  * Ricoh 2C0x PPU (Rx2C02x series) PPU for Famicom/NES
  * Ricoh 2A10 SOC (RP2A10x series) SOC for Atari 2600 "Unicorn" models
  * Ricoh 65C02 CPU (RP65C02x series) CPU second-source of 65C02



It may work on other chips produced by Ricoh, but this cannot be guaranteed. 

This date format documentation is possible in no small part from [Fiskbit](User_Fiskbit.xhtml "User:Fiskbit"). 

Subsequent research notes this format is identical to the [Hitachi date format](https://telcontar.net/KBK/tech/IC_dates#Hitachi%7C). 

#### Format
    
    
    2A0x:	 YMW ??
    2A1x: !! YMW ??
    Y  = Year
    M  = Month
    W  = Week
    ?? = Unknown
    !! = Unknown (present on the 2A10, always set to "38")
    

  
Year is a single digit 0 through 9. When 1990 rolled around, they seemed to just wrap back over to 0 and count back up, while changing the format a bit (see below). Month is represented with a letter, from A to M. Likely for readability reasons, letter I is omitted due to visual similarity to the 1 digit. Week is a digit, 0 through 9, much like Year. The "Unknown" fields serve a purpose not yet understood. 

For some reason, Week was updated in 1990 to include letters. It's not entirely known why. 

Month Table
    
    A = January
    B = February
    C = March
    D = April
    E = May
    F = June
    G = July
    H = August
    J = September
    K = October
    L = November
    M = December

### Example dates (decoded)

    Chip: RP2A03
    Original: 4C3 27
    Decoded: March 1984, Week 3 (March 11 - March 17)

  


    Chip: RP65C02
    Original: 4J2 16
    Decoded: September 1984, Week 2 (September 2 - September 8)

  


    Chip: RP2C02B
    Original: 4D2 89
    Decoded: April 1984, Week 2 (April 8 - April 14)

  


    Chip: RP2C02C
    Original: 4B4 85
    Decoded: February 1984, Week 4 (February 19 - February 25)

  


    Chip: RP2A10
    Original: 38 7K3 10
    Decoded: October 1987, Week 3 (October 11 - October 17)

  


    Chip: RP2A10
    Original: 38 8L1 54
    Decoded: November 1988, Week 1 (November 1 - November 5)

## WDL Date Code Format

Although this date code format also covers chips well beyond the scope of NESDev, as it is used in several other chips in clone consoles, the WDL 6528, and WDL UM02 and UM03 chips present in famiclones use this format. 

The format mostly seems to be a simple translation of the number dates to letters. 

Conversion Table
    
    A = 1
    B = 2
    C = 3
    D = 4
    E = 5
    F = 6
    G = 7
    H = 8
    J = 9
    K = 0

### Example dates (decoded)

    Chip: WDL 6528
    Original: JBBF
    Decoded: 9226 (Week 25 1992)

  


    Chip: WDL UM02 and WDL UM03
    Original: JBCF
    Decoded: 9236 (Week 36 1992)

#### Non-Famicom chip decodings (for further examples)

    Chip: "22C731" (SNES Clone chip)
    Original: JDKC
    Decoded: 9403 (Week 3 1994)

  


    Chip: "TA0201" (DMG CPU Clone)
    Original: JGDB
    Decoded: 9742 (Week 42 1997)

  


## Famicom Titler Composite

The Sharp Famicom Titler (AN-510) contains a RC2C05-99 PPU, whose RGB output is fed into a X0858CE "encoder" chip. This chip handles the conversion from RGB(S) into Composite, and S-Video. In the process, it seems to also desaturate the output palette by encoding YIQ, but with a 0.5 gain on the Q channel (`Q = Q * 0.5`) 

### RC2C05-99

| $x0  | $x1  | $x2  | $x3  | $x4  | $x5  | $x6  | $x7  | $x8  | $x9  | $xA  | $xB  | $xC  | $xD  | $xE  | $xF   
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
$0x  | $00  | $01  | $02  | $03  | $04  | $05  | $06  | $07  | $08  | $09  | $0A  | $0B  | $0C  | $0D  | $0E  | $0F   
$1x  | $10  | $11  | $12  | $13  | $14  | $15  | $16  | $17  | $18  | $19  | $1A  | $1B  | $1C  | $1D  | $1E  | $1F   
$2x  | $20  | $21  | $22  | $23  | $24  | $25  | $26  | $27  | $28  | $29  | $2A  | $2B  | $2C  | $2D  | $2E  | $2F   
$3x  | $30  | $31  | $32  | $33  | $34  | $35  | $36  | $37  | $38  | $39  | $3A  | $3B  | $3C  | $3D  | $3E  | $3F   
  
### RP2C04-0001

Note that the below is the _sorted_ 2C04 palette. An unsorted variant may be added sometime later. Despite this, the 2C04 largely resembles the 2C05-99, albeit with some interesting differences. 

This was accomplished by socketing the Titler's PPU, and simply running a modified palette test ROM. 

| $x0  | $x1  | $x2  | $x3  | $x4  | $x5  | $x6  | $x7  | $x8  | $x9  | $xA  | $xB  | $xC  | $xD  | $xE  | $xF   
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
$0x  | $00  | $01  | $02  | $03  | $04  | $05  | $06  | $07  | $08  | $09  | $0A  | $0B  | $0C  | $0D  | $0E  | $0F   
$1x  | $10  | $11  | $12  | $13  | $14  | $15  | $16  | $17  | $18  | $19  | $1A  | $1B  | $1C  | $1D  | $1E  | $1F   
$2x  | $20  | $21  | $22  | $23  | $24  | $25  | $26  | $27  | $28  | $29  | $2A  | $2B  | $2C  | $2D  | $2E  | $2F   
$3x  | $30  | $31  | $32  | $33  | $34  | $35  | $36  | $37  | $38  | $39  | $3A  | $3B  | $3C  | $3D  | $3E  | $3F   
  
## Observed Logic Chip Manufacturers

Official Famicom and NES systems often have different 7400 logic manufacturers used between boards. 

  * Sharp (LR74xx; LR74xx chips are used almost exclusively for HVC-CPU-05 consoles?)
  * Hitachi (HD74xx)
  * Texas Instruments (SN74xx)
  * Fujitsu
  * Fairchild (Exclusively used in HVC-CPU-03 boards?)



There may be more, feel free to edit this section if you find others! 

## Ricoh Package Types

The chips in NES/Famicom units vary in package styles a lot. This section tries to identify possible manufacturers of the chipset with cross-references. Please note this section is currently a work in progress and info may change. 

If you have any suggestions, ideas, info, etc; Please leave them on my user talk page. 

Chip  | Image  | Reference  | Possible Manufacturer  | Country of Manufacture  | Comment   
---|---|---|---|---|---  
RP2C02C  | [![PPU=RP2C02C 4B4 85.png](../wiki-images/PPU%3DRP2C02C_4B4_85.png)](File_PPU_RP2C02C_4B4_85_png.xhtml) | [![PSG=YM2149F 9510 GAIH.jpg](../wiki-images/PSG%3DYM2149F_9510_GAIH.jpg)](File_PSG_YM2149F_9510_GAIH_jpg.xhtml) | Hitachi  | Japan  | Supposedly identical bottomside imprint format as other Hitachi-made chips, such as some YM2149F. Pin 1 marking and font are near-identical.   
RP2C33-01A  | [![FDS=RP2C33 01A 6D1 26.png](../wiki-images/FDS%3DRP2C33_01A_6D1_26.png)](File_FDS_RP2C33_01A_6D1_26_png.xhtml) | [![VDP=V9938 2701C 9112 GAFE.jpg](../wiki-images/VDP%3DV9938_2701C_9112_GAFE.jpg)](File_VDP_V9938_2701C_9112_GAFE_jpg.xhtml) | Hitachi  | Japan  | Shares package style with V9938 VDP, and Ricoh 2A10.   
RP2A03G  | [![CPU=2A03G 2EM 3C.png](../wiki-images/CPU%3D2A03G_2EM_3C.png)](File_CPU_2A03G_2EM_3C_png.xhtml) | [![C64 CPU=MOS 6510 CBM 3483.jpg](../wiki-images/C64_CPU%3DMOS_6510_CBM_3483.jpg)](File_C64_CPU_MOS_6510_CBM_3483_jpg.xhtml) | Unknown Manufacturer  | Hong Kong  | Shares font and package style with MOS 6510 CPU, from one decade prior. Visually similar MOS 6510s are marked with "HONG KONG" on the bottom.   
  
## Hardware Observations

Weird things with hardware I might not be able explain, or understand. It's all documented here, per request of "org" due to it otherwise being lost to time on a discord channel. 

![](../wiki-images/Ambox_content.png) |  This information may contain incomplete or incorrect information as topics are more actively researched. Despite this, the section will try to be as accurate as possible at all times.   
---|---  
  
### Mapper 106 Behavior

[INES Mapper 106](INES_Mapper_106.xhtml "INES Mapper 106"), or at least my copy of this cartridge, **works on nearly all revisions of CPU and PPU I can test with but RP2C02B.** I don't know the full extent of why this happens. But it's my own theory that it is possibly either due to a change in CPU manufacturer leading to a slight behavioral difference, or the different PPU. 

Observations listed below: (Note: "Chip Manufacturer" lists the company when possible. Otherwise, it lists the origin country usually marked on the package, which may have variations.) 

#### Table of Results

PCB (eg: HVC-CPU-02)  | CPU (with Chip Manufacturer)  | PPU (with Chip Manufacturer)  | Status  | Notes   
---|---|---|---|---  
HVC-CPU-03  | RP2A03 (Hitachi)  | RP2C02A (Hitachi)  | Works fine.  | No issues.   
HVC-CPU-05  | RP2A03 (Hitachi)  | RP2C02C (Hitachi)  | Works fine.  | No issues.   
HVC-CPU-05  | RP2A03 (Korea Mfg. #1)  | RP2C02B (Hitachi)  | Crashes after some time. Game effects include halting with all background tiles vanishing (sprites are untouched, and remain uncorrupted), with the IRQ misbehaving, drawing a weird pattern of IRQs down the screen. The game seems to mostly lock up, but is still running extremely slowly. This continues for approximately 5 minutes until the game fully crashes.  | PPU known to exhibit weird behavior supposedly not present on other 2C02B PPUs. Otherwise acts (mostly) the same as a normal 2C02B.   
HVC-CPU-07  | RP2A03E (Korea Mfg. #2)  | RP2C02E-0 (Korea Mfg. #1)  | Works fine.  | No Issues.   
HVC-CPU-07  | RP2A03G (Korea Mfg. #1)  | RP2C02G (Korea Mfg. #1)  | Works fine.  | No Issues.   
HVC-CPU-07  | RP2A03G (Hong Kong)  | RP2C02G-0 (Hong Kong)  | Works fine.  | No Issues. Functionally identical results to G-revision CPU/PPU.   
HVC-CPU-07  | TA-03N  | TA-02N  | Works fine.  | No Issues. Functionally identical results to G-revision CPU/PPU.   
HVC-CPU-07  | UA6527P  | UA6528  | Works fine.  | PAL clone CPU in NTSC system, with NTSC PPU. No Issues. Results would likely be somewhat close to E-revision CPU/PPU.   
HVC-CPU-07  | 2011  | 2010  | Works fine.  | No Issues. Functionally identical results to G-revision CPU/PPU.   
?? (MicroNewton PCB)  | XYZ-6783  | WDL 6528  | Works fine.  | No Issues. Functionally identical results to G-revision CPU/PPU.   
?? (DarYar PCB)  | "6538N" | "6528" | Works fine.  | No Issues. Functionally identical results to E-revision CPU/PPU.   
  
Things to note: 

  * Korea manufacturers are determined by the bottom package eject holes either _printing_ "KOREA" on the bottom, or as part of a "stamp" embossed onto one of the bottom eject holes.
  * All HVC-CPU-07 tests were done with the same PCB, as it is socketed.
  * PAL 6527P with NTSC 6528 is _mostly_ similar to an NTSC Famicom system with E-revision (or compatible) chipset. Major difference is CPU speed, and APU generated frequencies.
  * Despite being an earlier revision, the 2C02B board and its chipset were made _after_ the 2C02C's board and chipset.
  * The RP2C02B PCB is the only PCB I own containing a Korea-manufactured 2A03.



#### Testing Method

The bug was originally found during a casual playthrough of the cartridge (Which contained Super Mario Bros. 3). When I originally discovered the game crashed after a set time, I tried to beat the game as fast as possible, to try and beat it before it crashed. All test runs from this point on use the "Double Warp Whistle" trick, as a result. 

#### Possible Cause

  * According to org, "This happens when NMI is executed more than once per frame or its execution diverges from VBlank. This can be caused either by an IRQ from the timer, an IRQ from the cartridge, or an incorrect CPU/PPU timing alignment."



#### Todo

  * Check cartridge IRQ and PPU /INT lines of the "faulty" unit with a logic analyzer, and cross compare to a _working_ console.



  


### Columbus Circle Cartridge Behavior

When I got a HVC-CPU-08 board, I [recently discovered](https://www.youtube.com/watch?v=MpSTMmKfwKM) that all games with it worked _except_ that of Kira Kira Star Night DX. For unknown reasons, the game failed to bankswitch in the correct graphics _at all_ for almost every scenario, and it seemed like tilemap fetches were buggy. I thought this was a one off deal just like the above Mapper 106; However another user (Lockster) has reported the same bug with a completely different game also released by Columbus Circle. 

However, it turns out that this is fault of the 74'139 in the system, as different models, manufacturers, and other reasons cause the ROMSEL signal to be too slow, and the cartridge thus sends the wrong data at the wrong time. 
