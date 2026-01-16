# NES 2.0 header for cc65

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_header_for_cc65) | View [other pages](Special_AllPages.xhtml#NES_2_0_header_for_cc65)

A cc65 compatible header file for generating a [NES 2.0](NES_2_0.xhtml "NES 2.0") with macros 

The original ASM version for ca65 is available [NES 2.0 header for ca65](NES_2_0_header_for_ca65.xhtml "NES 2.0 header for ca65")
    
    
    /**
     * NES 2.0 header generator for cc65 (nes2header.h)
     * 
     * USAGE: Generates a header for the NES2 format with C macros
     *
     * NES2_HEADER(segmentname) - Creates a NES header in the segment provided by segment name.
     * The values can be configured using all the of the options described below.
     *
     * EXAMPLE:
     * 
     * #define NES2_MAPPER 4    // REQUIRED: Choose mapper number 4
     * #define NES2_PRG 65536   // REQUIRED: With 64kb PRG ROM
     * #define NES2_BRAM 8192   // And 8kb PRG RAM
     * #define NES2_MIRROR 'V'  // Vertical screen mirroring
     * #define NES2_TV 'N'      // For NTSC tvs
     * NES2_HEADER("INESHDR");  // and now build the header into the segment named `INESHDR`
     * 
     * DESCRIPTION: The following values can be defined to set the corresponding NES 2.0 header fields
     *
     * - NES2_MAPPER, NES2_SUBMAPPER
     *    Sets the mapper (board class) ID.  For example, MMC3 is usually
     *    mapper 4, but TLSROM is 118 and TQROM is 119.  Some mappers have
     *    variants.
     *    NO DEFAULT (Required)
     *    Example: (sets the mapper to MMC3 with submapper variant 1 used by StarTropics)
     *     #define NES2_MAPPER 4
     *     #define NES2_SUBMAPPER 1
     * 
     * - NES2_PRG
     *    Sets the PRG ROM size to sz bytes. Must be multiple of 16384;
     *    should be a power of 2.
     *    NO DEFAULT (Required)
     *    Example: #define NES2_PRG 131072
     * 
     * - NES2_CHR
     *    Sets the CHR ROM size to sz bytes. Must be multiple of 8192;
     *    should be a power of 2.
     *    Default value: 0
     *    Example: #define NES2_CHR 32768
     * 
     * - NES2_WRAM
     *    Sets the (not battery-backed) work RAM size in bytes.
     *    Default is 0.
     *    Example: #define NES2_WRAM 8192
     * 
     * - NES2_BRAM
     *    Sets the battery-backed work RAM size in bytes.
     *    Default is 0.
     *    Example: #define NES2_BRAM 8192
     * 
     * - NES2_CHRBRAM
     *    Sets the battery-backed CHR RAM size in bytes.
     *    Default is 0.
     *    Example: #define NES2_CHRBRAM 8192
     * 
     * - NES2_MIRROR
     *    Sets the mirroring to one of these values:
     *    'H' (horizontal mirroring, vertical arrangement)
     *    'V' (vertical mirroring, horizontal arrangement)
     *    '4' (four-screen VRAM)
     *    '8' (four-screen and vertical bits on, primarily for mapper 218)
     *    Default is 'H'
     *    Example: #define NES2_MIRROR 'V'
     * 
     * - NES2_TV
     *    Sets the ROM's intended TV system:
     *    'N' for NTSC NES/FC/PC10
     *    'P' for PAL NES
     *    '1' for dual compatible, preferring NTSC
     *    '2' for dual compatible, preferring PAL NES
     *    Default is 'N'
     *    Example: #define NES2_TV '1'
     * 
     * Original ca65 macro Copyright 2016 Damian Yerrick
     * cc65 macro Copyright 2022 James Rowe
     * Copying and distribution of this file, with or without
     * modification, are permitted in any medium without royalty provided
     * the copyright notice and this notice are preserved in all source
     * code copies.  This file is offered as-is, without any warranty.
     * 
     */
    
    #ifndef NES2HEADER_P_H
    #define NES2HEADER_P_H
    
    #ifndef NES2_SUBMAPPER
      #define NES2_SUBMAPPER 0x0
    #endif //NES2_SUBMAPPER
    
    // Define the bytes for the mapper fields
    #define _NES2_MAPPER6 (((NES2_MAPPER) & 0x0F) << 4)
    #define _NES2_MAPPER7 ((NES2_MAPPER) & 0xF0)
    #define _NES2_MAPPER8 (((NES2_MAPPER) >> 8) | ((NES2_SUBMAPPER) << 4))
    
    #define _NES2_PRG_SIZE (((NES2_PRG) / 16384) & 0xFF)
    #define _NES2_PRG_SIZE_HI ((((NES2_PRG) / 16384) >> 8) & 0xFF)
    
    // Define the defaults for the constants
    #ifdef NES2_CHR
      #define _NES2_CHR_SIZE (((NES2_CHR) / 8192) & 0xFF)
      #define _NES2_CHR_SIZE_HI ((((NES2_CHR) / 8192) >> 8) & 0xFF)
    #else
      #define _NES2_CHR_SIZE 0
      #define _NES2_CHR_SIZE_HI 0
    #endif //NES2_CHR_SIZE
    
    // Helper function: Computes ceil(log2(sz / 64)) Used for NES2 RAM sizes.
    #define _NES2_ERROR -1
    #define _NES2_LOGSIZE(sz) \
      (((sz) < 1L)       ? 0  : \
       ((sz) <= 128L)    ? 1  : \
       ((sz) <= 256L)    ? 2  : \
       ((sz) <= 512L)    ? 3  : \
       ((sz) <= 1024L)   ? 4  : \
       ((sz) <= 2048L)   ? 5  : \
       ((sz) <= 4096L)   ? 6  : \
       ((sz) <= 8192L)   ? 7  : \
       ((sz) <= 16384L)  ? 8  : \
       ((sz) <= 32768L)  ? 9  : \
       ((sz) <= 65536L)  ? 10 : \
       ((sz) <= 131072L) ? 11 : \
       ((sz) <= 262144L) ? 12 : \
       ((sz) <= 524288L) ? 13 : \
       ((sz) <= 1048576L)? 14 : _NES2_ERROR)
    
    #define _NES2_MIRROR_HELPER(x) \
        (((x) == 'h' || (x) == 'H') ? 0 : \
         ((x) == 'v' || (x) == 'V') ? 1 : \
         ((x) == '4') ? 8 : \
         ((x) == '8') ? 9 : _NES2_ERROR)
    
    #ifdef NES2_MIRROR
      #define _NES2_MIRROR_OUT (_NES2_MIRROR_HELPER(NES2_MIRROR))
    #else
      #define _NES2_MIRROR_OUT (_NES2_MIRROR_HELPER('H'))
    #endif //NES2_MIRROR
    
    #ifdef NES2_WRAM
      #define _NES2_WRAM_SIZE (_NES2_LOGSIZE(NES2_WRAM))
    #else
      #define _NES2_WRAM_SIZE (_NES2_LOGSIZE(0))
    #endif //NES2_WRAM
    
    #ifdef NES2_BRAM
      #define _NES2_BRAM_SIZE (_NES2_LOGSIZE(NES2_BRAM))
    #else
      #define _NES2_BRAM_SIZE (_NES2_LOGSIZE(0))
    #endif //NES2_BRAM
    
    #ifdef NES2_CHRBRAM
      #define _NES2_CHRBRAM_SIZE (_NES2_LOGSIZE(NES2_CHRBRAM))
    #else
      #define _NES2_CHRBRAM_SIZE (_NES2_LOGSIZE(0))
    #endif //NES2_CHRBRAM
    
    #ifdef NES2_CHRRAM
      #define _NES2_CHRRAM_SIZE (_NES2_LOGSIZE(NES2_CHRRAM))
    #else
      // CHRRAM size depends on if the mapper has CHRBRAM or CHR already defined 
    #define _NES2_CHRRAM_SIZE \
      ((((_NES2_CHR_SIZE) + _NES2_CHR_SIZE_HI + (_NES2_CHRBRAM_SIZE)) > 0) ? 0 : _NES2_LOGSIZE(32768L))
    #endif
    
    // Battery bit is set if we have either BRAM or CHRBRAM
    #define _NES2_BATTERY_BIT \
      ((((_NES2_BRAM_SIZE) + (_NES2_CHRBRAM_SIZE)) > 0) ? 0x02 : 0x0)
    
    #define _NES2_TV_HELPER(x) \
        (((x) == 'n' || (x) == 'N') ? 0 : \
         ((x) == 'p' || (x) == 'P') ? 1 : \
         ((x) == '1') ? 2 : \
         ((x) == '2') ? 3 : -1)
    
    #ifdef NES2_TV
      #define _NES2_TV_SYSTEM _NES2_TV_HELPER(NES2_TV)
    #else
      #define _NES2_TV_SYSTEM _NES2_TV_HELPER('N')
    #endif //NES2_TV
    
    #define _NES2_STRINGIFY(a) #a
    
    #define NES2_HEADER(segment) \
      _Pragma( _NES2_STRINGIFY( rodata-name ## ( ## push, ## segment ## ) ## ) ); \
      _Static_assert (_NES2_WRAM_SIZE != _NES2_ERROR, "WRAM size must be 0 to 1048576"); \
      _Static_assert (_NES2_BRAM_SIZE != _NES2_ERROR, "BRAM size must be 0 to 1048576"); \
      _Static_assert (_NES2_CHRBRAM_SIZE != _NES2_ERROR, "CHRBRAM size must be 0 to 1048576"); \
      _Static_assert (_NES2_MIRROR_OUT != _NES2_ERROR, "Mirroring mode must be 'H', 'V', '4', or '8'"); \
      _Static_assert (_NES2_TV_SYSTEM != _NES2_ERROR, "TV system must be 'N', 'P', '1', or '2'"); \
      const unsigned char nes2header[16] = {\
        'N', 'E', 'S', 0x1A,\
        _NES2_PRG_SIZE, _NES2_CHR_SIZE,\
        _NES2_MAPPER6 | _NES2_MIRROR_OUT | _NES2_BATTERY_BIT, \
        _NES2_MAPPER7 | 0x08, \
        _NES2_MAPPER8, \
        (_NES2_CHR_SIZE_HI << 4) | _NES2_PRG_SIZE_HI, \
        (_NES2_BRAM_SIZE << 4) | _NES2_WRAM_SIZE, \
        (_NES2_CHRBRAM_SIZE << 4) | _NES2_CHRRAM_SIZE, \
        _NES2_TV_SYSTEM, 0, 0, 0 \
      };\
      _Pragma("rodata-name(pop)");
    
    #endif //NES2HEADER_P_H
    

  


## Linker script requirement

Your linker configuration file will need to have a segment for the header in the first ROM memory area. This segment can have any name, but should have 16 bytes of size. 
    
    
    MEMORY {
      HEADER: start = 0, size = $0010, type = ro, file = %O, fill=yes, fillval=$00;
      # Other memory area definitions appropriate for your board
    }
    SEGMENTS {
      INESHDR:  load = HEADER, type = ro, align = $10;
      # Other segment definitions appropriate for your board
    }
    

  


## Examples

A [CNROM](CNROM.xhtml "CNROM") board with horizontal [mirroring](Mirroring.xhtml "Mirroring") (V pad) for NTSC systems: 
    
    
    #include "nes2header.h"
    #define NES2_MAPPER 3
    #define NES2_PRG 32768
    #define NES2_CHR 32768
    #define NES2_MIRROR 'H'
    #define NES2_TV 'N'
    NES2_HEADER("INESHDR");
    

An [SLROM](MMC1.xhtml "SxROM") board with 128 KiB PRG ROM, 128 KiB CHR ROM, 8 KiB battery-backed WRAM, and PAL-preferred but dual-compatible program: 
    
    
    #include "nes2header.inc"
    #define NES2_MAPPER 1
    #define NES2_PRG 131072
    #define NES2_CHR 131072
    #define NES2_BRAM 8192
    #define NES2_TV 'P','N'
    NES2_HEADER("INESHDR");
    

An [MMC3](MMC3.xhtml "MMC3") board with 512 KiB PRG ROM, 32 KiB CHR RAM, and NTSC-only program: 
    
    
    #include "nes2header.inc"
    #define NES2_MAPPER 4
    #define NES2_PRG 524288
    #define NES2_CHRRAM 32768
    #define NES2_TV 'N'
    NES2_HEADER("INESHDR");
    
