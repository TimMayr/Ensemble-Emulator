# INES Mapper 045

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_045) | View [other pages](Special_AllPages.xhtml#INES_Mapper_045)

iNES Mapper 045 denotes multicart PCBs using the GA23C ASIC in its standard configuration. It is an [MMC3](MMC3.xhtml "MMC3") clone with four outer bank registers. 

## Contents

  * 1 Registers
    * 1.1 MMC3-compatible registers
    * 1.2 CHR-OR LSB ($6000 #0, write)
    * 1.3 PRG-OR LSB ($6000 #1, write)
    * 1.4 CHR-AND, CHR-OR/PRG-OR MSB ($6000 #2, write)
    * 1.5 PRG-AND, register lock ($6000 #3, write)
    * 1.6 Reset outer bank registers and release lock ($6001, write)
    * 1.7 Menu Selection #1 ($5000-$5FFF, read)
    * 1.8 Menu Selection #2
  * 2 Notes



# Registers

The four outer bank registers are accessed at address $6000. The first write goes to the first register, the second write to the second, and so on; the fifth write goes to the first register again. The outer bank registers overlay WRAM and function regardless of the MMC3's WRAM bits' setting. 

## MMC3-compatible registers

  * Bank select ($8000-$9FFE, even)
  * Bank data ($8001-$9FFF, odd)
  * Mirroring ($A000-$BFFE, even)
  * PRG RAM protect ($A001-$BFFF, odd)
  * IRQ latch ($C000-$DFFE, even)
  * IRQ reload ($C001-$DFFF, odd)
  * IRQ disable ($E000-$FFFE, even)
  * IRQ enable ($E001-$FFFF, odd)



These registers function identically to the [MMC3](MMC3.xhtml "MMC3"), with the PRG/CHR bank registers subject to masking by the outer bank registers. 

## CHR-OR LSB ($6000 #0, write)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      CCCC CCCC
      ++++-++++- Select CHR A10-A17, OR'd with MMC3's CHR A10-A17 masked according to CHR-AND
    

## PRG-OR LSB ($6000 #1, write)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      ppPP PPPP
      ||++-++++- Select PRG A13-A18, OR'd with MMC3's PRG A13-A18 masked according to PRG-AND
      ++-------- Select PRG A19-A20
    

## CHR-AND, CHR-OR/PRG-OR MSB ($6000 #2, write)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      PPCC cccc
      |||| ++++- Select number of CHR bits from MMC3
      ||||       ($F: 256 KiB, $E: 128 KiB ... $7-$0: 1 KiB)
      ||++------ Select CHR A18-A19
      ++-------- Select CHR A20-A21 and PRG A21-22
    

## PRG-AND, register lock ($6000 #3, write)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      1LPP PPPP
       |++ ++++- Select PRG-AND mask (inverted)
       |||       ($00: 512 KiB, $20: 256 KiB...)
       +-------- 1=Lock outer bank registers
    

## Reset outer bank registers and release lock ($6001, write)

Writing any value to $6001 (mask: $F001) resets the outer bank registers as a soft reset would, clearing the "Lock" bit and making the next write to $6000 go to register #0. 

## Menu Selection #1 ($5000-$5FFF, read)

The _超强年度新卡 (Super New Year Cart) 15-in-1_ multicart has a DIP switch whose setting causes the menu program to select one of eight different menus. 
    
    
    A~FEDC BA98 7654 3210  D~7654 3210
      -------------------    ---------
      0101 AAAA AAAA ....    .... ...D
           ++++-++++-----------------+- 1=If DIP switch is in the position selected by the corresponding A bit
    

This means that: 

  * If the dip switch is in the "0" position, reads from $5010 return with D0 set while reads from $5020, $5040...$5800 return with D0 clear.
  * If the dip switch is in the "1" position, reads from $5020 return with D0 set while reads from $5010, $5040...$5800 return with D0 clear.
  * If the dip switch is in the "7" position, reads from $5800 return with D0 set while reads from $5010, $5020...$5400 return with D0 clear.



## Menu Selection #2

Some multicarts select between five different menus by connecting one of the higher address lines to PRG /CE. The menu code selects between menus by checking which of the higher address lines disables PRG-ROM when set. 
    
    
    Selection 0: No address line connected to PRG /CE
    Selection 1: PRG A20 connected to PRG /CE
    Selection 2: CHR A20 connected to PRG /CE
    Selection 3: PRG A19 connected to PRG /CE
    Selection 4: CHR A19 connected to PRG /CE
    

Cartridges with 1 MiB of PRG-/CHR-ROM cannot use selections 3/4, cartridges with 2 MiB of more of PRG-/CHR-ROM cannot use this menu selection mechanism. 

# Notes

  * Some multicart menus write the same values to the outer bank registers multiple times. Experiments during dumping do not suggest that this is necessary.
  * NES 2.0 Mappers [356](NES_2_0_Mapper_356.xhtml "NES 2.0 Mapper 356"), [372](NES_2_0_Mapper_372.xhtml "NES 2.0 Mapper 372") and [373](NES_2_0_Mapper_373.xhtml "NES 2.0 Mapper 373") repurpose bits from outer bank register #2 for other things.
  * [NES 2.0 Mapper 269](NES_2_0_Mapper_269.xhtml "NES 2.0 Mapper 269") is an enhanced plug-and-play console variant of this mapper.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
