# INES Mapper 120

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_120) | View [other pages](Special_AllPages.xhtml#INES_Mapper_120)

**iNES Mapper 120** denotes Whirlwind Manu's **LH15** cartridge conversion of Square's [FDS](Family_Computer_Disk_System.xhtml "FDS") game とびだせ大作戦 (Tobidase Daisakusen). 

# Banks

  * CPU $6000-$7FFF: 8 KiB switchable PRG ROM bank
  * CPU $8000-$FFFF: 32 KiB hard-wired PRG ROM bank #2
  * PPU $0000-$1FFF: 8 KiB unbanked CHR RAM
  * Nametable mirroring: hard-wired (to vertical)



# Registers

## PRG ROM switch ($41FF, write)
    
    
    Address Mask: $E100
    
    D~[.... .PPP]
             +++- Select 8 KiB PRG ROM bank at CPU $6000-$7FFF
    

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
