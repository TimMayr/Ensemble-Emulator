# VT02+ Registers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VT02%2B_Registers) | View [other pages](Special_AllPages.xhtml#VT02__Registers)

The [VT03](VTxx.xhtml "VT03") is a famiclone with added capabilities. It uses "OneBus" instead of separate CPU and PPU buses, and supports a 4-bit color mode. 

## Contents

  * 1 Registers
    * 1.1 System Registers
      * 1.1.1 Program Bank 1, Video Bank 2 ($4100) > write
      * 1.1.2 Timer Interrupt Preload Times ($4101) > write
      * 1.1.3 Load Timer Interrupt Value ($4102) > write
      * 1.1.4 Timer Interrupt Disable ($4103) > write
      * 1.1.5 Timer Interrupt Enable ($4104) > write
      * 1.1.6 V Bank 0 decode type, P Bank 0 decode type, Inter Char VRAM ($4105) > write
        * 1.1.6.1 Program Bank 0 Decoding
        * 1.1.6.2 Video Bank 0 Decoding
      * 1.1.7 Horizontal/Vertical Scrolling Selector ($4106) > write
      * 1.1.8 Program Bank 0 Register 0 ($4107) > write
      * 1.1.9 Program Bank 0 Register 1 ($4108) > write
      * 1.1.10 Program Bank 0 Register 2 ($4109) > write
      * 1.1.11 Program Bank 0 Register 3 ($410A) > write
      * 1.1.12 Timer interrupt clock selector, Program Bank 0 register 2 enable/disable, RS232 enable/disable, Bus output normal/tristate, Program Bank 0 selector ($410B) > write
      * 1.1.13 I/O Port Control ($410D) > write
      * 1.1.14 I/O Port Output Data, Ports 0 and 1 ($410E) <> read/write
      * 1.1.15 I/O Port Output Data, Ports 2 and 3 ($410F) <> read/write
      * 1.1.16 RS232 Timer
        * 1.1.16.1 RS232 Timer Low Byte ($4114) > write
        * 1.1.16.2 RS232 Timer High Byte ($4115) > write
      * 1.1.17 RS232 Register ($4119) > write
      * 1.1.18 RS232 Flags ($4119) < read
      * 1.1.19 RS232 TX data ($411A) > write
      * 1.1.20 RS232 RX data ($411B) < read
    * 1.2 Graphics Registers
      * 1.2.1 PPU Control ($2000) > write
      * 1.2.2 PPU Mask ($2001) > write
      * 1.2.3 PPU Status ($2002) < read
      * 1.2.4 OAM Address ($2003) > write
      * 1.2.5 OAM Data ($2004) > write
      * 1.2.6 PPU Scroll ($2005) >> write x2
      * 1.2.7 PPU Address ($2006) >> write x2
      * 1.2.8 PPU Data ($2007) <> read/write
      * 1.2.9 Extended Graphics Control 1 ($2010) > write
        * 1.2.9.1 New Color Mapping Mode
      * 1.2.10 Extended Graphics Control 2 ($2011) > write
      * 1.2.11 Video Bank 0 Register 0 ($2012) > write
      * 1.2.12 Video Bank 0 Register 1 ($2013) > write
      * 1.2.13 Video Bank 0 Register 2 ($2014) > write
      * 1.2.14 Video Bank 0 Register 3 ($2015) > write
      * 1.2.15 Video Bank 0 Register 4 ($2016) > write
      * 1.2.16 Video Bank 0 Register 5 ($2017) > write
      * 1.2.17 Video Bank 1 Register, BKPAGE, Video RW Bank ($2018) > write
      * 1.2.18 Gun Port Reset ($2019) > write
      * 1.2.19 Video Bank 0 Register 6, Video Bank 0 Selector ($201A) > write
      * 1.2.20 Gun Port 1 X Coordinate ($201C) < read
      * 1.2.21 Gun Port 1 Y Coordinate ($201D) < read
      * 1.2.22 Gun Port 2 X Coordinate ($201E) < read
      * 1.2.23 Gun Port 2 Y Coordinate ($201F) < read
    * 1.3 Sound Registers
    * 1.4 Miscellaneous Registers
      * 1.4.1 DMA Source Address High Byte ($4014) > write
      * 1.4.2 DMA Settings ($4034) > write
      * 1.4.3 Enable/Disable XOP1 & DWS IRQ ($4015) > write
      * 1.4.4 Read XOP1 Flag ($4015) < read
      * 1.4.5 Enable/Disable XOP2 ($4035) > write
      * 1.4.6 Read XOP2 Flag ($4035) < read
      * 1.4.7 Set Output Pin XQ[2:0] ($4016) > write
      * 1.4.8 Read peripheral data ($4016) < read
      * 1.4.9 APU Frame Counter ($4017) > write
      * 1.4.10 Read peripheral data ($4017) < read
  * 2 References



## Registers

### System Registers

#### Program Bank 1, Video Bank 2 ($4100) > write
    
    
    7  bit  0
    ---- ----
    PPPP VVVV
    |||| ||||
    |||| ++++- Video Bank 2   (VA24, VA23, VA22, VA21)
    ++++------ Program Bank 1 (PA24, PA23, PA22, PA21)
    

#### Timer Interrupt Preload Times ($4101) > write

When TSYNEN is 0, this register sets "the number of AD12 switching high low". When TSYNEN is 1, this register sets "the number of HSYNC switching high low" instead. (See $410B.) 

#### Load Timer Interrupt Value ($4102) > write

Writing any value to this register will set the timer's value and start counting. 

#### Timer Interrupt Disable ($4103) > write

Writing any value to this register will disable the timer interrupt. 

#### Timer Interrupt Enable ($4104) > write

Writing any value to this register will enable the timer interrupt. 

#### V Bank 0 decode type, P Bank 0 decode type, Inter Char VRAM ($4105) > write
    
    
    7  bit  0
    ---- ----
    VPIx xxxx
    |||| ||||
    |||+-++++- unused
    ||+------- Internal VRAM as CHR RAM (0: disabled; 1: enabled)
    |+-------- Program Bank 0 Decoding (see below table)
    +--------- Video Bank 0 Decoding (see below table)
    

##### Program Bank 0 Decoding

PQ2EN ($410B) | COMR6 | A[14:13] (CPU) | TPA20 | TPA19 | TPA18 | TPA17 | TPA16 | TPA15 | TPA14 | TPA13   
---|---|---|---|---|---|---|---|---|---|---  
0  | 0  | PQ07 | PQ06 | PQ05 | PQ04 | PQ03 | PQ02 | PQ01 | PQ00   
0  | 1  | PQ17 | PQ16 | PQ15 | PQ14 | PQ13 | PQ12 | PQ11 | PQ10   
0  | 2  | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 0   
0  | 3  | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1   
0  | 4  | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 0   
0  | 5  | PQ17 | PQ16 | PQ15 | PQ14 | PQ13 | PQ12 | PQ11 | PQ10   
0  | 6  | PQ07 | PQ06 | PQ05 | PQ04 | PQ03 | PQ02 | PQ01 | PQ00   
0  | 7  | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1   
1  | 0  | PQ07 | PQ06 | PQ05 | PQ04 | PQ03 | PQ02 | PQ01 | PQ00   
1  | 1  | PQ17 | PQ16 | PQ15 | PQ14 | PQ13 | PQ12 | PQ11 | PQ10   
1  | 2  | PQ27 | PQ26 | PQ25 | PQ24 | PQ23 | PQ22 | PQ21 | PQ20   
1  | 3  | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1   
1  | 4  | PQ27 | PQ26 | PQ25 | PQ24 | PQ23 | PQ22 | PQ21 | PQ20   
1  | 5  | PQ17 | PQ16 | PQ15 | PQ14 | PQ13 | PQ12 | PQ11 | PQ10   
1  | 6  | PQ07 | PQ06 | PQ05 | PQ04 | PQ03 | PQ02 | PQ01 | PQ00   
1  | 7  | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1   
  
##### Video Bank 0 Decoding

COMR7 | AD[12:10] | TVA17 | TVA16 | TVA15 | TVA14 | TVA13 | TVA12 | TVA11 | TVA10   
---|---|---|---|---|---|---|---|---|---  
0,1,C,D  | RV47 | RV46 | RV45 | RV44 | RV43 | RV42 | RV41 | AD10   
2,3,E,F  | RV57 | RV56 | RV55 | RV54 | RV53 | RV52 | RV51 | AD10   
4,8  | RV07 | RV06 | RV05 | RV04 | RV03 | RV02 | RV01 | RV00   
5,9  | RV17 | RV16 | RV15 | RV14 | RV13 | RV12 | RV11 | RV10   
6,A  | RV27 | RV26 | RV25 | RV24 | RV23 | RV22 | RV21 | RV20   
7,B  | RV37 | RV36 | RV35 | RV34 | RV33 | RV32 | RV31 | RV30   
  
#### Horizontal/Vertical Scrolling Selector ($4106) > write
    
    
    7  bit  0
    ---- ----
    VPIx xxxx
    |||| ||||
    |||| |||+- H/V scrolling selector (0: Horizontal; 1: Vertical)
    ++++-+++-- unused
    

#### Program Bank 0 Register 0 ($4107) > write
    
    
    7  bit  0
    ---- ----
    PPPP PPPP
    |||| ||||
    |||| |||+- PQ00
    |||| ||+-- PQ01
    |||| |+--- PQ02
    |||| +---- PQ03
    |||+------ PQ04
    ||+------- PQ05
    |+-------- PQ06
    +--------- PQ07
    

#### Program Bank 0 Register 1 ($4108) > write
    
    
    7  bit  0
    ---- ----
    PPPP PPPP
    |||| ||||
    |||| |||+- PQ10
    |||| ||+-- PQ11
    |||| |+--- PQ12
    |||| +---- PQ13
    |||+------ PQ14
    ||+------- PQ15
    |+-------- PQ16
    +--------- PQ17
    

#### Program Bank 0 Register 2 ($4109) > write
    
    
    7  bit  0
    ---- ----
    PPPP PPPP
    |||| ||||
    |||| |||+- PQ20
    |||| ||+-- PQ21
    |||| |+--- PQ22
    |||| +---- PQ23
    |||+------ PQ24
    ||+------- PQ25
    |+-------- PQ26
    +--------- PQ27
    

#### Program Bank 0 Register 3 ($410A) > write
    
    
    7  bit  0
    ---- ----
    PPPP PPPP
    |||| ||||
    |||| |||+- PQ30
    |||| ||+-- PQ31
    |||| |+--- PQ32
    |||| +---- PQ33
    |||+------ PQ34
    ||+------- PQ35
    |+-------- PQ36
    +--------- PQ37
    

#### Timer interrupt clock selector, Program Bank 0 register 2 enable/disable, RS232 enable/disable, Bus output normal/tristate, Program Bank 0 selector ($410B) > write
    
    
    7  bit  0
    ---- ----
    TpRB XPPP
    |||| ||||
    |||| |+++- Program Bank 0 Selector
    |||| +---- XRWB switch (0: Writes to $6000-$FFFF will not activate XRWB; 1: writes to $6000-$FFFF will activate XRWB)
    |||+------ Bus output control (0: Normal bus output; 1: Tri-state bus)
    ||+------- RS232 (0: disabled; 1: enabled)
    |+-------- Program Bank 0 Register 2 enable (0: disabled; 1: enabled)
    +--------- Timer interrupt clock select (0: AD12; 1: HSYNC)
    

#### I/O Port Control ($410D) > write
    
    
    7  bit  0
    ---- ----
    DdCc BbAa
    |||| ||||
    |||| |||+- I/O port 0 mode (0: input; 1: output)
    |||| ||+-- I/O port 0 enable (0: disabled; 1:enabled)
    |||| |+--- I/O port 1 mode (0: input; 1: output)
    |||| +---- I/O port 1 enable (0: disabled; 1:enabled)
    |||+------ I/O port 2 mode (0: input; 1: output)
    ||+------- I/O port 2 enable (0: disabled; 1:enabled)
    |+-------- I/O port 3 mode (0: input; 1: output)
    +--------- I/O port 3 enable (0: disabled; 1:enabled)
    

  * Bits d0-d3 must be set to $A if using flash memory in 16-bit mode.
  * External SRAM is not available if using flash memory in 16-bit mode.



#### I/O Port Output Data, Ports 0 and 1 ($410E) <> read/write
    
    
    7  bit  0
    ---- ----
    DDDD dddd
    |||| ||||
    |||| ++++- I/O port 0 data (XVD3, XVD2, XVD1, XVD0)
    ++++------ I/O port 1 data (XVD7, XVD6, XVD5, XVD4)
    

#### I/O Port Output Data, Ports 2 and 3 ($410F) <> read/write
    
    
    7  bit  0
    ---- ----
    DDDD dddd
    |||| ||||
    |||| ++++- I/O port 2 data (XAD12, XAD11, XAD10, XRA10)
    ++++------ I/O port 3 data (XVRW, VXOEB, XRCB, XRC)
    

#### RS232 Timer

"In PAL system, CK21M is 26.601712MHz, in NTSC system is 21.47727MHz. RS232T=#4115,#4114 data. Baud rate will be CK21M/((RS232T+2)*2). For example, In PAL system, the baud rate 9600, RS232T=0567."

##### RS232 Timer Low Byte ($4114) > write

Write the low byte of the RS232 timer. 

##### RS232 Timer High Byte ($4115) > write

Write the high byte of the RS232 timer. 

#### RS232 Register ($4119) > write
    
    
    7  bit  0
    ---- ----
    xxBx xxxT
    |||| ||||
    |||| |||+- TX bit 8
    |||+-+++-- unused
    ||+------- Bit 8 enable (0: 10 bits mode including start, end bits, and bit 7-0 data; 1: 11 bits mode including start, end bits, bit 8 and bit7-0 data)
    ++-------- unused
    

#### RS232 Flags ($4119) < read
    
    
    7  bit  0
    ---- ----
    rsTF PxER
    |||| ||||
    |||| |||+- RX bit 8
    |||| ||+-- RERRF (If 1, receiving error occurred)
    |||| |+--- unused
    |||| +---- XPORN (0: NTSC; 1: PAL)
    |||+------ XF5OR6 (0: 60Hz; 1: 50Hz)
    ||+------- RINGF (If 1, currently receiving data)
    |+-------- TIFLAG (If 1, completed sending data)
    +--------- RIFLAG (If 1, completed receiving data)
    

#### RS232 TX data ($411A) > write

Write RS232 data (bits 7-0). 

#### RS232 RX data ($411B) < read

Read RS232 data (bits 7-0). 

### Graphics Registers

#### PPU Control ($2000) > write
    
    
    7  bit  0
    ---- ----
    VxHB SINN
    |||| ||||
    |||| ||++- Base nametable address
    |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    |||| |+--- VRAM address increment (0: +1/horizontal; 1: +32/vertical)
    |||| +---- Sprite pattern table address (0: $0000; 1: $1000)
    |||+------ Background pattern table address (0: $0000; 1: $1000)
    ||+------- Sprite Size (0: Small Sprite (8x8 or 16x8); 1: Big Sprite (8x16 or 16x16))
    |+-------- unused ("Test pin control I/O")
    +--------- NMI control (0: enabled; 1: disabled)
    

#### PPU Mask ($2001) > write
    
    
    7  bit  0
    ---- ----
    xxxs bMmG
    |||| ||||
    |||| |||+- Grayscale (0: Color; 1: Grayscale)
    |||| ||+-- "Background initial coordinate" (0: "righter"; 1: "lefter")
    |||| |+--- "Sprite initial coordinate" (0: "righter"; 1: "lefter")
    |||| +---- Show background (0: disabled; 1: enabled)
    |||+------ Show sprites (0: disabled; 1: enabled)
    +++------- unused
    

#### PPU Status ($2002) < read
    
    
    7  bit  0
    ---- ----
    VSOx xxxx
    |||| ||||
    |||+-++++- unused
    ||+------- Sprite overload indicator
    |+-------- "Priority indicator"
    +--------- Blanking indicator (0: display; 1: blanking)
    

#### OAM Address ($2003) > write

"Set sprite pool counter initial data by this register."
    
    
    7  bit  0
    ---- ----
    AAAA AAAA
    |||| ||||
    ++++-++++- Initial sprite data address
    

#### OAM Data ($2004) > write

"Write data into sprite pool and increment sprite counter."
    
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    ++++-++++- Data to write
    

#### PPU Scroll ($2005) >> write x2

#### PPU Address ($2006) >> write x2

#### PPU Data ($2007) <> read/write

#### Extended Graphics Control 1 ($2010) > write

"Old color compatible, Background/Sprite address Extension enable, Sprite 16 colors or 16 pixels enable, Background 16 colors enable, Sprite 16 colors or 16 pixels selector."
    
    
    7  bit  0
    ---- ----
    Cxxb sSBM
    |||| ||||
    |||| |||+- Sprite mode (0: 16 colors; 1: 16 pixels)
    |||| ||+-- 16 color backgrounds (0: disabled, 1: enabled)
    |||| |+--- 16 color/pixel sprites (0: disabled, 1: enabled)
    |||| +---- Sprite address extension (0: disabled; 1: enabled)
    |||+------ Background address extension (0: disabled; 1: enabled)
    |++------- unused
    +--------- Color Mode (0: old/compatible; 1: new color mapping)
    

##### New Color Mapping Mode

RC=1  | $3F80,$3F81,$3F82...  | $3F00,$3F01,$3F02...   
---|---|---  
Data  | D5 | D4 | D3 | D2 | D1 | D0  | D5 | D4 | D3 | D2 | D1 | D0   
Function  | SAT3 | SAT2 | SAT1 | SAT0 | LUM3 | LUM2  | LUM1 | LUM0 | PHA3 | PHA2 | PHA1 | PHA0   
  
> 
>     4 <= LUM[3:0] X 2 + SAT[3:0] <= 1F
>     If you set LUM = F, SAT must be <= 1.
>     LUM = E, SAT must be <= 3.
>     .
>     .
>     .
>     LUM = 3, SAT must be <= 2.
>     LUM = 2, SAT must be = 0.

#### Extended Graphics Control 2 ($2011) > write

"Option of Vertical line number of LCD display, B/W 2 color mode, Composted Video DA Enable, Video Extension Address EVA12 selector, Enable the internal VRAM or not"
    
    
    7  bit  0
    ---- ----
    xxVV IBCE
    |||| ||||
    |||| |||+- Video Extension Address EVA selector (0: reg. BKPAGE; 1: HV)
    |||| ||+-- Composited Video DA (0: enabled; 1: disabled)
    |||| |+--- Black/White two color mode (0: disabled; 1: enabled)
    |||| +---- Internal VRAM enable (0: enabled; 1: disabled)
    ||++------ LCD display vertical lines (0: 240 lines; 1: 160 lines; 2: 120 lines; 3: 80 lines)
    ++-------- unused
    

#### Video Bank 0 Register 0 ($2012) > write
    
    
    7  bit  0
    ---- ----
    7654 3210
    |||| ||||
    |||| |||+- RV00
    |||| ||+-- RV01
    |||| |+--- RV02
    |||| +---- RV03
    |||+------ RV04
    ||+------- RV05
    |+-------- RV06
    +--------- RV07
    

#### Video Bank 0 Register 1 ($2013) > write
    
    
    7  bit  0
    ---- ----
    7654 3210
    |||| ||||
    |||| |||+- RV10
    |||| ||+-- RV11
    |||| |+--- RV12
    |||| +---- RV13
    |||+------ RV14
    ||+------- RV15
    |+-------- RV16
    +--------- RV17
    

#### Video Bank 0 Register 2 ($2014) > write
    
    
    7  bit  0
    ---- ----
    7654 3210
    |||| ||||
    |||| |||+- RV20
    |||| ||+-- RV21
    |||| |+--- RV22
    |||| +---- RV23
    |||+------ RV24
    ||+------- RV25
    |+-------- RV26
    +--------- RV27
    

#### Video Bank 0 Register 3 ($2015) > write
    
    
    7  bit  0
    ---- ----
    7654 3210
    |||| ||||
    |||| |||+- RV30
    |||| ||+-- RV31
    |||| |+--- RV32
    |||| +---- RV33
    |||+------ RV34
    ||+------- RV35
    |+-------- RV36
    +--------- RV37
    

#### Video Bank 0 Register 4 ($2016) > write
    
    
    7  bit  0
    ---- ----
    7654 3210
    |||| ||||
    |||| |||+- RV40
    |||| ||+-- RV41
    |||| |+--- RV42
    |||| +---- RV43
    |||+------ RV44
    ||+------- RV45
    |+-------- RV46
    +--------- RV47
    

#### Video Bank 0 Register 5 ($2017) > write
    
    
    7  bit  0
    ---- ----
    7654 3210
    |||| ||||
    |||| |||+- RV50
    |||| ||+-- RV51
    |||| |+--- RV52
    |||| +---- RV53
    |||+------ RV54
    ||+------- RV55
    |+-------- RV56
    +--------- RV57
    

#### Video Bank 1 Register, BKPAGE, Video RW Bank ($2018) > write
    
    
    7  bit  0
    ---- ----
    xRRR BVVV
    |||| ||||
    |||| |+++- Video bank when accessing video data
    |||| +---- Reg. BKPAGE address is EVA12 when EVA12S=0
    |+++------ Video Bank 1 Register
    +--------- unused
    

#### Gun Port Reset ($2019) > write

Writing any value to this register will reset the X and Y coordinates of the guns in ports 1 and 2. 

#### Video Bank 0 Register 6, Video Bank 0 Selector ($201A) > write
    
    
    7  bit  0
    ---- ----
    RRRR RBBB
    |||| ||||
    |||| |+++- Video Bank 0 selector
    ++++-+---- Video Bank 0 register 6
    

#### Gun Port 1 X Coordinate ($201C) < read
    
    
    7  bit  0
    ---- ----
    XXXX XXXX
    |||| ||||
    ++++-++++- Gun port 1 X coordinate
    

#### Gun Port 1 Y Coordinate ($201D) < read
    
    
    7  bit  0
    ---- ----
    YYYY YYYY
    |||| ||||
    ++++-++++- Gun port 1 Y coordinate
    

#### Gun Port 2 X Coordinate ($201E) < read
    
    
    7  bit  0
    ---- ----
    XXXX XXXX
    |||| ||||
    ++++-++++- Gun port 2 X coordinate
    

#### Gun Port 2 Y Coordinate ($201F) < read
    
    
    7  bit  0
    ---- ----
    YYYY YYYY
    |||| ||||
    ++++-++++- Gun port 2 Y coordinate
    

### Sound Registers

### Miscellaneous Registers

#### DMA Source Address High Byte ($4014) > write

Two bytes are required to set a DMA source address. This register sets the high byte of the address ($[XX]X0). Writing to this register also starts the DMA. The VT03 is capable of DMA for both video and sprite data; see register $4034 for more details. 
    
    
    7  bit  0
    ---- ----
    AAAA AAAA
    |||| ||||
    ++++-++++- DMA source address high byte
    

#### DMA Settings ($4034) > write
    
    
    7  bit  0
    ---- ----
    SSSS LLLT
    |||| ||||
    |||| |||+- DMA type (0: Sprite data, using $2004; 1: Video data, using $2007)
    |||| +++-- DMA length (000: 256 bytes; 100: 16 bytes; 101: 32 bytes; 110: 64 bytes; 111: 128 bytes)
    +++------- DMA source address low byte (d7-d4)
    

"Under 64byte mode, VT03 cut the memory into 4 pieces. If you want to access complete 64 bytes. The low byte of address must be 00H, 40H, 80H or C0H, because VT03 will stop accessing when address counted to 3FH, 7FH, BFH or FFH respectively. Under 16 byte mode, VT03 cut the memory into 16 pieces. Under 128 byte mode, VT03 cut the memory into 2 pieces."

#### Enable/Disable XOP1 & DWS IRQ ($4015) > write

Similar to [$4015 on the NES APU](APU.xhtml#Status_.28.244015.29 "APU"). 
    
    
    7  bit  0
    ---- ----
    xxxD NT21
    |||| ||||
    |||| |||+- Rhythm/Pulse A (0: stop; 1: start)
    |||| ||+-- Rhythm/Pulse B (0: stop; 1: start)
    |||| |+--- Envelope/Triangle (0: stop; 1: start)
    |||| +---- Noise (0: stop; 1: start)
    |||+------ DWS/PCM (0: stop; 1: start)
    +++------- unused
    

#### Read XOP1 Flag ($4015) < read

Similar to [[on the NES APU](http://wiki.nesdev.org/w/index.php/APU#Status_.28.244015.29%7C%244015)]. 
    
    
    7  bit  0
    ---- ----
    IFxD NT21
    |||| ||||
    |||| |||+- Rhythm/Pulse A (0: ended; 1: playing)
    |||| ||+-- Rhythm/Pulse B (0: ended; 1: playing)
    |||| |+--- Envelope/Triangle (0: ended; 1: playing)
    |||| +---- Noise (0: ended; 1: playing)
    |||+------ DWS/PCM (0: ended; 1: playing)
    ||+------- unused
    |+-------- IRQ (0: inactive; 1: active)
    +--------- DWS/PCM IRQ (0: inactive; 1: active)
    

#### Enable/Disable XOP2 ($4035) > write
    
    
    7  bit  0
    ---- ----
    xxxx NT21
    |||| ||||
    |||| |||+- Rhythm/Pulse A (0: stop; 1: start)
    |||| ||+-- Rhythm/Pulse B (0: stop; 1: start)
    |||| |+--- Envelope/Triangle (0: stop; 1: start)
    |||| +---- Noise (0: stop; 1: start)
    ++++------ unused
    

#### Read XOP2 Flag ($4035) < read
    
    
    7  bit  0
    ---- ----
    xxxx NT21
    |||| ||||
    |||| |||+- Rhythm/Pulse A (0: ended; 1: playing)
    |||| ||+-- Rhythm/Pulse B (0: ended; 1: playing)
    |||| |+--- Envelope/Triangle (0: ended; 1: playing)
    |||| +---- Noise (0: ended; 1: playing)
    ++++------ unused
    

#### Set Output Pin XQ[2:0] ($4016) > write
    
    
    7  bit  0
    ---- ----
    xxxx x210
    |||| ||||
    |||| |||+- Set output pin XQ0
    |||| ||+-- Set output pin XQ1
    |||| |+--- Set output pin XQ2
    ++++-+---- unused
    

#### Read peripheral data ($4016) < read
    
    
    7  bit  0
    ---- ----
    xxxx xMFJ
    |||| ||||
    |||| |||+- First joystick data
    |||| ||+-- Floppy disk data
    |||| |+--- Microphone data
    ++++-+---- unused
    

#### APU Frame Counter ($4017) > write
    
    
    7  bit  0
    ---- ----
    MIxx xxxx
    |||| ||||
    ||++-++++- unused
    |+-------- Clock IRQ (0: enabled, 60Hz; 1: disabled)
    +--------- BLCK1/BLCK2 (0: BLCK1=250Hz, BLCK2=120Hz; 1: BLCK1=200Hz, BLCK2=100Hz)
    

#### Read peripheral data ($4017) < read
    
    
    7  bit  0
    ---- ----
    xxxF FFFJ
    |||| ||||
    |||| |||+- Second joystick data
    |||+-+++-- Floppy disk data
    +++------- unused
    

## References

  * [Datasheets](http://www.vrt.com.tw/datasheet.htm)
  * [Downloads](http://www.vrt.com.tw/download.htm) (including EmuVT)



[Template:Stub](https://www.nesdev.org/w/index.php?title=Template:Stub&action=edit&redlink=1 "Template:Stub \(page does not exist\)")
