# User:Zzo38/Mapper C

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_C) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_C)

This mapper is unusual; other than the ordinary ROM and RAM, it is intended to be possible to implement using only a single quad-NAND gate IC, despite having as many features as can fit in such an IC. It has the following features: 

  * 64K PRG ROM, in two 32K banks, but one of them cannot be accessed during rendering (or 32K unswitched PRG ROM)
  * 8K or 16K CHR ROM (or CHR RAM), with bankswitching by selecting tiles or nametables
  * 8K PRG RAM, mapped at $5xxx and $7xxx, with write-only mirrors at $1xxx and $3xxx (attempting to read these mirrors causes bus-conflicts)



If the PPU address is set to $3xxx then the second PRG bank is available. (This means that it will switch when setting up the palette!) 

## CHR bankswitching

CHR bankswitching is a bit more complicated. If there is only 8K ROM, then $0xxx and $1xxx each access the same 4K bank. 

Bank switching is done by bit10 and bit11 of the PPU address bus. If both bits are set, the bank is unchanged. If bit11 is set and bit10 is clear, then the high bank is selected. If bit11 is clear and bit10 is set, then the low bank is selected. If both bits are clear, then the high bank is selected although the bank will be undefined if the next access sets both bits, until one of them is cleared. All bank switches function for the current fetch too, not only for the next one. 

Because it bank switches even when fetching the name table, sprites, garbage bytes, etc, normally you should use only the name table at $2C00, set the bank at the beginning of each row (otherwise you will have problems), and don't use tiles 192-255 for sprites. 

  

