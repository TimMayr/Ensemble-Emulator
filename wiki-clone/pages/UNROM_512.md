# UNROM 512

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UNROM_512) | View [other pages](Special_AllPages.xhtml#UNROM_512)

**UNROM 512**

**Company** | RetroUSB, InfiniteNESLives, Broke Studio   
---|---  
**Complexity** | Discrete logic   
**Boards** | UNROM-512-8, UNROM-512-16, UNROM-512-32, INL-D-RAM, UNROM-512-F   
**PRG ROM capacity** | 256K or 512K   
**PRG ROM window** | 16K + 16K fixed ($C000)   
**PRG RAM capacity** | None   
**CHR capacity** | 32K   
**CHR window** | 8K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H or V (solder pad), 1-screen (solder pad or switchable), 4-screen fixed   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Yes (in non-flashable config), else No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [030](UNROM_512.xhtml "INES Mapper 030")  
  
**UNROM 512** is a discrete-logic board made by RetroUSB as an extension of [UNROM](UxROM.xhtml "UNROM") with up to 512kB of PRG ROM, bankable CHR RAM, an option for mapper-controlled single-screen mirroring, as well as a self-flashable configuration for rewriting PRG. Since its initial creation InfiniteNESLives has replicated its design and also added an optional four-screen mirroring variation. 

The [iNES](INES.xhtml "INES") format assigns **iNES Mapper 030** to UNROM 512. 

The [UNIF](UNIF.xhtml "UNIF") names for this board are **UNROM-512-8** , **UNROM-512-16** and **UNROM-512-32** , depending on how much CHR RAM is present. 

In GoodNES 3.23b, a few ROMS using the Waixing FS005 board ([INES Mapper 176](INES_Mapper_176.xhtml "INES Mapper 176")) are also set to Mapper 30. These ROMs were converted from Waixing's proprietary WXN/MFC format, which has a 16 byte header that is somewhat similar but not the same as the [INES](INES.xhtml "INES") header, and when interpreting this header's data as if it were an iNES header, the resulting mapper number would be 30. 

Examples: 

  * _Study Hall_
  * _[Battle Kid 2: Mountain of Torment](User_Sivak_Battle_Kid_2__Mountain_of_Torment.xhtml "User:Sivak/Battle Kid 2: Mountain of Torment")_
  * _Black Box Challenge_ ([ROM download](http://slydogstudios.org/), 4-screen)
  * _Mystic Origins_
  * _NES Maker_



## Contents

  * 1 RetroUSB board
    * 1.1 Electrical schematic
    * 1.2 Board wiring configurations
  * 2 InfiniteNESLives 4-screen board
    * 2.1 PPU memory map
    * 2.2 Electrical schematic
  * 3 Mapper
    * 3.1 Nametable Configuration
  * 4 See Also
  * 5 References



## RetroUSB board

The example board in question is marked as follows: "Sealie Computing", "12/29/2011 revA", "UNROM 512". The backside of the board was mislabelled with "ReproPak MMC3". 

The board includes: 

  * 32x 16kB (512kB total) PRG banks. The 8BIT XMAS board variant only permits 16x 16kB (256kB total); see _Electrical schematic_ below
  * 4x banks worth of full 8kB CHR-RAM (configurable by jumpers)
  * Wiring for non-flashable or self-flashable configurations
  * Mirroring selection: horizontal, vertical, and mapper-controlled one-screen
  * Bus conflicts are only present on the non-flashable configuration of the board.



Some images of the board: 

  * [Study Hall PCB Front](https://www.nesdev.org/w/images/default/9/90/Unrom512_front_study_hall.jpg "Unrom512 front study hall.jpg")
  * [Battle Kid 2 PCB Front](https://www.nesdev.org/w/images/default/f/f1/Unrom512_front_bk2.jpg "Unrom512 front bk2.jpg")
  * [PCB Back](https://www.nesdev.org/w/images/default/a/a8/Unrom512_back.jpg "Unrom512 back.jpg")
  * [8BIT XMAS revD PCB Front](https://www.nesdev.org/w/images/default/d/dc/8BIT_XMAS_revD.jpg "8BIT XMAS revD.jpg")
  * [8BIT XMAX revD PCB Back](https://www.nesdev.org/w/images/default/7/75/8BIT_XMAS_revD_back.jpg "8BIT XMAS revD back.jpg")



When board is not flashable: 
    
    
      Registers:
      ---------------------------
      
      Range,Mask:   $8000-FFFF, $8000
      
        $8000:  [MCCP PPPP]
          M = One screen Mirroring select
          C = CHR RAM bank
          P = PRG ROM bank
      
      PRG Setup:
      ---------------------------
           $8000   $A000   $C000   $E000  
          +-------------------------------+
          |     $8000     |     { -1}     |
          +---------------+---------------+
      
      CHR Setup:
      ---------------------------
            $0000   $1000   
          +-------+-------+
          |     $0000     |
          +-------+-------+
    

When board is self flashable: 
    
    
      Registers:
      ---------------------------
      
      Mask: $C000
      
        $8000:  [.... ....]
          Write a sequence of bytes here for writing to the flash. The sequence is as follows 
    
    (This code must exist in RAM):
            Erase 4KB Flash Sector:
            $C000:$01, $9555:$AA
            $C000:$00, $AAAA:$55
            $C000:$01, $9555:$80
            $C000:$01, $9555:$AA
            $C000:$00, $AAAA:$55
            $C000:BANK, ADDR:$30 (Where BANK is $00-1F, ADDR is $8000,$9000,$A000,$B000)
            Read the written location twice until you get $FF twice.
    
            Write a byte:
            $C000:$01, $9555:$AA
            $C000:$00, $AAAA:$55
            $C000:$01, $9555:$A0
            $C000:BANK, ADDR:DATA (Where BANK is $00-1F, ADDR is $8000-$BFFF, DATA is $00-$FF)
            Read the written location twice until you get DATA twice.
    
    Additionally, on 8BIT XMAS boards, any writes here controls the LEDs as follows.
    
        $8000:  [BYGR bygr]
            B = Blue 3, Blue 4
            Y = Yellow 3, Yellow 4
            G = Green 3, Green 4
            R = Red 3, Red 4
            b = Blue 1, Blue 2
            y = Yellow 1, Yellow 2
            g = Green 1, Green 2
            r = Red 1, Red 2
    
      
        $C000:  [MCCP PPPP]
          M = One screen Mirroring select
          C = CHR RAM bank
          P = PRG ROM bank
      
      PRG Setup:
      ---------------------------
           $8000   $A000   $C000   $E000  
          +-------------------------------+
          |     $8000     |     { -1}     |
          +---------------+---------------+
      
      CHR Setup:
      ---------------------------
            $0000   $1000   
          +-------+-------+
          |     $8000     |
          +-------+-------+
    

### Electrical schematic
    
    
                 ____
    A14 ---------\    \
                  )    >--- ROM A14
    377 Q0 ------/____/
    
                 ____
    A14 ---------\    \
                  )    >--- ROM A15
    377 Q1 ------/____/
    
                 ____
    A14 ---------\    \
                  )    >--- ROM A16
    377 Q2 ------/____/
    
                 ____
    A14 ---------\    \
                  )    >--- ROM A17
    377 Q3 ------/____/
    
                 ____
    A14 ---------\    \
                  )    >--- ROM A18
    377 Q4 ------/____/
    
    16/32KB CHRRAM
    377 Q5 --[]--+-- CHRRAM A13
                 |
    VCC -----[]--+
    8KB CHRRAM
    
    8/16KB CHRRAM
    VCC -----[]--+
                 |
    377 Q6 --[]--+-- CHRRAM A14
    32KB CHRRAM
    
    ONE
    377 Q7 --[]--+
                 |
    VERT         |
    PPU A10 -[]--+-- CIRAM_A10
                 |
    HORIZ        |
    PPU A11 -[]--+
    
    FLASHLOCKED3
    R/W ------[]-+
                 |
    FLASHABLE    |
    GND ------[]-+-- 377_/CE
    
    FLASHLOCKED1
    VCC ------[]-------+
              ½74LS139 |
              .------. |
    CPUA14----|A   Y0|-+- PRGROM_/WE
    R/W ------|B   Y1|-+- 377_CLK
              |    Y2| |
    /ROMSEL-+-|G   Y3| |
            | '------' |
            |          |
            +---[]-----+
           FLASHLOCKED2
    
    FLASHLOCKED4
    GND ------[]-----+
            ½74LS139 |
            .------. |
    R/W ----|A   Y0| |
    GND ----|B   Y1|-+- PRGROM_/OE
            |    Y2|
    /ROMSEL-|G   Y3|
            '------'
    
    With 8BIT Xmas Board, there is a second 377 chip for the LEDs, and 2 jumpers to control whether the 377 is sourcing
    current to the LEDs or sinking current from them.  Additionally, the board has no flash control jumpers, and is hard-wired
    flashable,  and has no CHR ram control jumpers, is hard-wired as 32KB, and it does not have the second 74LS32 chip,
    required for a 512KB board, so its max size is 256KB.
    
    Q0-----R1-----RED1-----RED2-------+
    Q1--+--R6-----GRN1----------------+
        +--R5-----GRN2----------------+
    Q2-----R3-----YEL1-----YEL2-------+
    Q3--+--R9-----BLU1----------------+
        +--R10----BLU2----------------+
    Q4-----R2-----RED3-----RED4-------+
    Q5--+--R7-----GRN3----------------+
        +--R8-----GRN4----------------+
    Q6-----R4-----YEL3-----YEL4-------+
    Q7--+--R11----BLU3----------------+
        +--R12----BLU4----------------+
                                      |
    VCC---------[]--------------------+
    GND---------[]--------------------+
    

### Board wiring configurations

Not Flashable: 

  * All 4 Flash Locked jumpers are soldered.
  * Flashable jumper **is not** soldered.
  * 74*139 **is not** mounted.
  * This configuration is subject to bus conflicts.



Flashable: 

  * All 4 Flash Locked jumpers **are not** soldered.
  * Flashable jumper is soldered.
  * 74*139 is mounted.
  * This configuration **is not** subject to bus conflicts.



The Self-flashable configuration should be implemented according to the [SST39SF040](http://www.microchip.com/wwwproducts/Devices.aspx?product=SST39SF040)'s datasheet. 

Example flash writing code - [Flash.asm](https://www.nesdev.org/w/images/default/f/fa/Flash.asm.txt "Flash.asm.txt")

## InfiniteNESLives 4-screen board

  * Two of the unused OR gates are used in combination with jumpers to disable NES mainboard VRAM/CIRAM and utilize cartridge CHR RAM for all pattern, name, & attribute tables.
  * This variation is only well defined with 32 KiB of CHR-RAM, but note that the last bank of CHR-RAM now contains the nametables.



    \- As such, only the first 3 CHR-RAM banks (24KB) and the second half of the last bank can be used freely for pattern table tile data.

  * The last 8KB CHR-RAM bank is permanently mapped to PPU $2000-3FFF (effectively $2000-3EFF due to palette data.) 
    * The first 4KB of this last CHR-RAM bank is mapped to PPU $2000-2FFF to provide four-screen mirroring.
    * PPU $3000-3EFF are no longer mirrors of the nametables, and actually have independent RAM mapped.



    

    \- If not used for PT1 in bank 3, this 3.75KB of RAM can be used by the developer as a 'scratch RAM' space accessible via CPU $2006/2007.
    \- The last 256bytes of the last RAM bank are not accessible at $3Fxx due to palette data being mapped there instead.

  * Setting of both CHR RAM bank bits ($C000: 0bx11xxxxx) is permissible, but the pattern table at $0000 is now the same physical RAM as holds the nametables.



    \- Improperly operating in this mode will render NT/AT data as tiles in PT0, and writes to PT0 will corrupt NT/AT data.
    \- PT1 in this mode remains usable at $1000-$1FFF. This is the only means of gaining access to the last 256bytes of the last CHR-RAM bank which are normally hidden behind PPU palette address space.

### PPU memory map

Address range | Size | Description   
---|---|---  
$0000-$0FFF | $1000 | Pattern Table 0   
$1000-$1FFF | $1000 | Pattern Table 1   
$2000-$23FF | $0400 | Nametable 0   
$2400-$27FF | $0400 | Nametable 1   
$2800-$2BFF | $0400 | Nametable 2   
$2C00-$2FFF | $0400 | Nametable 3   
$3000-$3EFF | $0F00 | Scratch RAM   
$3F00-$3F1F | $0020 | Palette RAM indexes   
$3F20-$3FFF | $00E0 | Mirrors of $3F00-$3F1F   
  
### Electrical schematic
    
    
                 ____
    PPU A13 -----\    \
                  )    >--- CHRRAM A13
    377 Q5 ------/____/
    
                 ____
    PPU A13 -----\    \
                  )    >--- CHRRAM A14
    377 Q6 ------/____/
    
    NON-4SCREEN
    PPU A13 ----[]--+
                    |
    GND --------[]--+---- CHRRAM /CE (Cartridge VRAM)
    4SCREEN MIRROR
    
    NON-4SCREEN
    PPU /A13 ---[]--+
                    |
    VCC --------[]--+---- CIRAM /CE (NES mainboard VRAM)
    4SCREEN MIRROR
    

## Mapper

[NES 2.0](NES_2_0.xhtml "NES 2.0") may specify the CHR-RAM size from 8 to 32 kB, but the default for iNES should to use 32 kB. 

If the Battery flag is set to 1, the mapper is flashable. This means no bus conflicts should be emulated. On the other hand, bus conflicts should be emulated if the battery flag is cleared. 

Mapper 30 is currently only using sub mapper 0. The other sub mappers are reserved for future use. 

Supported in FCEUX as of r3071, Supported in BizHawk as of r6322 

Supplemental mappers exist for flash carts as well: 

  * PowerPak: [forum post](https://forums.nesdev.org/viewtopic.php?p=236715#p236715) (limited flash save)
  * Everdrive N8: [forum post](https://forums.nesdev.org/viewtopic.php?p=235457#p235457) (no flash save)



The PowerPak flash save capability lacks the sector erase and chip erase capabilities, due to the underlying hardware being RAM at the time of write. As long as the game writes every byte it intends to read back, this is a compatible implementation. 

### Nametable Configuration

The nametable mirroring bits in byte 6 of the [iNES](INES.xhtml "INES") header select one of four configurations of nametables: 

  * `%....0..0` \- Horizontal mirroring (aka: vertical arrangement)
  * `%....0..1` \- Vertical mirroring (aka: horizontal arrangement)
  * `%....1..0` \- 1-Screen, switchable
  * `%....1..1` \- 4-Screen, cartridge VRAM



## See Also

  * [GTROM](GTROM.xhtml "GTROM") \- A very similar homebrew mapper, but with 32k PRG banking.



## References

  * [INES Mapper DischDocs](INES_Mapper_DischDocs.xhtml "INES Mapper DischDocs")



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml), [Mappers with flash save](Category_Mappers_with_flash_save.xhtml)
