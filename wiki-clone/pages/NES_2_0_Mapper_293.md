# NES 2.0 Mapper 293

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_293) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_293)

NES 2.0 Mapper 293 is used for the _NewStar 12-in-1_ and _76-in-1_ multicarts. Both use 8 KiB of unbanked CHR-RAM rather than CHR-ROM. 

## Contents

  * 1 Registers
    * 1.1 First Banking Register ($8000-$9FFF, $C000-$DFFF)
    * 1.2 Second Banking Register ($8000-$BFFF)
  * 2 PRG banking modes
  * 3 Notes



# Registers

## First Banking Register ($8000-$9FFF, $C000-$DFFF)
    
    
    Mask: $A000
    
    7654 3210
    ---------
    .... PIII
         |+++- Select inner 16 KiB PRG-ROM bank
         +---- Bit 1 of the PRG banking mode number
    

## Second Banking Register ($8000-$BFFF)
    
    
    Mask: $C000
    
    7654 3210
    ---------
    MP10 ...2
    ||++----+- Select outer 128 KiB PRG-ROM bank (note the bit distribution)
    |+-------- Bit 0 of the PRG banking mode number
    +--------- Select nametable mirroring type
                0: Vertical
                1: Horizontal
    

# PRG banking modes

  * 0 (UNROM)



    $8000-$BFFF: Switchable 16 KiB Inner PRG-ROM bank
    $C000-$FFFF: Fixed 16 KiB Inner PRG-ROM bank #7

  * 1



    $8000-$BFFF: Switchable 16 KiB Inner PRG-ROM bank AND $FE
    $C000-$FFFF: Fixed 16 KiB Inner PRG-ROM bank #7

  * 2 (NROM-128)



    $8000-$BFFF: Switchable 16 KiB Inner PRG-ROM bank
    $C000-$FFFF: Mirror of $8000-$BFFF

  * 3 (NROM-256)



    $8000-$FFFF: Switchable 32 KiB Inner PRG-ROM bank (bit 0 of inner bank number ignored)

# Notes

  * The address ranges of the two banking registers do indeed partially overlap.
  * Nestopia Plus! assigns this board to Mapper 281 instead.



Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
