# NES 2.0 Mapper 264

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_264) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_264)

NES 2.0 Mapper 264 is used for two fighting games from [Yoko Soft (aka Cony)](http://bootleggames.wikia.com/wiki/Cony_Soft): 

  * _Mortal Kombat II/V Pro_
  * _Master Fighter VI'_



It is similar but not quite identical to [several other Cony mappers](INES_Mapper_083.xhtml "INES Mapper 083"). Its UNIF board name is **UNL-YOKO**. 

## Contents

  * 1 Registers
    * 1.1 DIP switch(read)
    * 1.2 Scratch RAM (read/write)
    * 1.3 Outer PRG-ROM Bank ($8000)
    * 1.4 Mode ($8400)
    * 1.5 IRQ Acknowledge and Counter Low ($8800)
    * 1.6 IRQ Enable and Counter High ($8801)
    * 1.7 Inner PRG-ROM Bank ($8C0x)
    * 1.8 2 KiB CHR-ROM Banks ($8C1x)
  * 2 IRQ Operation
  * 3 Notes



## Registers

### DIP switch(read)

Mask: Probably $D400 
    
    
    $5000: Bit 7654 3210
               ---------
               .... ..DD
                      ++- DIP switch value
    

### Scratch RAM (read/write)

Mask: Probably $D403 
    
    
    $5400: Scratch RAM byte #0
    $5401: Scratch RAM byte #1
    $5402: Scratch RAM byte #2
    $5403: Scratch RAM byte #3
    

### Outer PRG-ROM Bank ($8000)

Mask: Probably $8C17 
    
    
    7654 3210
    ---------
    .... OPPp
         |||+- Select 16 KiB PRG-ROM bank at CPU $8000-$FFFF in PRG banking mode 0
         |++-- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF in PRG banking modes 0 and 1
         +---- Select 128 KiB PRG-ROM bank at CPU $8000-$FFFF in all PRG banking modes
    

### Mode ($8400)

Mask: Probably $8C17 
    
    
    7654 3210
    ---------
    ID.B B.MM
    || | | ++- Select nametable mirroring
    || | |      0: Vertical
    || | |      1: Horizontal
    || | |      2: One screen, page 0
    || | |      3: One screen, page 1
    || +-+---- Select PRG banking mode
    ||          0: 16 KiB UNROM-like:
    ||             CPU $8000-$BFFF: 16 KiB PRG-ROM bank selected by Outer Bank register ($8000)
    ||             CPU $C000-$FFFF: 16 KiB PRG-ROM bank fixed to last bank (UNROM-like)
    ||          1: 32 KiB:
    ||             CPU $8000-$FFFF: 32 KiB PRG-ROM bank selected by Outer Bank register ($8000) SHR 1
    ||          2: 8 KiB:
    ||             CPU $8000-$9FFF: 8 KiB PRG-ROM bank selected by Inner PRG-ROM Bank register 0 ($8C00)
    ||                              and Outer Bank register bit 3 SHL 1
    ||             CPU $A000-$BFFF: 8 KiB PRG-ROM bank selected by Inner PRG-ROM Bank register 1 ($8C01)
    ||                              and Outer Bank register bit 3 SHL 1
    ||             CPU $C000-$DFFF: 8 KiB PRG-ROM bank selected by Inner PRG-ROM Bank register 2 ($8C02)
    ||                              and Outer Bank register bit 3 SHL 1
    ||             CPU $E000-$FFFF: 8 KiB PRG-ROM bank selected by Inner PRG-ROM Bank register 3 ($8C03)
    ||                              and Outer Bank register bit 3 SHL 1
    ||          3: Same as 2.
    |+--------- IRQ counting mode
    |            0: Increase
    |            1: Decrease
    +---------- IRQ Enable latch
    

### IRQ Acknowledge and Counter Low ($8800)

A write directly changes the low eight bits of the 16-bit IRQ counter and acknowledges the IRQ. 

Mask: Probably $8C17 

### IRQ Enable and Counter High ($8801)

A write directly changes the high eight bits of the 16-bit IRQ counter, and copies bit 7 of the Mode register to the actual IRQ Enable register (which is not directly accessible). 

Mask: Probably $8C17 

### Inner PRG-ROM Bank ($8C0x)

These registers only have an effect in 8 KiB PRG-ROM banking mode (see $8400 bits 3 and 4). They are ORed with bit 8 SHL 1 of the Outer PRG-ROM bank register ($8000). 

Mask: Probably $8C17 
    
    
    7654 3210
    ---------
    .... PPPP
         ++++- Set Inner 8 KiB PRG-ROM bank
    
    8C00: Set Inner 8 KiB PRG-ROM bank at CPU $8000-$9FFF
    8C01: Set Inner 8 KiB PRG-ROM bank at CPU $A000-$BFFF
    8C02: Set Inner 8 KiB PRG-ROM bank at CPU $C000-$DFFF
    8C03: Set Inner 8 KiB PRG-ROM bank at CPU $E000-$FFFF
    

### 2 KiB CHR-ROM Banks ($8C1x)

Mask: Probably $8C17 
    
    
    8C10: Set 2 KiB CHR-ROM bank at PPU $0000-$07FF
    8C11: Set 2 KiB CHR-ROM bank at PPU $0800-$0FFF
    8C16: Set 2 KiB CHR-ROM bank at PPU $1000-$17FF
    8C17: Set 2 KiB CHR-ROM bank at PPU $1800-$1FFF
    

## IRQ Operation

If the IRQ counter is enabled (by writing a value to $8801 after setting bit 7 in the Mode register), and the counter is not zero, the counter is increased (Mode register bit 6 clear) or decreased (Mode register bit 6 set) on every M2 cycle. If it reaches zero, an IRQ is raised, and the IRQ counter is automatically disabled. 

## Notes

  * Although the DIP switch has two bits (the value read is ANDed with $03), both games only check whether it is zero/nonzero.
  * _Master Fighter VI'_ used the value read from DIP switch ($5200) to determine if game should be run in NTSC/Dendy (lowest bit equals 1) or PAL (lowest bit equals 0) mode
  * Power-up state is unknown, but game Master Fighter VI' executes from $E000 and first what it does is to change mode to 2 and continues to execute code from the same place. So either power-up mode is 0/2/3 plus $8C03 needs to be set to $0F (or maybe every bit of every register is initialized with ones)



Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
