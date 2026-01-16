# INES Mapper 020

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_020) | View [other pages](Special_AllPages.xhtml#INES_Mapper_020)

Mapper 20 is defined as reserved for the Famicom Disk System ([FDS](Family_Computer_Disk_System.xhtml "FDS")) in official [iNES](INES.xhtml "INES") specs. The mapper number is mostly intended for being internally used by emulators (to indicate to themselves that they are emulating the FDS hardware). 

Actual FDS game disk images normally use the [FDS disk format](FDS_disk_format.xhtml "FDS disk format"). 

The mapper number could be theoretically used for the FDS BIOS ROM-image, but this isn't done in practice. The FDS BIOS is typically stored as [NROM](NROM.xhtml "NROM") if it is dumped in [iNES](INES.xhtml "INES") format, or it's stored as raw BIOS dump (without iNES header and thus without mapper number). 

Categories: [Bad iNES Mappers](Category_Bad_iNES_Mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
