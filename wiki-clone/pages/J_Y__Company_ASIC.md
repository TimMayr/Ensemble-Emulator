# J.Y. Company ASIC

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/J.Y._Company_ASIC) | View [other pages](Special_AllPages.xhtml#J_Y__Company_ASIC)

晶太 (Jīngtài, also known as J.Y. Company)'s proprietary ASIC is used for their later single-game cartridges as well as for most of their multicarts. Due to its latter purpose, it allows flexibly switching between several PRG and CHR banking modes, supports ROM nametables, and permits specifying the CIRAM bank for each nametable separately (Extended Mirroring), automatic [MMC4](MMC4.xhtml "MMC4")-like CHR bankswitching, and a very flexible IRQ counter. It is part of several iNES/NES 2.0 mappers which differ in their PCB configuration: 

  * [INES Mapper 209](J_Y__Company_ASIC.xhtml "INES Mapper 209") is the standard implementation, used for most single-game cartridges. The outer PRG bank size is 512 KiB.
  * [INES Mapper 090](J_Y__Company_ASIC.xhtml "INES Mapper 090") inhibits, via jumper, ROM nametables and Extended Mirroring that would otherwise be enabled via register.
  * [INES Mapper 211](J_Y__Company_ASIC.xhtml "INES Mapper 211") supposedly has ROM nametables and Extended Mirroring always enabled regardless of register content. No such PCB exists in real hardware; the mapper was defined when the function of register $D001 bit 3 was not known, and is a mere duplicate of Mapper 209 with correct emulation.
  * [INES Mapper 035](J_Y__Company_ASIC.xhtml "INES Mapper 035") is like Mapper 209 but explicitly specifies 8 KiB of WRAM. In the presence of a valid NES 2.0 header, which has its own PRG-RAM field, Mapper 35 is an exact duplicate of Mappers 209.
  * [NES 2.0 Mapper 281](NES_2_0_Mapper_281.xhtml "NES 2.0 Mapper 281") and [NES 2.0 Mapper 282](NES_2_0_Mapper_282.xhtml "NES 2.0 Mapper 282") specify two PCBs with an outer PRG bank size of 256 KiB but different bit assignments in the Outer Bank register.
  * [NES 2.0 Mapper 295](NES_2_0_Mapper_295.xhtml "NES 2.0 Mapper 295") specifies a PCB with an outer PRG bank size of 128 KiB.
  * [NES 2.0 Mapper 358](NES_2_0_Mapper_358.xhtml "NES 2.0 Mapper 358") specifies a PCB with an outer PRG bank size of 512 KiB selected in part by the CHR banking mode.
  * In addition, these mappers also use the J.Y. Company ASIC: 
    * [NES 2.0 Mapper 386](NES_2_0_Mapper_386.xhtml "NES 2.0 Mapper 386")
    * [NES 2.0 Mapper 387](NES_2_0_Mapper_387.xhtml "NES 2.0 Mapper 387")
    * [NES 2.0 Mapper 388](NES_2_0_Mapper_388.xhtml "NES 2.0 Mapper 388")
    * [NES 2.0 Mapper 397](NES_2_0_Mapper_397.xhtml "NES 2.0 Mapper 397")



## Contents

  * 1 Miscellaneous Registers
    * 1.1 Jumper Register ($5000/$5400/$5C00, read)
    * 1.2 Hardware Multiplier ($5800-$5801, read/write)
    * 1.3 Accumulator and Test Register ($5802-$5803, read/write)
  * 2 Banking Registers
    * 2.1 PRG-ROM Bank Select ($8000-$8003, write)
    * 2.2 CHR Bank Select LSB ($9000-$9007), CHR Bank Select MSB ($A000-$A007)
    * 2.3 ROM Nametable/CIRAM Bank Select LSB ($B000-$B003)/MSB ($B004-$B007)
  * 3 IRQ Operation and Registers
    * 3.1 IRQ Disable/Enable ($C000)
    * 3.2 IRQ Mode Select ($C001)
    * 3.3 IRQ Unknown Mode Configuration ($C007)
  * 4 Mode Registers
    * 4.1 Mode Select ($D000)
    * 4.2 Mirroring Select ($D001)
    * 4.3 PPU Address Space Configuration ($D002)
    * 4.4 Outer Bank Select ($D003)
      * 4.4.1 iNES Mappers 90/209/211/35
    * 4.5 Example Games
    * 4.6 Notes



# Miscellaneous Registers

## Jumper Register ($5000/$5400/$5C00, read)
    
    
    Mask: $FFFF
    
    D~7654 3210
      ---------
      JJ.. ....
      ++-------- Jumper/solder pad configuration
    

Depending on the game, the jumper setting selects different title screens or game counts on multicarts. Apparently, the address of this register is fully decoded, as the SC-128 version of _Final Fight 3_ , which otherwise happily randomizes the masked-off address bits, keeps A0-A11 constantly at zero to read from this register. 

## Hardware Multiplier ($5800-$5801, read/write)
    
    
    Mask: $F803
    
    $5800 write: Write operand 1
    $5801 write: Write operand 2 and start multiplying
    $5800 read: Read result LSB
    $5801 read: Read result MSB
    

After writing the two operands to $5800 and $5801, the result can be read eight M2 cycles later; if the registers are read from earlier than that, intermediate results are obtained. 

## Accumulator and Test Register ($5802-$5803, read/write)
    
    
    Mask: $F803
    
    $5802 write: Increase accumulator by written value
    $5803 write: Reset accumulator to 0 and set test register to written value
    $5802 read: Read accumulator value
    $5803 read: Read test register
    

# Banking Registers

## PRG-ROM Bank Select ($8000-$8003, write)

Mask: $F803. This means that writes are ignored if address bit 11 is set (i.e. these registers extend only through addressing range $8000-87FF). 
    
    
    D~7654 3210
      ---------
      .PPP PPPP
       +++-++++- Select PRG-ROM bank number
    

In 8 KiB PRG banking mode: 

  * $8000: Select 8 KiB PRG-ROM bank at CPU $8000-$9FFF
  * $8001: Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF
  * $8002: Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF
  * $8003: Select 8 KiB PRG-ROM bank at CPU $E000-$FFFF if switchable last bank is selected via $D000 bit 2



In 16 KiB PRG banking mode: 

  * $8001: Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF
  * $8003: Select 16 KiB PRG-ROM bank at CPU $C000-$FFFF if switchable last bank is selected via $D000 bit 2



In 32 KiB PRG banking mode: 

  * $8003: Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF if switchable last bank is selected via $D000 bit 2



If $D000 bit 2 is clear, the 8/16/32 KiB bank at $E000/$C000/$8000-$FFFF is hard-wired to the last bank within the outer bank. Furthermore, if register $D000 bit 7 is set, $8003 also selects an 8 KiB PRG-ROM bank at CPU $6000-$7FFF, which is 

  * $8003 in 8 KiB banking mode;
  * $8003 SHL 1 OR 1 in 16 KiB banking mode;
  * $8003 SHL 2 OR 3 in 32 KiB banking mode.



The PRG-ROM bank selected via $8000-$8003 is always masked to the outer bank size (512/256/128 KiB, depending on PCB/mapper number). Accessing more than that requires manipulating the Outer Bank register at $D003. 

## CHR Bank Select LSB ($9000-$9007), CHR Bank Select MSB ($A000-$A007)

Mask: $F807. This means that writes are ignored if address bit 11 is set (i.e. these registers extend only through addressing ranges $9000-97FF and $A000-A7FF). 

In 8 KiB CHR banking mode: 

  * $9000/$A000: Select 8 KiB CHR bank at PPU $0000-$1FFF



In 4 KiB CHR banking mode with MMC4 mode off: 

  * $9000/$A000: Select 4 KiB CHR bank at PPU $0000-$0FFF
  * $9004/$A004: Select 4 KiB CHR bank at PPU $1000-$1FFF



In 4 KiB CHR banking mode with MMC4 mode on: 

  * $9000/$A000 (LSB/MSB): Select 4 KiB CHR bank at PPU $0000-$0FFF when latch =0
  * $9002/$A002 (LSB/MSB): Select 4 KiB CHR bank at PPU $0000-$0FFF when latch =1
  * $9004/$A004 (LSB/MSB): Select 4 KiB CHR bank at PPU $1000-$1FFF when latch =0
  * $9006/$A006 (LSB/MSB): Select 4 KiB CHR bank at PPU $1000-$1FFF when latch =1



In 2 KiB CHR banking mode: 

  * $9000/$A000 (LSB/MSB): Select 2 KiB CHR bank at PPU $0000-$07FF
  * $9002/$A002 (LSB/MSB): Select 2 KiB CHR bank at PPU $0800-$0FFF
  * $9004/$A004 (LSB/MSB): Select 2 KiB CHR bank at PPU $1000-$17FF
  * $9006/$A006 (LSB/MSB): Select 2 KiB CHR bank at PPU $1800-$1FFF



In 1 KiB CHR banking mode: 

  * $9000/$A000 (LSB/MSB): Select 1 KiB CHR bank at PPU $0000-$03FF
  * $9001/$A001 (LSB/MSB): Select 1 KiB CHR bank at PPU $0400-$07FF
  * $9002/$A002 (LSB/MSB): Select 1 KiB CHR bank at PPU $0800-$0BFF
  * $9003/$A003 (LSB/MSB): Select 1 KiB CHR bank at PPU $0C00-$0FFF
  * $9004/$A004 (LSB/MSB): Select 1 KiB CHR bank at PPU $1000-$13FF
  * $9005/$A005 (LSB/MSB): Select 1 KiB CHR bank at PPU $1400-$17FF
  * $9006/$A006 (LSB/MSB): Select 1 KiB CHR bank at PPU $1800-$1BFF
  * $9007/$A007 (LSB/MSB): Select 1 KiB CHR bank at PPU $1C00-$1FFF



The CHR bank selected via $9000-$A007 is always masked to the outer bank size (512/256/128 KiB, depending on PCB/mapper number and $D003 value). Accessing more than that requires manipulating the Outer Bank register at $D003. If [MMC4](MMC4.xhtml "MMC4")-like bankswitching mode is enabled via $D003 bit 7, the latch is set to 0 when the PPU reads from $0FD8-$0FDF/$1FD8-$1FDF and to 1 when the PPU reads from $0FE8-$0FEF/$1FE8-$1FEF. 

## ROM Nametable/CIRAM Bank Select LSB ($B000-$B003)/MSB ($B004-$B007)

Mask: $F807. This means that writes are ignored if address bit 11 is set (i.e. these registers extend only through addressing ranges $B000-B7FF). 

These registers have no function if neither ROM nametables ($D000 bit 5) nor Extended Mirroring ($D001 bit 3) are enabled, or their function is supressed via a jumper on some PCB variants that are denoted by [iNES Mapper 090](J_Y__Company_ASIC.xhtml "INES Mapper 090"). 

If Extended Mirroring is enabled ($D001 bit 3 set): 

  * $B000 bit 0: Select 1 KiB CIRAM bank at PPU $2000-$23FF
  * $B001 bit 0: Select 1 KiB CIRAM bank at PPU $2400-$27FF
  * $B002 bit 0: Select 1 KiB CIRAM bank at PPU $2800-$2BFF
  * $B003 bit 0: Select 1 KiB CIRAM bank at PPU $2C00-$2FFF



If ROM nametables are enabled ($D000 bit 5 set) for all nametables ($D000 bit 6 set): 

  * $B000/$B004 (LSB/MSB): Select 1 KiB CHR bank at PPU $2000-$23FF
  * $B001/$B005 (LSB/MSB): Select 1 KiB CHR bank at PPU $2400-$27FF
  * $B002/$B006 (LSB/MSB): Select 1 KiB CHR bank at PPU $2800-$2BFF
  * $B003/$B007 (LSB/MSB): Select 1 KiB CHR bank at PPU $2C00-$2FFF



If ROM nametables are enabled ($D000 bit 5 set) for selected nametables ($D000 bit 6 clear): 

  * $B000 bit 0: Select 1 KiB CIRAM bank at PPU $2000-$23FF if $B000 bit 7 matches $D002 bit 7
  * $B001 bit 0: Select 1 KiB CIRAM bank at PPU $2400-$27FF if $B001 bit 7 matches $D002 bit 7
  * $B002 bit 0: Select 1 KiB CIRAM bank at PPU $2800-$2BFF if $B002 bit 7 matches $D002 bit 7
  * $B003 bit 0: Select 1 KiB CIRAM bank at PPU $2C00-$2FFF if $B003 bit 7 matches $D002 bit 7
  * $B000/$B004 (LSB/MSB): Select 1 KiB CHR bank at PPU $2000-$23FF if $B000 bit 7 differs from $D002 bit 7
  * $B001/$B005 (LSB/MSB): Select 1 KiB CHR bank at PPU $2400-$27FF if $B001 bit 7 differs from $D002 bit 7
  * $B002/$B006 (LSB/MSB): Select 1 KiB CHR bank at PPU $2800-$2BFF if $B002 bit 7 differs from $D002 bit 7
  * $B003/$B007 (LSB/MSB): Select 1 KiB CHR bank at PPU $2C00-$2FFF if $B003 bit 7 differs from $D002 bit 7



This means that enabling ROM nametables via $D005 bit 5 and all $B00x registers having bit 7 match $D002 bit 7 effectively duplicates the functionality of Extended Mirroring ($D001 bit 3). 

# IRQ Operation and Registers

  1. When counting is enabled and the counter is clocked from the source selected by $C001 bits 0-1, the prescaler is increased/decreased (depending on $C001 bits 6-7).
  2. If the AND-masked prescaler (according to $C001 bit 2) wraps, meaning it is zero after increasing or matches the prescaler mask after decreasing, the IRQ counter is increased/decreased (likewise depending on $C001 bits 6-7).
  3. If the IRQ counter wraps, meaning it is zero after increasing or $FF after decreasing, an IRQ is generated. Disabling the IRQ via registers $C000 or $C002 acknowledges it.


    
    
    Mask: $F007. Writes are accepted even if address bit 11 is set.
    $C000: IRQ Disable/Enable
    $C001: IRQ Mode/Flags
    $C002: IRQ Disable
    $C003: IRQ Enable
    $C004: Set prescaler value (*)
    $C005: Set counter value (*)
    $C006: Set XOR value
    $C007: Configure unknown mode
    (*) The written value is XORed with the content of register $C006 before being stored.
    

"Disabling" acknowledges a pending IRQ, inhibits counting, and resets the prescaler to zero. "Enabling" means that counting is no longer inhibited, and IRQs will be generated once prescaler and counter wrap. 

## IRQ Disable/Enable ($C000)
    
    
    D~7654 3210
      ---------
      .... ...I
              +- Disable (0)/Enable (1) IRQ generation.         
    

## IRQ Mode Select ($C001)
    
    
    D~7654 3210
      ---------
      DD.. FPSS
      ||   ||++- Select IRQ source
      ||   ||     0: CPU M2 rise
      ||   ||     1: PPU A12 rise (unfiltered, eight per scanline)
      ||   ||     2: PPU reads (170 per scanline)
      ||   ||     3: CPU writes
      ||   |+--- Select prescaler mask
      ||   |      0: $FF
      ||   |      1: $07
      ||   +---- Disable (0)/Enable (1) $C007 mode.
      ++-------- Select counting direction
                  0: Counting disabled
                  1: Increase
                  2: Decrease
                  3: Counting disabled
    

## IRQ Unknown Mode Configuration ($C007)

Mask: $F007. Writes are accepted even if address bit 11 is set. 

This register is used together with register $C001 bit 3. Its functionality is not known, and no known game uses it. 

# Mode Registers

## Mode Select ($D000)

Mask: $F803. This means that writes are ignored if address bit 11 is set (i.e. these registers extend only through addressing ranges $D000-D7FF). 
    
    
    D~7654 3210
      ---------
      6GRC CLPP
      |||| ||++- Select PRG-ROM banking mode
      |||| ||     0: 32 KiB banking
      |||| ||     1: 16 KiB banking
      |||| ||     2: 8 KiB banking
      |||| ||     3: 8 KiB banking, but with bank numbers bits 0-6 reversed
      |||| |+--- Select bank $8000-(32 KiB)/$C000-(16 KiB)/$E000-(8KiB)-$FFFF
      |||| |      0: Hard-wired to last bank
      |||| |      1: Switchable via $8003
      |||+-+---- Select CHR banking mode
      |||         0: 8 KiB banking
      |||         1: 4 KiB banking
      |||         2: 2 KiB banking
      |||         3: 1 KiB banking
      ||+------- Select ROM nametable status
      ||          0: ROM nametables disabled
      ||          1: ROM nametables enabled (for all or some nametables, depending on bit 6)
      |+-------- Select ROM nametable selection method if R=1, ignored otherwise
      |           0: ROM nametables selected separately by $B00x bit 7 XOR $D002 bit 7
      |           1: ROM nametables enabled globally for all nametables
      +--------- Select CPU $6000-$7FFF mapping
                  0: Map WRAM, if present, otherwise open bus
                  1: Map 8 KiB PRG-ROM bank selected via $8003
                    (appropriately shifted and right-filled with binary 1 in 16/32 KiB PRG modes)
    

## Mirroring Select ($D001)

Mask: $F803. This means that writes are ignored if address bit 11 is set (i.e. these registers extend only through addressing ranges $D000-D7FF). 
    
    
    D~7654 3210
      ---------
      .... E.MM
           | ++- Select nametable mirroring type if E=0 and $D000 bit 5=0
           |      0: Vertical
           |      1: Horizontal
           |      2: One-screen, page 0
           |      3: One-screen, page 1
           +---- Select Extended Mirroring
                  0: Disabled, use MM bits
                  1: Enabled, ignore MM bits, use $B000-$B003 bit 0
    

## PPU Address Space Configuration ($D002)

Mask: $F803. This means that writes are ignored if address bit 11 is set (i.e. these registers extend only through addressing ranges $D000-D7FF). 
    
    
    D~7654 3210
      ---------
      RW.. ....
      |+-------- CHR write-enable
      |           0: disabled, CHR memory is write-protected
      |           1: enabled, CHR memory is write-enabled
      +--------- Define ROM nametable selection. Functional only if ROM nametables are
                 generally enabled, but individually selected ($D000 bit 5 set and
                 $D000 bit 6 clear)
                  0: $B00x bit 7=0 selects CIRAM, $B00x bit 7=1 selects ROM nametable
                  1: $B00x bit 7=1 selects CIRAM, $B00x bit 7=0 selects ROM nametable
                  Note: x=0..3
    

Bit 6 only matters when using CHR-RAM. 

## Outer Bank Select ($D003)

Mask: $F803. This means that writes are ignored if address bit 11 is set (i.e. these registers extend only through addressing ranges $D000-D7FF). 

The exact meaning of this register depends on the PCB, specified by the mapper number. 
    
    
    D~7654 3210
      ---------
      4.MM MMMM
      | ++-++++- PCB/Mapper-specific
      +--------- Select [MMC4](MMC4.xhtml "MMC4")-like automatic CHR-ROM bankswitching mode
                  0: Disable
                  1: Enable (only meaningful in 4 KiB CHR-ROM banking mode)
    

### iNES Mappers 90/209/211/35

Mask: $F803. This means that writes are ignored if address bit 11 is set. 
    
    
    D~7654 3210
      ---------
      4.Lc cPPC
      | || |||+- If L=0: Select 256 KiB outer CHR-ROM bank (CHR A18), ignored if L=1
      | || |++-- Select 512 KiB outer PRG-ROM bank (PRG A19-A20)
      | |+-+---- Select 512 KiB outer CHR-ROM bank (CHR A19-A20)
      | +------- Select outer CHR-ROM bank size
      |           0: Mask $900x/$A00x to 256 KiB, use C
      |           1: Mask $900x/$A00x to 512 KiB, ignore C
      +--------- Select [MMC4](MMC4.xhtml "MMC4")-like automatic CHR-ROM bankswitching mode
                  0: Disable
                  1: Enable (only meaningful in 4 KiB CHR-ROM banking mode)
    

## Example Games

  * Mapper 209 is the standard implementation. 
    * _Mighty Morphin' Power Rangers III_ , alternative title _Mighty Morphin' Power Rangers: The Movie IV_ (uses MMC4-like mode for displaying title screen)
    * _Mike Tyson's Punch-Out!!_
    * _Shin Samurai Spirits 2_


  * Mapper 90 inhibits, via jumper, ROM nametables and Extended Mirroring that would otherwise be enabled via register. 
    * _1998 Super 45-in-1_ (JY-120 (A))
    * _Aladdin_ (Hummer Team's port of Capcom's SNES game), alternative titles _Aladdin II_ /_III_ and _Popeye II: Travels in Persia_
    * _Contra Spirits_
    * _Final Fight 3_
    * _Mickey Mania 7_ , alternative title _China Rabbit Baby_
    * _Mortal Kombat II Special_ , also known as _Mortal Kombat 3 Special 56 Pepoles_ (sic)
    * _Super Aladdin: The Return of Jaffar_
    * _Super Mario World_
    * _Tekken 2_ , alternative title _Battle Arena Toushinden_


  * Mapper 211 supposedly has ROM nametables and Extended Mirroring always enabled regardless of register content. No such PCB exists in real hardware; the mapper was defined when the function of register [$D001 bit 3](J_Y__Company_ASIC.xhtml#Mirroring_Select_.28.24D001.29 "J.Y. Company ASIC") was not known, and is a mere duplicate of Mapper 209 with correct emulation. 
    * _Donkey Kong Country 4_ , alternative title _The Jungle Book 2_
    * _Tiny Toon Adventures 6_ , alternative title _Porky Pig & Daffy Duck_


  * Mapper 35 is a duplicate of Mapper 209 which explicitly specifies 8 KiB of WRAM. In the presence of a valid NES 2.0 header, which has its own PRG-RAM field, Mapper 35 is a duplicate of Mappers 209. 
    * _Warioland II_ (hack of _Hoshi no Kirby: Yume no Izumi no Monogatari_



## Notes

  * This chip is only known to exist as a glob. The pinout can be found here from krzysiobal: <https://forums.nesdev.org/viewtopic.php?t=19017>
  * Some boards do not connect CPU A11, changing the $F800 address masks to $F000, and effectively making the hardware multiplier at $5800 inaccessible.
  * See also: <https://forums.nesdev.org/viewtopic.php?t=15333>



Categories: [Mappers using J.Y. Company ASIC](Category_Mappers_using_J_Y__Company_ASIC.xhtml)
