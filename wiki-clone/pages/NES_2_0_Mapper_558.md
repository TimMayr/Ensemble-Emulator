# NES 2.0 Mapper 558

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_558) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_558)

**NES 2.0 Mapper 558** denotes the 燕城 (Yànchéng) **YC-03-09** circuit board, used for the following games: 

  * _口袋精靈: 水晶_ (Pokémon: Crystal Edition)
  * _大話三國_ (Dàhuà Sānguó)



A compatible circuit board with an unknown PCB code is used for the original editions of a few games from 外星 (Wàixīng): 

  * _Pet Family - 宠物大家族: 部落纷争_ (ES-1081)
  * _Pet Evolve - 宠物进化史_ (ES-1085)
  * _口袋妖怪꞉ 鑽石版_ (Pokémon: Diamond Edition, ES-1090)
  * _口袋精靈: 紅_ and its title screen hack _宠物: 红_ (Pokémon: Red Edition, ES-1088)
  * _数码暴龙 4: 水晶版_ and its title screen hack _数码宝贝_ (Digimon 4: Crystal Edition)
  * _盟军敢死队 - Commandos_



The Yànchéng games save the game state to a 93C66 512-byte serial EEPROM, its "ORG" pin tied to GND, selecting a word size of 8 bits. The Wàixīng games save to 8 KiB battery-backed WRAM. The NES 2.0 Header's PRG-NVRAM field distinguishes between the two board variants. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Bank Low ($5000, write)
    * 2.2 PRG-ROM Bank High ($5100, write)
    * 2.3 Microwire Interface ($5200, write)
    * 2.4 Mode ($5300, write)
    * 2.5 Microwire Interface ($5500, read)
  * 3 Notes
  * 4 See also



# Banks

  * CPU $6000-$7FFF: up to 8 KiB unbanked PRG-RAM, can be battery-backed
  * CPU $8000-$FFFF: 32 KiB switchable PRG-ROM bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM bank
  * Nametable mirroring: hard-wired



# Registers

All registers are initialized to $00 on reset. 

## PRG-ROM Bank Low ($5000, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... PPPP
           ++++- PRG A18..A15
      
    

  * Note that if $5300.0 is 0, the two lowest bits are swapped during writing.



## PRG-ROM Bank High ($5100, write)
    
    
    Mask: $FB00
    
    D~7654 3210
      ---------
      .... ..PP
             ++- PRG A20..A19
    

  * Note that if $5300.0 is 0, the two lowest bits are swapped during writing.



## Microwire Interface ($5200, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... .SDC
            ||+- 93C66 EEPROM CLK ($5300.0=0)/DAT ($5300.0=1) output
            |+-- 93C66 EEPROM DAT ($5300.0=0)/CLK ($5300.0=1) output
            +--- 93C66 EEPROM CS output
    

## Mode ($5300, write)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... .AB.
            |+-  0: Swap D0,D1 on all register writes to $5000-$5200.
            |    1: Leave D0,D1 unmodified on register writes to $5000-$5200.
            +--- 0: PRG A15/A16=1
                 1: PRG A15/A16 from $5000
    

  * Writes to this register are _not_ subject to D0/D1 bit-swapping, i.e. the register is not affected by its own bit 0.
  * Because this register is initialized to $00 on reset, thus clearing bit 2 which causes PRG A15/A16 to be held high, the game will boot in 32 KiB PRG-ROM bank #3.
  * Changing the value of $5300.0 without writing to $5000-$5200 afterwards does not change the latched bits.



## Microwire Interface ($5500, read)
    
    
    Mask: $FF00
    
    D~7654 3210
      ---------
      .... .A..
            |
            |
            +--- 93C66 EEPROM DAT input
    

Unlike [INES Mapper 164](INES_Mapper_164.xhtml "INES Mapper 164"), the EEPROM input is _not_ inverted. 

# Notes

  * CPU D0 and D1 are connected in reverse to the mapper blob compared to other circuit boards using this ASIC. This means that setting $5300.0 to 0, not to 1, will effectively swap PRG A15/A16 and PRG A19/A20. Cartridges using this mapper must be dumped with $5300=$07 during dumping to obtain a readout that matches that of a desoldered ROM chip.
  * The Wàixīng games using this mapper were previously set to 162/163 or 164, resulting in incompatibility either because of the different address of the PRG-ROM Bank High register (mapper 162/163) or the different startup bank (164).



# See also

  * [INES Mapper 162](INES_Mapper_162.xhtml "INES Mapper 162")/163 swap the effective meaning of registers $5100 and $5200, the meaning of the D0/D1 bit swap, and use the microwire interface only for protection purposes.
  * [INES Mapper 164](INES_Mapper_164.xhtml "INES Mapper 164") is an earlier implementation without D0/D1 swapping, optional UNROM-like bankswitching, and a 1 bpp CHR mode.
  * [PCB images and analysis]()



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
