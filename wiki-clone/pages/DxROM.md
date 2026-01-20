# DxROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/DxROM) | View [other pages](Special_AllPages.xhtml#DxROM)

The generic designation **DxROM** refers to cartridge boards made by Nintendo that use the seemingly-identical Namco 108, 109, 118 or 119 mappers. Nintendo did not manufacture the Famicom counterparts. The [iNES](INES.xhtml "INES") format assigns [Mapper 206](INES_Mapper_206.xhtml "INES Mapper 206") to DxROM. 

The following DxROM boards are known to exist: 

Board | PRG ROM | CHR ROM | Comments   
---|---|---|---  
DEROM | 64 KiB | 32,64 KiB ROM | Nintendo's clone of Tengen's 800002 and Namco's 3413 and 3414   
DE1ROM | 128 KiB | 64 KiB | Nintendo's clone of Tengen's 800030 and Namco's 3406   
DRROM | 128 KiB | 64 KiB | Nintendo's clone of Tengen's 800004, with four-screen mirroring.   
  
Tengen initially tried to get around Nintendo's publication restrictions by manufacturing all parts of the games themselves. They were eventually prohibited from continuing, but Nintendo only re-released a subset of all the games that Tengen brought to the US. 

Namco published a wide number of differently-numbered Famicom counterparts. Some of them have significantly different behavior and have had their own mappers allocated. 

Board | PRG ROM | CHR ROM | Comments   
---|---|---|---  
3401 | 32 KiB | 32 KiB |   
3405, 3415 | 128 KiB | 32 KiB |   
3406, 3416 | 128 KiB | 64 KiB |   
3407, 3417, 3451 | 32 KiB | 16,32 KiB | PRG A13, A14 connected directly to CPU, no PRG banking available   
3413 | 64 KiB | 32 KiB |   
3414 | 64 KiB | 64 KiB |   
[3425](INES_Mapper_095.xhtml "INES Mapper 095") | 128 KiB | 32 KiB | CHR A15 controls CIRAM A10   
[3433, 3443](INES_Mapper_088.xhtml "INES Mapper 088") | 128 KiB | 128 KiB | PA12 controls CHR A16   
[3446](INES_Mapper_076.xhtml "INES Mapper 076") | 128 KiB | 128 KiB | 2K banks using regs 2-5   
[3453](INES_Mapper_154.xhtml "INES Mapper 154") | 128 KiB | 128 KiB | Bit 6 of written value controls CIRAM A10; PA12 controls CHR A16   
  
Otherwise, the Namco 108 family is just a subset of [MMC3](MMC3.xhtml#Variants "MMC3")

Categories: [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
