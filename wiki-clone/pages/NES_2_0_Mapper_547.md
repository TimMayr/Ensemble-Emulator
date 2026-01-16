# NES 2.0 Mapper 547

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_547) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_547)

**NES 2.0 Mapper 547** denotes games for the [ Konami QTa adapter](Konami_QT_adapter_pinout.xhtml "Konami QT adapter pinout"), based on the Konami [ VRC5 ASIC](VRC5_pinout.xhtml "VRC5 pinout"). Its UNIF board name is **KONAMI-QTAI** (sic). It is known to be used for the following games: 

  * _NHK 学園 - Space School: 算数 4年 (上)_
  * _NHK 学園 - Space School: 算数 4年 (下)_
  * _NHK 学園 - Space School: 算数 5年 (上)_
  * _NHK 学園 - Space School: 算数 5年 (下)_
  * _NHK 学園 - Space School: 算数 6年 (上)_
  * _NHK 学園 - Space School: 算数 6年 (下)_
  * _出光 Space College: 危険物のやさしい物理と化学_



The Konami Q-Ta adapter is inserted into the console's cartridge slot. It provides 128 KiB of internal PRG-ROM, 128 KiB of Kanji CHR pattern data, 8 KiB of non-battery-backed WRAM, and 8 KiB of CHR-RAM. It cannot be used alone, as the internal PRG-ROM contains no reset vector. Instead, custom 40-pin cartridges must be inserted. Each 40-pin game cartridge contains an additional 512 KiB of PRG-ROM plus 8 KiB of additional WRAM, which is battery-backed for all known games. 

[UNIF](UNIF.xhtml "UNIF")-format files contain the internal PRG- and CHR-ROM chips' data in PRG0 and CHR0 chunks, the 40-pin cartridges' PRG-ROM in a PRG1 chunk, and the 128 KiB CHR-ROM as a 256 KiB CHR0 chunk padded from 1 bpp to 2bpp as it appears to the PPU. [NES-2.0](NES_2_0.xhtml "NES 2.0")-format files first contain the 128 KiB internal PRG-ROM, then the external 512 KiB PRG-ROM, for a total of 640 KiB of PRG-ROM, plus 128 KiB of Kanji CHR-ROM representing the actual mask ROM content. PRG-RAM size must be 8 KiB battery-backed and 8 KiB non-battery backed; CHR-RAM size must be 8 KiB. 

The Q-Ta main adapter also has 2 KiB of "shadow" nametable memory (named _QTRAM_) that allows for automatic CHR bank switching, somewhat similarly to the [MMC5](MMC5.xhtml "MMC5")'s EXRAM. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 WRAM Bank Select ($D000/$D100, write)
    * 2.2 PRG-ROM Bank Select ($D200/$D300/$D400, write)
    * 2.3 CHR-RAM Bank Select ($D500, write)
    * 2.4 IRQ Latch Write ($D600/$D700, write)
    * 2.5 IRQ Acknowledge ($D800, write)
    * 2.6 IRQ Control ($D900, write)
    * 2.7 Nametable Control ($DA00, write)
    * 2.8 Character Translation Input ($DB00/$DC00/$DD00, write)
    * 2.9 Character Translation Output ($DC00/$DD00, read)
  * 3 CHR Banking Operation
  * 4 Character Translation Operation
  * 5 Kanji ROM layout
  * 6 See also



# Banks

  * CPU $6000-$6FFF: 4 KiB switchable PRG-RAM bank
  * CPU $7000-$7FFF: 4 KiB switchable PRG-RAM bank
  * CPU $8000-$9FFF: 8 KiB switchable PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG-ROM bank
  * CPU $C000-$DFFF: 8 KiB switchable PRG-ROM bank
  * CPU $E000-$FFFF: 8 KiB fixed PRG-ROM bank, always last bank of external cartridge
  * PPU $0000-$0FFF: 4 KiB switchable CHR-RAM bank
  * PPU $1000-$1FFF: 4 KiB fixed CHR-RAM bank #1



FCEUX' source code mentions that the PPU $1000-$1FFF bank can be switched to map to the Kanji CHR-ROM, but the details are not known, and this feature seems to be unused by all known games. 

# Registers

## WRAM Bank Select ($D000/$D100, write)
    
    
    Mask: probably $FF00
    
    $D000: Select 4 KiB PRG-RAM bank at CPU $6000-$6FFF
    $D100: Select 4 KiB PRG-RAM bank at CPU $7000-$7FFF
    
    D~7654 3210
      ---------
      .... C..B
           |  +- PRG A12
           +---- Chip select
                 0: External cartridge's 8 KiB (battery-backed)
                 1: Internal 8 KiB (not battery-backed)
                 
    

## PRG-ROM Bank Select ($D200/$D300/$D400, write)
    
    
    Mask: probably $FF00
    
    $D200: Select 8 KiB PRG-ROM bank at CPU $8000-$9FFF
    $D300: Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF
    $D400: Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF
    
    D~7654 3210
      ---------
      .CBB BBBB
       |++-++++- PRG A13-A18
       +-------- Chip select
                 0: Internal PRG-ROM (128 KiB)
                 1: External PRG-ROM (512 KiB)
                 
    

## CHR-RAM Bank Select ($D500, write)
    
    
    Mask: probably $FF00
    
    D~7654 3210
      ---------
      .... ...B
              +- CHR A12 for PPU $0000-$0FFF
    

This CHR-RAM bank is only relevant for sprite data fetches, as the bank for background tiles is determined differently. 

## IRQ Latch Write ($D600/$D700, write)
    
    
    Mask: $FF00
    
    $D600: Set 16-bit IRQ latch LSB
    $D700: Set 16-bit IRQ latch MSB
    
    

## IRQ Acknowledge ($D800, write)

Mask: probably $FF00 

Writing to this register acknowledges a pending IRQ and moves the 'A' bit to the 'E' bit. 

## IRQ Control ($D900, write)
    
    
    Mask: probably $FF00
    
    D~7654 3210
      ---------
      .... ..EA
             |+- IRQ Enable after acknowledgement
             +-- IRQ Enable (1=enabled)
    

Any write to this register will acknowledge a pending IRQ. If 'E' is set, the 16-bit counter is loaded with the 16-bit latch value. If enabled, the IRQ counter is incremented on every M2 cycle until it wraps from $FFFF to $0000, which generates an IRQ and reloads the counter with the latch. There seems to be no pseudo-scanline mode. 

## Nametable Control ($DA00, write)
    
    
    Mask: probably $FF00
    
    D~7654 3210
      ---------
      .... ..MQ
             |+- 0: Writes to PPU $2000-$2FFF go to CIRAM
             |   1: Writes to PPU $2000-$2FFF go to QTRAM
             +-- 0: Vertical mirroring
                 1: Horizontal mirroring
    

## Character Translation Input ($DB00/$DC00/$DD00, write)
    
    
    Mask: probably $FF00 
    
    $DB00: Set character tile position and attribute
    D~7654 3210
      ---------
      .... .APP
            |++- Position within 16x16 tile
            |     0: Top left
            |     1: Top right
            |     2: Bottom left
            |     3: Bottom right
            +--- Attribute
                  0: normal
                  1: alternate
    
    $DC00: Set 7-bit [JIS X 0208](https://en.wikipedia.org/wiki/JIS_X_0208 "wikipedia:JIS X 0208") column
    $DD00: Set 7-bit JIS X 0208 row
    
    

After writing to these three registers, the correct CIRAM tile number and QTRAM banking byte may be read from $DC00 and $DD00. 

## Character Translation Output ($DC00/$DD00, read)
    
    
    Mask: probably $FF00 (ROM data filled with $FF from $D400-DFFF)
    
    $DC00: Read translated tile number to be written to CIRAM
    $DD00: Read translated bank byte to be written to QTRAM
    

# CHR Banking Operation

Any time the PPU fetches graphical data for the background, it will [first fetch a nametable and attribute byte, then two pattern table bytes based on the tile number from the nametable byte](PPU_rendering.xhtml "PPU rendering"). The Konami VRC5 ASIC intercepts this process: Whenever the NES PPU fetches a background nametable byte from CIRAM, the VRC5 fetches an additional byte from QTRAM that indicates whether the next CHR pattern data that will be fetched should come from CHR-ROM or CHR-RAM and from which 4 KiB bank. 

Games will first write a tile number to CIRAM with $DA00 bit 0 clear, then write a bank byte to QTRAM by writing to the same PPU address with $DA00 bit 0 set. For normal game graphics, the tile number will point to a normal graphical pattern that was previously written to CHR-RAM, with the QTRAM bank byte only determining whether to use the left ($0xxx) or right ($1xxx) pattern table; PPU A12 is ignored during background CHR pattern fetches. This allows using 512 tiles for background graphics. 

To display 16x16 Kanji characters from CHR-ROM, the game first writes the character's double-byte 7-bit JIS X 0208 code to $DC00/$DD00 as well as the attribute and position of the current 8x8 tile within the 16x16 glyph. It then reads the correct CIRAM tile number and QTRAM bank byte from $DC00 and $DD00, which are then written to CIRAM and QTRAM using the same $DA00 bit 0 procedure. 

Format of the QTRAM bank byte: 
    
    
    D~7654 3210
      ---------
      RCBB BBBb
      ||++-++++- CHR A12..A17 if C=1, only b used if C=0
      |+-------- Chip select: 0=CHR-RAM, 1=CHR-ROM
      +--------- If CHR-ROM is selected, 
                 0: Kanji use attributes 0 and 1
                 1: Kanji use attributes 2 and 3
    

Sprite CHR pattern fetches work normally, PPU A12 applies. 

# Character Translation Operation

The [JIS X 0208](https://en.wikipedia.org/wiki/JIS_X_0208 "wikipedia:JIS X 0208") code map is arranged into 94 rows ($21-$7E) with 94 columns per row ($21-$7E), with unassigned code points in-between, in particular rows $29-$30. Even without those unassigned code points, 94 columns per row means that the code map is not contiguous. 

The hardware translates double-byte 7-bit JIS X 0208 codes into a 4 KiB bank and a tile number into the Kanji ROM. While the actual conversion table, and presumably operations, used by the hardware are replicated by FCEUX according to source code comments, the following description explains the idea behind it and produces identical results for valid JIS code points. The translation involves two steps: remapping the non-contiguous JIS X 0208 code map to a continuous code map, and remapping duplicate and unused "pages" (each consisting of 256 code points). 

To form a contiguous code map, assume 

  * 96 columns per row, divided into three column-thirds consisting of 32 columns each;
  * 96 rows, divided into six row-thirds consisting of 16 rows each.



This 9,216-element contiguous code map is built in the order 32 columns within a column-third, 16 rows within a row-third, three column thirds, six row-thirds: 
    
    
    // "row" and "col" are the first and second 7-bit JIS X 0208 code byte, respectively, each minus the $21 offset.
    code = (col %32)		// First, go through 32 columns of a column-third.
          +(row %16)*32 		// Then, through 16 rows of a row-third.
          +(col /32)*32*16		// Then, through three column-thirds.
          +(row /16)*32*16*3 	// Finally, through six row-thirds.
    ;
    

The actual Kanji ROM does not contain every character; in particular, it only contains the [Level 1](https://en.wikipedia.org/wiki/JIS_X_0208#Level_partitioning "wikipedia:JIS X 0208") kanji (up to row 47), and skips the empty rows $29-$30. Therefore, a second step of the translation requires remapping from the full contiguous 9,216-element table to a 4,096-element table. Given the contiguous code map created previously, this remapping can be done on a page (256-code) level: 
    
    
    static const uint8_t pageTable[0x24] ={
    	0x0,0x0,0x2,0x2,0x1,0x1,0x4,0x5,0x6,0x7,0x8,0x9,0xA,0xB,0xC,0xD,0xE,0xF, // JIS X 0208 rows $20-$4F. $20 is not a valid row number.
    	0x0,0x1,0x2,0x3,0x4,0x5,0x6,0x7,0x8,0x9,0xA,0xB,0xC,0xD,0xE,0xF,0xD,0xD  // JIS X 0208 rows $50-$7F. $7F is not a valid row number.
    };
    glyph =(code &0xFF) | (pageTable[code >>8] <<8);
    

Since the Kanji ROM contains 16x16 glyphs, there are four tiles per gylph: 
    
    
    tile =glyph *4;
    ciramByte =(tile &0xFF) | (reg[0xDB00] &3);
    qtramByte =(tile >>8);
    
    // Indicate CHR-ROM
    qtramByte |=0x40;
    
    // Indicate alternate attribute
    if (reg[0xDB00] &0x04) qtramByte |=0x80;
    

# Kanji ROM layout

The Kanji ROM contains 1 bpp data with one 16-bit word per scanline and 16 such words per character. It is connected to the PPU bus in such a way as to _appear_ as 2 bpp data to the PPU, with one glyph consisting of four 8x8 tiles in the order top-left, top-right, bottom-left, bottom-right. The second bitplane, selected by the PPU when PPU A3=1, is created by the VRC5 as either all 0s or all 1s, depending on bit 7 of the QTRAM byte: 
    
    
    PPU A0 (Tile row D0) -> CHR-ROM A1
    PPU A1 (Tile row D1) -> CHR-ROM A2
    PPU A2 (Tile row D2) -> CHR-ROM A3
    PPU A3 (bit plane) -> CHR-ROM /CE (if PPU A3=1, output all 0s or 1s, depending on QTRAM byte bit)
    PPU A4 (Tile number D0) -> CHR-ROM A0
    PPU A5 (Tile number D1) -> CHR-ROM A4
    PPU A6 (Tile number D2) -> CHR-ROM A5
    PPU A7 (Tile number D3) -> CHR-ROM A6
    PPU A8 (Tile number D4) -> CHR-ROM A7
    PPU A9 (Tile number D5) -> CHR-ROM A8
    PPU A10 (Tile number D6) -> CHR-ROM A9
    PPU A11 (Tile number D7) -> CHR-ROM A10
    VRC5 CHR-ROM A11, pin 47 (bank number D0) -> CHR-ROM A11
    VRC5 CHR-ROM A12, pin 48 (bank number D1) -> CHR-ROM A12
    VRC5 CHR-ROM A13, pin 49 (bank number D2) -> CHR-ROM A13
    VRC5 CHR-ROM A14, pin 50 (bank number D3) -> CHR-ROM A14
    VRC5 CHR-ROM A15, pin 51 (bank number D4) -> CHR-ROM A15
    VRC5 CHR-ROM A16, pin 53 (bank number D5) -> CHR-ROM A16
    

To address the 128 KiB CHR-ROM, representing actual Kanji ROM content, using offsets into 256 KiB CHR-ROM data as it appears to the PPU, the following formula may be used: 
    
    
    addr128K =((addr256K &0x00007) <<1) | ((addr256K &0x00010) >>4) | ((addr256K &0x3FFE0) >>1)
    

# See also

  * [FCEUX emulation source code](https://github.com/TASVideos/fceux/blob/master/src/boards/vrc5.cpp)
  * [Video demonstrating the system](https://www.youtube.com/watch?v=1rnf13b3dG4)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [Mappers with CHR ROM and CHR RAM](Category_Mappers_with_CHR_ROM_and_CHR_RAM.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
