# NES 2.0 Mapper 312

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_312) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_312)

NES 2.0 Mapper 312 is used for Kaiser's bootleg version of _Highway Star_. Its UNIF board name is **UNL-KS7013B**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Select ($6000-$7FFF)
    * 2.2 Nametable Mirroring Type Select ($8000-$FFFF)
    * 2.3 See also



# Banks

  * CPU $8000-$BFFF: Switchable 16 KiB PRG-ROM bank
  * CPU $C000-$FFFF: Fixed 16 KiB PRG-ROM bank #7
  * PPU $0000-$1FFF: Unbanked 8 KiB of CHR-RAM



# Registers

## PRG-ROM Select ($6000-$7FFF)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      .... .PPP
            +++- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF
    

## Nametable Mirroring Type Select ($8000-$FFFF)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      .... ...M
              +- Select nametable mirroring type
                  0: Vertical
                  1: Horizontal
    

## See also

  * [PCB photos](https://forums.nesdev.org/viewtopic.php?p=256930#p256930)
  * [Krzysiobal's reverse engineering from above photos](https://forums.nesdev.org/viewtopic.php?p=256961#p256961)


