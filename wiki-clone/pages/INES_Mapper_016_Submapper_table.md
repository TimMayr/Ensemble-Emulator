# INES Mapper 016/Submapper table

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_016/Submapper_table) | View [other pages](Special_AllPages.xhtml#INES_Mapper_016_Submapper_table)

INES Mapper 016 submapper table  
---  
Submapper # | Meaning | Note   
0 | Unspecified | Emulate both FCG-1/2 and LZ93D50 chips in their respective CPU address ranges.   
_1_ | _LZ93D50 with 128 byte serial EEPROM (24C01)_ | _Deprecated, use[INES Mapper 159](INES_Mapper_159.xhtml "INES Mapper 159") instead._  
_2_ | _Datach Joint ROM System_ | _Deprecated, use[INES Mapper 157](INES_Mapper_157.xhtml "INES Mapper 157") instead._  
_3_ | _8 KiB of WRAM instead of serial EEPROM_ | _Deprecated, use[INES Mapper 153](INES_Mapper_153.xhtml "INES Mapper 153") instead._  
4 | FCG-1/2 | Responds only in the CPU $6000-$7FFF address range; IRQ counter is not latched.   
5 | LZ93D50 with no or 256-byte serial EEPROM (24C02) | Responds only in the CPU $8000-$FFFF address range; IRQ counter is latched.   
  
If there are 256 bytes of serial EEPROM, they will be denoted as PRG-NVRAM in the [NES 2.0](NES_2_0.xhtml "NES 2.0") header (byte value $20). See: [INES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016")
