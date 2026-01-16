# INES Mapper 167

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_167) | View [other pages](Special_AllPages.xhtml#INES_Mapper_167)

**iNES Mapper 167** denotes the circuit board for Subor's Chinese **小霸王 中英文电脑学习机 IV** ("Subor Chinese and English Computer Learning Machine IV") and Russian **Сюбор Обучающий Компьютер** ("Subor Educational Computer") cartridges. 

# Overview

  * CPU $6000-$7FFF: 8 KiB unbanked PRG-RAM
  * CPU $8000-$BFFF: 16 KiB fixed or switchable window into 1 MiB PRG-ROM
  * CPU $C000-$FFFF: 16 KiB fixed or switchable window into 1 MiB PRG-ROM
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM
  * Nametable arrangement: Switchable between Horizontal and Vertical



# Registers

There are four writable registers in the CPU $8000-$FFFF address range. 

**Register 1: Responds to writes to CPU $8000-$9FFF**
    
    
    D~[...F ...N] A~[100. .... .... ....]
          |    +- Nametable arrangement
          |       0: Horizontal
          |       1: Vertical
          +------ PRG A19 for switchable window (XOR'd with "f" bit from register 2)
    

**Register 2: Responds to writes to CPU $A000-$BFFF**
    
    
    D~[...f MM..] A~[101. .... .... ....]
          | ++--- PRG-ROM bankswitching mode
          |       0: [UNROM](UxROM.xhtml "UNROM")-512 with fixed bank number $20
          |       1: [Reverse UNROM](INES_Mapper_097.xhtml "INES Mapper 097")-512 with fixed bank number $1F
          |       2: [NROM](NROM.xhtml "NROM")-256 with PRG A14 being inverted from CPU A14
          |       3: same as mode 2
          +------ PRG A19 for switchable window (XOR'd with "F" bit from register 1)
    

**Register 3: Responds to writes to CPU $C000-$DFFF**
    
    
    D~[...E DCBA] A~[110. .... .... ....]
          +-++++- PRG A18..A14 for switchable window (XOR'd with "edcba" bits from register 4)
    

**Register 4: Responds to writes to CPU $E000-$FFFF**
    
    
    D~[...e dcba] A~[111. .... .... ....]
          +-++++- PRG A18..A14 for switchable window (XOR'd with "EDCBA" bits from register 3)
    

Each bank number bit for the switchable window exists twice. The final bank number is the XOR'd value of both bits. This provides the necessary flexibility for including the various mini-games, that come from various sources, on the cartridges. The combination of mode and XOR'd bank bits results in the following three possible layouts: 

Mode | 16 KiB bank number (binary) for CPU address range ...   
---|---  
8000-BFFF | C000-FFFF   
0 | FE'DCBA XOR fe'dcba | 10'0000   
1 | 01'1111 | FE'DCBA XOR fe'dcba   
2/3 | FE'DCB1 XOR fe'dcb0 | FE'DCB0 XOR fe'dcb0   
  
# Errata

[INES Mapper 166](INES_Mapper_167.xhtml "INES Mapper 166") had been used at one time to denote mapper 167 ROM image files with an incorrect bank order, namely, a fixed bank $07 rather than $20 in mode 0, and a non-inverted PRG A14 in modes 2 and 3. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
