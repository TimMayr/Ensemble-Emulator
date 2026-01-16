# Programming UNROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Programming_UNROM) | View [other pages](Special_AllPages.xhtml#Programming_UNROM)

**UNROM** is the common name for a discrete mapper found on the UNROM board as well as the less common **UOROM** board. UNROM has 64 KB or 128 KB PRG-ROM (divided into 8 16k banks) and CHR-RAM. The UOROM board works the same way and can take PRG-ROM up to 256 KB (16 banks). It is very easy to use once you know how to load CHR RAM. 

## Contents

  * 1 iNES header
  * 2 Bankswitching
  * 3 Variants with non-consecutive bank numbers
    * 3.1 UN1ROM
    * 3.2 Sunsoft-3R
    * 3.3 Watermarking
  * 4 See also
  * 5 External links



## iNES header

Here is an [NES 2.0](NES_2_0.xhtml "NES 2.0") header for mapper 2 on the UNROM and UOROM boards. It should be backward-compatible with emulators supporting only the older [iNES](INES.xhtml "INES") header format, but they may emulate [extra RAM at $6000-$7FFF](PRG_RAM_circuit.xhtml "PRG RAM circuit"), where official boards have [open bus](Open_bus_behavior.xhtml "Open bus"). 
    
    
    .segment "HEADER"
      .byte "NES", $1A
      .byte 8         ;UNROM has 8 16k banks; change this to 4 or 16 as needed
      .byte 0         ;No CHR ROM
      .byte $20, $08  ;Mapper 2, horizontal mirroring, NES 2.0
      .byte $00       ;No submapper
      .byte $00       ;PRG ROM not 4 MiB or larger
      .byte $00       ;No PRG RAM
      .byte $07       ;8192 (64 * 2^7) bytes CHR RAM, no battery
      .byte $00       ;NTSC; use $01 for PAL
      .byte $00       ;No special PPU
    

To use vertical [mirroring](Mirroring.xhtml "Mirroring") instead of horizontal mirroring, change the `.byte $20, $08` to `.byte $21, $08`

## Bankswitching

UNROM has four or eight banks 16 KB in size; UOROM has 16 banks. The last of these banks is fixed at $C000-$FFFF. The rest (numbered 0-2, 0-6, or 0-14) are switchable at $8000-$BFFF. 

Switching banks requires a write to $8000-$FFFF. In UNROM, bits 0-2 of the byte written to $8000-$FFFF will select the bank; UOROM uses bits 0-3. When writing to $8000-$FFFF, the value you are writing must match the value located at the destination address in ROM (see [Bus conflict](Bus_conflict.xhtml "Bus conflict")). One way to ensure this is to have a bankswitch lookup table. You can read from this table and then immediately write that value back to the table. 
    
    
    .segment "RODATA"
    banktable:              ; Write to this table to switch banks.
      .byte $00, $01, $02, $03, $04, $05, $06
      .byte $07, $08, $09, $0A, $0B, $0C, $0D, $0E
      ; UNROM needs only the first line of this table (0-6)
      ; but UOROM needs both lines (0-14).
    
    .segment "ZEROPAGE":    ; The mapper is read-only; need to track its state separately
    current_bank: .res 1
    
    .segment "CODE"
    bankswitch_y:
      sty current_bank      ; save the current bank in RAM so the NMI handler can restore it
    bankswitch_nosave:
      tya                   ; transfer the bank value to A
      sta banktable, y      ; and write it to the corresponding entry, switching banks
      rts
    

The lookup table and the bankswitching subroutine are normally located in the fixed bank ($C000-$FFFF). 

To save 12 cycles per bankswitch at a cost of 5 bytes of ROM, the `bankswitch_y` subroutine can be made into a macro. 

With the lookup table and bankswitching subroutine in place, switching banks is as easy as this: 
    
    
      ldy #$02
      jsr bankswitch_y     ;switch to bank 2
    

If you switch banks in the NMI handler, such as to run a sound engine, do not write to `current_bank`. Instead, do this at the end of the NMI handler just before pulling registers: 
    
    
      ldy current_bank
      jsr bankswitch_nosave
    

## Variants with non-consecutive bank numbers

Some mappers behave like UNROM except with non-consecutive PRG bank numbers. For example, they might not start the bank number at the least significant bit. For these, a program can arrange its bus conflict prevention table to map consecutive bank numbers to the format needed. This both permits the bus conflict prevention table to be contiguous, as well as to encode any other state needed for correct operation. 

This requires a one-line change to the `bankswitch_y` routine. 
    
    
    bankswitch_y:
      sty current_bank      ; save the current bank in RAM so the NMI handler can restore it
    bankswitch_nosave:
      lda banktable, y      ; remap from the convenient banks numbered 0 through 6 to the value needed by the hardware
      sta banktable, y      ; and write it to the corresponding entry, switching banks
      rts
    

Some applications: 

### [UN1ROM](INES_Mapper_094.xhtml "INES Mapper 094")
    
    
    .segment "RODATA"
    banktable:
      .byte $00, $04, $08, $0C, $10, $14, $18
    

### [Sunsoft-3R](INES_Mapper_093.xhtml "INES Mapper 093")
    
    
    .segment "RODATA"
    banktable:
      .byte $01, $11, $21, $31, $41, $51, $61
    

Sunsoft-3R requires the $01 or else it will disable its CHR RAM. 

### [Watermarking](Watermarking.xhtml "Watermarking")

Even on ordinary UNROM, one could shuffle the contents of `banktable` to watermark the program. 

## See also

  * [UxROM](UxROM.xhtml "UxROM") technical reference
  * [How to load CHR RAM](CHR_ROM_vs__CHR_RAM.xhtml#Switching_to_CHR_RAM "CHR-ROM vs CHR-RAM")



## External links

  * [SNROM project template](https://github.com/pinobatch/snrom-template) by Damian Yerrick on GitHub: also covers UNROM and UOROM


