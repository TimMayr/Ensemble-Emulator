# User:Zzo38/Mapper H

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_H) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_H)

This mapper design is intended to support multiple vertical scrolling regions (for example, like those in multiplayer Panel de Pon). It is untested. 

## Registers

There are three mapper registers: 

  * MODE (8-bits)
  * COLOR (2-bits; same data is repeated four times so 8-bits in total)
  * SCROLL (3-bits)



All registers are only updated by PPU reads, as follows: 

  * When reading from PPU address where bit11 and bit10 are both low, bits 13, 12, 9, 8, 7, 6, 5, and 4 of the PPU address are copied into the MODE register.
  * When reading from PPU address where bit9, bit8, bit7, and bit6 are not all high, the two low bits of the PPU data are copied into the COLOR register.
  * When reading from PPU address where bit13 is low, the low three bits of the PPU address are copied into the SCROLL register.



## PPU address space

PPU memory is mapped as follows: 

  * $0000-$1FFF = CHR ROM; the low three address bits are determined by the low three bits of the sum of the SCROLL register and MODE register, while the high four bits of the MODE register select an 8K bank
  * $2000-$3FFF (unless bit9, bit8, bit7, bit6 all high) = CIRAM; bit3 of the result of the sum of the SCROLL register and the MODE register selects between name tables
  * $2000-$3FFF (when bit9, bit8, bit7, bit6 all high) = the COLOR register, extended to fill the entire byte



## CPU address space

PRG ROM is mapped to $8000-$FFFF. PRG bankswitching is an optional feature; if present, banks are 32K and the other bank is selected by setting bit13 and bit12 of the PPU address (reading is not necessary). 

PRG RAM is optional. If present, it is 4K or 8K, and is mapped at $5000-$5FFF and $7000-$7FFF, with mirrors at $1000-$1FFF and $3000-$3FFF (reading from the mirrors will cause bus conflicts; writing through the mirrors is OK). PRG RAM is not battery backed. 
