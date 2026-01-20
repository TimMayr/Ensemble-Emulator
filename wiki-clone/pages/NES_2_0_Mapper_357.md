# NES 2.0 Mapper 357

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_357) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_357)

**NES 2.0 Mapper 357** is used for a _4-in-1_ multicart (cartridge ID _4602_) from Bit Corp. The first game is Bit Corp's hack of the [YUNG-08](NES_2_0_Mapper_368.xhtml "NES 2.0 Mapper 368") conversion of _Super Mario Brothers 2 (J)_ named _Mr. Mary 2_ , the other three games are [UNROM](UxROM.xhtml "UNROM") games. 

Unlike most multicarts, there is no menu; instead, the desired game is selected via two DIP Switches that protrude through the cartridge shell. The DIP switches directly select the outer 128 KiB PRG-ROM bank, game-specific nametable mirroring and mapper mode --- outer bank 0 selects SMB2J mode, all others select [UNROM](UxROM.xhtml "UNROM") mode. All four games use 8 KiB of unbanked CHR-RAM. Mirroring is Vertical for banks 0-2 and Horizontal for bank 3. 

## Contents

  * 1 Banks in SMB2J mode
  * 2 Registers in SMB2J mode
    * 2.1 PRG Bank Select 1 ($4022, write)
    * 2.2 PRG Bank Select 2 ($4120, write)
    * 2.3 IRQ Control ($4122)
  * 3 See also



# Banks in SMB2J mode

  * CPU $5000-$5FFF: 4 KiB PRG-ROM bank, fixed to #16
  * CPU $6000-$7FFF: 8 KiB PRG-ROM bank, switchable to #2 or #0
  * CPU $8000-$9FFF: 8 KiB PRG-ROM bank, fixed to #1
  * CPU $A000-$BFFF: 8 KiB PRG-ROM bank, fixed to #0
  * CPU $C000-$DFFF: 8 KiB PRG-ROM bank, switchable
  * CPU $E000-$FFFF: 8 KiB PRG-ROM bank, switchable to #10 or #8
  * PPU $0000-$1FFF: unbanked 8 KiB CHR-RAM



# Registers in SMB2J mode

## PRG Bank Select 1 ($4022, write)
    
    
    Mask: probably $71FF
    
    Bit 7654 3210
        ---------
        .... .CCC
              +++- Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF.
    

The actual bank number is: 
    
    
    Value  Bank#
    ------------
    0      4
    1      3
    2      5
    3      3
    4      6
    5      3
    6      7
    7      3
    

## PRG Bank Select 2 ($4120, write)
    
    
    Mask: probably $71FF
    
    Bit 7654 3210
        ---------
        .... ...I
                +- 0: CPU $6000-$7FFF to bank 2,
                      CPU $E000-$FFFF to bank 10
                   1: CPU $6000-$7FFF to bank 0,
                      CPU $E000-$FFFF to bank 8
    

## IRQ Control ($4122)
    
    
    Mask: probably $F1FF
    
    Bit 7654 3210
        ---------
        .... ...I
                +- 0: Acknowledge and disable IRQ, reset counter
                   1: Enable IRQ
    

When enabled, the 12-bit IRQ counter increases on every M2 cycle until it overflows, upon which an IRQ is fired. 

# See also

[PCB image analysis](https://forums.nesdev.org/viewtopic.php?f=9&t=17399)

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
