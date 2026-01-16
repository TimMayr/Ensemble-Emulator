# NES 2.0 Mapper 539

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_539) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_539)

NES 2.0 Mapper 539 denotes a bootleg cartridge conversion of the original FDS version of _Kid Icarus_ (パルテナの鏡). 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Bank Select ($A000, write)
    * 2.2 Nametable Mirroring Select ($FFFF, write)
  * 3 Notes



# Banks

  * CPU $6000-$7FFF: 8 KiB fixed PRG-ROM bank $D
  * CPU $8000-$9FFF: 8 KiB fixed PRG-ROM bank $C
  * CPU $A000-$BFFF: 8 KiB switchable PRG-ROM bank
  * CPU $C000-$DFFF: 8 KiB fixed PRG-ROM bank $E
  * CPU $E000-$FFFF: 8 KiB fixed PRG-ROM bank $F
  * PPU $0000-$1FFF: 8 KiB of unbanked CHR-RAM



Certain ranges in the CPU address space are overlaid with portions of 8 KiB of PRG-RAM as follows: 

  * CPU $6000-$60FF
  * CPU $6200-$62FF
  * CPU $6400-$65FF
  * CPU $8200-$82FF
  * CPU $C000-$D1FF
  * CPU $DF00-$DFFF



# Registers

## PRG Bank Select ($A000, write)
    
    
    Mask: unknown
    
    D~7654 3210
      ---------
      .... BBBB
           ++++- Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF
    

## Nametable Mirroring Select ($FFFF, write)
    
    
    Mask: unknown
    
    D~7654 3210
      ---------
      .... M...
           +---- Select nametable mirroring type
                  0: Vertical
                  1: Horizontal
    

# Notes

  * Save data is not retained.
  * The conversion retains all writes to the FDS sound channel registers. An emulator could provide the expansion sound channel even though the original cartridge did not.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
