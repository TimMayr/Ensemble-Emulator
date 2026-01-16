# NES 2.0 Mapper 298

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_298) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_298)

**NES 2.0 Mapper 298** is used for the unlicensed game _Lethal Weapon_ , which is actually a backport of [Lethal Enforcers](https://en.wikipedia.org/wiki/Lethal_Enforcers), and not to be confused with the [licensed game of the same name](http://bootgod.dyndns.org:7777/profile.php?id=702). Its UNIF board name is **UNL-TF1201**. 

The board uses a [VRC4](VRC2_and_VRC4.xhtml "VRC4") clone in the VRC4b configuration (CPU 0x02s to VRC4 A0, CPU 0x01s to VRC4 A1). The only difference is that register $F003 is implemented differently than on the original ASIC: writing to $F003 only acknowledges a pending IRQ, but it does not move the 'A' control bit to the 'E' control bit [the way an original VRC4 does](VRC_IRQ.xhtml#IRQ_Acknowledge "VRC IRQ"). This is relevant for emulation, as the game writes to $F003 immediately after enabling the IRQ by writing $02 to $F002. 

The level selected by the middle icon in the top row of the level selection screen [will shake greatly on real hardware](https://youtu.be/YIdkWZuoRXM), and therefore is not an emulation fault. 

Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
