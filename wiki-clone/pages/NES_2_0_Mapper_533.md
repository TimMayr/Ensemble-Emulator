# NES 2.0 Mapper 533

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_533) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_533)

NES 2.0 Mapper 533 is used for the **Sachen 3014** board, used for the game _動動腦 II: 國中英文(一)_ (Dòngdòngnǎo II: Guózhōng Yīngwén (I), also known as _Middle School English II_ , SA-003). It's a [CNROM](CNROM.xhtml "CNROM")-like board with the added ability to read back the latch content for protection purposes. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 CHR Bank Select and Latch Write ($8000-$FFFF, write)
    * 2.2 Latch Read ($E000-$EFFF, Read)
  * 3 Notes
  * 4 See also



# Banks

  * CPU $8000-$FFFF: Fixed 32 KiB PRG-ROM bank
  * PPU $0000-$1FFF: Switchable 8 KiB CHR-ROM bank
  * Mirroring: hard-wired



# Registers

## CHR Bank Select and Latch Write ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      LLLl ....
      ++++------ Set latched value that can be read back at $E000-$EFFF
         +------ Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF
    

## Latch Read ($E000-$EFFF, Read)
    
    
    Mask: $F000
    
    D~7654 3210
      ---------
      .... LLLL
      |||| ++++- Latched value previously written to D4-D7 of $8000-$FFFF
      ++++------ value from ROM
    

# Notes

  * The board is subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict") when writing or reading. The software reads the latch ten times, assuming that after enough tries the driver will overpower the ROM.



# See also

  * [forum thread with pictures of PCB](https://forums.nesdev.org/viewtopic.php?t=17817)



Categories: [CNROM-like mappers](Category_CNROM_like_mappers.xhtml)
