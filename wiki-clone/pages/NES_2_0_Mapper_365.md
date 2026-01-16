# NES 2.0 Mapper 365

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_365) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_365)

NES 2.0 Mapper 365 is used for the Polish version, and probably other language versions, of the Asder PC-95 educational computer. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Bank Select ($8000-$9FFF, write)
    * 2.2 CHR-RAM Bank Select ($A000-$BFFF, write)
    * 2.3 Unknown Registers ($C000-$DFFF/$E000-$FFFF, write)
    * 2.4 Unknown Register ($4903, read)
    * 2.5 Keyboard Row Select ($4904, write)
    * 2.6 Unknown register ($4905, write)
    * 2.7 Keyboard Button State ($4906, read)
    * 2.8 Unknown Registers ($4910-$491F, read/write)
    * 2.9 Unknown Registers ($4111, write; $5000-5003/$5080-$5083, read)
    * 2.10 Printer Port Status ($4902, read)
    * 2.11 Printer Port Output ($4900/$4901, write)
  * 3 Note



# Banks

  * CPU $6000-$7FFF: 8 KiB fixed PRG-RAM bank
  * CPU $8000-$9FFF: 8 KiB switchable PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG-ROM bank
  * CPU $C000-$DFFF: 8 KiB switchable PRG-ROM bank
  * CPU $E000-$FFFF: 8 KiB switchable PRG-ROM bank
  * PPU $0000-$03FF: 1 KiB switchable CHR-RAM bank
  * PPU $0400-$07FF: 1 KiB switchable CHR-RAM bank
  * PPU $0800-$0BFF: 1 KiB switchable CHR-RAM bank
  * PPU $0C00-$0FFF: 1 KiB switchable CHR-RAM bank
  * PPU $1000-$13FF: 1 KiB switchable CHR-RAM bank
  * PPU $1400-$17FF: 1 KiB switchable CHR-RAM bank
  * PPU $1800-$1BFF: 1 KiB switchable CHR-RAM bank
  * PPU $1C00-$1FFF: 1 KiB switchable CHR-RAM bank



# Registers

## PRG-ROM Bank Select ($8000-$9FFF, write)
    
    
    Mask: $E003
    
    $8000: Select 8 KiB PRG-ROM bank at CPU $8000-$9FFF
    $8001: Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF
    $8002: Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF
    $8003: Select 8 KiB PRG-ROM bank at CPU $E000-$FFFF. Written value is ORed with $01.
    

## CHR-RAM Bank Select ($A000-$BFFF, write)
    
    
    Mask: $E007
    
    $A000: Select 1 KiB CHR-RAM bank at PPU $0000-$03FF
    $A001: Select 1 KiB CHR-RAM bank at PPU $0400-$07FF
    $A002: Select 1 KiB CHR-RAM bank at PPU $0800-$0BFF
    $A003: Select 1 KiB CHR-RAM bank at PPU $0C00-$0FFF
    $A004: Select 1 KiB CHR-RAM bank at PPU $1000-$13FF
    $A005: Select 1 KiB CHR-RAM bank at PPU $1400-$17FF
    $A006: Select 1 KiB CHR-RAM bank at PPU $1800-$1BFF
    $A007: Select 1 KiB CHR-RAM bank at PPU $1C00-$1FFF
    

## Unknown Registers ($C000-$DFFF/$E000-$FFFF, write)

Mask: $E001 

One of them will select the nametable mirroring, but it is not known which one, and what the other register does. 

## Unknown Register ($4903, read)
    
    
    Mask: unknown
    
    D~7654 3210
      ---------
      .... .S..
            +--- Status bit of unknown meaning
    

$4903 seems to indicate the presence of a device of some kind and is only read when the computer's menu, or some of its individual applications, boot. 

## Keyboard Row Select ($4904, write)

Mask: unknown 

Selects the row of the keyboard from which the button state will be returned when reading from $4906. 

## Unknown register ($4905, write)

Mask: unknown 
    
    
    D~7654 3210
      ---------
      ?... ..??
    

This register is written to by some of of the applications at the end of their NMI handler. 

## Keyboard Button State ($4906, read)

Mask: unknown 

Returns the button states of the keyboard row selected via $4904. The mapping of keys to bits and rows matches that of the Subor keyboard. 

## Unknown Registers ($4910-$491F, read/write)

## Unknown Registers ($4111, write; $5000-5003/$5080-$5083, read)

These registers are accessed when selecting "save" and "load" from the text editor ("Edytor tekstu"). 

## Printer Port Status ($4902, read)
    
    
    Mask: unknown
    
    D~7654 3210
      ---------
      B... ....
      +--------- 0=Ready, 1=Busy
    

## Printer Port Output ($4900/$4901, write)

Mask: unknown 

The text editor writes every byte to be printed both to $4900 and $4901. 

# Note

  * The Arabic version of the Asder PC-95 Educational Computer most likely originally used this mapper as well. The publicly-available ROM image has obviously been hacked for [iNES Mapper 241](INES_Mapper_241.xhtml "INES Mapper 241") and crashes at several points.
  * The Korean version uses different hardware and adds a speech chip, which are described by [NES 2.0 Mapper 531](NES_2_0_Mapper_531.xhtml "NES 2.0 Mapper 531").


