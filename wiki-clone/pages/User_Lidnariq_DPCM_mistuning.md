# User:Lidnariq/DPCM mistuning

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ALidnariq/DPCM_mistuning) | View [other pages](Special_AllPages.xhtml#User_Lidnariq_DPCM_mistuning)

How to read the tables: 

Pick the table for your system. Find the rate (written to $4010) on the top or side. Numbers inside are measured in cents of detuning relative to the other rate. Red and blue indicate rates that will sound noticeably flat or sharp, using a [Just-noticeable difference](https://en.wikipedia.org/wiki/Just-noticeable_difference "wikipedia:Just-noticeable difference") of 6 cents. Numbers in green specify the number of other rates that are in tune with the one selected. 

kHz | 33.14 | 24.86 | 21.31 | 16.88 | 13.98 | 12.60 | 11.19 | 9.42 | 8.36 | 7.92 | 7.05 | 6.26 | 5.59 | 5.26 | 4.71 | 4.18   
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
period | 54 | 72 | 84 | 106 | 128 | 142 | 160 | 190 | 214 | 226 | 254 | 286 | 320 | 340 | 380 | 428   
NTSC | $F | $E | $D | $C | $B | $A | $9 | $8 | $7 | $6 | $5 | $4 | $3 | $2 | $1 | $0   
**$0** | 16.1 | 14.1 | -19.0 | -16.3 | 10.2 | -10.1 | -3.4 | -5.9 | 0.0 | -5.5 | -3.3 | 2.1 | -3.4 | 1.5 | -5.9 | 9   
**$1** | 22.0 | 20.1 | -13.0 | -10.3 | 16.2 | -4.1 | 2.5 | 0.0 | 5.9 | 0.4 | 2.6 | 8.0 | 2.5 | 7.4 | 8   
**$2** | 14.6 | 12.6 | -20.5 | -17.8 | 8.7 | -11.6 | -5.0 | -7.4 | -1.5 | -7.1 | -4.8 | 0.6 | -5.0 | 6   
**$3** | 19.6 | 17.6 | -15.5 | -12.8 | 13.7 | -6.6 | 0.0 | -2.5 | 3.4 | -2.1 | 0.1 | 5.5 | 9   
**$4** | 14.0 | 12.1 | -21.1 | -18.3 | 8.2 | -12.1 | -5.5 | -8.0 | -2.1 | -7.6 | -5.4 | 6   
**$5** | 19.4 | 17.5 | -15.6 | -12.9 | 13.6 | -6.7 | -0.1 | -2.6 | 3.3 | -2.2 | 9   
**$6** | 21.7 | 19.7 | -13.4 | -10.7 | 15.8 | -4.5 | 2.1 | -0.4 | 5.5 | 8   
**$7** | 16.1 | 14.1 | -19.0 | -16.3 | 10.2 | -10.1 | -3.4 | -5.9 | 9   
**$8** | 22.0 | 20.1 | -13.0 | -10.3 | 16.2 | -4.1 | 2.5 | 8   
**$9** | 19.6 | 17.6 | -15.5 | -12.8 | 13.7 | -6.6 | 9   
**$A** | 26.2 | 24.2 | -8.9 | -6.2 | 20.3 | 3   
**$B** | 5.9 | 3.9 | -29.2 | -26.5 | 2   
**$C** | 32.4 | 30.4 | -2.7 | 1   
**$D** | 35.1 | 33.1 | 1   
**$E** | 2.0 | 2   
**$F** | 2   
kHz  | 33.25 | 25.19 | 21.32 | 16.97 | 14.09 | 12.60 | 11.23 | 9.45 | 8.40 | 7.92 | 7.04 | 6.02 | 5.58 | 5.26 | 4.70 | 4.18   
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
period | 50 | 66 | 78 | 98 | 118 | 132 | 148 | 176 | 198 | 210 | 236 | 276 | 298 | 316 | 354 | 398   
PAL | $F | $E | $D | $C | $B | $A | $9 | $8 | $7 | $6 | $5 | $4 | $3 | $2 | $1 | $0   
**$0** | 8.7 | -10.7 | -21.5 | -26.3 | -4.8 | -10.7 | -12.6 | -12.6 | -8.7 | -6.9 | -4.8 | -33.7 | -0.9 | 0.6 | -2.8 | 5   
**$1** | 11.5 | -7.9 | -18.6 | -23.5 | -2.0 | -7.9 | -9.8 | -9.8 | -5.9 | -4.0 | -2.0 | -30.9 | 1.9 | 3.4 | 7   
**$2** | 8.1 | -11.3 | -22.1 | -26.9 | -5.4 | -11.3 | -13.2 | -13.2 | -9.3 | -7.4 | -5.4 | -34.3 | -1.5 | 5   
**$3** | 9.6 | -9.7 | -20.5 | -25.4 | -3.8 | -9.7 | -11.7 | -11.7 | -7.8 | -5.9 | -3.8 | -32.8 | 6   
**$4** | 42.4 | 23.0 | 12.3 | 7.4 | 28.9 | 23.0 | 21.1 | 21.1 | 25.0 | 26.9 | 28.9 | 0   
**$5** | 13.5 | -5.9 | -16.7 | -21.5 | 0.0 | -5.9 | -7.8 | -7.9 | -3.9 | -2.1 | 9   
**$6** | 15.5 | -3.8 | -14.6 | -19.4 | 2.1 | -3.8 | -5.8 | -5.8 | -1.9 | 9   
**$7** | 17.4 | -2.0 | -12.7 | -17.6 | 3.9 | -2.0 | -3.9 | -3.9 | 8   
**$8** | 21.3 | 2.0 | -8.8 | -13.7 | 7.9 | 2.0 | 0.0 | 5   
**$9** | 21.3 | 1.9 | -8.9 | -13.7 | 7.8 | 1.9 | 5   
**$A** | 19.4 | 0.0 | -10.8 | -15.6 | 5.9 | 7   
**$B** | 13.5 | -5.9 | -16.7 | -21.5 | 9   
**$C** | 35.0 | 15.6 | 4.8 | 1   
**$D** | 30.1 | 10.8 | 1   
**$E** | 19.4 | 7   
**$F** | 0   
  
  


In a fictional alternate universe, where the DPCM period tables had been chosen to maximize in-tune choices, perhaps the periods would have instead been some subset of the following rates: 

interval | P1 | m2 | M2 | m3 | M3 | P4 | d5 | P5 | m6 | M6 | m7 | M7   
---|---|---|---|---|---|---|---|---|---|---|---|---  
period | 12 |  |  |  |  | 16 |  | 18 |  |  |  |   
24 |  |  |  |  | 32 | 34 | 36 | 38 |  |  |   
48 |  | 54 |  |  | 64 | 68 | 72 | 76 |  |  |   
96 | 102 | 108 | 114 |  | 128 | 136 | 144 | 152 | 162 |  |   
192 | 204 | 216 | 228 | 242 | 256 | 272 | 288 | 304 | 322 | 342 | 362   
  
All of these are even and within 6 cents of an exact number of semitones relative to period 48. 

Better would be to have the user specify the actual divider, for numbers from 2 to 512. Numbers less than 50 may have a reason to be missing from the current table, so maybe some workaround (prohibition or offset) would be warranted. the IRQ and Loop flags would have to be moved for this behavior, though. 
