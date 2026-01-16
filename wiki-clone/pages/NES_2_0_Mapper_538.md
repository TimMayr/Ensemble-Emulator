# NES 2.0 Mapper 538

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_538) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_538)

NES 2.0 Mapper 538 denotes the **60-1064-16L** PCB, used for a bootleg cartridge conversion named _Super Soccer Champion_ of the Konami FDS game _Exciting Soccer_. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Bank switch ($C000-$DFFF, write)
  * 3 Notes



# Banks

  * CPU $6000-$7FFF: 8 KiB switchable PRG-ROM bank
  * CPU $8000-$9FFF: 8 KiB switchable PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB PRG ROM bank, fixed to bank #13
  * CPU $C000-$DFFF: 8 KiB PRG ROM bank, fixed to bank #14
  * CPU $E000-$FFFF: 8 KiB PRG ROM bank, fixed to bank #15
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



Mirroring is hard-wired to Vertical. 

# Registers

## PRG-ROM Bank switch ($C000-$DFFF, write)

Mask: $E000 

The low nibble sets the PRG-ROM banks both for the $6000-$7FFF and $8000-$9FFF CPU address ranges according to the following table: 
    
    
    Value written    0 1 2 3 4 5 6 7 8 9 A B C D E F
    ------------------------------------------------
    $6000-$7FFF bank 1 1 3 3 5 5 7 7 9 9 B B D D F F
    $8000-$9FFF bank 0 A 2 A 4 A 6 A 8 8 A A C C E E
    

Or as table-less C code: 
    
    
    bank6 = value |1;
    bank8 = value &1 && ~value &8? 10: value &~1;
    

# Notes

  * Save data is not retained.
  * The conversion retains all writes to the FDS sound channel registers. An emulator could provide the expansion sound channel even though the original cartridge did not.
  * [PCB image and description](https://forums.nesdev.org/viewtopic.php?f=3&t=18723#p273223)



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
