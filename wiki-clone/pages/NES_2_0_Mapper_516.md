# NES 2.0 Mapper 516

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_516) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_516)

INES 2.0 Mapper 516 is used for the two _Brilliant Com Cocoma Pack_ cartridges from _EduBank_. They consist of an [MMC3](MMC3.xhtml "MMC3") clone (AX5202P) with an additional PAL. 

## Registers

Mask: $E011 

  * $8000, $8001, $A000, $A001, $C000, $C001, $E000, $E001; addresses where CPU A4 is low: As normal [MMC3](MMC3.xhtml "MMC3"). Note that the PRG and CHR banks are limited to a 128 KiB inner bank.



Mask: $8010 
    
    
    $8010: A~[1xxx xxxx xxx1 CCPP]
                             ||||
                             ||++-- Select 128 KiB outer PRG bank (i.e. PRG A18 and A17)
                             ++---- Select 128 KiB outer CHR bank (i.e. CHR A18 and A17)
    

## See also

[http://forums.nesdev.org/viewtopic.php?f=3&t=16660](http://forums.nesdev.org/viewtopic.php?f=3&t=16660)
