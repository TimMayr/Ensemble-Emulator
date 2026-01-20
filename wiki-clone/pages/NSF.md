# NSF

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NSF) | View [other pages](Special_AllPages.xhtml#NSF)

The NES Sound Format (.nsf) is used for storing and playing music from the NES and related systems. It is similar to the PSID file format for C64 music/sound, where one rips the music/sound code from an NES game and prepends a small header to the data. An NSF player puts the music code into memory at the proper place, based on the header, prepares sound hardware, then runs it to make music. An NSF can be played on NES/Famicom hardware or in an emulator (NSF player or NES emulator). 

There are two extensions of the NSF format: 

  * [NSFe](NSFe.xhtml "NSFe") \- Allows a playlist with track titles and times, as well as other metadata.
  * [NSF2](NSF2.xhtml "NSF2") \- A backward compatible extension including NSFe's metadata, IRQ and other features.



## Contents

  * 1 Header Overview
  * 2 Loading a tune into RAM
    * 2.1 Bank Switching
      * 2.1.1 FDS Bankswitching
      * 2.1.2 Example
  * 3 Initializing a tune
  * 4 Playing a tune
  * 5 Sound Chip Support
    * 5.1 APU
    * 5.2 VRCVI
    * 5.3 VRCVII
    * 5.4 FDS Sound
    * 5.5 MMC5 Sound
    * 5.6 Namco 163 Sound
    * 5.7 Sunsoft 5B Sound
    * 5.8 Multi-chip tunes
  * 6 Caveats
  * 7 Summary of Addresses
  * 8 Pseudo-IRQ Technique
  * 9 See also
  * 10 References



## Header Overview

All 2-byte address and period values are little endian. For example, an NTSC play speed of `FF 40` means $40FF, or 16639 microseconds. 
    
    
    offset  # of bytes   Function
    ----------------------------
    $000    5   STRING  'N','E','S','M',$1A (denotes an NES sound format file)
    $005    1   BYTE    Version number $01 (or $02 for [NSF2](NSF2.xhtml "NSF2"))
    $006    1   BYTE    Total songs   (1=1 song, 2=2 songs, etc)
    $007    1   BYTE    Starting song (1=1st song, 2=2nd song, etc)
    $008    2   WORD    (lo, hi) load address of data ($8000-FFFF)
    $00A    2   WORD    (lo, hi) init address of data ($8000-FFFF)
    $00C    2   WORD    (lo, hi) play address of data ($8000-FFFF)
    $00E    32  STRING  The name of the song, null terminated
    $02E    32  STRING  The artist, if known, null terminated
    $04E    32  STRING  The copyright holder, null terminated
    $06E    2   WORD    (lo, hi) Play speed, in 1/1000000th sec ticks, NTSC (see text)
    $070    8   BYTE    Bankswitch init values (see text, and FDS section)
    $078    2   WORD    (lo, hi) Play speed, in 1/1000000th sec ticks, PAL (see text)
    $07A    1   BYTE    PAL/NTSC bits
                    bit 0: if clear, this is an NTSC tune
                    bit 0: if set, this is a PAL tune
                    bit 1: if set, this is a dual PAL/NTSC tune
                    bits 2-7: reserved, must be 0
    $07B    1   BYTE    Extra Sound Chip Support
                    bit 0: if set, this song uses [VRC6 audio](VRC6_audio.xhtml "VRC6 audio")
                    bit 1: if set, this song uses [VRC7 audio](VRC7_audio.xhtml "VRC7 audio")
                    bit 2: if set, this song uses [FDS audio](FDS_audio.xhtml "FDS audio")
                    bit 3: if set, this song uses [MMC5 audio](MMC5_audio.xhtml "MMC5 audio")
                    bit 4: if set, this song uses [Namco 163 audio](Namco_163_audio.xhtml "Namco 163 audio")
                    bit 5: if set, this song uses [Sunsoft 5B audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio")
                    bit 6: if set, this song uses [VT02+ audio](VT02__Sound.xhtml "VT02+ Sound")
                    bit 7: reserved, must be zero
    $07C    1   BYTE    Reserved for [NSF2](NSF2.xhtml "NSF2")
    $07D    3   BYTES   24-bit length of contained program data.
                    If 0, all data until end of file is part of the program.
                    If used, can be used to provide [NSF2 metadata](NSF2.xhtml#Metadata "NSF2")
                    in a backward compatible way.
    $080    nnn ----    The music program/data follows
    

Strings are usually encoded in plain ASCII. In most game rips Japanese titles have been romanized into plain ASCII. More rarely NSFs have also used extended characters, the most common of which is [Windows CP-932](https://en.wikipedia.org/wiki/Code_page_932_\(Microsoft_Windows\)) (Shift-JIS) for Japanese titles. [Windows CP-1252](https://en.wikipedia.org/wiki/Windows-1252) (Latin) encoding examples may also exist. 

[NSF2](NSF2.xhtml "NSF2") and [NSFe](NSFe.xhtml "NSFe") should use UTF-8 instead, or plain ASCII for backward compatibility. 

## Loading a tune into RAM

If file offsets $070 to $077 have $00 in them, then bank switching is _not_ used. Data should be read from the file beginning at $080 and loaded contiguously into the 6502 address space beginning at the load address until the end of file is reached. 

Some FDS NSFs use a load address below $8000 to fill in the $6000-7FFF range. It is recommended to use bankswitching to accomplish this instead, because it is not universally supported. 

### Bank Switching

If any of the bytes from $070 to $077 in the file header are nonzero then bank switching is used. In this case, take the logical AND of the load address with $0FFF, and the result specifies the number of bytes of padding at the start of the ROM image. The ROM image should consist of a contiguous set of 4k banks, read directly from the NSF file beginning at $080 after inserting the requested number of pad bytes. If the file does not have enough data to fill the last bank completely, it may be padded out. 

The 6502's address space is divided into 8 4k bank switchable blocks. For each block the current bank is controlled by writing the bank number to at corresponding register at $5FF8 through $5FFF. The initial bank assignment is determined by bytes $070 through $077 in the file. 
    
    
    NSF    Address      Register
    ====   ==========   ========
    $070   $8000-8FFF   $5FF8
    $071   $9000-9FFF   $5FF9
    $072   $A000-AFFF   $5FFA
    $073   $B000-BFFF   $5FFB
    $074   $C000-CFFF   $5FFC
    $075   $D000-DFFF   $5FFD
    $076   $E000-EFFF   $5FFE
    $077   $F000-FFFF   $5FFF
    

The initial bank assignment should be done before any call to the _INIT_ routine. Once the ROM image has been built from the NSF file, this can be set up simply by writing the 8 values from the file header $070-077 to the corresponding registers $5FF8-$5FFF. 

If the _INIT_ routine needs to change the bank assignments based on the selected song, it may do so by writing the bank control registers. 

#### FDS Bankswitching

If the FDS expansion is enabled, bank switching operates slightly differently. Two additional registers at $5FF6 and $5FF7 control the banks $6000-6FFF and $7000-7FFF respectively, and the initial load values at $076 and $077 now specify the banks used for $6000-7FFF as well as $E000-FFFF (these regions will both be set up to use the same banks before _INIT_ is called). 

Because the FDS has a RAM area at $8000-DFFF for the disk image to be loaded to, that means this area is writable when the FDS expansion is enabled. Some NSF player implementations will treat this like bankswitched RAM, and some players will treat an FDS bank switch operation as a copy into RAM. Hardware players are more likely to use bankswitched RAM. 

This has a number of caveats: 

  * Writes may be mirrored if the same bank is used in multiple places. Care should be taken to avoid accidental overwrites when the same bank appears more than once in the bankswitch table. In particular, unique banks should be used for memory regions that must be written to.
  * Since the FDS itself was incapable of mirrored writes like this and many players will not have them, mirrored writes should not intentionally be used to store the same data in two memory locations. It is a side effect, not a supported feature.
  * Writes to the area may or may not persist in the bank written to if it is switched out and then switched back in. This is another side effect that should be accounted for, but not relied upon.
  * Writes may or may not persist between songs, depending on whether the NSF player reloads the NSF image when the song is changed. Hardware players are not likely to reload, but software players may.



See also the notes on multi-chip tunes below. 

#### Example

METROID.NSF will be used for the following explanation. 
    
    
    The file is set up like so:  (starting at $070 in the file)
    
    $070: 05 05 05 05 05 05 05 05
    $078: 00 00 00 00 00 00 00 00
    $080: ... music data goes here...
    

Since $070-$077 are something other than $00, this NSF is using bank switching. The load address given is $8000. The load address AND $0FFF specifies 0 bytes of padding, so we set up our ROM image with contiguous data starting from $080 in the file. 

This NSF has 6 4k banks in it, numbered 0 through 5. It specifies that each of the 8 memory regions should be switched to bank 5, which begins at $05 * $1000 bytes in the ROM image. 

## Initializing a tune

The desired song number is loaded into the accumulator register A, and the X register is set to specify specify PAL (X=1) or NTSC (X=0). 

Valid song numbers are 0 to one less than the number of songs (specified at $006 in the header). The first selected song is in the header at $007. The NSF player should display to the user song numbers from 1 up to and including the number of songs, and these should correspond to the same number - 1 loaded into register A. Note that when choosing the first song from the value in $007, subtract 1 from it before loading that value into register A. 

  1. Write $00 to all RAM at $0000-$07FF and $6000-$7FFF.
  2. Initialize the sound registers by writing $00 to $4000-$4013, and $00 then $0F to $4015.
  3. Initialize the [frame counter](APU_Frame_Counter.xhtml "APU Frame Counter") to 4-step mode ($40 to $4017).
  4. If the tune is bank switched, load the bank values from $070-$077 into $5FF8-$5FFF.
  5. Set the A register for the desired song.
  6. Set the X register for PAL or NTSC.
  7. Call the music _INIT_ routine.



The _INIT_ routine MUST finish with an RTS instruction before music playback will begin. At this point, the NSF player will begin executing the _PLAY_ routine at the specified interval. 

If this is a single standard tune (PAL or NTSC but not both) the _INIT_ routine MAY ignore the X register. Otherwise, it SHOULD use this value to determine how to set pitches and tempo for the appropriate platform. 

The use of the $4017 register is not well supported by existing NSF players. The NSF should not normally clear bit 6 (the IRQ disable bit), though the Pseudo-IRQ Technique relies on being able to do this. 

While the NSF1 specification never guaranteed anything for Y on entry to INIT, for better forward compatibility with [NSF2's non-returning INIT](NSF2.xhtml#Non-Returning_INIT "NSF2") feature, it is recommended that the player set Y to 0, or at least some value that is not $80 or $81, before calling INIT. 

## Playing a tune

Once the tune has been initialized, it can now be played. To do this, simply call the routine at the _PLAY_ address at the rate determined by the file header at $06E-06F (NTSC) or $078-079 (PAL). 

The playback rate is determined by this formula: 
    
    
            1000000                  1000000
    rate = ---------       period = ---------
            period                    speed
    

Where period is the value you place at $06E-$06F in the file, and rate is how often the _PLAY_ routine should be called in Hz. 

The following playback rates are common: 

  * 60.002 Hz (recommended by the original NSF specification, close to APU timer IRQ rate): 16666 ($411A)
  * 60.099 Hz (actual NTSC NES [frame rate](Cycle_reference_chart.xhtml#Clock_rates "Cycle reference chart")): 16639 ($40FF)
  * 50.007 Hz (suggested PAL NES frame rate): 19997 ($4E1D)



Nonstandard rates may be difficult for hardware players. If the rate is much faster the _PLAY_ routine may not be short enough to execute in the specified amount of time. 

The _PLAY_ routine will be called at the specified interval. If the X register passed to _INIT_ was 1 (PAL), it will be called at the rate specified by $078-079, and if 0 (NTSC), it will use the rate at $06E-06F. 

A _PLAY_ routine should normally finish with an RTS instruction, but is not required to do so. A non-returning _PLAY_ will cause problems for NSF players that use the same CPU to control the user interface and to run the NSF, such as NSF players that run on an NES. It is strongly recommended to return every few frames if at all possible, such as when no PCM is playing. If _PLAY_ takes longer to finish than the specified interval, that interval may be skipped and _PLAY_ may not be called again until the next one. 

Some popular modern NSF engines use a non-returning _PLAY_ to implement an output stream of PCM sound (e.g. SuperNSF, MUSE, Deflemask), and this can also be combined with a Pseudo-IRQ technique. 

## Sound Chip Support

Byte $07B of the file stores the sound chip flags. If a particular flag is set, those sound registers should be enabled. If the flag is clear, then those registers should be disabled. All I/O registers within $8000-FFFF are _write only_ and must not disrupt music code that happens to be stored there. Some audio register addresses have mirrors in their original hardware mappers, but NSF code should use only the lowest address for each register, listed here. 

### APU

  * Uses registers $4000-4013, and $4015. See [APU](APU.xhtml "APU") for more information.
  * $4015 is set to 0F on reset by most players. It is better if the NSF does not assume this and initializes this register itself, but there are several existing NSF files that require it (Battletoads, Castlevania and Gremlins 2 are examples).
  * The APU interrupts that can be generated via $4015 and $4017 are not reliably available across NSF players, and have usually been considered out of bounds for NSF rips. [NSF2](NSF2.xhtml "NSF2") can explicitly allow them, however.
  * $4017 has other features that are not consistently supported across NSF players.



### VRCVI

  * Uses registers $9000-9003, $A000-A002, and $B000-B002, write only. See [VRC6 Audio](VRC6_audio.xhtml "VRC6 audio") for more information.
  * The A0 and A1 lines are flipped on a few games. If you rip the music and it sounds all funny, flip around the xxx1 and xxx2 register pairs. (i.e. 9001 and 9002) Esper2 and Madara will need this change, while Castlevania 3j will not.



### VRCVII

  * Uses registers $9010 and $9030, write only. See [VRC7 Audio](VRC7_audio.xhtml "VRC7 audio") for more information.



### FDS Sound

  * Uses registers from $4040 through $4092. See [FDS Audio](FDS_audio.xhtml "FDS audio") for more information.



Notes: 

  * $6000-DFFF is assumed to be RAM, since $6000-DFFF is RAM on the FDS. $E000-FFFF is usually not included in FDS games because it is the BIOS ROM. However, it can be used on FDS rips to help the ripper (for modified _PLAY_ /_INIT_ addresses).
  * Bank switching is different if FDS is enabled. $5FF6 and $5FF7 control banks at $6000-6FFF and $7000-7FFF, and the NSF header $076-$077 initialized both $6000-7FFF and $E000-FFFF. See above.



### MMC5 Sound

  * Uses registers $5000-5015, write only as well as $5205 and $5206, and $5C00-5FF5. see [MMC5 Audio](MMC5_audio.xhtml "MMC5 audio") for more information.



Notes: 

  * $5205 and $5206 are a hardware 8 * 8 multiplier. The idea being you write your two bytes to be multiplied into 5205 and 5206 and after doing so, you read the result back out.
  * $5C00-5FF5 should be RAM to emulate EXRAM while in MMC5 mode.



### Namco 163 Sound

  * Uses registers $4800 and $F800. See [Namco 163 audio](Namco_163_audio.xhtml "Namco 163 audio") for more information.



### Sunsoft 5B Sound

  * Audio in the Sunsoft 5B mapper, a variant of the [FME-7](Sunsoft_FME_7.xhtml "FME-7"), uses registers $C000 and $E000. See [Sunsoft audio](Sunsoft_5B_audio.xhtml "Sunsoft audio").
  * Many players do not implement the noise or envelope capabilities of the 5B, as they were not used in the only 5B game, Gimmick.



### Multi-chip tunes

Multiple expansion chips can be used at the same time, but because this was not something that was ever supported by an original Famicom games, actual practice with multi-expansion NSF varies. 

Some mappers mirror their audio registers at addresses that would conflict. Many NSF players only support the lowest address, which avoids these conflicts, but the following conflicts may need resolution in an attempted hardware multi-chip implementation: 

  * N163's address port $F800 overlaps a mirror of Sunsoft 5B's data port $E000. This can be avoided by setting Sunsoft 5B's address port $C000 to $0E or $0F (unused internal registers) before writing to the N163.
  * VRC6 and VRC7 have a conflict at ports $9010 and $9030, where the VRC6's pulse 1 control port is mirrored.
  * VRC7 and N163 each have a mute or reset register at $E000, which conflicts with Sunsoft 5B's data port. Since writing $E000 with bit 6 set will silence either of these, an emulator may wish to ignore writes to $E000 for VRC7/N163 if 5B is also present.
  * FDS will make the normally read-only area from $8000-$DFFF writable. This may cause corruption of these areas when writing to VRC6, VRC7, or 5B audio registers. The safest way to avoid this is to make sure your code and data do not fall within these addresses, so that you may safely write to them. NSF player implementations may wish to disable memory writes at these addresses to avoid the conflict.



## Caveats

  * The starting song number and maximum song numbers start counting at 1, while the _INIT_ address of the tune starts counting at 0. Remember to pass the desired song number _minus 1_ to the _INIT_ routine.
  * The NTSC speed word is used _only_ for NTSC tunes and dual PAL/NTSC tunes. The PAL speed word is used _only_ for PAL tunes and dual PAL/NTSC tunes.
  * If bit 1 of the PAL/NTSC is set, indicating a dual PAL/NTSC NSF, bit 0 may be interpreted as a preference for PAL or NTSC. Most players do not support this, however, and some older players may have problems if bit 0 is set.
  * The length of the text in the name, artist, and copyright fields must be 31 characters or less. There has to be at least a single NULL byte ($00) after the text, between fields.
  * If a field is not known (name, artist, copyright) then the field must contain the string "<?>" (without quotes).
  * There should be 8K of RAM present at $6000-7FFF. MMC5 tunes need RAM at $5C00-5FF7 to emulate its $EXRAM. $8000-FFFF should be read-only (not writable) after a tune has loaded. The only time this area should be writable is if an FDS tune is being played.
  * Do not assume the state of _anything_ on entry to the _INIT_ routine except A and X. Y can be anything, as can the flags.
  * Do not assume the state of _anything_ on entry to the _PLAY_ routine. Flags, X, A, and Y could be at any state.
  * The stack sits at $01FF and grows down. The precise position of the stack on _INIT_ or _PLAY_ is not guaranteed, as the NSF player may need to use the top area of the stack for its own internal purpose. Make sure the tune does not attempt to modify $01F0-01FF directly. (Armed Dragon Villigust did, and was relocated to 2xx for its NSF.)
  * The NSF should not initialize the stack pointer in _INIT_ or _PLAY_. These subroutines are called from the player; some software emulators may not have a problem with this, but it will almost certainly cause an error on a hardware player. It is the player's job to initialize the stack pointer, and some hardware players (e.g. [PowerPak](PowerPak.xhtml "PowerPak")) will place their own variables on the stack.
  * RAM should be addressed from $0000-07FF, and should not expect mirror addresses to work. If the tune writes outside this range, e.g. $1400 it should be relocated. (Terminator 3 did this and was relocated to 04xx for NSF.)
  * The vector table at $FFFA-FFFF should not be filled with code or data by the NSF. These can be overridden by hardware NSF players.
  * Instructions which modify the stack, PLP, PHP, and TXS must be used with great care, as a player may need to rely on being able to store data at the end of the stack. An NSF should use the stack pointer given; the stack past this pointer should remain intact, as it may be needed by the player.
  * Instructions CLI, SEI, and BRK are problematic, and should usually be avoided. The NSF itself should generally not attempt to interfere with IRQs, as many NSF players do not have an IRQ implementation. One notable exception is using SEI in a non-returning _PLAY_ routine for a pseudo-IRQ technique. BRK should generally not be used, as it reads the IRQ routine address from the vector table which is reserved for the player's use.
  * PowerPak's NSF play incorrectly does not restore the startup banks when switching tracks, so unless the PLAY routine always leaves with the INIT routine in its starting bank, switching tracks will fail on it.



## Summary of Addresses

These lists all the addresses which should be readable by the code in the NSF; no other addresses should ever be accessed for reading: 

  * $0000-$01EF
  * $01F0-$01FF (may be used internally by NSF player)
  * $0200-$07FF
  * $4015
  * $4040-$407F (if FDS is enabled)
  * $4090 (if FDS is enabled)
  * $4092 (if FDS is enabled)
  * $4800 (if Namco 163 is enabled)
  * $5205-$5206 (if MMC5 is enabled)
  * $5C00-$5FF5 (if MMC5 is enabled)
  * $6000-$FFF9



These lists all the addresses which should be writable by the code in the NSF; no other addresses should ever be accessed for writing: 

  * $0000-$01EF
  * $01F0-$01FF (may be used internally by NSF player; do not use for non-stack variables)
  * $0200-$07FF
  * $4000-$4013 (always clear bit7 of $4010)
  * $4015
  * $4040-$4080 (if FDS is enabled)
  * $4082-$408A (if FDS is enabled)
  * $4800 (if Namco 163 is enabled)
  * $5205-$5206 (if MMC5 is enabled)
  * $5C00-$5FF5 (if MMC5 is enabled)
  * $5FF6-$5FF7 (if bankswitching and FDS is enabled)
  * $5FF8-$5FFF (if bankswitching is enabled)
  * $6000-$7FFF
  * $8000-$DFFF (if FDS is enabled)
  * $9000-$9003 (if VRC6 is enabled)
  * $9010 (if VRC7 is enabled)
  * $9030 (if VRC7 is enabled)
  * $A000-$A002 (if VRC6 is enabled)
  * $B000-$B002 (if VRC6 is enabled)
  * $C000 (if Sunsoft 5B is enabled)
  * $E000 (if Sunsoft 5B is enabled)
  * $F800 (if Namco 163 is enabled)



Reading/writing anything other than specified here results in undefined behaviour. 

## Pseudo-IRQ Technique

Some modern NSFs use a trick[1] first made popular by [Deflemask](http://www.delek.com.ar/deflemask), primarily intended to support PCM sample playback. This technique is not universally supported, because it may rely on a lack of conflict with the player's implementation. Some hardware implementations do support it correctly (e.g. [PowerPak](PowerPak.xhtml "PowerPak")), and it also works with several software NSF players. 

The technique uses a non-returning _PLAY_ in the following way: 

  1. Use SEI to mask interrupts.
  2. Enable the APU interrupt by writing to $4017.
  3. Enter a sample playback loop, polling $4015 to see if an APU IRQ is pending.
  4. When the poll registers the APU IRQ flag (occurring at 60 hz on NTSC), temporarily exit the sample playback loop to do other tasks.
  5. Return to step 3, never returning from _PLAY_.



## See also

  * [List of NES music composers](List_of_NES_music_composers.xhtml "List of NES music composers")
  * [Emulation Libraries: NSF Players](Emulation_libraries.xhtml#NSF_Players "Emulation Libraries")
  * [iNES Mapper 031](INES_Mapper_031.xhtml "INES Mapper 031"): Cart mapper with NSF-inspired bankswitching
  * [NSFDRV](NSFDRV.xhtml "NSFDRV")



## References

  * [Kevtris' Official NSF spec](http://kevtris.org/nes/nsfspec.txt) \- the original NSF specification
  * [Kevtris' HardNES](http://kevtris.org/Projects/hardnes/index.html) \- a hardware NSF player project



  1. ↑ [NSF PCM technique (via Deflemask)](http://forums.nesdev.org/viewtopic.php?f=6&t=9296) forum post



Categories: [Audio](Category_Audio.xhtml), [File formats](Category_File_formats.xhtml)
