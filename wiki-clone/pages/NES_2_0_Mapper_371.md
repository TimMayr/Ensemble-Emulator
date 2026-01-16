# NES 2.0 Mapper 371

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_371) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_371)

[![](../wiki-images/PEC-586_Spanish_PCB_F.jpg)](File_PEC_586_Spanish_PCB_F_jpg.xhtml)

[](File_PEC_586_Spanish_PCB_F_jpg.xhtml "Enlarge")

PCB front

[![](../wiki-images/PEC-586_Spanish_PCB_B.jpg)](File_PEC_586_Spanish_PCB_B_jpg.xhtml)

[](File_PEC_586_Spanish_PCB_B_jpg.xhtml "Enlarge")

PCB back (mirrored, pin 31 on the left)

NES 2.0 Mapper 371 denotes the Spanish PEC-586 home computer main cartridge, which differs from both the Chinese and Russian equivalents. It has two PRG-ROM chips, one with 64 KiB, one with 512 KiB, for a total of 576 KiB. The first chip contains normal home computer applications such as a BASIC interpreter and a text editor, while the second chip contains a selection of common NROM games. Like all PEC-586 machines and cartridges, it adds a 1bpp video mode by appropriately mapping CHR-RAM, and requires the [PEC-586 Keyboard](https://www.nesdev.org/w/index.php?title=PEC-586_Keyboard&action=edit&redlink=1 "PEC-586 Keyboard \(page does not exist\)") to operate. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Main Register ($5000, write)
    * 2.2 Auxiliary Register ($5100, write)
    * 2.3 Tape In ($5500, read)
  * 3 Protection
  * 4 1bpp CHR mode



# Banks

Common: 

  * CPU $6000-$7FFF: 8 KiB of unbanked PRG-RAM
  * PPU $0000-$1FFF: 8 KiB of unbanked CHR-RAM



When the first (64 KiB) PRG-ROM chip is selected 

  * CPU $8000-$BFFF: Switchable 16 KiB PRG-ROM bank (#0-#3)
  * CPU $C000-$FFFF: Fixed 16 KiB PRG-ROM bank #3



When the second (512 KiB) PRG-ROM chip is selected: 

  * CPU $8000-$BFFF: Switchable 16 KiB PRG-ROM bank
  * CPU $C000-$FFFF: Mirror of CPU $8000-$BFFF



# Registers

## Main Register ($5000, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      1CCC PPPP
      |||| ++++- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF (PRG A14..A17)
      |+++------ Select PRG-ROM source
      |           0: First chip (64 KiB)
      |           5: Second chip (512 KiB)
      |           7: Expansion slot
      +--------- Select 1bpp CHR mode
                  0: Disabled
                  1: Enabled
    

Power-on value: $00 

## Auxiliary Register ($5100, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... ..MP
             |+- If first PRG-ROM chip selected: Tape Out data
             |   If second PRG-ROM chip selected: PRG A18 (256 KiB)
             +-- Select nametable mirroring type
                  0: Horizontal
                  1: Vertical
    

Power-on value: $03 

## Tape In ($5500, read)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... .T..
            +--- Tape In data
    

# Protection

When the second PRG-ROM chip is selected _and_ PRG A18=0, the cartridge hardware will forcibly reset on attempts to read from CPU $D100-$D1FF. As the first 256 KiB bank of the second PRG-ROM chip only contains data for CHR-RAM as well as some code that is copied into and executed from PRG-RAM, and $C000-$FFFF is a mirror of $8000-$BFFF in that configuration, the $D100-$D1FF range is never read from during normal operation. 

# 1bpp CHR mode

When register $5000 bit 7 is set, CHR A3 is substituted with PPU A0 from the last nametable (not attribute table) read, and CHR A12 is substituted with PPU A9 from the last nametable (not attribute table) read. This creates a 1bpp all-points-addressable video mode that allows the 8 KiB CHR-RAM to hold just enough data for one screen. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
