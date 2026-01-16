# NES 2.0 Mapper 407

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_407) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_407)

**NES 2.0 Mapper 407** is used by the _Win, Lose or Draw_ Plug-n-Play which came built into a tablet. The game uses the [VT03](VTxx.xhtml "VT03") enhanced NOAC. The submapper field is set to 15 to denote that the same CPU opcode encryption method as Mapper [256](NES_2_0_Mapper_256.xhtml "NES 2.0 Mapper 256")'s, submapper 15. The NOAC's PRG A24 selects between PRG-ROM (0) and 64 KiB of PRG-RAM (1), which the OneBus hardware can map to both CPU and PPU address space. 
