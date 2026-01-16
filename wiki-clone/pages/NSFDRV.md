# NSFDRV

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NSFDRV) | View [other pages](Special_AllPages.xhtml#NSFDRV)

**NSFDRV** is an 8-byte header ID in the ROM of an [NSF](NSF.xhtml "NSF") file used to identify the sound driver used. 

## Contents

  * 1 Usage and applications
  * 2 File Format
  * 3 List of NSFDRV sound driver IDs
  * 4 Player implementations



## Usage and applications

  * NSF players can use this ID to identify old sound drivers developed with inaccurate emulation, and therefore patch the drivers accordingly.


  * Authors can assert that their NSF is original (i.e. NSF is not ripped from a ROM).


  * In sound driver bug reporting, the developer can use this ID to identify the driver version.



## File Format

The tag consists of 8 bytes on the actual program data of the NSF file. 

Since the program data follows immediately after the NSF header, these 8 bytes are defined as follows: 
    
    
    Offset          Bytes   Function
    -------------------------------------------------
    $0000 - $007F   128     NSF Header
    $0080 - $0085   6       Sound driver ID
    $0086           1       Major version number
    $0087           1       Minor version number
    

## List of NSFDRV sound driver IDs

  * OFGS (<http://offgao.no-ip.org/ofgs/>)


    
    
    ASCII  :      "OFGS  "
    Binary :      $4F $46 $47 $53 $20 $20
    

  * FamiTracker NSF Driver (<http://www.famitracker.com>)


    
    
    ASCII  :      "FTDRV "
    Binary :      $46 $54 $44 $52 $56 $20
    

  * NES Sound Driver & Library (<https://github.com/Shaw02/nsdlib>)


    
    
    ASCII  :      "NSDL  "
    Binary :      $4E $53 $44 $4C $20 $20
    

A blank NSFDRV ID may be used for sound drivers under development. 
    
    
    ASCII  :      "      "
    Binary :      $20 $20 $20 $20 $20 $20
    

## Player implementations

The following players implement NSFDRV identification: 

  * TNS-HFC4
  * hoot



Categories: [File formats](Category_File_formats.xhtml)
