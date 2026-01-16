# User:Zzo38/Compound NSF

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Compound_NSF) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Compound_NSF)

This is a draft proposal of "Compound NSF" format. The file is a Hamster archive. All chunks are optional other than `PAYLOAD.NSF`. Unrecognized chunks should be ignored. An implementation does not need to read all chunks; it can skip any chunk that it doesn't need. 

**Hamster archive** is an [archive format used by OHRRPGCE](http://rpg.hamsterrepublic.com/ohrrpgce/RPG_format). Each file in the archive, or "lump", consists of an ASCII filename, a NUL byte ($00), a length in PDP-11 byte order (medium high byte, high byte, low byte, medium low byte), and the file contents. Filenames are case-insensitive, and contents are uncompressed. The [HAMARC tool](http://rpg.hamsterrepublic.com/ohrrpgce/Ohrtool1#HAMARC) manipulates Hamster archives. 

## Contents

  * 1 COMMENT.TXT
  * 2 COMPOSER.TXT
  * 3 CONTROLS.BIN
  * 4 DURATION.TXT
  * 5 METADATA.RDF
  * 6 PAYLOAD.NSF
  * 7 SONGTYPE.BIN
  * 8 TITLE.TXT



## COMMENT.TXT

Contains arbitrary text, and is a comment made by the composer and/or programmer. 

## COMPOSER.TXT

List of composers of each song, one per each line. First line for song $00, second line for song $01, etc. 

## CONTROLS.BIN

Contains a number of small-endian 16-bit numbers for various optional features. List using word offsets (not byte offsets): 

  * $00: Start RAM offset to load random data into.
  * $01: End RAM offset to load random data into. If supported, all from the start and end offset but not including the end offset, must be RAM addresses, and are filled with random data before calling init.
  * $02: If nonzero, write to this RAM address (just before calling play) to request to start a sound effect. The play routine should clear it if it wants to keep track when the same data is written again.
  * $03: If nonzero, write to this RAM address to request to stop a sound effect.
  * $04: If nonzero, read this RAM address just after play/init returns for trigger events (similar to FastTracker's Wxx command).
  * $05: The suggested console variant to emulate for mixing purposes: 0=nothing specified, 1=RF Famicom, 2=AV Famicom, 3=NTSC NES, 4=PAL NES.



Notes: 

  * If the start and end address of random data are both the same (such as both zero), then this function is not used.
  * If random data isn't supported, the contents of RAM will be zeroed out, as per the standard NSF specification.
  * The RAM addresses to request sound effects would probably not be used when playing back on an actual NES/Famicom, nor in most other cases. It may be used when the NSF is used in a game for PC or another system; for example if MegaZeux hypothetically supported this format, the MOD SAM command would poke its second parameter into the RAM address given at word offset $02 (and ignore the first parameter).
  * Requesting to start a sound effect here does not necessarily actually mean to start a sound effect; the NSF code may do other things with it, such as add/remove/change accompaniment, change the playback rate, play the music an octave lower, transpose into a minor key, start playing backwards from the current point, etc.
  * The trigger events also aren't used in an actual NES/Famicom (although it is still possible, such as to implement visuals) nor in most other cases.
  * The console variant field is not used for hardware playback. Emulators may choose to use this information to decide mixing volume in order to sound more like the console that the game was tested on or that the music was written for. People who make up their own NSF though should be aware that this can be ignored like everything else outside of the PAYLOAD.NSF lump.



## DURATION.TXT

Default duration for each song when using with playlists, in frames. 

## METADATA.RDF

Contains data in RDF Turtle format which is irrelevant to playback software/hardware. Uses may include: 

  * Media library software may use it to fill in additional data (and/or alternate data for title such as if the actual title is longer) when adding records into its media library database.
  * Submission to servers on the internet that host such music (whether only NSF or other kinds too) may use this to full in additional data (and/or alternate data) when adding records to its database.



Notes: 

  * Unless whoever makes this data is using it with specific software and knows how it uses this lump, you should be aware that any other program will possibly ignore some or all of the data in this lump.
  * You might use <gopher://zzo38computer.org/1ns/meta:primary> to specify which node (may be a URI, blank node, or possibly the default base URI) is the identifier of this music file.



## PAYLOAD.NSF

This chunk has the same as ordinary [NSF](NSF.xhtml "NSF") (version 1) except the header modified as follows: 

  * Offset $003: If this is "`M`", then it is a normal NSF. If this is "`m`", then it is a interrupt-based NSF; see below.
  * Offset $006: If this is zero, then there are 256 songs in total.
  * Offset $007: If this is zero, then the default song is number $FF.
  * Offset $07C: Volume controls. Bit3 is MMC5 volume and bit2-bit0 is N163 volume. Other bits should always be clear. (This may be subject to moving to another offset for compatibility with NSF2)



MMC5 volume: 

  * 0 = use default (same as 2A03 square waves)
  * 1 = use marked resistance value



N163 volume: 

  * 0 = default (automatically)
  * 1 = 3.6x (Final Lap)
  * 2 = 4.0x (Megami Tensei II)
  * 3 = 6.0x
  * 4 = 6.5x (Rolling Thunder)
  * 5 = 7.3x (King of Kings)
  * 6 = 8.0x
  * 7 = 8.5x (Erika to Satoru no Yumebouken)



Interrupt-based NSF: 

  * The play address and playback rates are not used at all, and must be zero.
  * IRQ can now be used, including DPCM IRQ, APU frame counter IRQ, MMC5 PCM IRQ, and the new one mentioned below.
  * IRQ is initially disabled (you can use the CLI instruction to enable it).
  * The IRQ vector is no longer reserved by the host, therefore it can be reprogrammed by bankswitching.
  * Init should not return. (If it does return, it is an error.)
  * Address $401C is low 16-bits of IRQ reload value, write-only.
  * Address $401D is high 16-bits of IRQ reload value, write-only.
  * Address $401F is IRQ status. Write 0 to turn off timer, write 1 to turn on timer, read/write anything to acknowledge.



Notes: 

  * Adding ADPCM would be a mess (for several reasons, including that different cartridges use different addresses, and that it won't be able to load a copy of the ROM into original hardware (although it may still be usable with clone hardware)), although it is unlikely that ADPCM would be needed for music, so it is probably OK to not add it.
  * An expansion chip that may be added in future though may be [UNL-DripGame](UNIF_UNL_DripGame.xhtml "UNIF/UNL-DripGame"). (Would probably use $5000 read-only and $5800 read-only and $8000-$8007 write-only.)
  * If the new features listed above aren't used, then you can simply extract this lump into a separate file and use any ordinary NSF playback program. It might also be possible to use the volume controls without confusing some existing software (although the playback volume might then be wrong).
  * Interrupt-based NSF is based somewhat on NSF2, but now it does not interfere with APU test mode.
  * It is recommended not to use interrupt-based NSF if you can avoid it. However, sometimes it is useful.



## SONGTYPE.BIN

Starting bit7 of first byte for song $00, bit6 for song $01, etc, having 32 bytes in total, bit is set if it is a sound effect rather than a full song. 

## TITLE.TXT

List of song titles, one per each line. First line for song $00, second line for song $01, etc. 
