# User:Zzo38/DotFami

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/DotFami) | View [other pages](Special_AllPages.xhtml#User_Zzo38_DotFami)

This is a draft for `.fami` format of NES/Famicom ROM image files. This format allows title text, debugging information, custom mapper, and a few more. 

(N.B. Do not delete or move this file until the draft is completed.) 

Difference of this format with others includes: 

  * You can specify NTSC and PAL.
  * Can include descriptive text.
  * There is no fixed set of mapper numbers; instead the components of the cartridge are configured individually.
  * Debugging symbols can be included.



## Contents

  * 1 File Order
  * 2 Header
  * 3 Mapper Codes
    * 3.1 ($00) Add non-audio component
    * 3.2 ($01) Add audio component
    * 3.3 ($04) Connection of components to components
    * 3.4 ($05) Connection of components to cartridge pins
    * 3.5 ($06) Connection of cartridge to cartridge
    * 3.6 ($FD) External condition
    * 3.7 ($FE) External parameter
    * 3.8 ($FF) External selector
  * 4 Symbol Data
  * 5 Mapper Components
    * 5.1 ($00) ROM
    * 5.2 ($01) RAM
    * 5.3 ($02) Logic gates
    * 5.4 ($03) Multiplication
    * 5.5 ($04) Shift register
    * 5.6 ($05) Flip-flops
    * 5.7 ($06) Multiplexer
    * 5.8 ($07) Diskette drive
    * 5.9 ($08) File loading interface
    * 5.10 ($09) Bandai EEPROM
    * 5.11 ($0A) Expansion terminal
    * 5.12 ($0B) Counter
    * 5.13 ($0C) System reset initializer
    * 5.14 ($0D) Addressable register
  * 6 Mapper Audio
    * 6.1 ($00) Extra 2A03 audio
    * 6.2 ($01) Famicom Disk System audio
    * 6.3 ($02) VRC6 audio
    * 6.4 ($03) VRC7 audio
    * 6.5 ($04) MMC5 audio
    * 6.6 ($05) Sunsoft audio
    * 6.7 ($06) Namco audio
    * 6.8 ($07) JF-13 audio
    * 6.9 ($08) Digital to analog converter
  * 7 ASCII/shortkana Table
  * 8 Other File Formats
    * 8.1 .rom
    * 8.2 .cart
    * 8.3 .symb
    * 8.4 ines.map
    * 8.5 unif.map
    * 8.6 .native
  * 9 Supported programs
    * 9.1 Emulators (compile time)
    * 9.2 Emulators (run time)
    * 9.3 Hardware programmers
    * 9.4 Assemblers/compilers
    * 9.5 Hardware description language
    * 9.6 Miscellaneous



## File Order

  * Header
  * ROM data
  * Mapper codes
  * Symbol data
  * Descriptive text



(Note: Sixteen bit numbers throughout the file are small-endian form.) 

## Header
    
    
      0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    |'F'|'A'|'M'|'I'|cpu| * | banks |textLen|mapLen |symLen | * | * |
    +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    

  * The first four bytes of the header must be the ASCII codes for "`FAMI`".
  * `banks`: Number 8K ROM banks.
  * `cpu`: CPU mode flag: 
    * bit0: Set if decimal mode should work (not used in NES/Famicom, although some Famiclones may have decimal mode).
    * bit1: Set if the 2A03 APU noise Mode flag should be ignored.
    * bit2: Swap PPUCTRL and PPUMASK (used in some [Vs. System](Vs__System.xhtml "Vs. System") games).
    * bit3: RGB PPU (tint bits maximize intensity, and some other differences).
    * bit4: Buggy Famiclone audio (switched bit7/bit6 or duty 1 and 2 are switched).
    * bit5: Can be used with Dendy TV mode.
    * bit6: Can be used with PAL TV mode.
    * bit7: Can be used with NTSC TV mode.
  * `*`: Reserved bytes; set to zero.
  * `textLen`: Length of descriptive text.
  * `mapLen`: Length of mapper codes.
  * `symLen`: Length of symbol data.



Note for TV modes: It can set multiple bits to indicate if it is OK with multiple TV modes. If none of these bits are set, it means to use NTSC timing, but use RGB palettes, no NTSC color artifacts, square pixels, and the entire 256x240 screen to be visible. 

Note: It is recommended that the user be able to change all the settings listed in the CPU mode flag. 

## Mapper Codes

Each mapper command consists of a length byte, followed by a command code, an ID number, and the parameters which have the length indicated by the length byte (the first byte of parameters is called parameter `$00`, etc). 

### ($00) Add non-audio component

  * ID number: ID of added component (must be unique for each component; non-audio and audio components additionally may not share ID numbers with themselves or each other).
  * Parameter `$00`: Type of component to add.
  * Parameter `$01`...: Parameters of component (depends on type of component).



### ($01) Add audio component

  * ID number: ID of added component (must be unique for each component; non-audio and audio components additionally may not share ID numbers with themselves or each other).
  * Parameter `$00`: Type of component to add.
  * Parameter `$01`: Volume from 0 to 64.
  * Parameter `$02`: Panning from 0 (left) to 64 (right), or 100 for surround. This parameter can be safely ignored by emulators and hardware.
  * Parameter `$03`...: Parameters of component (depends on type of component).



### ($04) Connection of components to components

  * ID number: ID of first component.
  * Parameter `$00`: ID of second component.
  * Parameter `$01`: Low 4 bits = impedance (0=direct, 15=high). High 4 bits = propagation delay (0=not applicable).
  * Parameter `$02`: Pin of first component.
  * Parameter `$03`: Pin of second component.
  * Parameter `$04`...: Repeat of parameters `$02` and `$03` for each connection.



### ($05) Connection of components to cartridge pins

  * ID number: ID of component.
  * Parameter `$00`: Reserved for future use; please set to zero.
  * Parameter `$01`: Low 4 bits = impedance (0=direct, 15=high). High 4 bits = propagation delay (0=not applicable).
  * Parameter `$02`: Pin of component.
  * Parameter `$03`: Pin of cartridge.
  * Parameter `$04`...: Repeat of parameters `$02` and `$03` for each connection.



The pin numbers are the same as the 60-pin Famicom cartridge, except that pins 16, 31, 45, and 46 are not used. 

### ($06) Connection of cartridge to cartridge

  * ID number: Reserved for future use; please set to zero.
  * Parameter `$00`: Reserved for future use; please set to zero.
  * Parameter `$01`: Low 4 bits = impedance (0=direct, 15=high). High 4 bits = propagation delay (0=not applicable).
  * Parameter `$02`: First pin.
  * Parameter `$03`: Second pin.
  * Parameter `$04`...: Repeat of parameters `$02` and `$03` for each connection.



### ($FD) External condition

This command should only appear in a `.cart` file; not in a `.fami` file. The ID number is the ID number of an external parameter, parameter `$00` is the number of commands in a block, and the rest are values which, if the external parameter has any of them the commands in the block will be used, and if they don't have them then the commands in the block will be ignored. 

### ($FE) External parameter

This command should only appear in a `.cart` file; not in a `.fami` file. The ID number is the ID number of an external parameter, and the rest are numbers of parameter slots of the next command which should be XORed by the external parameter value. 

For example: 
    
    
    01 FE 00 03
    04 01 00 05 40 20 00
    

This adds a Sunsoft audio but the clock speed is configured from external parameter number zero. 

### ($FF) External selector

This command should only appear in a `.cart` file; not in a `.fami` file. The ID number is the ID number of an external parameter, and the rest are numbers of commands in the corresponding block (parameter `$00` for block zero, parameter `$01` for block one, etc). Whatever the value of the external parameter is, that is the only following block which will be used; the other blocks mentioned will be ignored. (If the value specifies a block which there are not enough parameters to specify length of a used block, the length is treated as zero.) 

## Symbol Data

Symbol data consists of zero or more blocks consisting of a 16-bit ID followed by a 16-bit address followed by data. (If the mode is 0, 1, or 2, then the address must be an address within the ROM, mapper component, or CPU memory, and the data is an ASCII text giving its name). 

The ID is formed as follows: 

  * bit0-bit5: Length of data (0 to 63 bytes).
  * bit6-bit7: Mode (0=ROM, 1=mapper, 2=CPU memory, 3=extra)
  * bit8-bit15: Depend on mode as follows: 
    * 0: High bits of address.
    * 1: Component ID.
    * 2: bit8=read, bit9=write, bit10=execute, bit11=16-bit data, bit12-bit15=reserved (set to zero).
    * 3: ID number of what kind of extra it is.



The extras would be: 

  * `$00`: Arcade cabinet switch. Bit0-bit2 of address is the switch number, bit8 of address is the default setting (1=on, 0=off). Data is the name of the switch.
  * `$01`: Cartridge component name. Address bit0-bit7 = component ID, bit8-bit15 = pin. Data is name of component.
  * `$02`: ASCII/shortkana text. Address indicate what the text means: 0=title, 1=author, 2=copyright, 3=mapper name, 4=license, 5=publisher, 6=version, 7=date, 8=composer of original, 9=composer of cover, 10=Nintendo four-letter code (do not use for homebrew/unofficial unless an emulated version is made available officially on Wii Virtual Console or whatever), 11=arcade cabinet model number, 12=programmer.
  * `$03`: RGB palette. Address is from 0 to 63 indicating which palette index is used at first, data is the RGB triples (one byte per channel) starting from the specified palette index, ranging from 0 for black to 255 for full intensity. (This is basically like the [.pal](_pal.xhtml ".pal") format but made into 63-byte chunks) It is recommended that the user can change this setting.
  * `$04`: Track name. Address is track number (0 to 255 only). Data is the name of the track in ASCII/shortkana text. Names can be blank, and if higher track number is given name but lower one is not, the lower numbered tracks still exist and have blank name. If this field is used, the initial value of accumulator is the track number to start at, and the mapper codes can also use the track number (which will not change at runtime unless the mapper code tells it to) to decide which ROM bank to enable and so on.
  * `$05`: CIC region code. Data should be empty or ASCII name of region code. Address should be a number identifying the region code (0=unused or Japan). (needs more list? does Nintendo or any other format define 8-bit ID numbers for CIC?)



## Mapper Components

Note: In all cases, unconnected/unmentioned pins are not used, rather than being undefined or whatever else. 

### ($00) ROM

Parameters: 

  * `$01`...`$20`: Address of ROM data in ROM area of file.



Pins: 

  * `$00`...`$07`: Data
  * `$08`...`$7D`: Address
  * `$7E`: Chip select (inverted)
  * `$80`...`$FE`: Second port, same as above



### ($01) RAM

The RAM can be used as a single byte register if none of the address pins are connected. 

Parameters: 

  * `$01`: Set to 1 for battery RAM, 0 if not battery.



Pins: 

  * `$00`...`$07`: Data
  * `$08`...`$7D`: Address
  * `$7E`: Chip select (inverted)
  * `$7F`: Write enable (inverted)
  * `$80`...`$FF`: Second port, same as above



### ($02) Logic gates

Parameters: 

  * `$01`: Gate type: 0=AND, 1=OR, 2=XOR, 16=NAND, 17=NOR, 18=XNOR.
  * `$02`: Number of inputs per gate (this can be 1 if just using as a buffer or inverter).



Pins: 

  * `$00`: Output of first gate.
  * `$01`...`$xx`: Inputs of first gate.
  * `$xx+1`: Output of second gate.
  * And so on for each gate.



### ($03) Multiplication

This is a hardware multiplication for [MMC5](MMC5.xhtml "MMC5"). 

Parameters: 

  * (none)



Pins: 

  * `$00`...`$0F`: First input data.
  * `$10`...`$1F`: Second input data.
  * `$20`...`$3F`: Output data.
  * `$40`: First input enable (inverted).
  * `$50`: Second input enable (inverted).
  * `$60`: Output enable (inverted).



### ($04) Shift register

### ($05) Flip-flops

This component represents thirty-two JK flip-flops. You can leave some pins unconnected to act as D flip-flop, SR flip-flop, etc. 

Parameters: 

  * (none)



Pins: 

  * `$x0`: Clock
  * `$x1`: Data
  * `$x2`: Set
  * `$x3`: Reset
  * `$x4`: J
  * `$x5`: K
  * `$x6`: Out
  * `$x7`: Out (inverted)



### ($06) Multiplexer

Parameters: 

  * (none)



Pins: 

  * `$00`...`$7F`: Signal inputs
  * `$80`...`$86`: Selector inputs
  * `$FE`: Output
  * `$FF`: Output (inverted)



### ($07) Diskette drive

Parameters: 

  * `$01`: Hardware copy protection enable (0=no protection, 1=use protection)



### ($08) File loading interface

Parameters: 

  * `$01`: Base 2 log of bits per byte of file. (Value: `$00`=1 bit, `$01`=2 bits, `$02`=4 bits, `$03`=8 bits, `$04`=16 bits, etc)



Pins: 

  * `$00`...`$7F`: Data out
  * `$80`: Chip select (inverted)
  * `$81`: Clock
  * `$82`: Rewind
  * `$83`: End of file



### ($09) Bandai EEPROM

### ($0A) Expansion terminal

### ($0B) Counter

### ($0C) System reset initializer

### ($0D) Addressable register

## Mapper Audio

Note that you can have multiple instances of each and that their parameters can differ as well as how the address lines are connected by using other mapper codes. 

### ($00) Extra 2A03 audio

(see [APU](APU.xhtml "APU")) 

This need not be indicated if using only the NES/Famicom internal audio; this is if you need an additional 2A03 audio as well as the internal one. 

Parameters: 

  * (none)



### ($01) Famicom Disk System audio

(see [FDS audio](FDS_audio.xhtml "FDS audio")) 

Parameters: 

  * (none)



### ($02) VRC6 audio

(see [VRC6 audio](VRC6_audio.xhtml "VRC6 audio")) 

Parameters: 

  * (none)



### ($03) VRC7 audio

(see [VRC7 audio](VRC7_audio.xhtml "VRC7 audio")) 

Parameters: 

  * Parameter `$03`: Register write delay (1=on/0=off) (should be on; turn it off if it is necessary to emulate a program that does not properly wait for the register to be ready)
  * Parameter `$04`..`$7B`: Internal patch set (8 bytes each for 15 instruments) (optional; please set the length to 4 if you are not using this parameter)



### ($04) MMC5 audio

(see [MMC5 audio](MMC5_audio.xhtml "MMC5 audio")) 

Parameters: 

  * (none)



### ($05) Sunsoft audio

(see [Sunsoft 5B audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio")) 

Parameters: 

  * Parameter `$03`: Clock speed (0=half, 1=full) (should be 0 to emulate Sunsoft 5B audio)



### ($06) Namco audio

(see [Namco 163 audio](Namco_163_audio.xhtml "Namco 163 audio")) 

Parameters: 

  * (none)



### ($07) JF-13 audio

Parameters: 

  * Parameter `$03`...`$22`: ROM address of audio data



### ($08) Digital to analog converter

Parameters: 

  * Parameter `$03`: Default normalized output level
  * Parameter `$04`: (linear/non-linear?)



Pins: 

  * Pin `$00`: 1x audio level.
  * Pin `$01`: 2x audio level.
  * Pin `$02`: 4x audio level.
  * Pin `$03`: 8x audio level.
  * etc, up to pin `$FF` for an extremely high level.



## ASCII/shortkana Table

` #| 0| 1| 2| 3| 4| 5| 6| 7| 8| 9| A| B| C| D| E| F  
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
0| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***  
1| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***| ***  
2| sp| !| "| #| $| %| &| '| (| )| *| +| ,| -| .| /  
3| 0| 1| 2| 3| 4| 5| 6| 7| 8| 9| :| ;| <| =| >| ?  
4| @| A| B| C| D| E| F| G| H| I| J| K| L| M| N| O  
5| P| Q| R| S| T| U| V| W| X| Y| Z| [| \| ]| ^| _  
6| `| a| b| c| d| e| f| g| h| i| j| k| l| m| n| o  
7| p| q| r| s| t| u| v| w| x| y| z| {| || }| ~| ***  
8| Ç| ü| é| â| ä| à| å| ç| ê| ë| è| ï| î| ì| Ä| Å  
9| É| æ| Æ| ô| ö| ò| û| ù| ÿ| Ö| Ü| ¢| £| ¥| «| »  
A| ア| カ| ガ| サ| ザ| タ| ダ| ナ| ハ| バ| パ| マ| ヤ| ャ| ラ| ァ  
B| イ| キ| ギ| シ| ジ| チ| ヂ| ニ| ヒ| ビ| ピ| ミ| ワ| ン| リ| ィ  
C| ウ| ク| グ| ス| ズ| ツ| ヅ| ヌ| フ| ブ| プ| ム| ユ| ュ| ル| ゥ  
D| エ| ケ| ゲ| セ| ゼ| テ| デ| ネ| ヘ| ベ| ペ| メ| ッ| ヲ| レ| ェ  
E| オ| コ| ゴ| ソ| ゾ| ト| ド| ノ| ホ| ボ| ポ| モ| ヨ| ョ| ロ| ォ  
F| á| í| ó| ú| ñ| Ñ| ???| ???| ¿| ???| ???| ???| ???| ¡| ー| ***  
`

  * `***` indicates permanently reserved codes would should not be used in ASCII/shortkana text.
  * `???` indicates codes which are reserved for future use.



There are suggestions to use different encodings (such as UTF-8 or UTF-16, although I don't want UTF-16 or anything else that is incompatible with ASCII), so it might be changed in future. **For now use only ASCII please.**

## Other File Formats

### .rom

This file consists of the ROM data only, with no header. The file size should be a multiple of 8K. 

### .cart

This file consists of the mapper codes only, with no header. 

### .symb

This file consists of the symbols only, with no header. 

### ines.map

This plain text file is used to automatically associate `.cart` files with `.nes` files, based on mapper numbers, ROM sizes, checksums, and other things. Values of external parameters can also be specified. 

I intend that it can make the official "`ines.map`" and FamicomHDL files on this wiki, and can then be updated emulators, both static (the `ines.map` and `*.cart` are compiled into a C code and compiled with the emulator), and dynamic (the emulator converts `.nes` to `.fami` at runtime and JIT-compiles and emulates the mapper codes). 

Description of file: [User:Zzo38/ines.map and unif.map](User_Zzo38_ines_map_and_unif_map.xhtml "User:Zzo38/ines.map and unif.map")

### unif.map

This plain text file is used to automatically associate `.cart` files with `.unif` files based on the contents of the file. Values of external parameters can also be specified. 

Description of file: [User:Zzo38/ines.map and unif.map](User_Zzo38_ines_map_and_unif_map.xhtml "User:Zzo38/ines.map and unif.map")

### .native

The format of this file depends on the emulator using it, and some might not use it; however, the first eight bytes should indicate what emulator it is for and what computer it is for, so that one program does not get mixed up by another program. 

Generally this file would contain a native code for emulating the mapper codes. 

## Supported programs

### Emulators (compile time)

### Emulators (run time)

### Hardware programmers

### Assemblers/compilers

### Hardware description language

  * FamicomHDL (Haskell library)



### Miscellaneous
