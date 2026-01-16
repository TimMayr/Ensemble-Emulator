# PPU variants

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PPU_variants) | View [other pages](Special_AllPages.xhtml#PPU_variants)

Beyond the well-studied 2C02G, we know of the following PPU revisions, both made by Ricoh and other manufacturers: 

## Contents

  * 1 Official
    * 1.1 Composite
      * 1.1.1 NTSC
      * 1.1.2 PAL
    * 1.2 RGB
  * 2 Unofficial
  * 3 See also



## Official

Chips officially licensed by Nintendo for use in official consoles and arcade systems. 

### Composite

* * *

#### NTSC

Part  | Picture  | First Seen  | Last Seen  | Notes   
---|---|---|---|---  
RP2C02  | [![PPU=RP2C02 3F4 13.jpg](../wiki-images/PPU%3DRP2C02_3F4_13.jpg)](File_PPU_RP2C02_3F4_13_jpg.xhtml)   
[![PPU=RP2C02 3G2 23.jpg](../wiki-images/PPU%3DRP2C02_3G2_23.jpg)](File_PPU_RP2C02_3G2_23_jpg.xhtml) | 1983-06   
3F4 13  | 1983-08   
3H2 10  | Extremely rare. Likely only a few thousand made. <http://web.archive.org/web/20160315221802/kitayama3800.publog.jp/archives/cat_915765.html> (pictures not archived)]   
RP2C02A  | [![PPU=RP2C02A 3H1 43.jpg](../wiki-images/PPU%3DRP2C02A_3H1_43.jpg)](File_PPU_RP2C02A_3H1_43_jpg.xhtml) | 1983-08   
3H1 43  | 1983-08   
3H2 10  | Sometimes erroneous sprite pixels appear in X=255. Some modern PCBs generate almost exclusively glitchy pattern fetches. [PPUMASK](PPU_registers.xhtml "PPUMASK") and [PPUCTRL](PPU_registers.xhtml "PPUCTRL") seem to be entirely asynchronous, and writes to [PPUMASK](PPU_registers.xhtml "PPUMASK") can disable rendering for one pixel with the commensurate bugs resulting.   
RP2C02B  | [![PPU=RP2C02B 4D4 28.jpg](../wiki-images/PPU%3DRP2C02B_4D4_28.jpg)](File_PPU_RP2C02B_4D4_28_jpg.xhtml) | 1983-12   
3M3 73  | 1984-04   
4D4 60  | Erroneous sprite pixels appear at X=255, just like 2C02A. Production seems to have halted to produce the 2C02C, but 2C02C production stopped and 2C02B production resumed, for unknown reasons.   
RP2C02C  | [![PPU=RP2C02C 3M1 10.jpg](../wiki-images/PPU%3DRP2C02C_3M1_10.jpg)](File_PPU_RP2C02C_3M1_10_jpg.xhtml) | 1983-12   
3M1 10  | 1984-02   
4B1 76  | Eventually stopped being produced in favor of resuming 2C02B production, for unknown reasons.   
RC2C02C  | [![PPU=RC2C02C 4A3 15.jpg](../wiki-images/PPU%3DRC2C02C_4A3_15.jpg)](File_PPU_RC2C02C_4A3_15_jpg.xhtml) | 1984-01   
4A3 15  | Comes in a ceramic package.[[1]](http://offgao.blog112.fc2.com/blog-entry-28.html) Currently only found inside serviced Famicoms.   
RP2C02D  | [![PPU=RP2C02D 4H3 19.jpg](../wiki-images/PPU%3DRP2C02D_4H3_19.jpg)](File_PPU_RP2C02D_4H3_19_jpg.xhtml) | 1984-08   
4H2 60  | 1984-12   
4M2 29  |   
RP2C02D-0  | [![PPU=RP2C02D-0 4K3 63.JPG](../wiki-images/PPU%3DRP2C02D-0_4K3_63.JPG)](File_PPU_RP2C02D_0_4K3_63_JPG.xhtml) | 1984-10   
4K3 63  | 1984-12   
4M1 25  |   
RP2C02E  | [![PPU=RP2C02E 4M3 14.jpg](../wiki-images/PPU%3DRP2C02E_4M3_14.jpg)](File_PPU_RP2C02E_4M3_14_jpg.xhtml) | 1984-12   
4M3 14  | 1985-10   
5K4 36  |   
RP2C02E-0  | [![PPU=RP2C02E-0 5K1 50.jpg](../wiki-images/PPU%3DRP2C02E-0_5K1_50.jpg)](File_PPU_RP2C02E_0_5K1_50_jpg.xhtml) | 1985-03   
5C5 46  | 1986-06   
6F4 39  | In this and all previous revisions, [OAMDATA](PPU_registers.xhtml "OAMDATA") and palette RAM are not readable. [Various OAM evaluation bugs](https://forums.nesdev.org/viewtopic.php?p=194740#p194740)  
RP2C02G-0  | [![PPU=RP2C02G-0 7M3 52.jpg](../wiki-images/PPU%3DRP2C02G-0_7M3_52.jpg)](File_PPU_RP2C02G_0_7M3_52_jpg.xhtml) | 1987-05   
7E2 80  | 1993-10   
3KM 1H  | Writes to [OAMADDR](PPU_registers.xhtml "OAMADDR") cause corruption of OAM. Leaving OAMADDR at a value of 8 or greater causes OAM corruption when rendering starts. Particularly susceptible to reflections causing OAM corruption, fixed by putting series resistors between the CPU and the cartridge ROM.   
RP2C02H-0  | [![PPU=RP2C02H-0 4AM 4B.jpg](../wiki-images/PPU%3DRP2C02H-0_4AM_4B.jpg)](File_PPU_RP2C02H_0_4AM_4B_jpg.xhtml) | 1993-12   
3MM 40  | 1999-05   
9EM 5B  | Thought have to have fixed some of the glitches in the previous 2C02G-0 revision(which?).   
RP2C02H-0 (laser)  | [![PPU=RP2C02H-0 1CL 42 \(laser\).jpg](../wiki-images/PPU%3DRP2C02H-0_1CL_42_%28laser%29.jpg)](File_PPU_RP2C02H_0_1CL_42__laser__jpg.xhtml) | 2000-10   
0KL 40  | 2003-01   
3AL 4B  | Reported (along with PAL PPUs) to have some kind of difference that caused problems with address bus filtering in earlier versions of the Hi-Def NES firmware.   
  
#### PAL

Part  | Picture  | First Seen  | Last Seen  | Notes   
---|---|---|---|---  
RP2C07  | [![PPU=RP2C07 5M4 26.jpg](../wiki-images/PPU%3DRP2C07_5M4_26.jpg)](File_PPU_RP2C07_5M4_26_jpg.xhtml) | 1985-12   
5M4 26  | PAL-B PPU. Vblanking is 71 scanlines long. OAM evaluation can never be fully disabled. Red/green color emphasis swapped. [OAMADDR](PPU_registers.xhtml "OAMADDR") works correctly, not corrupting OAM contents]   
RP2C07-0  | [![PPU=RP2C07-0 8F3 07.jpg](../wiki-images/PPU%3DRP2C07-0_8F3_07.jpg)](File_PPU_RP2C07_0_8F3_07_jpg.xhtml) | 1987-10   
7K3 27  | 1991-10   
1KM 13  |   
RP2C07A-0  | [![PPU=RP2C07A-0 2FM 22.jpg](../wiki-images/PPU%3DRP2C07A-0_2FM_22.jpg)](File_PPU_RP2C07A_0_2FM_22_jpg.xhtml) | 1992-06   
2FM 22  | Some subtle differences in PPU that make this work better with Kevtris's HDNES, but otherwise believed identical to 2C07.   
  
### RGB

* * *

Part  | Picture  | First Seen  | Last Seen  | Notes   
---|---|---|---|---  
RC2C03  | [![PPU=RC2C03 3L4 15.jpg](../wiki-images/PPU%3DRC2C03_3L4_15.jpg)](File_PPU_RC2C03_3L4_15_jpg.xhtml) | 1983-11   
3L4 15  | Found in the Sharp C1 TV. Suspected to be same core as 2C02 letterless.   
RP2C03B  | [![PPU=RP2C03B 9C4 23.jpg](../wiki-images/PPU%3DRP2C03B_9C4_23.jpg)](File_PPU_RP2C03B_9C4_23_jpg.xhtml)   
[![PPU=RP2C03B 4B4 41.jpg](../wiki-images/PPU%3DRP2C03B_4B4_41.jpg)](File_PPU_RP2C03B_4B4_41_jpg.xhtml) | 1984-02   
4B3 59  | 1989-03   
9C4 23  | RGB PPU. Color emphasis bits set the corresponding channel to full brightness. Otherwise believed identical to 2C02B. OAMDATA and PPU palette are not readable.   
RC2C03B  | [![PPU=RC2C03B 4A4 30.jpg](../wiki-images/PPU%3DRC2C03B_4A4_30.jpg)](File_PPU_RC2C03B_4A4_30_jpg.xhtml) | 1984-01   
4A2 14  | 1984-01   
4A4 30  |   
RP2C03C  | [![PPU=RP2C03C 4C2 63.jpg](../wiki-images/PPU%3DRP2C03C_4C2_63.jpg)](File_PPU_RP2C03C_4C2_63_jpg.xhtml) | 1984-03   
4C2 63  | RGB PPU. Believed to be same core as 2C02C.   
RC2C03C  | [![PPU=RC2C03C 4A2 10.jpg](../wiki-images/PPU%3DRC2C03C_4A2_10.jpg)](File_PPU_RC2C03C_4A2_10_jpg.xhtml) | 1984-01   
4A2 10  | Believed identical to RP2C03C.   
RP2C04   
0001  | [![PPU=RP2C04 0001 4C4 15.png](../wiki-images/PPU%3DRP2C04_0001_4C4_15.png)](File_PPU_RP2C04_0001_4C4_15_png.xhtml) | 1984-03   
4C2 01  | 1984-05   
4E4 35  | RGB PPU with a [scrambled palette](PPU_palettes.xhtml#2C04 "PPU palettes") and some added colors, for copy protection purposes. Otherwise behaves like 2C03C.   
RP2C04   
0002  | [![PPU=RP2C04 0002 4K2 35.jpg](../wiki-images/PPU%3DRP2C04_0002_4K2_35.jpg)](File_PPU_RP2C04_0002_4K2_35_jpg.xhtml) | 1984-07   
4G4 19  | 1984-10   
4K2 35  | Scrambles the palette uniquely compared to the previous revision.   
RP2C04   
0003  | [![PPU=RP2C04 0003 4K5 13.jpg](../wiki-images/PPU%3DRP2C04_0003_4K5_13.jpg)](File_PPU_RP2C04_0003_4K5_13_jpg.xhtml) | 1984-10   
4K5 13  | 1984-11   
4L4 33  | Scrambles the palette uniquely compared to the previous revisions.   
RP2C04   
0004  | [![PPU=RP2C04 0004 4L4 31.jpg](../wiki-images/PPU%3DRP2C04_0004_4L4_31.jpg)](File_PPU_RP2C04_0004_4L4_31_jpg.xhtml) | 1984-11   
4L3 18  | 1984-11   
4L5 36  | Scrambles the palette uniquely compared to the previous revisions.   
RC2C05-01  | [![PPU=RC2C05-01 5F5 10.jpg](../wiki-images/PPU%3DRC2C05-01_5F5_10.jpg)](File_PPU_RC2C05_01_5F5_10_jpg.xhtml) | 1985-06   
5F5 10  | 1985-06   
5F5 11  | [PPUCTRL](PPU_registers.xhtml "PPUCTRL") and [PPUMASK](PPU_registers.xhtml "PPUMASK") swap locations. Five LSBs of [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") return a constant. Otherwise believed to behave like 2C03.   
RC2C05-02  |  |  | Believed to be the same as 2C05-01, with the PPUSTATUS constant changed.   
RC2C05-03  | [![PPU=RC2C05-03 5G1 12.JPG](../wiki-images/PPU%3DRC2C05-03_5G1_12.JPG)](File_PPU_RC2C05_03_5G1_12_JPG.xhtml) | 1985-07   
5G1 12  | Believed to be the same as 2C05-01, with the PPUSTATUS constant changed.   
RC2C05-04  | [![PPU=RC2C05-04 7C3 12.jpg](../wiki-images/PPU%3DRC2C05-04_7C3_12.jpg)](File_PPU_RC2C05_04_7C3_12_jpg.xhtml) | 1987-03   
7C3 12  | Believed to be the same as 2C05-01, with the PPUSTATUS constant changed.   
RC2C05-99  | [![PPU=RC2C05-99 8M4 14.jpg](../wiki-images/PPU%3DRC2C05-99_8M4_14.jpg)](File_PPU_RC2C05_99_8M4_14_jpg.xhtml) | 1988-11   
8L4 11  | 1988-12   
8M4 14  | An RGB PPU with RP2C02E behavior: OAM and palettes are unreadable, but OAM corruption added in revision E is present. [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") behavior is normal (the low 5 bits are PPU open bus). Grayscale is normal (palette & $30). Uses 2C02 timing (a dot is skipped every other frame), unlike other RGB PPUs. The palette and emphasis behavior match other RGB PPUs. [PPUCTRL](PPU_registers.xhtml "PPUCTRL") and [PPUMASK](PPU_registers.xhtml "PPUMASK") aren't swapped. Used in the [Famicom Titler](https://en.wikipedia.org/wiki/Famicom_Titler "wikipedia:Famicom Titler"). ([picture of Titler PCBs](http://photozou.jp/photo/photo_only/165213/17829839))   
  
## Unofficial

These chips are found exclusively inside of Famiclone systems, made by multiple companies. 

Part  | Picture  | Notes   
---|---|---  
UA6528  | [![PPU=UA6528-8929-CS.jpg](../wiki-images/PPU%3DUA6528-8929-CS.jpg)](File_PPU_UA6528_8929_CS_jpg.xhtml) [![PPU=UA6528-9310-CS-426610.jpg](../wiki-images/PPU%3DUA6528-9310-CS-426610.jpg)](File_PPU_UA6528_9310_CS_426610_jpg.xhtml) | UMC-made clone of 2C02E (or something earlier) [[2]](https://forums.nesdev.org/viewtopic.php?&t=20591)  
UA6528P  | [![PPU=UA6528P-8911S.jpg](../wiki-images/PPU%3DUA6528P-8911S.jpg)](File_PPU_UA6528P_8911S_jpg.xhtml) | UMC-made variant for PAL-N (Argentina) [[3]](https://forums.nesdev.org/viewtopic.php?f=9&t=13530) System crystal is 21492337.5 Hz (exactly 6×229.2516×15625)   
UA6538(P??)  | [![PPU=UA6538 9214-BA 221530.jpg](../wiki-images/PPU%3DUA6538_9214-BA_221530.jpg)](File_PPU_UA6538_9214_BA_221530_jpg.xhtml) | UMC-made variant for playing NTSC games in PAL countries. Emits PAL-B video. Vblank interrupt intentionally emitted 50 scanlines later than 2C07. Video DAC is brighter and more saturated (how so?). See also [Clock rate](Cycle_reference_chart.xhtml "Clock rate").   
UA6541  | [![PPU=UA6541 8832S.JPG](../wiki-images/PPU%3DUA6541_8832S.JPG)](File_PPU_UA6541_8832S_JPG.xhtml) | UMC-made clone of 2C07 [[4]](https://forums.nesdev.org/viewtopic.php?t=17257)  
UA6548  | [![PPU=UA6548 9232-BS 825520.jpg](../wiki-images/PPU%3DUA6548_9232-BS_825520.jpg)](File_PPU_UA6548_9232_BS_825520_jpg.xhtml) | UMC-made variant for PAL-M (Brazil) System crystal is 21453671… Hz (exactly 6×227.25×4500000÷286)   
UM6558  | [![PPU=UM6558 9342S 924510.jpg](../wiki-images/PPU%3DUM6558_9342S_924510.jpg)](File_PPU_UM6558_9342S_924510_jpg.xhtml) | UMC-made variant of UA6538 for SECAM countries. Emits 8-bit "[Color Data](https://forums.nesdev.org/viewtopic.php?p=194423#p194423)" digital bus, for conversion into SECAM by UM6559 IC. Color palette [noticeably off](http://www.emu-land.net/forum/index.php/topic,27910.msg1091380.html#msg1091380). System crystal is 21312500 Hz (exactly 4×341×15625). Maybe supports both 50 and 60 Hz operation?   
UM6561xx-1  | [![NOAC=UM6561CF-1 0028A E6276.jpg](../wiki-images/NOAC%3DUM6561CF-1_0028A_E6276.jpg)](File_NOAC_UM6561CF_1_0028A_E6276_jpg.xhtml) | UMC-made NES-on-a-chip for NTSC. PPU half believed to be mostly identical to UA6528. Revisions "xx" F, AF, BF, CF known. Emphasis is much stronger than on UA6528 and official PPUs.   
UM6561xx-2  | [![UM6561F-2.jpg](../wiki-images/UM6561F-2.jpg)](File_UM6561F_2_jpg.xhtml) [![NOAC=UM6561 AF-2 9440A R81040.jpg](../wiki-images/NOAC%3DUM6561_AF-2_9440A_R81040.jpg)](File_NOAC_UM6561_AF_2_9440A_R81040_jpg.xhtml) | UMC-made NES-on-a-chip for PAL-B. PPU half believed to be mostly identical to UA6538. Revisions "xx" F, AF, BF, CF known. AF revision has graphical and timing glitches in Prince of Persia perhaps caused by sprite 0 hit being missed for reasons not yet understood. F and BF revisions are not affected. Emphasis is much stronger than on UA6538 and official PPUs.   
TA-02N  | [![PPU=TA-02N 6528 9248.jpg](../wiki-images/PPU%3DTA-02N_6528_9248.jpg)](File_PPU_TA_02N_6528_9248_jpg.xhtml) [![PPU=TA-02N Underside NT0181 609860A.jpg](../wiki-images/PPU%3DTA-02N_Underside_NT0181_609860A.jpg)](File_PPU_TA_02N_Underside_NT0181_609860A_jpg.xhtml) | ??-made die-mask clone of 2C02G, despite the "6528" label.[[5]](https://forums.nesdev.org/viewtopic.php?p=286601#p286601) Chip underside also has two codes of currently unknown purpose.   
TA-02NP  | [![PPU=TA-02NP 6538 9229.jpg](../wiki-images/PPU%3DTA-02NP_6538_9229.jpg)](File_PPU_TA_02NP_6538_9229_jpg.xhtml) [![PPU=TA-02NP WDW23616.jpg](../wiki-images/PPU%3DTA-02NP_WDW23616.jpg)](File_PPU_TA_02NP_WDW23616_jpg.xhtml) | ??-made PPU, Dendy compatible (not a 6538 clone). Pins 14-17 are background color in, like normal 2C03[[6]](https://forums.nesdev.org/viewtopic.php?p=274318#p274318)  
TA-02NPB  | [![PPU=TA-02NPB 9234.jpg](../wiki-images/PPU%3DTA-02NPB_9234.jpg)](File_PPU_TA_02NPB_9234_jpg.xhtml) | ??-made PPU, "NTSC for PAL-B" (NPB) Dendy-compatible. Has a unique video switching capability shared between it and other TA-02NPx chipsets, as well as the WDL chipset.   
TA-08  | [![PPU=TA-08 9345.jpeg](../wiki-images/PPU%3DTA-08_9345.jpeg)](File_PPU_TA_08_9345_jpeg.xhtml) | ??-made PPU, Second source alternative to UM6558. Color Data bus bit-reversed?   
MG-N-502  | [![PPU=MG-N-502 8933.jpg](../wiki-images/PPU%3DMG-N-502_8933.jpg)](File_PPU_MG_N_502_8933_jpg.xhtml) |   
MG-P-502  | [![PPU=MG-P-502 9221M 403600.jpg](../wiki-images/PPU%3DMG-P-502_9221M_403600.jpg)](File_PPU_MG_P_502_9221M_403600_jpg.xhtml) | Micro Genius / TXC. Die shot matches UA6538.   
1818N  | [![CPU = 1818N 9218.png](../wiki-images/CPU_%3D_1818N_9218.png)](File_CPU___1818N_9218_png.xhtml) | ??-made NES-on-a-chip, NTSC timing.   
1818P  | [![NOAC=1818P 8250.jpg](../wiki-images/NOAC%3D1818P_8250.jpg)](File_NOAC_1818P_8250_jpg.xhtml) | SiS-made[[7]](https://forums.nesdev.org/viewtopic.php?t=24499)] NES-on-a-chip.[[8]](https://forums.nesdev.org/viewtopic.php?p=228515#p228515). Requires external 2KiB RAMs for CPU and PPU. UA6538 timing.   
PM02-1  | [![PPU=PM02-1 HI17.jpg](../wiki-images/PPU%3DPM02-1_HI17.jpg)](File_PPU_PM02_1_HI17_jpg.xhtml) | [Gradiente](https://en.wikipedia.org/wiki/IGB_Eletr%C3%B4nica "wikipedia:IGB Eletrônica")-made variant for PAL-M (Brazil). [[9]](https://forums.nesdev.org/viewtopic.php?p=195175#p195175)  
VT01  |  | V.R.Tech-made clone of UA6561. Only seen as chip-on-board. Supports composite out; RGB out; or 2bpp STN LCD, either greyscale or [red/cyan checkerboard](VT01_STN_Palette.xhtml "VT01 STN Palette"), 240 px wide, 80/120/160/240px tall. The chip was extended significantly in [VT02](VTxx.xhtml#VT02 "VTxx") and newer NOACs.   
GS87008  | [![PPU=GS87008 8827.jpg](../wiki-images/PPU%3DGS87008_8827.jpg)](File_PPU_GS87008_8827_jpg.xhtml) | (Goldstar??)-made NTSC clone [[10]](http://gxemu.blog67.fc2.com/blog-entry-363.html)  
KC-6078  | [![PPU=KC-6078.jpg](../wiki-images/PPU%3DKC-6078.jpg)](File_PPU_KC_6078_jpg.xhtml) | Found in MT777-DX famiclone, behaves exactly like UA6538   
6022  | [![PPU=6022.jpg](../wiki-images/PPU%3D6022.jpg)](File_PPU_6022_jpg.xhtml) | ??-made NTSC clone. Details unknown.   
2010  | [![PPU=2010-9129M 5018.jpg](../wiki-images/PPU%3D2010-9129M_5018.jpg)](File_PPU_2010_9129M_5018_jpg.xhtml) |   
2A02E  | [![PPU=USC 2A02E 9136S 919620.jpg](../wiki-images/PPU%3DUSC_2A02E_9136S_919620.jpg)](File_PPU_USC_2A02E_9136S_919620_jpg.xhtml) | Dendy Timing. On a large solid-color background (especially yellow, green), every other row has a horizontal stripe.   
02  | [![PPU=02 HG-35.jpg](../wiki-images/PPU%3D02_HG-35.jpg)](File_PPU_02_HG_35_jpg.xhtml) |   
GT-01  | [![PPU=GT-01 9045.jpg](../wiki-images/PPU%3DGT-01_9045.jpg)](File_PPU_GT_01_9045_jpg.xhtml) | In PLCC   
HA6538  | [![PPU=HA6538 DRB1.png](../wiki-images/PPU%3DHA6538_DRB1.png)](File_PPU_HA6538_DRB1_png.xhtml) [![PPU=HA6538 CJ7H.png](../wiki-images/PPU%3DHA6538_CJ7H.png)](File_PPU_HA6538_CJ7H_png.xhtml) | PAL video output.   
SENITON 6538U-8  | [![PPU=SENITON 6538U-8.jpg](../wiki-images/PPU%3DSENITON_6538U-8.jpg)](File_PPU_SENITON_6538U_8_jpg.xhtml) | PAL video output.   
SENITON 6538A  | [![PPU=SENITON 6538A 9122D CFC2.jpg](../wiki-images/PPU%3DSENITON_6538A_9122D_CFC2.jpg)](File_PPU_SENITON_6538A_9122D_CFC2_jpg.xhtml) | PAL video output.   
6528 (WDL6528)  | [![PPU=WDL 6528 JBBG.jpg](../wiki-images/PPU%3DWDL_6528_JBBG.jpg)](File_PPU_WDL_6528_JBBG_jpg.xhtml) [![PPU=WDL 6528 JBBG Bottom.jpg](../wiki-images/PPU%3DWDL_6528_JBBG_Bottom.jpg)](File_PPU_WDL_6528_JBBG_Bottom_jpg.xhtml) | WDL made chip. NTSC timing. Palette like RP2C02. Often referred to as "WDL6528".   
8Z02  | [![8Z02 PPU.jpeg](../wiki-images/8Z02_PPU.jpeg)](File_8Z02_PPU_jpeg.xhtml) |   
TECH28  | [![PPU=TECH28 21 9120 910559.png](../wiki-images/PPU%3DTECH28_21_9120_910559.png)](File_PPU_TECH28_21_9120_910559_png.xhtml) | PAL video output.   
  
If you know of other differences or other revisions, please add them! 

## See also

  * [CPU variants](CPU_variants.xhtml "CPU variants")
  * [Clock rate](Cycle_reference_chart.xhtml "Clock rate")
  * [NES_2.0#Byte_13_.28Vs._hardware.29](NES_2_0.xhtml#Byte_13_.28Vs._hardware.29 "NES 2.0")
  * <https://forums.nesdev.org/viewtopic.php?p=150127#p150127>


