# CPU variants

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPU_variants) | View [other pages](Special_AllPages.xhtml#CPU_variants)

Beyond the well-studied 2A03G, we know of the following CPU revisions, both made by Ricoh and other manufacturers: 

## Contents

  * 1 Official (NTSC)
  * 2 Official (PAL)
  * 3 Unofficial
  * 4 See also



## Official (NTSC)

Part  | Picture  | Notes   
---|---|---  
RP2A03  | [![RP2A03 3G1 09.jpg](../wiki-images/RP2A03_3G1_09.jpg)](File_RP2A03_3G1_09_jpg.xhtml) [![CPU=RP2A03 3L4 27.jpg](../wiki-images/CPU%3DRP2A03_3L4_27.jpg)](File_CPU_RP2A03_3L4_27_jpg.xhtml) [![CPU=RP2A03 3M294.jpg](../wiki-images/CPU%3DRP2A03_3M294.jpg)](File_CPU_RP2A03_3M294_jpg.xhtml) | M2 duty cycle is 17/24 instead of 15/24 [[1]](https://forums.nesdev.org/viewtopic.php?p=166761#p166761). Lacks tonal noise mode. [APU Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") not restarted on reset. Has broken and disabled [programmable interval timer](RP2A03_Programmable_Interval_Timer.xhtml "RP2A03 Programmable Interval Timer") on-die. Pin 30 connects to nothing. Other differences?   
RP2A03E  | [![CPU=RP2A03E 5K5 26.jpg](../wiki-images/CPU%3DRP2A03E_5K5_26.jpg)](File_CPU_RP2A03E_5K5_26_jpg.xhtml) [![CPU=RP2A03E VF4109 5H4 8534.jpg](../wiki-images/CPU%3DRP2A03E_VF4109_5H4_8534.jpg)](File_CPU_RP2A03E_VF4109_5H4_8534_jpg.xhtml) | Pin 30 is /RDY - combined with internal signals before feeding to internal 6502 +RDY.   
RP2A03G  | [![CPU=RP2A03G 9B3 PV A.jpg](../wiki-images/CPU%3DRP2A03G_9B3_PV_A.jpg)](File_CPU_RP2A03G_9B3_PV_A_jpg.xhtml) [![CPU=RP2A03G 8815AAL.jpg](../wiki-images/CPU%3DRP2A03G_8815AAL.jpg)](File_CPU_RP2A03G_8815AAL_jpg.xhtml) | Reference model. Pin 30 enables a [CPU test mode](CPU_Test_Mode.xhtml "CPU Test Mode"). Later runs introduced a DMC DMA bug [[2]](https://forums.nesdev.org/viewtopic.php?p=275359#p275359).   
RP2A03H  | [![CPU=RP2A03H 4AM 4F.jpg](../wiki-images/CPU%3DRP2A03H_4AM_4F.jpg)](File_CPU_RP2A03H_4AM_4F_jpg.xhtml) [![CPU=RP2A03H 4BM 4Z.jpg](../wiki-images/CPU%3DRP2A03H_4BM_4Z.jpg)](File_CPU_RP2A03H_4BM_4Z_jpg.xhtml) | No known differences from late RP2A03G.   
RP2A04  | [![CPU=RP2A04 2J 6C2 01.jpg](../wiki-images/CPU%3DRP2A04_2J_6C2_01.jpg)](File_CPU_RP2A04_2J_6C2_01_jpg.xhtml) | Not actually a CPU at all, just a jumper in a 40-pin PDIP. Used in place of CPUs in [Vs. System](Vs__System.xhtml "Vs. System") boards (and thus with NTSC timing).   
  
## Official (PAL)

Part  | Picture  | Notes   
---|---|---  
RP2A07  | [![CPU=RP2A07 7C4 39.jpg](../wiki-images/CPU%3DRP2A07_7C4_39.jpg)](File_CPU_RP2A07_7C4_39_jpg.xhtml) [![CPU=RP2A07 8F3 78.jpg](../wiki-images/CPU%3DRP2A07_8F3_78.jpg)](File_CPU_RP2A07_8F3_78_jpg.xhtml) | Input clock divider is 16. M2 duty cycle is 19/32 [[3]](https://forums.nesdev.org/viewtopic.php?p=166761#p166761). Changes to noise, DPCM, frame timer tables. Fixed DPCM RDY address bus glitches. Pin 30 connects to 6502 /RDY input.   
RP2A07A  | [![CPU=RP2A07A 1GM 36.jpg](../wiki-images/CPU%3DRP2A07A_1GM_36.jpg)](File_CPU_RP2A07A_1GM_36_jpg.xhtml) [![CPU=RP2A07A 2JM 3L.jpg](../wiki-images/CPU%3DRP2A07A_2JM_3L.jpg)](File_CPU_RP2A07A_2JM_3L_jpg.xhtml) | no known differences relative to 2A07letterless   
  
## Unofficial

Part  | Picture  | Notes   
---|---|---  
MG-N-501  | [![CPU=MG-N-501 8933.jpg](../wiki-images/CPU%3DMG-N-501_8933.jpg)](File_CPU_MG_N_501_8933_jpg.xhtml) |   
[MG-P-501](https://forums.nesdev.org/viewtopic.php?p=154574#p154574) | [![CPU=MG-P-501 9221S 415521.jpg](../wiki-images/CPU%3DMG-P-501_9221S_415521.jpg)](File_CPU_MG_P_501_9221S_415521_jpg.xhtml) | Micro Genius-made clone. Die has the same (UMC) © Ⓜ B6167F marking as a UA6527P.   
UA6527  | [![CPU=UA6527-8909-BS.jpg](../wiki-images/CPU%3DUA6527-8909-BS.jpg)](File_CPU_UA6527_8909_BS_jpg.xhtml) [![CPU=UA6527-9310-CG-C12520.jpg](../wiki-images/CPU%3DUA6527-9310-CG-C12520.jpg)](File_CPU_UA6527_9310_CG_C12520_jpg.xhtml) | UMC-made clone of 2A03G. Has swapped pulse channel duty cycles. Input clock Divider is 12.   
UA6527P | UMC-made clone of 2A03G for compatibility with NTSC software in PAL countries. Different input clock divider. Still has swapped pulse channel duty cycles. Otherwise believed same as 6527. One revision has (UMC) © Ⓜ B6167F 1989 09 on the die. DMC status bit is cleared 1 APU cycle later than on RP2A03 CPUs. The cause is not known. This changes the timing for [DMC DMA implicit-stop glitches](DMA.xhtml#Bugs "DMA") (the sample must be started 1 APU cycle earlier to trigger the glitches), and it is suspected that it delays DMC IRQ by 1 APU cycle. Noise channel is slightly louder than others.   
[![CPU=UA6527P 8931S.jpg](../wiki-images/CPU%3DUA6527P_8931S.jpg)](File_CPU_UA6527P_8931S_jpg.xhtml) | Runs hot. Revisions without "-" in the date stamp have a ÷16 CPU divider, like 6540 and 2A07   
[![CPU=UA6527P 9019-BS.jpg](../wiki-images/CPU%3DUA6527P_9019-BS.jpg)](File_CPU_UA6527P_9019_BS_jpg.xhtml) | Runs hot. Revisions with "-" in the date stamp have a ÷15 CPU divider   
[![CPU=UA6527P 9214-BS 310551.jpg](../wiki-images/CPU%3DUA6527P_9214-BS_310551.jpg)](File_CPU_UA6527P_9214_BS_310551_jpg.xhtml) | Runs cooler   
UA6527PQ  | [![CPU=UA6527PQ 9306.JPG](../wiki-images/CPU%3DUA6527PQ_9306.JPG)](File_CPU_UA6527PQ_9306_JPG.xhtml) |   
UA6540  | [![CPU=UA6540 8833S.JPG](../wiki-images/CPU%3DUA6540_8833S.JPG)](File_CPU_UA6540_8833S_JPG.xhtml) [![CPU=UA6540-8834S.jpg](../wiki-images/CPU%3DUA6540-8834S.jpg)](File_CPU_UA6540_8834S_jpg.xhtml) | UMC-made clone of 2A07 [[4]](https://forums.nesdev.org/viewtopic.php?t=17257). Has swapped pulse duty cycles. Subsequent research implies this is identical to the early 6527P - NTSC tuning tables, ÷16 CPU divider. [[5]](https://forums.nesdev.org/viewtopic.php?p=283514#p283514)  
UA6547  | [![CPU=UA6547 8950S.JPG](../wiki-images/CPU%3DUA6547_8950S.JPG)](File_CPU_UA6547_8950S_JPG.xhtml) | Believed to be a 100% duplicate of UA6527, for use in PAL-M region.   
UM6557  | [![UM6557.JPG](../wiki-images/UM6557.JPG)](File_UM6557_JPG.xhtml) | Believed to be a 100% duplicate of UA6527, for use in SECAM regions.   
UM6561xx-1  | [![NOAC=UM6561CF-1 0028A E6276.jpg](../wiki-images/NOAC%3DUM6561CF-1_0028A_E6276.jpg)](File_NOAC_UM6561CF_1_0028A_E6276_jpg.xhtml) | NES-on-a-chip for NTSC. Revisions "xx" F, AF, BF, CF known.   
UM6561xx-2  | [![UM6561F-2.jpg](../wiki-images/UM6561F-2.jpg)](File_UM6561F_2_jpg.xhtml) [![NOAC=UM6561 AF-2 9440A R81040.jpg](../wiki-images/NOAC%3DUM6561_AF-2_9440A_R81040.jpg)](File_NOAC_UM6561_AF_2_9440A_R81040_jpg.xhtml) | NES-on-a-chip for PAL-B. Revisions "xx" F, AF, BF, CF known. F and AF revision pulse wave duty cycles match RP2A03, and DMC status bit is cleared 1 APU cycle later than on RP2A03 CPUs. AF revision observed to have incorrect ASR #imm ($4B) behavior, but other stable illegal instructions work properly.   
1818N  | [![CPU = 1818N 9218.png](../wiki-images/CPU_%3D_1818N_9218.png)](File_CPU___1818N_9218_png.xhtml) | ??-made NES-on-a-chip, NTSC timing.   
T1818P  | [![1818P 0.jpeg](../wiki-images/1818P_0.jpeg)](File_1818P_0_jpeg.xhtml) | ??-made NES-on-a-chip[[[6]](https://forums.nesdev.org/viewtopic.php?p=228515#p228515). Requires external 2 KiB RAMs for CPU and PPU. Swapped pulse duty cycles. DMC status bit is cleared 1 APU cycle later than on RP2A03 CPUs.   
TA-03N  | [![CPU=TA-03N 6527 9250.jpg](../wiki-images/CPU%3DTA-03N_6527_9250.jpg)](File_CPU_TA_03N_6527_9250_jpg.xhtml) [![CPU=TA-03N 9172N 12450820.jpg](../wiki-images/CPU%3DTA-03N_9172N_12450820.jpg)](File_CPU_TA_03N_9172N_12450820_jpg.xhtml) | ??-made die-mask clone of 2A03G. Chip underside also has two codes of currently unknown purpose. Pin 30 activates CPU Test Mode like on 2A03G. Clock Divisor is 12. Illegal opcodes are the same. Early 1991 dated chips are reported to have problems with APU DMC playback, but this was corrected in 1992 onward. Runs hot.   
TA-03NP  | [![CPU=TA-03NP EWP0124.jpg](../wiki-images/CPU%3DTA-03NP_EWP0124.jpg)](File_CPU_TA_03NP_EWP0124_jpg.xhtml) | ??-made clone of 2A03G for NTSC compatibility in PAL countries. Input clock divider is 15? But not this one, this input clock divider is 12.   
TA-03NP1  | [![CPU=TA-03NP1 6527P 9231.jpg](../wiki-images/CPU%3DTA-03NP1_6527P_9231.jpg)](File_CPU_TA_03NP1_6527P_9231_jpg.xhtml) | ??-made clone of 2A03G for NTSC compatibility in PAL countries. Input clock divider is 15. Fixed DPCM problems? Correct pulse channel duties. Noise channel is slightly louder than others. DMC status bit is cleared 1 APU cycle later than on RP2A03 CPUs.   
PM03  | [![CPU=PM03 HI25.jpg](../wiki-images/CPU%3DPM03_HI25.jpg)](File_CPU_PM03_HI25_jpg.xhtml) | [Gradiente](https://en.wikipedia.org/wiki/IGB_Eletr%C3%B4nica "wikipedia:IGB Eletrônica")-made clone of 2A03G. [[7]](https://forums.nesdev.org/viewtopic.php?p=195175#p195175)  
GS870007  | [![CPU=GS87007 8827.jpg](../wiki-images/CPU%3DGS87007_8827.jpg)](File_CPU_GS87007_8827_jpg.xhtml) | (Goldstar??)-made clone of 2A03 - has functioning decimal mode? [[8]](http://gxemu.blog67.fc2.com/blog-entry-363.html)  
KC-6005  | [![CPU=KC-6005.jpg](../wiki-images/CPU%3DKC-6005.jpg)](File_CPU_KC_6005_jpg.xhtml) | Found in MT777-DX famiclone, behaves exactly like UA6527P   
6005B  | [![CPU=6005B.jpg](../wiki-images/CPU%3D6005B.jpg)](File_CPU_6005B_jpg.xhtml) |   
2011  | [![CPU=2011.jpg](../wiki-images/CPU%3D2011.jpg)](File_CPU_2011_jpg.xhtml) |   
“2A03E”  | [![CPU=USC 2A03E 9118S 314531.jpg](../wiki-images/CPU%3DUSC_2A03E_9118S_314531.jpg)](File_CPU_USC_2A03E_9118S_314531_jpg.xhtml) [![CPU=2A03E 9122A 422951.jpg](../wiki-images/CPU%3D2A03E_9122A_422951.jpg)](File_CPU_2A03E_9122A_422951_jpg.xhtml) | Both with and without USC insignia   
KP2B03E  | [![CPU=KP2B03E DHG 44.jpg](../wiki-images/CPU%3DKP2B03E_DHG_44.jpg)](File_CPU_KP2B03E_DHG_44_jpg.xhtml) |   
6527-21 P03  | [![CPU=6527-21 P03 91240 5009.png](../wiki-images/CPU%3D6527-21_P03_91240_5009.png)](File_CPU_6527_21_P03_91240_5009_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are correct. Pin 30 is RDY.   
6527  | [![CPU=6527 9132E.png](../wiki-images/CPU%3D6527_9132E.png)](File_CPU_6527_9132E_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are correct. Pin 30 is RDY.   
6527P  | [![CPU=6527P CFE1 1PC.png](../wiki-images/CPU%3D6527P_CFE1_1PC.png)](File_CPU_6527P_CFE1_1PC_png.xhtml) | Clock divider is /16. Pulse channel duty cycles are swapped. Pin 30 is RDY.   
[![CPU=6527P 9040.jpg](../wiki-images/CPU%3D6527P_9040.jpg)](File_CPU_6527P_9040_jpg.xhtml) | Clock divider is /15. Pulse channel duty cycles are correct. Pin 30 is TEST.   
HA6527P  | [![CPU=HA6527P CRI2.png](../wiki-images/CPU%3DHA6527P_CRI2.png)](File_CPU_HA6527P_CRI2_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are swapped. Pin 30 is RDY.   
SENITON 6527P-SS-P03  | [![CPU=SENITON 6527P-SS-P03 2.png](../wiki-images/CPU%3DSENITON_6527P-SS-P03_2.png)](File_CPU_SENITON_6527P_SS_P03_2_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are correct. Pin 30 is TEST.   
SENITON 6527UP-8  | [![CPU=SENITON 6527UP-8.png](../wiki-images/CPU%3DSENITON_6527UP-8.png)](File_CPU_SENITON_6527UP_8_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are swapped. Pin 30 is RDY.   
SENITON 6527AP  | [![CPU=SENITON 6527AP 9109C.png](../wiki-images/CPU%3DSENITON_6527AP_9109C.png)](File_CPU_SENITON_6527AP_9109C_png.xhtml) | Clock divider is /16. Pulse channel duty cycles are swapped. Pin 30 is RDY.   
SL/WH6527AP  | [![CPU=SL-WH6527AP 9120.png](../wiki-images/CPU%3DSL-WH6527AP_9120.png)](File_CPU_SL_WH6527AP_9120_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are swapped. Pin 30 is RDY.   
SNC6527P  | [![CPU=SNC6527P 9104.png](../wiki-images/CPU%3DSNC6527P_9104.png)](File_CPU_SNC6527P_9104_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are correct. Pin 30 is TEST.   
XYZ-6783  | [![CPU = XYZ-6783 9129.jpg](../wiki-images/CPU_%3D_XYZ-6783_9129.jpg)](File_CPU___XYZ_6783_9129_jpg.xhtml) | Lacks tonal noise mode like original RP2A03, but resets APU Frame Counter on console reset like 2A03E/2A03G. Otherwise behaves like letterless RP2A03.   
6538N  | [![CPU = 6538N CW6B 3NQ.jpg](../wiki-images/CPU_%3D_6538N_CW6B_3NQ.jpg)](File_CPU___6538N_CW6B_3NQ_jpg.xhtml) | ??-made CPU, despite the part number being similar to UMC PPU. Has inverted duty cycles like UA6527. DPCM works.   
8Z01N  | [![8Z01N CPU.jpg](../wiki-images/8Z01N_CPU.jpg)](File_8Z01N_CPU_jpg.xhtml) |   
TECH 27  | [![CPU=TECH 27.png](../wiki-images/CPU%3DTECH_27.png)](File_CPU_TECH_27_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are correct. Pin 30 is TEST.   
HITEX-6527P-P03 GX  | [![CPU=HITEX-6527P-P03 GX 9108.png](../wiki-images/CPU%3DHITEX-6527P-P03_GX_9108.png)](File_CPU_HITEX_6527P_P03_GX_9108_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are correct. Pin 30 is TEST.   
WDL6527P  | [![CPU=WDL6527P 9213.png](../wiki-images/CPU%3DWDL6527P_9213.png)](File_CPU_WDL6527P_9213_png.xhtml) | Clock divider is /15. Pulse channel duty cycles are correct. Pin 30 is RDY.   
  
If you know of other differences or other revisions, please add them! 

## See also

  * [PPU variants](PPU_variants.xhtml "PPU variants")
  * <https://forums.nesdev.org/viewtopic.php?p=45889#p45889>
  * <https://forums.nesdev.org/viewtopic.php?t=23916> (More Info on CPU Clones)
  * <https://forums.nesdev.org/viewtopic.php?t=23682> (Lots of Images and die-shots)


