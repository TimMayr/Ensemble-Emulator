# INES Mapper 178

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_178) | View [other pages](Special_AllPages.xhtml#INES_Mapper_178)

**iNES Mapper 178** is used by some games from Waixing (on a PCB named **FS305**), Nanjing (on a PCB named **NJ0430**), Jncota and Henge Dianzi as well as some educational computers from GameStar, a.k.a. Belsonic. The chipset is designed for using 8 KiB of chip-internal unbanked CHR-RAM, but can address external RAM/ROM as well, banked in 8 KiB amounts. Circuit boards described by iNES Mapper 178 connect external RAM as bankable PRG-RAM mapped to CPU $6000-$7FFF. 

A company named _Gameinis_ uses this chipset along with an infrared sensor built right into the cartridge shell. This sensor will generate IRQs upon detecting movement. As some non-sensor mapper 178 games are bothered by IRQs that they are not expecting, **Submapper 0** denotes the absence, **Submapper 1** the presence of the infrared sensor, used by at least the following cartridges: 

  * _Gameinis Boxing+Soccer_
  * _Gameinis Ping Pong_



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Mode Register ($4800)
    * 2.2 Low PRG Bank Register ($4801)
    * 2.3 High PRG Bank Register ($4802)
    * 2.4 PRG-RAM Bank register ($4803)
    * 2.5 Infrared
  * 3 Errata
  * 4 See also



# Banks

  * CPU $6000-$7FFF: 8 KiB switchable PRG-RAM bank
  * CPU $8000-$FFFF: 16/32 KiB PRG-ROM bank, switchable
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM
  * Nametable mirroring: Switchable horizontal/vertical settings



# Registers

## Mode Register ($4800)
    
    
    7654 3210
    ---------
    .... .SSM
          ||+- Nametable mirroring
          ||    0: Vertical
          ||    1: Horizontal
          ++-- PRG banking mode
                0: NROM-256/BNROM (PRG A14=CPU A14)
                1: UNROM (PRG A14..16=111b if CPU A14=1)
                2: NROM-128
                3: UNROM but with bit 0 of "fixed" bank selectable
                   (PRG A15..16=11b if CPU A14=1)
    

## Low PRG Bank Register ($4801)
    
    
    7654 3210
    ---------
    .... .LLL
          +++- PRG A16..A14
    

This can be considered an inner bank register for UNROM mode. 

## High PRG Bank Register ($4802)
    
    
    7654 3210
    ---------
    HHHH HHHH
    ++++-++++- PRG A24?..A17
    

This can be considered an outer bank register for UNROM mode. 

## PRG-RAM Bank register ($4803)
    
    
    7654 3210
    ---------
    BBBB BBBB
    ++++-++++- PRG-RAM A20?..A13
    

PRG-RAM is mapped to CPU $6000-$7FFF and is banked in 8 KiB amounts. 

## Infrared

Some cartridges have an additional infrared sensor. Infrared receiving is working by having two registers: 
    
    
     $5000-$5FFF:  [.... ...V]  (read only)
        V = read current bit from IR sensor
     $6000-$7FFF:  [I... ....]  (write only)
        I = set to activate interrupts by infrared signal
    

The I bit is set at power-on, i.e. without writing to the $6000-$7FFF register, interrupts will be _enabled_. 

# Errata

  * The commonly-available ROM images of 
    * _宠物: 小精灵 IV (Chǒngwù: Xiǎo Jīnglíng IV)_ , headerless CRC32 0xB0B13DBD
    * _Education Computer 32-in-1 (Game Star) [!]_ , headerless CRC32 0xF834F634



    will not work when emulated using the above description. The meaning of registers $4801 and $4802 is swapped, and an emulator must shift the value of $4802 one bit to the left before ORing with $4801. While it cannot be ruled out that these games use an alternative version of the hardware, the more likely explanation is that these are simply bad dumps (notwithstanding GoodNES 3.23b's [!] tag) where the dumper confused the two bank registers.

  * Supposedly, the new PRG bank is only committed when $4801 is written. Emulating this supposed behavior however will break _Ping Pong_ , while commiting the PRG bank after any $480x write does not seem to break anything else.
  * Most of the educational computers using this mapper require Dendy video timing and will freeze at various points when run with NTSC timing.
  * The Waixing FS305 circuit board connects PRG A14 directly to CPU A14, making all PRG banking modes except BNROM unusable. Since it writes the correct banking mode to $4800, it does not require any special treatment.



# See also

  * <https://forums.nesdev.org/viewtopic.php?t=16657>



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with large PRG RAM](Category_Mappers_with_large_PRG_RAM.xhtml)
