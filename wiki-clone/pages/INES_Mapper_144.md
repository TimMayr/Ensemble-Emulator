# INES Mapper 144

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_144) | View [other pages](Special_AllPages.xhtml#INES_Mapper_144)

iNES Mapper 144, allocated for the game [Death Race](https://nescartdb.com/profile/view/1223/death-race), describes a intentionally defective variant of the [Color Dreams](Color_Dreams.xhtml "Color Dreams") board (**mapper 11**). 

This game's PCB (labelled **50282**) is almost identical to the [revision B Color Dreams boards](https://nescartdb.com/search/advanced?pcb=UNK-COLORDREAMS-REVB), but a 300Ω resistor was added between CPU D0 and the combination of mapper hardware and ROM. This addition means that _only_ the ROM's _least significant bit_ always wins bus conflicts. 

Nestopia-1.4.0 implements this as `EffectiveData = (WrittenData & ROM[address]) | (ROM[address] & 1)`

FCEUX-2.1.5 works around this by ignoring writes to 0x8000 

The most succinct description is `EffectiveData = (ROM[address] & (WrittenData|1))`

Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
