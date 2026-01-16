# NES 2.0 Mapper 487

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_487) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_487)

[![](../wiki-images/AVE_NINA08_front.jpg)](File_AVE_NINA08_front_jpg.xhtml)

[](File_AVE_NINA08_front_jpg.xhtml "Enlarge")

PCB front

[![](../wiki-images/AVE_NINA08_back.jpg)](File_AVE_NINA08_back_jpg.xhtml)

[](File_AVE_NINA08_back_jpg.xhtml "Enlarge")

PCB back

**NES 2.0 Mapper 487** denotes the AVE **NINA-08** circuit board, used only for the unreleased original 30-in-1 version of the Maxivision multicart. Unlike the released version, which uses [INES Mapper 234](INES_Mapper_234.xhtml "INES Mapper 234"), the 30-in-1 contained both AVE and Color Dreams games completely unmodified, so the board supports both original mappers' bankswitching schemes. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Outer Bank Register ($4180, write)
    * 2.2 NINA-03-compatible Inner Bank Register ($4100, write)
    * 2.3 Color-Dreams-compatible Inner Bank Register ($8000, write)
  * 3 Notes



# Banks

  * CPU $8000-$FFFF: 32 KiB window into 1536 KiB of PRG-ROM
  * PPU $0000-$1FFF: 8 KiB window into 1536 KiB of CHR-ROM
  * Nametable arrangement: switchable between horizontal and vertical



# Registers

## Outer Bank Register ($4180, write)
    
    
    D~[NMCB BBBb] A~[01.. ...1 1... ....]
       |||| |||+- PRG/CHR A15 if M=0
       |||+-+++-- PRG/CHR A19..A16 (A19 only matters when C=1)
       ||+------- Chips select and inner bank mode
       ||          0: First chip (512 KiB PRG+512 KiB CHR), [NINA-03](NINA_003_006.xhtml "INES Mapper 079") banking
       ||          1: Second and third chips (1024 KiB PRG+1024 KiB CHR), [Color Dreams](Color_Dreams.xhtml "Color Dreams") banking
       |+-------- PRG/CHR inner bank size
       |           0: 32 KiB, PRG/CHR A15 from $4180's b bit
       |           1: 64 KiB, PRG/CHR A15 from inner bank bits
       +--------- Nametable arrangement
                   0: Horizontal arrangement (Vertical mirroring)
                   1: Vertical arrangement (Horizontal mirroring)
    

## NINA-03-compatible Inner Bank Register ($4100, write)
    
    
    D~[.... PcCC] A~[01.. ...1 0... ....]
            ||++- CHR A14..13
            |+--- CHR A15 (only if $4180's M bit=1)
            +---- PRG A15 (only if $4180's M bit=1)
    

This register is only active is $4180's C bit=0. 

## Color-Dreams-compatible Inner Bank Register ($8000, write)
    
    
    D~[.cCC ...P] A~[1... .... .... ....]
        |||    +---- PRG A15 (only if $4180's M bit=1)
        |++--------- CHR A14..13
        +----------- CHR A15 (only if $4180's M bit=1)
    

This register is only active is $4180's C bit=1. 

# Notes

  * The dual use of $4180's C bit implies that games in the first 512 KiB must use NINA-03 banking, and the games in the second and third 512 KiB must use Color Dreams banking.
  * $4180's M bit switching between 32 and 64 KiB both for PRG and CHR at the same time implies that games with only PRG or only CHR having 64 KiB must still reserve 64 KiB ROM space for both PRG and CHR.
  * Though occupying non-overlapping address ranges, emulators must not recognize writes to both inner bank registers at the same time, as Mermaids of Atlantis retains [GNROM](GxROM.xhtml "GNROM") writes to $8000-$FFFF that must be ignored.



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
