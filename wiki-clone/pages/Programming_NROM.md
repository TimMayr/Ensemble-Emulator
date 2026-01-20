# Programming NROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Programming_NROM) | View [other pages](Special_AllPages.xhtml#Programming_NROM)

[NROM](NROM.xhtml "NROM") and the other boards that make up **mapper 0** are the simplest of all NES cartridge boards. All address decoding and chip enable handling are handled by the NES hardware; the only integrated circuits on the board are the PRG ROM, CHR ROM, and (in 72-pin carts) CIC. 

NROM has two configurations: 

  * NROM-256 with 32 KiB PRG ROM and 8 KiB CHR ROM
  * NROM-128 with 16 KiB PRG ROM and 8 KiB CHR ROM



Your program is mapped into $8000-$FFFF (NROM-256) or [both](Mirroring.xhtml "Mirroring") $8000-$BFFF and $C000-$FFFF (NROM-128). Most NROM-128 games actually run in $C000-$FFFF rather than $8000-$BFFF because it makes the program easier to assemble and link. Some kinds of data used by the NES CPU, such as the vectors and [sampled sound](APU_DMC.xhtml "APU DMC"), have to be in $C000-$FFFF, and it simplifies the linker script if everything is in the same memory region. There are probably a few games that rely on the mirroring, but experiments with [a multicart engine](Forbidden_Four.xhtml "Forbidden Four") show that most can run with garbage in $8000-$BFFF. 

## [NES 2.0](NES_2_0.xhtml "NES 2.0") header
    
    
    .segment "HEADER"
      .byte "NES", $1A
      .byte 2         ; 1 or 2 for NROM-128 or NROM-256 respectively
      .byte 1         ; 8 KiB CHR ROM
      .byte $00       ; Mapper 0; $00 or $01 for horizontal or vertical mirroring respectively
      .byte $08       ; Mapper 0; NES 2.0
      .byte $00       ; No submapper
      .byte $00       ; PRG ROM not 4 MiB or larger
      .byte $00       ; No PRG RAM
      .byte $00       ; No CHR RAM
      .byte $00       ; 0 or 1 for NTSC or PAL respectively
      .byte $00       ; No special PPU
    
