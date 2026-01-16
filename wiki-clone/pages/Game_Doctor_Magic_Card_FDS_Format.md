# Game Doctor/Magic Card FDS Format

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Game_Doctor/Magic_Card_FDS_Format) | View [other pages](Special_AllPages.xhtml#Game_Doctor_Magic_Card_FDS_Format)

The Front Fareast (Super) Magic Card, Bung (Super) Game Doctor/Game Master and Venus Game Doctor [RAM cartridges](RAM_cartridge.xhtml "RAM cartridge") load games from the same 2.8" floppy disk media as the [Famicom Disk System](Family_Computer_Disk_System.xhtml "Famicom Disk System"). The basic header/block structure is identical to the [FDS disk format](FDS_disk_format.xhtml "FDS disk format"), with the Doctor Header file having a special significance. 

## Contents

  * 1 Doctor Header file
    * 1.1 Front Fareast Magic Card 1M/2M/4M disks
    * 1.2 Front Fareast Super Magic Card disks
    * 1.3 Bung Super Game Doctor disks
    * 1.4 Venus Turbo Game Doctor disks
  * 2 File Header (block type 3)/File Data (block type 4)
  * 3 Hidden file
  * 4 Magic Card Trainer (block type 5)



## Doctor Header file

  * Address: CPU $43FF or $4FFF
  * Size: 8 bytes



Game Doctor/Magic Card disks are run by placing the device between the FDS RAM Adapter and the console. By the default, the Game Doctor/Magic Card merely passes through all signals, so that the normal FDS BIOS screen is shown. 

The FDS BIOS boots the disk and tries to store the first data byte of the Doctor Header file to $43FF or $4FFF. This operation switches the Game Doctor/Magic Card from Pass-through into Load mode, switching-in its replacement BIOS at $E000-$FFFF. Since the FDS BIOS' instruction that stores bytes read from disk is at offset $E53A, the entry point into the Game Doctor/Magic Card BIOS is offset $E53C. 

The Game Doctor BIOS will read and interpret the Doctor Header file and all subsequent blocks as well as prompt for the insertion of additional disks. All blocks must have valid CRC fields, and the gap length between blocks is like on any regular FDS disk. 

The Front Fareast Super Magic Card is different in that it displays its own GUI at power-on from which Magic Card disks may be run, but can be set to pass-through mode via a menu choice ("RUN IC CARD" or "RUN NTD 2.8"). 

### Front Fareast Magic Card 1M/2M/4M disks
    
    
    Offset Meaning
    $0     Flag byte        
           1TMM ....
           ||++------ 0: Not a Magic Card 4M game
           ||         1: Magic Card 4M game, 128+256 KiB data
           ||         2: Magic Card 4M game, 256+128 KiB data
           ||         3: Magic Card 4M game, 256+256 KiB data
           |+-------- 1: Trainer present
           |          0: No trainer present
           +--------- 1: Magic Card disk (if offset $7=0 as well)
                   
    $1     Mode byte
           This byte is written to register $42FF.
    
    $2-$6  Catalogue number
           Can be in ASCII or FDS BIOS tile numbers.
    
    $7     Must be $00 to indicate Magic Card 1M/2M/4M disk
    
    

  * A Magic Card disk is identified by having byte $0 bit 7 set, and byte $7=00.
  * The number of expected disk sides is: 
    * If byte $0 bit 4 or 5 is set, 6 sides if one of the two bits is set, or 8 sides if both are set.
    * Otherwise, it is derived from bits 5-7 of byte $1: 
      * 0: 2 sides (UNROM)
      * 1-3: 4 sides (UOROM)
      * 4: 3 sides (GNROM)
      * 5-7: 1 side (CNROM/NROM)
  * If byte $0 bit 6 is set, following all the files on the first side, there must be a block type $5 containing 512 bytes of trainer data to be loaded to $7000, and an RTS-returning Init routine at $7003 that will be called by BIOS prior to JMPing to the game's Reset handler.
  * No data loaded to RAM prior to encountering the Doctor Header file can be assumed to remain present.
  * "128+256 KiB data" on Magic Card 4M games means that the first 128 KiB of data are loaded to the beginning of the 512 KiB PRG-RAM, then there is a 128 KiB hole, followed by the remaining 256 KiB of data. The second 256 KiB will contain CHR pattern data that is copied from PRG-RAM to the 32 KiB of CHR-RAM as needed by the Trainer program.



### Front Fareast Super Magic Card disks
    
    
    Offset Meaning
    $0     Flag byte        
           1T.. ....
           |+-------- 1: Trainer present
           |          0: No trainer present
           +--------- 1: Super Magic Card disk (if offset $7=$AA as well)
    
    $1     Mode byte
           This byte is written to register $42FF.
    
    $2     Trainer address MSB 
    $3     Number of 8 KiB PRG banks 
    $4     Number of 8 KiB CHR banks
    $5     8 KiB PRG bank number to be initially mapped to $E000-$FFFF
    $6     Number of disk sides minus 1
    $7     Must be $AA to indicate Super Magic Card disk
    
    

  * A Super Magic Card disk is identified by having byte $0 bit 7 set, and byte $7=$AA.
  * If byte $0 bit 6 is set, a 512-byte trainer must have been loaded to CPU RAM at $0600 prior to encountering the Doctor Header file. BIOS will move it to the address specified at Doctor Header byte $2, and JMP to it _instead_ of JMPing to the game's Reset handler.



### Bung Super Game Doctor disks
    
    
    Offset Meaning
    $0     Flag byte        
           1T.. ....
           |+-------- 1: Trainer present
           |          0: No trainer present
           +--------- 1: Super Game Doctor disk (if offset $7 !=0 as well)
    
    $1     Mode byte
           This byte is written to register $42FF.
    
    $2-$6  Catalogue number
           Can be in ASCII or FDS BIOS tile numbers.
    
    $7     Number of disk sides minus 1.   
    
    

  * A Super Game Doctor disk is identified by having byte $0 bit 7 set, and byte $7 !=0 (and not $AA).
  * If byte $0 bit 6 is set, an RTS-returning Init routine at $7003 that will be called by BIOS prior to JMPing to the game's Reset handler. This trainer data must have been loaded to $7000 as a normal FDS file prior to encountering the Doctor Header file.
  * A game disk is furthermore free to load data to any other address between $6000-$7FFF, or even CPU RAM at $0000-$07FF, prior to encountering the Doctor Header file, and JMPing or JSRing to it from the normal game code.
  * A special value $FE of byte $0 is used by the Makko Copy Master. In its presence, BIOS keeps the Super Game Doctor in Load mode and just JMPs to $6000 after loading all files from the first disk side.



### Venus Turbo Game Doctor disks
    
    
    Offset Meaning
    $0     Flag byte        
           0TL. ....
           ||+------- 1: Special load hooks present
           |+-------- 1: Trainer present
           |          0: No trainer present
           +--------- 0: Turbo Game Doctor disk
    
    $1     Mode byte
           This byte is written to register $42FF.
    
    $2-$6  Catalogue number
           Can be in ASCII or FDS BIOS tile numbers.
     
    $7     Number of disk sides minus 1.
    
    

  * A Turbo Game Doctor disk is identified by having byte $0 bit 7 clear.
  * If byte $0 bit 6 is set, an RTS-returning Init routine at $7003 that will be called by BIOS prior to JMPing to the game's Reset handler. This trainer data must have been loaded to $7000 as a normal FDS file prior to encountering the Doctor Header file.
  * If byte $0 bit 5 is set, BIOS will JMP to certain RAM addresses during the load process. The code must have been loaded to $0400 as a normal FDS file prior to encountering the Doctor Header file. 
    * $0400 ... after reading block type 2 of sides B and later. It must JMP to $E569 to continue with BIOS.
    * $0403 ... after encountering Doctor Header file. It must JMP to $E55F to continue with BIOS.
    * $0406 ... before JMPing to ($FFFC). It must RTS or JMP to ($FFFC) by itself.
  * A game disk is furthermore free to load data to any other address between $6000-$7FFF, or even CPU RAM at $0000-$07FF, prior to encountering the Doctor Header file, and JMPing or JSRing to it from the normal game code.
  * A special value $FE of byte $0 is used by the Makko Copy Master. In its presence, BIOS keeps the Turbo Game Doctor in Load mode and just JMPs to $6000 after loading all files from the first disk side.



## File Header (block type 3)/File Data (block type 4)

As with normal FDS disks, file data is stored as pairs of File Header (block type 3) and File Data (block type 4) blocks. 

  * Magic Card and Super Game Doctor disks ignore the address and size fields of the File Header block, only paying attention to bit 0 of the file type (CPU/PPU) byte. CPU files are always assumed to have a size of 32768 bytes, and PPU bytes 8192, 16384 or 32768 bytes depending on byte $1 of the Doctor Header file, each being loaded sequentially into the Magic Card/Game Doctor's own RAM. The target address is then automatically incremented. The File Header block thereby containing potentially incorrect file sizes serves as a simple copy protection method.
  * Super Magic Card disks faithfully obey the address and size fields of the File Header block, and use the file ID (offset $1 in block type 3) as PRG A13-A18/CHR A10-A17.
  * Turbo Game Doctor disks faithfully obey the address and size fields of the File Header block, and use bits 1-7 of the file type byte as higher target address bits:


    
    
     File Header byte $E:
     .PPP P..0
      ||| |  +- Denote CPU address space
      +++-+---- PRG A18..A15
      
     ...C CC.1
        | || +- Denote PPU address space
        +-++--- CHR A17..A15
    

## Hidden file

The Bung Game Doctor 1M and the Venus Game Converter 1M expect a hidden file, meaning an additional block type 3/block type 4 pair, after the file count denoted in block type 2. Content, size and address do not matter. Later Game Doctors have no such requirement. 

## Magic Card Trainer (block type 5)

On Magic Card 1M/2M/4M games only, if the Doctor Header file's byte $0 has bit 6 set, a block type 5 must be follow the file blocks denoted in block type 2. The block type 5 byte is followed directly by 512 bytes of Trainer data to be stored at CPU $7000, followed by a standard CRC word. 

This method of storing Trainer data is unique to Magic Card 1M/2M/4M disks; Super and Turbo Game Doctor disks store Trainers as regular FDS files prior to the Doctor Header file. 
