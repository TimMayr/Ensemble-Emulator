# FDS BIOS

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FDS_BIOS) | View [other pages](Special_AllPages.xhtml#FDS_BIOS)

The [Famicom Disk System](Family_Computer_Disk_System.xhtml "Famicom Disk System") contains a fixed 8KB BIOS at $E000-FFFF. It controls the Famicom at power-on and reset, dispatches the NMI and IRQ, and offers an API for accessing the [data on disk](FDS_disk_format.xhtml "FDS disk format"). Routines for common tasks including controller reading and PPU handling are also provided for programmer convenience. 

## Contents

  * 1 Power-on and reset
    * 1.1 Approval check
      * 1.1.1 Bypass method
  * 2 Zero-page variables
  * 3 Interrupt/Reset vector controls
  * 4 BIOS calls
    * 4.1 Disk access routines
    * 4.2 Low-Level Disk access routines
    * 4.3 Example code to load files
    * 4.4 Example code to save a file
    * 4.5 Disk ID structure
    * 4.6 File List structure
    * 4.7 File Header structure
    * 4.8 Error list
  * 5 Other BIOS calls
    * 5.1 VRAM transfer structure
    * 5.2 VRAM buffer notes
    * 5.3 Read routines and VRAM buffer
    * 5.4 Load Tileset notes
    * 5.5 Object structure
  * 6 See Also
  * 7 Notes
  * 8 References



## Power-on and reset

The BIOS contains a [ reset handler](Init_code.xhtml "Init code") which initialises the Famicom Disk System and [PPU](PPU.xhtml "PPU") registers (including a [warm-up loop](PPU_power_up_state.xhtml#Best_practice "PPU power up state")), then checks for an inserted/loaded disk. A basic interface is set up to display error messages if disk loading fails. Once the disk has loaded (or the reset flag and type are set to appropriate values - see vector controls below), execution jumps to the game's reset vector. 

On power-on and reset, the nametable arrangement is set to vertical ("horizontal mirroring"), the disk drive motor is turned off, the stack pointer is set to $FF, and the I flag is _cleared_. System RAM is filled with values used by the BIOS, and PRG RAM is uninitialized, except for parts of it which have files loaded in. 

### Approval check

The FDS has a trademark security system similar to [what Sega used on some of its consoles](http://segaretro.org/TradeMark_Security_System): 

  * The 224-byte string at PPU $2800-$28DF must match the data in the BIOS, starting at $ED37.
  * This data contains an English licensing message stored in the SMB1/Zelda character encoding. The BIOS ensures that it is present in the nametable and displays it on the screen for several seconds after the boot files are loaded.
  * Traditionally, the first file on a disk is a nametable type file loaded into $2800, which is named `KYODAKU-` (きょだく or [許諾](https://en.wiktionary.org/wiki/%E8%A8%B1%E8%AB%BE "wiktionary:許諾") means approval). It must be one of the boot files, or the license message test will fail (error $20) before it proceeds to run the program.



#### Bypass method

The license screen test and display can be bypassed by [using a boot file to enable NMIs](https://forums.nesdev.org/viewtopic.php?p=194826#p194826), which will interrupt the boot loading process early. 

Unlicensed titles (such as those from I2 and Hacker International) are known to use this method. Licensed titles from Namco also use this method. However, they check for an [IPL interface](Namco_IPL_Interface.xhtml "Namco IPL Interface") before displaying a fake license screen. 

## Zero-page variables

The FDS BIOS uses the zero-page to store temporary values, controller reads, and the states of several write-only registers: 
    
    
    $00..$0F:	Used as temporary variables.
    $F1..$F8:	Used by controller reading routines.
    [$F9]:		Mirror of [$4026](Family_Computer_Disk_System.xhtml#External_connector_.28.244026.29 "Family Computer Disk System").   $FF on reset.
    [$FA]:		Mirror of [$4025](Family_Computer_Disk_System.xhtml#FDS_Control_.28.244025.29 "Family Computer Disk System").   $2E on reset.
    [$FB]:		Mirror of [$4016](Controller_reading.xhtml "Controller reading").   $00 on reset.
    [$FC]:		Mirror of [$2005/2](PPU_registers.xhtml "PPUSCROLL"). $00 on reset.
    [$FD]:		Mirror of [$2005/1](PPU_registers.xhtml "PPUSCROLL"). $00 on reset.
    [$FE]:		Mirror of [$2001](PPU_registers.xhtml "PPUMASK").   $06 on reset.
    [$FF]:		Mirror of [$2000](PPU_registers.xhtml "PPUCTRL").   $10 on reset.
    

## Interrupt/Reset vector controls

The FDS BIOS uses 4 bytes at the lower end of the stack page to control the behaviour of interrupt/reset vectors: 
    
    
    [$0100]:	PC action on NMI. $C0 (NMI #3) on reset.
    [$0101]:	PC action on IRQ. $80 (BIOS acknowledge and delay) on reset.
    [$0102]:	RESET flag. $35 on reset after the boot files have loaded.
    [$0103]:	RESET type. $AC = first boot of the game, $53 = the game was soft-reset by the user.
    

Disk programs must be careful not to clobber these values through the use of [init code](Init_code.xhtml "Init code"), nested subroutines, and/or stack manipulation. The following vectors are used depending on the controls: 
    
    
    NMI:
    ($FFFA):	BIOS NMI                   (always runs)
     $E19D :	BIOS VINTWait handler      (if [$0100] = %00xxxxxx)
    ($DFF6):	Disk game NMI vector #1    (if [$0100] = %01xxxxxx)
    ($DFF8):	Disk game NMI vector #2    (if [$0100] = %10xxxxxx)
    ($DFFA):	Disk game NMI vector #3    (if [$0100] = %11xxxxxx)
    
    
    
    RESET:
    ($FFFC):	BIOS RESET                 (always runs)
    ($DFFC):	Disk game RESET vector     (if [$0102] = $35 and [$0103] = $53 or $AC)
    
    
    
    IRQ:
    ($FFFE):	BIOS IRQ                   (always runs)
     $E1D9 :	BIOS disk skip bytes       (if [$0101] = %00nnnnnn, n = # of bytes to skip)
     $E1CE :	BIOS disk transfer         (if [$0101] = %01xxxxxx)
     $E1EF :	BIOS acknowledge and delay (if [$0101] = %10xxxxxx)
    ($DFFE):	Disk game IRQ vector       (if [$0101] = %11xxxxxx)
    

## BIOS calls

### Disk access routines

  * These routines take one or two direct pointers as arguments. They must be placed directly after the JSR instruction: the return address in the stack is used to fetch the pointers and is subsequently fixed.
  * Zero-page memory at $00-$0F will be affected by these routines.
  * Unlike the vast majority of disk drives, the FDS lacks any kind of intelligent tracking system. All BIOS load and save functions will do access to the whole disk, no matter which data they load/save. A simple way to overcome this problem is to have a custom loading routine, similar to the BIOS one but forcing the # of files to a smaller number than it actually is. That way the later files are not accessed at all and the earlier files load faster. Of course the maximal time is still taken when loading files that are late on the disk.
  * Dummy loads are used (i.e. file contents are ignored) if the destination address is below $2000, excluding the $x200 page. Loads to $0200-$02FF and its mirror $1200-$12FF are permitted, likely to allow a disk to prepare [ OAM DMA](PPU_OAM.xhtml#DMA "PPU OAM").
  * All non-disk IRQ sources (timer, DMC and APU frame) should be properly disabled before calling any of these routines. The value at [$0101] however, is preserved on entry, and restored on exit.
  * NMIs should also be disabled (e.g. by calling VINTWait) before calling these routines to avoid potential race conditions during disk accesses.
  * On return of those routines, A = $00 means no error occurred, other number is error #. Main program should test if an error occurred with the BEQ or BNE instruction, BEQ will branch if no error, and BNE will branch if there is an error.
  * The structures defined below are used to identify files & disks in the access routines. The argument pointers should point to these structures in the program.

Address | Name | Input parameters | Output parameters | Description   
---|---|---|---|---  
$E1F8 | LoadFiles | Direct pointer = Disk ID, Direct pointer = File List | A = error #, Y = # of files loaded | Load files specified by Disk ID into memory from disk, attempting twice before returning any errors. Load addresses are decided by the file's header. Loads to memory locations below $2000 are ignored but the $x200-$x2FF page is permitted. The file list can contain a maximum of 20 file IDs to load, with shorter lists terminated by $FF. If the file list only contains the $FF terminator, then files with IDs less than or equal to the boot read file code in the [disk info block](FDS_disk_format.xhtml#Disk_info_block_\(block_1\) "FDS disk format") will be loaded instead (this is used by the BIOS during startup).   
$E237 | AppendFile | Direct pointer = Disk ID, Direct pointer = File Header | A = error # | Append the file data given by Disk ID to the disk, attempting twice before returning any errors. This means that the file is tacked onto the end of the disk, and the disk file count is incremented. The file is then read back to verify the write. If an error occurs during verification, the disk's file count is decremented (logically hiding the written file).   
$E239 | WriteFile | Direct pointer = Disk ID, Direct pointer = File Header, A = file # | A = error # | Same as "AppendFile", but instead of writing the file to the end of the disk, A specifies the sequential position on the disk to write the file (0 is the first). This also has the effect of setting the disk's file count to the A value, therefore logically hiding any other files that may reside after the written one.   
$E2B7 | CheckFileCount | Direct pointer = Disk ID, A = # to set file count to | A = error # | Read in disk's file count, compare it to A, then set the disk's file count to A. Error 31 is returned if A is greater than the original file count.   
$E2BB | AdjustFileCount | Direct pointer = Disk ID, A = # to reduce current file count by | A = error # | Read in disk's file count, decrement it by A, then writes the new value back. Error 31 is returned if A is greater than the original file count.   
$E301 | SetFileCount1 | Direct pointer = Disk ID, A = file count minus one = # of the last file | A = error # | Set the file count to A + 1.   
$E305 | SetFileCount | Direct pointer = Disk ID, A = file count | A = error # | Set the file count to A.   
$E32A | GetDiskInfo | Direct pointer = Disk ID | A = error # | Fill a given location with the current Disk ID data.   
  
### Low-Level Disk access routines

Address | Name | Input parameters | Output parameters | Description   
---|---|---|---|---  
$E445 | CheckDiskHeader | $00-$01 = Pointer to 10 byte string |  | Checks that the '*NINTENDO-HVC*' string is present on the current disk, then compares the Disk ID structure to data pointed to by Ptr($00). To bypass the checking of any byte, $FF can be placed in the equivalent field of the specified data. If the comparison fails, an appropriate error will be generated.   
$E484 | GetNumFiles |  | $06 = # of files | Read the number of files from the file amount block.   
$E492 | SetNumFiles | A = # of files |  | Write the number of files in A to the file amount block.   
$E4A0 | FileMatchTest | $02-$03 = Pointer to file ID list |  | Use a byte string pointed at by Ptr($02) to tell the disk system which files to load. The file ID's number is searched for in the string. If an exact match is found, [$09] is set to 0 and [$0E] is incremented. If no matches are found after 20 bytes, or a -1 entry is encountered, [$09] is set to -1. If the first byte in the string is -1, the boot ID number is used for matching files (any file ID that is not greater than the boot ID qualifies as a match).   
$E4DA | SkipFiles | $06 = # of files to skip |  | Skip over a specified number of files.   
  
### Example code to load files

This is a basic example of loading multiple files from disk, such as code/data and graphics for a new level. 
    
    
    Load:
       JSR VINTWait
       JSR LoadFiles
       .dw DiskID
       .dw LoadList
       BNE _Error     ; Check if there is an error
       RTS
    _Error:
       JSR PrintError ; If so print the error number and message to screen (include side/disk changing prompts)
    _sideError:
       LDA $4032
       AND #$01
       BEQ _sideError ; Wait until disk is ejected
    _insert:
       LDA $4032
       AND #$01
       BNE _insert    ; Wait until disk is inserted
       JMP Load
    
    DiskID:
       .db $01        ; Licensee code
       .db "NAM "     ; 3-letter code of game, plus space for normal disk
       .db $00        ; Version
       .db $01        ; Disk side
       .db $00        ; Disk number
       .db $00        ; Disk type
       .db $00        ; Unknown
    
    LoadList          ; In this example the files with IDs equal to $02, $03 or $04 will be loaded into memory (max = 20 IDs, excluding $FF terminator)
       .db $02, $03, $04, $FF
    

### Example code to save a file

This is a basic example of saving/overwriting a fixed file on disk, such as a game save. The file being saved should be the last file on a given disk side, otherwise files after it will be logically hidden by the BIOS. 
    
    
    Write:
       JSR VINTWait
       LDA #SAVE_NUM  ; File number should be the last one on the disk side
       JSR WriteFile
       .dw DiskID
       .dw FileHeader
       BNE _Error     ; Check if there is an error
       RTS
    _Error:
       JSR PrintError ; If so print the error number and message to screen (include side/disk changing prompts)
    _sideError:
       LDA $4032
       AND #$01
       BEQ _sideError ; Wait until disk is ejected
    _insert:
       LDA $4032
       AND #$01
       BNE _insert    ; Wait until disk is inserted
       JMP Write
    
    DiskID:
       .db $01        ; Licensee code
       .db "NAM "     ; 3-letter code of game, plus space for normal disk
       .db $00        ; Version
       .db $01        ; Disk side
       .db $00        ; Disk number
       .db $00        ; Disk type
       .db $00        ; Unknown
    
    FileHeader:       ; In this example, we are saving a game save to disk
       .db SAVE_ID    ; file ID code (used by LoadFiles)
       .db "SAVEDATA" ; file name
       .dw SaveDest   ; load address
       .dw SaveLen    ; file data size
       .db $00        ; file type (PRG)
    ; the following specify the file data source (NOT written to disk)
       .dw SaveSrc    ; source address of file data
       .db $00        ; source address type (RAM)
    

### Disk ID structure

This is a structure which is comprised of bytes $0F..$18 of the disk's [info block](FDS_disk_format.xhtml#Disk_info_block_\(block_1\) "FDS disk format") (right after the '*NINTENDO-HVC*' string). Disk access routines call CheckDiskHeader to compare the structure from the disk against specified data. If any of the bytes fail the comparison, an appropriate error # is generated. Comparisons of immaterial data can be skipped by placing an $FF byte in the appropriate field of the Disk ID structure (for example, when the ROM BIOS boots a disk, it sets all the fields in the DiskID string to $FF except for disk side #, and disk #, which are set to 0 (so these fields have to match 0)). 

CheckDiskHeader compares the following 10 bytes between the disk and the specified data, and can return the corresponding error #'s. 
    
    
    offset	size	error#	description
    ------	----	------	-----------
    0	1	$04	licensee code
    1	4	$05	game ASCII name string + game type
    5	1	$06	game version
    6	1	$07	disk side #
    7	1	$08	disk #
    8	1	$09	disk type
    9	1	$10	unknown
    

When the Disk ID structure is returned by GetDiskInfo, the above 10 bytes are followed by additional disk information. 
    
    
    A	1		# of files on disk (from the [file amount block](FDS_disk_format.xhtml#File_amount_block_\(block_2\) "FDS disk format"))
    

(The following block will appear for as many files as indicated on the disk) 
    
    
    B	1		file ID code
    C	8		file name (ASCII)
    

(The following is present after the last file info block. Disk size is equal to the sum of each file's size entry, plus an extra 261 per file.) 
    
    
    x	1		disk size high byte
    x+1	1		disk size low  byte
    x+2			-
    

### File List structure

This is a list of 1-byte IDs of files to load. All files that matches any ID in the list are loaded. A list of up to 20 IDs is possible at a time, smaller lists should be terminated by a $ff byte (this implies a file ID can never be $ff). 

Multiple files are loaded in the order as they exist on the disk, not in the order of the list. 

### File Header structure

This structure is specified when a file is to be written to the disk. The first 14 bytes of this structure directly specifies the data to use for generating a [file header block](FDS_disk_format.xhtml#File_header_block_\(block_3\) "FDS disk format"). This is followed by 3 bytes which concern the [file data block](FDS_disk_format.xhtml#File_data_block_\(block_4\) "FDS disk format") to be written to the disk. The file count value is specified separately from the structure (e.g. the value of A when calling WriteFile). 
    
    
    offset	size	description
    ------	----	-----------
    00	1	file ID code
    01	8	file name
    09	2	load address
    0B	2	file data size
    0D	1	file type ($00 : Program; $01 : Character; $02 : Nametable)
    0E	2	source address of file data (NOT written to disk)
    10	1	source address type ($00 : RAM, $01 : VRAM, NOT written to disk)
    11		-
    

### Error list

These are the BCD error numbers returned by the BIOS disk I/O routines, along with the displayed messages and their explanations. 
    
    
    error#	message		explanation
    ------	-------		-----------
    $00			No error
    $01	disk set	($4032.0) Disk not set
    $02	battery		($4033.7) Power supply failure
    $03			($4032.2) Disk is write protected
    $04			Wrong licensee code
    $05			Wrong game ASCII name string or game type
    $06			Wrong game version
    $07	a,b side	Wrong side number
    $08	disk no.	Wrong disk number
    $09			Wrong disk type
    $10			Wrong value for unknown field
    $20	disk trouble	Approval check failed
    $21	disk trouble	'*NINTENDO-HVC*' string in [Disk info block](FDS_disk_format.xhtml#Disk_info_block_\(block_1\) "FDS disk format") doesn't match
    $22	disk trouble	[Disk info block](FDS_disk_format.xhtml#Disk_info_block_\(block_1\) "FDS disk format") expected
    $23	disk trouble	[File amount block](FDS_disk_format.xhtml#File_amount_block_\(block_2\) "FDS disk format") expected
    $24	disk trouble	[File header block](FDS_disk_format.xhtml#File_header_block_\(block_3\) "FDS disk format") expected
    $25	disk trouble	[File data block](FDS_disk_format.xhtml#File_data_block_\(block_4\) "FDS disk format") expected
    $26	disk trouble	AppendFile/WriteFile: Readback verification failed after writing
    $27	disk trouble	($4030.4) Block failed CRC
    $28	disk trouble	($4030.6) File ends prematurely during read (i.e. disk is full)
    $29	disk trouble	($4030.6) File ends prematurely during write (i.e. disk is full)
    $30	disk trouble	($4032.1) File ends prematurely during CRC writes (i.e. disk is full)
    $31	disk trouble	CheckFileCount/AdjustFileCount: A > original file count
    

Possibly game-specific errors: 
    
    
    error#	explanation
    ------	-----------
    $35	Failed to write test file (block 5)?
    $40	LoadFiles: Could not load all files from the File List
    $41~	Unknown (appears in manual for _Kineko II_)
    

## Other BIOS calls

Most of these routines accomplish common tasks including controller reading and PPU handling for programmer convenience. As a result, these routines can often be found in games ported from disk to cartridge format. 

Address | Name | Input parameters | Output parameters | Affected RAM/Registers | Description   
---|---|---|---|---|---  
$E149 | Delay131 |  |  |  | 131 CPU cycle delay (including the JSR instruction to call it) used solely by the "BIOS acknowledge and delay" IRQ handler.   
$E153 | Delayms | Y = delay in ms (approximate) |  | X, Y | Delay routine, where the time in CPU cycles is 1790*Y+5. Due to the pre-decrement check for Y, the maximum delay is achieved with Y = 0 (which yeilds ~256ms).   
$E161 | DisPFObj |  |  | A, $FE | Disable rendering for both sprites and background.   
$E16B | EnPFObj |  |  | A, $FE | Enable rendering for both sprites and background.   
$E171 | DisObj |  |  | A, $FE | Disable sprite rendering.   
$E178 | EnObj |  |  | A, $FE | Enable sprite rendering.   
$E17E | DisPF |  |  | A, $FE | Disable background rendering.   
$E185 | EnPF |  |  | A, $FE | Enable background rendering.   
$E1B2 | VINTWait |  |  | $FF | Wait until next VBlank NMI fires, and return (for programs which do "everything in main"). NMI vector selection at $0100 is preserved, but further VBlanks are disabled.   
$E7BB | VRAMStructWrite | Direct pointer = VRAM struct to be written |  | A, X, Y, $00, $01, $FF | Set VRAM increment to 1 (clear [PPUCTRL](PPU_registers.xhtml "PPUCTRL")/$FF bit 2), and transfer a VRAM struct to VRAM. Read below for information on the structure.   
$E844 | FetchDirectPtr |  | $00, $01 = pointer fetched | A, X, Y, $05, $06 | Fetch a direct pointer from the stack (the pointer should be placed after the return address of the routine calling this one), save the pointer at ($00) and fix the return address.   
$E86A | WriteVRAMBuffer |  |  | A, X, Y, $FF, $0301, $0302 | Transfer the VRAM buffer at $0302 to VRAM. Read below for information on the structure.   
$E8B3 | ReadVRAMBuffer | X = start address of read buffer, Y = # of bytes to read |  | A, X, Y | Read individual bytes from VRAM to the VRAM buffer. (see notes below)   
$E8D2 | PrepareVRAMString | A = High VRAM address, X = Low VRAM address, Y = string length, Direct pointer = data to be written to VRAM | A = $FF : no error, A = $01 : string didn't fit in buffer | A, X, Y, $00-$06 | Copy pointed data into the VRAM buffer.   
$E8E1 | PrepareVRAMStrings | A = High VRAM address, X = Low VRAM address, Direct pointer = data to be written to VRAM | A = $FF : no error, A = $01 : data didn't fit in buffer | A, X, Y, $00-$06 | Copy a 2D string into the VRAM buffer. The first byte of the data determines the width and height of the following string (in tiles): Upper nybble = height, lower nybble = width.   
$E94F | GetVRAMBufferByte | X = starting index of read buffer, Y = # of address to compare (starting at 1), $00, $01 = address to read from | carry clear : a previously read byte was returned, carry set : no byte was read, should wait next call to ReadVRAMBuffer | A, X, Y | This routine was likely planned to be used in order to avoid useless latency on a VRAM reads (see notes below). It compares the VRAM address in ($00) with the Yth (starting at 1) address of the read buffer. If both addresses match, the corresponding data byte is returned exit with c clear. If the addresses are different, the buffer address is overwritten by the address in ($00) and the routine exit with c set.   
$E97D | Pixel2NamConv | $02 = Pixel Y coord, $03 = Pixel X coord | $00 = High nametable address, $01 = Low nametable address | A | Convert pixel screen coordinates to the corresponding nametable address (assumes no scrolling, and points to first nametable at $2000-$23ff).   
$E997 | Nam2PixelConv | $00 = High nametable address, $01 = low nametable address | $02 = Pixel Y coord, $03 = Pixel X coord | A | Convert a nametable address to the corresponding pixel coordinates (assume no scrolling).   
$E9B1 | Random | X = Zero Page address where the random bytes are placed, Y = # of shift register bytes (normally $02) |  | A, X, Y, $00 | This is a shift-register based random number generator which normally takes 2 bytes (using more won't affect random sequence). On reset the program is supposed to write some non-zero values here (BIOS uses writes $d0, $d0), and call this routine several times before the data is actually random. Each call of this routine will shift the bytes _right_.   
$E9C8 | SpriteDMA |  |  | A | Do sprite DMA from RAM $0200-$02FF.   
$E9D3 | CounterLogic | A, Y = end zero-page address of counters, X = start zero-page address of counters |  | A, X, $00 | This decrements several counters in zero-page. The first counter is a decimal counter 9 -> 8 -> 7 -> ... -> 1 -> 0 -> 9 -> ... Counters 1...A are simply decremented and stays at 0. Counters A+1...Y are decremented when the first counter does a 0 -> 9 transition, and stays at 0.   
$E9EB | ReadPads |  | $F5 = Joypad #1 data, $F6 = Joypad #2 data, $00 = Expansion #1 data, $01 = Expansion #2 data | A, X | Read hardwired/expansion port joypads. Expansion port reads should be placed/used elsewhere if needed after calling this routine, otherwise they will be clobbered by other BIOS calls.   
$EA0D | OrPads | $F5 = Joypad #1 data, $F6 = Joypad #2 data, $00 = Expansion #1 data, $01 = Expansion #2 data | $F5 = Joypad #1 OR Expansion #1, $F6 = Joypad #2 OR Expansion #2 | A | Combine inputs from hardwired/expansion port joypads. Intended to be called after ReadPads.   
$EA1A | ReadDownPads |  | $F5 = Joypad #1 up->down transitions, $F6 = Joypad #2 up->down transitions $F7 = Joypad #1 data, $F8 = Joypad #2 data | A, X, $00, $01 | Read hardwired joypads, and detect up->down button transitions.   
$EA1F | ReadOrDownPads |  | $F5 = Joypad #1 up->down transitions, $F6 = Joypad #2 up->down transitions $F7 = Joypad #1 data, $F8 = Joypad #2 data | A, X, $00, $01 | Read both hardwired/expansion port joypads and detect up->down button transitions.   
$EA36 | ReadDownVerifyPads |  | $F5 = Joypad #1 up->down transitions, $F6 = Joypad #2 up->down transitions $F7 = Joypad #1 data, $F8 = Joypad #2 data | A, X, $00, $01 | Read hardwired joypads, and detect up->down button transitions. Data is read until two consecutive reads match to work around DMC glitches.   
$EA4C | ReadOrDownVerifyPads |  | $F5 = Joypad #1 up->down transitions, $f6 = Joypad #2 up->down transitions $f7 = Joypad #1 data, $f8 = Joypad #2 data | A, X, $00, $01 | Read both hardwired/expansion port joypads and detect up->down button transitions. Data is read until two consecutive reads match to work around DMC glitches, but could be prone to false reads as the reread loop for two controllers + expansion port is too slow for the maximum DMC fetch period (see [ here](Controller_reading_code.xhtml#DPCM_Safety_using_Repeated_Reads "Controller reading code") for an explanation). A page cross penalty incurred by the branch instruction controlling the polling loop in ReadPads _may_ be avoiding this issue.[footnotes 1] TODO: Do some measurements/tests to verify this.   
$EA68 | ReadDownExpPads |  | $F1-$F4 = up->down transitions, $F5-$F8 = Joypad data, in the order: Pad1, Pad2, Expansion1, Expansion2 | A, X, $00, $01 | Read both hardwired famicom and expansion port joypads, but store their data separately instead of ORing them together like the other routines do. This routine is NOT DMC fortified.   
$EA84 | VRAMFill | A = High VRAM Address (aka tile row #), X = Fill value, Y = # of tile rows OR attribute fill data |  | A, X, Y, $00-$02 | This routine does two things : If A < $20, it fills pattern table data with the value in X for 16 * Y tiles. If A >= $20, it fills the corresponding nametable with the value in X and attribute table with the value in Y.   
$EAD2 | MemFill | A = fill value, X = first page #, Y = last page # |  | A, X, Y, $00, $01 | Fill RAM pages with the value in A.   
$EAEA | SetScroll |  |  | A | Set the scroll registers according to values in $FC, $FD and $FF. Should typically be called in VBlank after VRAM updates.   
$EAFD | JumpEngine | A = Jump table entry |  | A, X, Y, $00, $01 | The instruction calling this is supposed to be followed by a jump table (16-bit pointers little endian, up to 128 pointers). A is the entry # to jump to, return address on stack is used to get jump table entries.   
$EB13 | ReadKeyboard |  | A = $FF : no error, A = $00 : no keyboard connected $00-$08 = keyboard data | A, X, Y, $00-$08, $FB | Attempt to read the [Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard"), returning with A=0 if nothing is connected. Keyboard data is stored starting from row 8 in descending order, with each byte containing column 0's data in the lower nybble and column 1's data in the upper nybble. A set bit means that a key was pressed.   
$EBAF | LoadTileset | A = Low VRAM Address & Flags, Y = Hi VRAM Address, X = # of tiles to transfer to/from VRAM, Direct pointer = source/destination address in RAM |  | A, X, Y, $00-$04 | Read/write 2BP and 1BP tilesets to/from VRAM. See appendix below regarding the flags.   
$EC22 | UploadObject | $00-$01 = Pointer to object structure |  | A, X, Y, $02-$0C | Upload an object to RAM $0200-$02FF. (see object structure below)   
  
  


### VRAM transfer structure
    
    
    SIZE   CONTENTS
    2      VRAM Address (**big endian**)
    1      bit 0-5 length of data ($0 means a length of 64)
           bit 6 : 0 = copy, 1 = fill
           bit 7 : 0 = increment by 1, 1 = increment by 32
    n      Data to copy to VRAM
    .....  repeated as many times as needed
    1      bit 7 : 1 = terminate structure
    

  * The main structure is terminated by a byte with bit 7 set - $FF is commonly used. (High address byte is always supposed to be in $00..$3F range)
  * If Fill mode is used, the routine takes only 1 byte of data which is repeated.
  * The interpretation of the length differs from the otherwise similar [Stripe Image buffer format](Tile_compression.xhtml#NES_Stripe_Image_RLE "Tile compression").
  * $4C (JMP Absolute) is a "call" command. The 2 bytes that follow is the address of a sub-VRAM structure, in **little endian**. The sub-structure can call another sub-structure and so on. A "return" address is pushed to the stack, so this command behaves closer to a JSR instruction. Therefore, the nesting depth must be limited to avoid clobbering the [interrupt/reset vector controls](FDS_BIOS.xhtml#Interrupt/Reset_vector_controls "FDS BIOS"), much like regular subroutine calls.
  * $60 (RTS) is a "return" command. It will terminate a sub-structure and "return" to the VRAM structure address popped from the stack. 
    * The "return" address used by these commands corresponds to the address where the "call" command byte was encountered in the structure. The "return" command adds an offset of 3 (skipping the entire "call" command) to the address before resuming the routine.



Example with sub-VRAM structure (separate the hi/lo address bytes if your assembler does not support placing words as big endian): 
    
    
    VRAMStruct:
      .dbyt $2000          ; VRAM address (big endian)
      .byte %00000000 | 4  ; increment by 1, copy, 4 bytes
      .byte "TEST"         ; data
    
      .byte $4c            ; call
      .word SubVRAMStruct  ; sub-VRAM structure address (little endian)
    
      .byte $ff            ; terminator
    
    SubVRAMStruct:
      .dbyt $2400          ; VRAM address (big endian)
      .byte %11000000 | 30 ; increment by 32, fill, 30 bytes
      .byte $aa            ; data
      
      .byte $60            ; return
    

### VRAM buffer notes

The VRAM buffer is located at $0300-$03xx. $300 holds the size of the buffer (maximum), and $301 holds the end index of the buffer. The actual buffer lies at $0302-$03xx, and is of variable length. 

  * $0300 is initialized to the value $7D, effectively making the buffer lie at $0300-$037F. It's possible to change the value here to make it bigger or smaller, but the biggest possible value is $FD, making the buffer lie at $0300-$03FF.
  * Format of the buffer is equivalent to the VRAM structure above, except that there are no sub-structures, no increment by 32 flag and no fill flag.
  * For this reason, the VRAM buffer at $0302 can be used as a sub-structure.
  * A call to WriteVRAMBuffer will execute faster than a call to VRAMStructWrite with $0302 as an argument, but both will have the same effect.



### Read routines and VRAM buffer

Unlike the write routines which are very complete, the read routines are somewhat incomplete and their functionality have to be completed by the user (which limits their usefulness). Most of what follows is some sort of speculation about the usefulness of the incomplete read routines of the BIOS. 

The read buffer is a part of the VRAM buffer at $0300-$03xx, but the same location can't be used to transfer read and writes on the same frame (for example a RMW operation). Instead the user must manually split the buffer in two parts, the "read" buffer and the "write" buffer. The read buffer is probably supposed to always lies after the write buffer, so that when the read buffer is in use, the value in $300 (size of the write buffer) should be adjusted accordingly. The user should manually keep track of how many bytes are used for the read buffer, as well as the starting point of the read buffer (they are argument to the ReadVRAMBuffer function). 

The structure of the read buffer itself is trivial - only single bytes are read (there's no runs of data). Very likely the purpose is to read one or a few individual attribute table data, in order to change the color mapping of individual 2x2 squares instead of whole 4x4 square (read-modify-write operation). All reads are mapped to a structure of 3 bytes in the read buffer : 
    
    
    SIZE   CONTENTS
    2      VRAM Address (big endian)
    1      data
    

Therfore, for each byte which is read from VRAM, 3 bytes have to be reserved in the read buffer. Once data from VRAM has been read, if it must be written back after a modification, the user need to copy it to the write buffer manually. 

The GetVRAMBufferByte was probably designed to prevent reading attribute tables when it has already been read in the past by a call to ReadVRAMBuffer. For example if you don't know if you're modifying another area of the same 4x4 attribute byte as previously (no need to read again), or if you're modifying part of another 4x4 attribute byte (need to read from VRAM). If the routine return with C=0, the accumulator contains the attribute byte that was already read, but if C=1, it means we should first call ReadVRAMBuffer before getting the data we want. 

Since reading VRAM for nothing is not a harmful operation, the user can call ReadVRAMBuffer every frame even if nothing has to be read (no need to check flags) which is probably why these routines are less complete than the write buffer related routines. 

### Load Tileset notes

The _flags_ parameters are as follows: 
    
    
    7  bit  0
    ---------
    AAAA MMIT
    |||| ||||
    |||| |||+- Fill bit
    |||| ||+-- Transfer direction (0 = Write tiles, 1 = Read tiles)
    |||| ++--- Bitplane type (see below)
    ++++------ Low VRAM Address (aka tile # within a row)
    
            1st bitplane	2nd bitplane     Description
            -----------	-----------      -----------
        0:  data           data+8           Normal 2-bitplane graphics
        1:  data           fill bit         Single bitplane graphics. Fill bit clear : Use colors 0&1  Fill bit set : Use colors 2&3
        2:  fill bit       data             Single bitplane graphics. Fill bit clear : Use colors 0&2  Fill bit set : Use colors 1&3
        3:  data^fill bit  data             Single bitplane graphics. Fill bit clear : Use colors 0&3  Fill bit set : Use colors 1&2
    

This makes it possible for single bitplane tiles to take all possible color schemes when they end up in VRAM. However, it is not possible to (natively) load single bitplane graphics directly from the disk into VRAM; it should be loaded into PRG-RAM before transferring the data into VRAM. In read mode, all non "data" bitplanes are replaced by dummy reads. 

### Object structure
    
    
    SIZE   CONTENTS
    1      Object render flag
                $00 - Render as normal
                $01-$7F - Skip rendering
                $80-$FF - Hide object
    1      Y position of upper left of object
    1      8-bit fractional portion of Y position
    1      X position of upper left of object
    1      8-bit fractional portion of X position
    1      Animation frame of object
    2      Object tile arrangement pointer (big endian)
    1      Object flags
                Bit 4: 1 to flip object x-wise
                Bit 0: 1 to flip object y-wise
    1      Palette of object
    1      Upper nybble - height of object, lower nybble - width of object
    1      Object index in OAM
    

X/Y positions directly correspond to X/Y positions in OAM. The fractional position bytes are not used directly by $EC22, so they can actually be used for any purpose. The intended purposes of those bytes were derived from other code in the BIOS. 

Object tiles can be arranged in two ways, by specifying a pointer to the arrangement data, or by simply specifying the upper-left tile ID. In both cases, the tiles are arranged column by column. 

If the pointer value is less than $100, the value directly specifies the upper left tile ID, which is then added to (animFrame * width * height) to get the actual first tile ID. For example, displaying animation frame $01 of a 3x3 object whose pointer value is $0010 results in the following sprite arrangement: 
    
    
    $19 $1c $1f
    $1a $1d $20
    $1b $1e $21
    

Otherwise, the pointer value is an actual pointer to the tile arrangement data. Let's say we're using 2x3 sprites, our pointer is $6000, and we have the following data at $6000: 
    
    
    $6000: $00 $01 $02 $03 $04 $05
    $6006: $ff $fe $fd $fc $fb $fa
    $600c: $22 $33 $44 $55 $66 $77
    

For animation frames $00-$02, we get the following tile arrangements: 
    
    
    Frame $00  Frame $01  Frame $02
     $00 $03    $ff $fc    $22 $55
     $01 $04    $fe $fb    $33 $66
     $02 $05    $fd $fa    $44 $77
    

## See Also

  * [GitHub repository:](https://github.com/TakuikaNinja/FDS-BIOS-Dumper) Disk program to dump the FDS BIOS.



## Notes

  1. ↑ [Forum thread:](https://forums.nesdev.org/viewtopic.php?p=296408#p296408) Controller polling oddities in _Tetris_ (Nintendo) and _Dr. Mario_ , which reuse FDS BIOS routines.



## References

  * [FDS technical reference.txt](https://nesdev.org/FDS%20technical%20reference.txt) by Brad Taylor, including a BIOS disassembly.
  * [Enri's Famicom Disk System page](http://cmpslv3.stars.ne.jp/Famic/Famdis.htm) (Japanese)


