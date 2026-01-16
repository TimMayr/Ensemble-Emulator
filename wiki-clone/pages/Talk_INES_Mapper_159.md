# Talk:INES Mapper 159

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_159) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_159)

## 24C01 vs. X24C01

What is the difference between the 24C01 and X24C01? The X24C01 is a cut-down version of the 2C401. While not strictly relevant to known NES and Famicom usage, eight 24C01/24C02s can be supported on a single address bus. The X24C01 does not have this feature. This requires sending the Slave Address on the 24C01 before the Write Address, the X24C01 just sends the write address. The 24C01 supports a two-byte write operation, the X24C01 supports four-bytes. There may be other differences as described in the data sheets for the two devices. 
