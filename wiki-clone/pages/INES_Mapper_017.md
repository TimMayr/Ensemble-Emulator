# INES Mapper 017

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_017) | View [other pages](Special_AllPages.xhtml#INES_Mapper_017)

**iNES Mapper 017** denotes ROM images that have been extracted from disk images for the _Front Fareast Super Magic Card_ [RAM cartridge](RAM_cartridge.xhtml "RAM cartridge"). They represent games whose [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Front_Fareast_Super_Magic_Card_disks "Game Doctor/Magic Card FDS Format") denotes a Super Magic Card disk (byte $0 bit 7 set, byte $7=$AA). Refer to the [Super Magic Card](Super_Magic_Card.xhtml "Super Magic Card") article for details on bankswitching. The Super Magic Card's registers are initialized to: 
    
    
    ; Play mode, WRAM bank 0, 1 KiB CHR mode enabled
    [$4500](Super_Magic_Card.xhtml#Super_Magic_Card_mode_.28.244500.2C_write-only.29 "Super Magic Card") = $47 
    
    ; PRG memory write-protected, two-screen mirroring
    [$42FF](Super_Magic_Card.xhtml#1M_banking_mode_.28.2442FC-.2442FF.2C_write-only.29 "Super Magic Card") = $20 | (verticalMirroring? 0x00: 0x10)
    
    ; 4M banking mode enabled
    [$43FC](Super_Magic_Card.xhtml#2M.2F4M_PRG_banking_mode_.28.2443FC-.2443FF.2C_write-only.29 "Super Magic Card") = $00
    
    ; Initial PRG register content
    $4504 = Number of 8 KiB PRG banks -4
    $4505 = Number of 8 KiB PRG banks -3
    $4506 = Number of 8 KiB PRG banks -2
    $4507 = Number of 8 KiB PRG banks -1 (originally [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Front_Fareast_Super_Magic_Card_disks "Game Doctor/Magic Card FDS Format") byte $5)
    

The [iNES](INES.xhtml "INES") header may specify a [512-byte trainer](INES.xhtml#Trainer "INES") (corresponding to [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Front_Fareast_Super_Magic_Card_disks "Game Doctor/Magic Card FDS Format")'s byte $0 bit 6 being set). The trainer must be loaded to an address originally denoted by the [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Front_Fareast_Super_Magic_Card_disks "Game Doctor/Magic Card FDS Format")'s byte $2, and is here denoted by the NES 2.0 submapper. In its presence, instead of jumping to the game's reset vector on a hard reset, trainer offset +$000 must be JMPed to. 
    
    
    Submapper    Trainer load address
     0           $7000
     1           $5D00
     2           $5E00
     3           $5F00
    

Battery-saving of WRAM content is not supported by any Magic Card model. Hard-resetting a game while restoring previously-saved WRAM content in emulators interferes with the correct operation of the trainer's program. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml)
