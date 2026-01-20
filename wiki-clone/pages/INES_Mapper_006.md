# INES Mapper 006

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_006) | View [other pages](Special_AllPages.xhtml#INES_Mapper_006)

**iNES Mapper 006** denotes ROM images that have been extracted from disk images for the _Front Fareast Magic Card 1M_ or _2M_ [RAM cartridges](RAM_cartridge.xhtml "RAM cartridge"). They represent games whose [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Front_Fareast_Magic_Card_1M.2F2M.2F4M_disks "Game Doctor/Magic Card FDS Format") denotes a Magic Card 1M/2M disk (byte $0 bit 7 set, bits 4 and 5 clear, byte $7=$00). Refer to the [Super Magic Card](Super_Magic_Card.xhtml "Super Magic Card") article for details on bankswitching. The Super Magic Card's registers are initialized to: 
    
    
    ; Play mode, WRAM bank 0, 1 KiB CHR mode disabled
    [$4500](Super_Magic_Card.xhtml#Super_Magic_Card_mode_.28.244500.2C_write-only.29 "Super Magic Card") = $42 
    
    ; PRG memory write-protected, two-screen mirroring
    [$42FF](Super_Magic_Card.xhtml#1M_banking_mode_.28.2442FC-.2442FF.2C_write-only.29 "Super Magic Card") = (submapper <<5) | (verticalMirroring? 0x00: 0x10)
    
    ; 2M/4M banking mode disabled
    [$43FF](Super_Magic_Card.xhtml#2M.2F4M_PRG_banking_mode_.28.2443FC-.2443FF.2C_write-only.29 "Super Magic Card") = $00 
    

The [NES 2.0](NES_2_0.xhtml "NES 2.0") Submapper field denotes the initial [latch-based banking mode](Super_Magic_Card.xhtml#Latch-based_modes "Super Magic Card") (0-7). [NES 1.0](INES.xhtml "INES") files correspond to submapper 1. **iNES[Mapper 8](INES_Mapper_006.xhtml "INES Mapper 008")** is a synonym of Mapper 6 submapper 4. 

The [iNES](INES.xhtml "INES") header may specify a [512-byte trainer](INES.xhtml#Trainer "INES") (corresponding to [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Front_Fareast_Magic_Card_1M.2F2M.2F4M_disks "Game Doctor/Magic Card FDS Format")'s byte $0 bit 6 being set), which must be loaded to $7000-$71FF, be writable, and (on a hard reset) initialized by JSRing to $7003 before JMPing to the game's reset vector. 

Battery-saving of WRAM content is not supported by any Magic Card model. Hard-resetting a game while restoring previously-saved WRAM content in emulators interferes with the correct operation of the trainer's program. 

# See also

  * [Mapper 6](https://nesdev.org/mapper6.txt) Info on the FFE mapper. By FanWen Yang (outdated).
  * [Info on various Famicom "copiers"](http://www.famicomdisksystem.com/game-doctor-copiers/)



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
