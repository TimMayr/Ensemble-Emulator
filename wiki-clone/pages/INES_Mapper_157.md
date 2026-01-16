# INES Mapper 157

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_157) | View [other pages](Special_AllPages.xhtml#INES_Mapper_157)

iNES Mapper 157 is used for games using the _Datach Joint ROM system_. This belongs in the [Bandai FCG](Bandai_FCG_board.xhtml "Bandai FCG") family of mappers. The Datach main unit is a device for insertion into the Famicom's cartridge port that provides a barcode scanner, and that accepts its own custom game cartridges. It contains an **internal** 256-byte serial EEPROM (X24C02) that is shared among all Datach games. Graphics are stored in 8 KiB of unbanked CHR-RAM instead of bankswitched CHR-ROM. 

One game, _Battle Rush: Build up Robot Tournament_ , has an additional **external** 128-byte serial EEPROM (X24C01) on the game cartridge. As part of that game's functionality is to transfer save data between the Datach main unit's EEPROM and the game cartridge's EEPROM, emulators should save each EEPROM's data to separate files, one for the main unit's EEPROM, and one for the game cartridge's EEPROM. 

The NES 2.0 header's PRG-NVRAM field will only denote whether the game cartridge has an additional 128-byte serial EEPROM (byte 10 value 0x10) or not (byte 10 value 0x00). The Datach main unit's serial EEPROM is not included in the NES 2.0 header's PRG-NVRAM field, as that EEPROM is not part of the game cartridge, but instead shared among all games using this mapper. 

## Contents

  * 1 Game List
  * 2 Banks
  * 3 Registers
    * 3.1 Read Serial EEPROM/Barcode ($6000-$7FFF read)
    * 3.2 External EEPROM Clock ($8000-$8003 write)
    * 3.3 PRG-ROM Bank Select ($8008 write)
    * 3.4 Nametable Mirroring Type Select ($8009 write)
    * 3.5 IRQ Control ($800A write)
    * 3.6 IRQ Latch ($800B-$800C write)
    * 3.7 EEPROM Control ($800D write)



# Game List

Name | External EEPROM | [NES 2.0 Byte 10](NES_2_0.xhtml#Byte_10_.28RAM_size.29 "NES 2.0")  
---|---|---  
_Battle Rush: Build up Robot Tournament_ | X24C01 | 0x10   
_Crayon Shin-chan: Ora to Poi Poi_ | - | 0x00   
_Dragon Ball Z: Gekitou Tenkaichi Budoukai_ | - | 0x00   
_J-League Super Top Players_ | - | 0x00   
_SD Gundam Wars_ | - | 0x00   
_Ultraman Club: Spokon Fight!!_ | - | 0x00   
_Yuu Yuu Hakusho - Bakutou Ankoku Bujutsu-kai_ | - | 0x00   
  
# Banks

  * CPU $8000-$BFFF: 16 KiB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG ROM bank, fixed to the last bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Registers

## Read Serial EEPROM/Barcode ($6000-$7FFF read)
    
    
    Mask: $E000
    
    7  bit  0
    ---- ----
    xxxE Bxxx
    |||| ||||
    +++|-|+++- Open bus
       | +---- Serial data out from Barcode scanner
       +------ Data out from I²C EEPROMs
    

  * If there is both an internal and an external EEPROM, the Data out from both EEPROMs is ANDed (open drain circuit).



The barcode scanner just returns a sequence of light and dark periods from a one-dimesional barcode as the user moves it past the sensor. The game only expects [UPCs](https://en.wikipedia.org/wiki/Universal_Product_Code "wikipedia:Universal Product Code") to be scanned, so an emulator may wish to provide an interface to convert a series of digits into a UPC, similar to [FCEUX](https://github.com/TASVideos/fceux/blob/master/src/boards/bandai.cpp#L303). 

## External EEPROM Clock ($8000-$8003 write)
    
    
    Mask: $800F
    
    7  bit  0
    ---- ----
    xxxx Cxxx
         |   
         +----- External EEPROM's I²C SCL
    

  * CHR A13 OUT is provided on the subcartridge header, and is used in _Battle Rush_ as an external I²C clock.
  * PPU rendering must either be disabled during traffic to the external EEPROM, or the same value must be written to all four registers.
  * Because the ASIC's PA12 and PA13 inputs are grounded, only registers $8000-$8003 instead of $8000-$8007 are relevant.
  * No CHR banking is available.



## PRG-ROM Bank Select ($8008 write)

## Nametable Mirroring Type Select ($8009 write)

## IRQ Control ($800A write)

## IRQ Latch ($800B-$800C write)

These four registers function the same way as on [INES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016"), submapper 5. 

## EEPROM Control ($800D write)
    
    
    Mask: $800F
    
    7  bit  0
    ---- ----
    RDC. ....
    |||
    ||+-------- Internal EEPROM's I²C SCL
    |+--------- Both EEPROM's I²C SDA
    +---------- Both EEPROM's Direction bit (1=Enable Read)
    

  * Please refer to generic I²C tutorials and the X24C01/24C02 datasheets on how to operate or emulate this register correctly.
  * The device address of the internal X24C02 EEPROM is set to 000b (with the highest "device type" bits being 1010b), whereas the external X24C01 EEPROM responds to _any_ I²C start condition. It is for this reason that there are two clock lines to address each EEPROM separately.
  * Note that while the 24C02 serializes data bits from MSB to LSB (i.e. first bit 7 of any byte, then 6..0), Bandai persisted in believing that the X24C01 serializes data bits from LSB to MSB instead. The X24C01 is actually still a big-endian device, and multi-byte reads and writes increment the address as MSB-first.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
