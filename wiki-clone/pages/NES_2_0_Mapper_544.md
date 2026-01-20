# NES 2.0 Mapper 544

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_544) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_544)

NES 2.0 Mapper 544 denotes the Waixing FS306 board, used for _覇王的大陸 (Bàwáng de Dàlù)_ , which is their Chinese translation of Namco's _三国志 II: 覇王の大陸 (Sangokushi II: Haō no Tairiku)_. It is similar to [INES Mapper 253](INES_Mapper_253.xhtml "INES Mapper 253") in combining a [VRC4](VRC2_and_VRC4.xhtml "VRC4") clone with a GAL chip that replaces a selectable two or four CHR banks with 2 KiB of CHR-RAM. It differs in the address lines to which the VRC4 responds, in adding a third selectable 8 KiB PRG-ROM bank, and in selecting the CIRAM bank for all four nametables independently. 

A later re-release of the game uses [INES Mapper 176](INES_Mapper_176.xhtml "INES Mapper 176"). 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 VRC4 registers
    * 2.2 PRG-ROM select #2 ($9C00, write)
    * 2.3 CIRAM select ($9C0C-$9C0F, write)
    * 2.4 CHR-RAM Bank Selection
  * 3 See also



# Banks

  * CPU $6000-$7FFF: 8 KiB fixed PRG-RAM bank (battery-backed)
  * CPU $8000-$9FFF: 8 KiB switchable PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG-ROM bank
  * CPU $C000-$DFFF: 8 KiB switchable PRG-ROM bank
  * CPU $E000-$FFFF: 8 KiB PRG-ROM bank, fixed to last bank
  * PPU $0000-$03FF: 1 KiB switchable CHR-ROM/RAM bank
  * PPU $0400-$07FF: 1 KiB switchable CHR-ROM/RAM bank
  * PPU $0800-$0BFF: 1 KiB switchable CHR-ROM/RAM bank
  * PPU $0C00-$0FFF: 1 KiB switchable CHR-ROM/RAM bank
  * PPU $1000-$13FF: 1 KiB switchable CHR-ROM/RAM bank
  * PPU $1400-$17FF: 1 KiB switchable CHR-ROM/RAM bank
  * PPU $1800-$1BFF: 1 KiB switchable CHR-ROM/RAM bank
  * PPU $1C00-$1FFF: 1 KiB switchable CHR-ROM/RAM bank
  * PPU $2000-$23FF: 1 KiB switchable CIRAM bank
  * PPU $2400-$27FF: 1 KiB switchable CIRAM bank
  * PPU $2800-$2BFF: 1 KiB switchable CIRAM bank
  * PPU $2C00-$2FFF: 1 KiB switchable CIRAM bank



# Registers

The VRC4 clone's A0 and A1 inputs are connected to CPU A10 and A11, respectively. Subaddresses $000/$001/$002/$003 therefore become $000/$400/$800/$C00. The VRC4's External Select at $9C00 (pin 18) is used to select the third PRG-ROM bank and the CIRAM bank for all four nametables. 

## VRC4 registers

  * PRG-ROM select #0 ($8000, write)
  * PRG-ROM select #1 ($A000, write)
  * CHR Select Low/High ($B000-$EFFF, write)
  * IRQ Control ($F000-$FFFF, write)



These registers function identically to a normal [VRC4](VRC2_and_VRC4.xhtml "VRC4"). Note that the game uses 512 KiB of CHR-ROM, which even the original VRC4 supported. The IRQ seems to occur one M2 cycle later than on an original VRC4. 

## PRG-ROM select #2 ($9C00, write)
    
    
    Mask: $9C04
    
    D~7654 3210
      ---------
      ...P PPPP
         | ||||
         +-++++- Select 8 KiB PRG bank at $C000-$DFFF
    

## CIRAM select ($9C0C-$9C0F, write)
    
    
    Mask: $9C07
    
    D~7654 3210
      ---------
      .... ...N
              +- Select 1 KiB CIRAM bank
    
    $9C0C: Select 1 KiB CIRAM bank at PPU $2000-$23FF
    $9C0D: Select 1 KiB CIRAM bank at PPU $2400-$27FF
    $9C0E: Select 1 KiB CIRAM bank at PPU $2800-$2BFF
    $9C0F: Select 1 KiB CIRAM bank at PPU $2C00-$2FFF
    

## CHR-RAM Bank Selection

The game writes, via the PPU, to specific CHR banks in order to select both which and how many CHR banks are mapped to RAM instead of ROM. Writes to CHR ROM in multiples of 1 KiB: 
    
    
    CHRB~[. 1Z.D L.L.]
            || | | |
            |+-|-+-+---------------- Select first bank and size of CHR RAM:
            || |                      $80 = $028-$02B, $128-$12B
            || |                      $82 = $000-$003, $100-$103
            || |                      $88 = $04C-$04F, $14C-$14F
            || |                      $8A = $064-$067, $164-$167
            || |                      $C0 = $046-$047, $146-$147
            || |                      $C2 = $07C-$07D, $17C-$17D
            || |                      $C8 = $004-$005, $104-$105
            || |                      $CA = only CHR ROM
            || +-------------------- If 1, ignore above and always enable CHR ROM / disable CHR RAM
            |+---------------------- Number of adjacent banks of CHR RAM, 0=four, 1=two
            +----------------------- Must be 1 
    Power-on value: $80
    

The meanings are identical to [INES Mapper 195](INES_Mapper_195.xhtml "INES Mapper 195") except for value $C8. The game uses different banks as CHR-RAM for different game situations, so the CHR-RAM Bank Selection feature must be emulated. Unlike Mapper 195, since there are only 2 KiB of CHR-RAM mounted on the circuit board, the 4 KiB settings cause CHR-RAM to be mirrored once, and all settings mapping the same 2 KiB of CHR-RAM. 

# See also

[Discussion with PCB pictures](http://forums.nesdev.org/viewtopic.php?f=9&t=19041)

Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml)
