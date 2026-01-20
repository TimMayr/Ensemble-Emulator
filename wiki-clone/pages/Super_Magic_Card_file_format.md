# Super Magic Card file format

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Super_Magic_Card_file_format) | View [other pages](Special_AllPages.xhtml#Super_Magic_Card_file_format)

The Front Fareast [Super Magic Card](Super_Magic_Card.xhtml "Super Magic Card") can load games and real-time save states from 3.5" MS-DOS FAT12 floppy disks in a custom file format that is also known just as the "FFE format" by fwNES and uCON64. Similar to [iNES](INES.xhtml "INES"), it contains a 512-byte header describing the game's mapper and size as well as trainer, PRG and CHR data. Unlike iNES, it only supports the mappers that the [Super Magic Card](Super_Magic_Card.xhtml "Super Magic Card") itself supports. 

Format of the 512-byte header: 
    
    
    +$0   8 bytes   [Doctor header](Game_Doctor_Magic_Card_FDS_Format.xhtml#Doctor_Header_file "Game Doctor/Magic Card FDS Format")
    +$8   2 bytes   Signature $AA $BB 
    +$A   1 byte    File type, 0=Game, 1=Realtime save state
    +$B   501 bytes Unused
    -----
    $200
    

In the case of "Game" files, the 512-byte header is followed by 

  * 512 bytes of trainer data if the [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Doctor_Header_file "Game Doctor/Magic Card FDS Format")'s byte $0 bit 6 was set;
  * PRG data, size determined by 
    * Doctor header byte $3 times 8192 bytes if Doctor header byte $7=$AA (i.e. a Super Magic Card game);
    * Doctor header byte $0 bit 5 clear=128 KiB, set=256 KiB bytes if Doctor header byte 0 bits 4 or 5 are set (i.e. a Magic Card 4M game);
    * Doctor header byte $1 bits 5-7 otherwise (i.e. a Magic Card 1M or 2M game):


    
    
    Byte $1 bits 5-7   PRG size
     0,4               128 KiB
     1-3               256 KiB
     5-7               32 KiB
    

  * CHR data, size determined by 
    * Doctor header byte $4 times 8192 bytes if Doctor header byte $7=$AA (i.e. a Super Magic Card game);
    * Doctor header byte $0 bit 4 clear=128 KiB, set=256 KiB bytes if Doctor header byte 0 bits 4 or 5 are set (i.e. a Magic Card 4M game);
    * Doctor header byte $1 bits 5-7 otherwise (i.e. a Magic Card 1M or 2M game):


    
    
    Byte $1 bits 5-7   CHR size
     0-3               0 KiB
     4-5               32 KiB
     6                 16 KiB
     7                 8 KiB
    

The SMC Game file format is supported (to varying degrees) by: 

  * the [Super Magic Card](Super_Magic_Card.xhtml "Super Magic Card")'s BIOS when using 3.5" MS-DOS FAT12 floppy disks;
  * Front Fareast's "VGS.EXE" utility to [remotely control the Super Magic Card from an MS-DOS PC](Super_Magic_Card.xhtml#Remote_Control "Super Magic Card");
  * [uCON64](https://ucon64.sourceforge.io/);
  * NintendulatorNRS;
  * fwNES.



Categories: [File formats](Category_File_formats.xhtml)
