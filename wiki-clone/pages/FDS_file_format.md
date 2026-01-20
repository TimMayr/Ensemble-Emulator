# FDS file format

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FDS_file_format) | View [other pages](Special_AllPages.xhtml#FDS_file_format)

**fwNES** was an [NES emulator](Emulators.xhtml "NES emulator") developed by Fan Wan Yang and Shu Kondo. Its most lasting contribution to the NES scene was its disk image file format, an image of the Quick Disk media. 

## fwNES FDS file format

The FDS format (filename suffix `.fds`) is a way to store [Famicom Disk System](Family_Computer_Disk_System.xhtml "Family Computer Disk System") disk data. It consists of the following sections, in order: 

  1. Header (16 bytes), **sometimes omitted**
  2. Disk data (65500 * _x_ bytes)



The format of the header is as follows: 

Bytes | Description   
---|---  
0-3 | Constant $46 $44 $53 $1A ("FDS" followed by MS-DOS end-of-file)   
4 | Number of disk sides   
5-15 | Zero filled padding   
  
Some .FDS images may **omit the header**. 

### Disk data

The disk data follows the [FDS disk format](FDS_disk_format.xhtml "FDS disk format"), but gaps and CRCs are not included in the .FDS image. 

Most games are an even number of sides. Ports from NROM were one side. No commercial FDS game had an odd number of sides greater than 1. Disk sides comes in the following order: 

  * Disk 1 Side A
  * Disk 1 Side B
  * Disk 2 Side A
  * Disk 2 Side B
  * etc...



After the last file block, fill a side with all 0 so that the disk side reaches exactly 65500 bytes. 

Categories: [File formats](Category_File_formats.xhtml)
