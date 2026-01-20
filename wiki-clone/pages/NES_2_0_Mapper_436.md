# NES 2.0 Mapper 436

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_436) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_436)

**NES 2.0 Mapper 436** denotes the **ZLX-08** plug-and-play VT02 console PCB, used by the _Entertainment System 620-in-1_ plug-and-play console. It is uses normal [OneBus banking](NES_2_0_Mapper_256.xhtml "NES 2.0 Mapper 256"), with one exception: the VT02's PRG A23 output (register $4100 bit 6 for PRG and bit 2 for CHR accesses) is connected to PRG-ROM A24, and PRG A23 comes from the VT02's I/O port at $410F, bit 5 instead. Since the I/O port is high-impedance on reset, which is pulled-up to a logical "1", the reset vectors are in the _second_ 8 MiB part of ROM. 
