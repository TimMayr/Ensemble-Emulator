# INES Mapper 018

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_018) | View [other pages](Special_AllPages.xhtml#INES_Mapper_018)

**SS 88006**

**Company** | Jaleco   
---|---  
**Games** | [15 in NesCartDB](https://nescartdb.com/search/advanced?ines=18)  
**Complexity** | ASIC   
**Boards** | JF-24, others   
**Pinout** | [Jaleco SS 88006 pinout](Jaleco_SS_88006_pinout.xhtml "Jaleco SS 88006 pinout")  
**PRG ROM capacity** | 512K   
**PRG ROM window** | 8K + 8K + 8K + 8K fixed   
**PRG RAM capacity** | 8K   
**PRG RAM window** | 8K   
**CHR capacity** | 256K   
**CHR window** | 1Kx8   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H, V, or 1, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | Yes   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 018  
  
**NESCartDB**

[iNES 018](https://nescartdb.com/search/advanced?ines=18)  
---  
  
**iNES Mapper 018** represents the [Jaleco SS 88006 mapper](Jaleco_SS_88006_pinout.xhtml "Jaleco SS 88006 pinout") used for _Magic John_ (Japanese version of _Totally Rad_) and about a dozen other games. 

This mapper is connected only to A12-A14, A0-A1, and D0-D3, so the PRG bank and CHR bank numbers are split over two sequential addresses. Bits 0-3 are in the lower address of a pair (e.g. $xxx0 or $xxx2), and bits 4-7 are in the next higher address. 

This mapper resembles a scrambled [VRC4](VRC2_and_VRC4.xhtml "VRC4"). 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Select 0 low($8000), high($8001)
    * 2.2 PRG Select 1 and 2 ($8002-$9FFD)
    * 2.3 PRG RAM protect ($9002)
    * 2.4 CHR Select 0 low($A000), high($A001)
    * 2.5 CHR Selects 1…7 ($A002-$DFFF)
    * 2.6 IRQ reload value ($E000-$EFFF)
    * 2.7 IRQ reload ($F000-$FFFC)
    * 2.8 IRQ counter size ($F001-$FFFD)
    * 2.9 Mirroring Control ($F002-$FFFE)
    * 2.10 Expansion sound ($F003-$FFFF)
  * 3 Disch's Notes
  * 4 See also



## Banks

  * CPU $6000-$7FFF: 8 KB PRG RAM bank (optional)
  * CPU $8000-$9FFF: 8 KB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KB switchable PRG ROM bank
  * CPU $C000-$DFFF: 8 KB switchable PRG ROM bank
  * CPU $E000-$FFFF: 8 KB PRG ROM bank, fixed to the last bank
  * PPU $0000-$03FF: 1 KiB switchable CHR bank
  * PPU $0400-$07FF: 1 KiB switchable CHR bank
  * PPU $0800-$0BFF: 1 KiB switchable CHR bank
  * PPU $0C00-$0FFF: 1 KiB switchable CHR bank
  * PPU $1000-$13FF: 1 KiB switchable CHR bank
  * PPU $1400-$17FF: 1 KiB switchable CHR bank
  * PPU $1800-$1BFF: 1 KiB switchable CHR bank
  * PPU $1C00-$1FFF: 1 KiB switchable CHR bank



## Registers

### PRG Select 0 low($8000), high($8001)
    
    
      $8000        $8001
    7  bit  0    7  bit  0
    ---------    ---------
    .... LLLL    .... ..HH
         ||||           ||
         ||||           ++- High 2 bits of 8 KiB PRG bank at CPU $8000
         ++++-------------- Low 4 bits
    

### PRG Select 1 and 2 ($8002-$9FFD)

The other two PRG bank selects continue similarly: 

Write to CPU address  | 8KB PRG bank affected   
---|---  
(low 4 bits) | (high 2 bits)   
$8002 | $8003 | $A000-$BFFF   
$9000 | $9001 | $C000-$DFFF   
  
### PRG RAM protect ($9002)
    
    
    7  bit  0
    ---------
    .... ..WR
           ||
           |+- PRG RAM chip enable (0: disable; 1: enable) (same as MMC3)
           +-- Write protection (0: deny writes; 1: allow writes) (opposite MMC3)
    

### CHR Select 0 low($A000), high($A001)
    
    
      $A000        $A001
    7  bit  0    7  bit  0
    ---------    ---------
    .... LLLL    .... HHHH
         ||||         ||||
         ||||         ++++- High 4 bits of 1 KiB CHR bank at PPU $0000
         ++++-------------- Low 4 bits
    

### CHR Selects 1…7 ($A002-$DFFF)

The other seven CHR bank selects continue similarly: 

Write to CPU address  | 1KB CHR bank affected   
---|---  
(low 4 bits) | (high 4 bits)   
$A002 | $A003 | $0400-$07FF   
$B000 | $B001 | $0800-$0BFF   
$B002 | $B003 | $0C00-$0FFF   
$C000 | $C001 | $1000-$13FF   
$C002 | $C003 | $1400-$17FF   
$D000 | $D001 | $1800-$1BFF   
$D002 | $D003 | $1C00-$1FFF   
  
### IRQ reload value ($E000-$EFFF)

The four registers here represent a 16-bit reload value, split into four four-bit numbers, least significant four bits first. 

### IRQ reload ($F000-$FFFC)

Any write to this register will immediately reload the IRQ counter from the above reload value and acknowledge the IRQ. 

### IRQ counter size ($F001-$FFFD)
    
    
    7  bit  0
    ---------
    .... FETC
         ||||
         |||+- 1: Enable counting 
         ||+-- 1: Don't propagate counter borrow to bit 12; instead assert IRQ
         |+--- 1: Don't propagate counter borrow to bit 8; instead assert IRQ
         +---- 1: Don't propagate counter borrow to bit 4; instead assert IRQ
    

F overrides E overrides T. If none are set, the counter is 16 bits wide. 

Writes to this register also acknowledge the IRQ. 

When enabled, the counter counts down. 

### Mirroring Control ($F002-$FFFE)
    
    
    7  bit  0
    ---------
    .... ..MM
           ||
           ++-- 0: Horizontal (A11)
                1: Vertical (A10)
                2: 1scA (Ground)
                3: 1scB (Vcc)
    

### Expansion sound ($F003-$FFFF)

Some games also have a µPD7755C or µPD7756C ADPCM sound IC. If so, 
    
    
    7  bit  0
    ---------
    NNNN NNRS
    |||| ||||
    |||| |||+-- 1: Assert (drive low) RESET on the ADPCM sound IC. 0: deassert (drive high)
    |||| ||+--- 1: Assert START on the ADPCM sound IC. 0: deassert
    ++++-++---- Which sample to start, if START becomes asserted while RESET is deasserted.
                
    

This is the same audio playback IC used in other Jaleco mappers, with the same timing constraints. RESET must be asserted for at least 34 cycles to be assured of stopping playback. START must be de-asserted before the sample finishes playback, or else a random sample (depending on the current contents of the data bus) will start playing. START will be ignored until 23 cycles after RESET is deasserted. 

## Disch's Notes
    
    
     Here are Disch's original notes:
     ========================
     =  Mapper 018          =
     ========================
     
     
     Example Games:
     --------------------------
     The Lord of King
     Magic John
     Pizza Pop
     
     
     Registers:
     ---------------------------
     
     Range,Mask:   $8000-FFFF, $F003
     
       $800x,$900x:  [.... PPPP]  PRG Regs
       $A00x-$D00x:  [.... CCCC]  CHR Regs
       $E00x:  [.... IIII]  IRQ Reload value
       $F000:  [.... ....]  IRQ Reset
       $F001:  [.... SSSE]  IRQ Control
          S = Size of IRQ counter
          E = Enable
     
       $F002:  [.... ..MM]  Mirroring
          %00 = Horz
          %01 = Vert
          %10 = 1ScA
          %11 = 1ScB
     
     
     CHR Setup:
     ---------------------------
     
     Only low 4 bits of written value significant [.... CCCC]
     
     2 regs combined to get an 8-bit page number
     
     $x000 or $x002 are the low 4 bits
     $x001 or $x003 are the high 4 bits
     
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +-------+-------+-------+-------+-------+-------+-------+-------+
         |$A000+1|$A002+3|$B000+1|$B002+3|$C000+1|$C002+3|$D000+1|$D002+3|
         +-------+-------+-------+-------+-------+-------+-------+-------+
     
     
     PRG Setup:
     ---------------------------
     
     Same as CHR, $x000 low, $x001 high
     
           $8000   $A000   $C000   $E000 
         +-------+-------+-------+-------+
         |$8000+1|$8002+3|$9000+1| { -1} |
         +-------+-------+-------+-------+
     
     
     IRQ:
     ---------------------------
     
     16-bit IRQ Reload value is set via regs $E00x.  $E000 sets the low 4 bits, $E003 sets the high 4 bits.
     
     When enabled, the IRQ counter counts down every CPU cycle.  When it wraps, an IRQ is generated.
     
     The 'S' bits in the control reg determine the size of the IRQ counter.  It can be 4, 8, 12, or 16 bits wide:
     
       %000 = 16 bits wide
       %001 = 12 bits wide
       %01x = 8 bits wide
       %1xx = 4 bits wide
     
     If the counter is less than 16 bits, the high bits are not altered by IRQ counter clocking; they retain their
     value.
     
     Example:  if the IRQ counter contains $1232, and is in 4-bit mode, it counts like so:
     
       $1232
       $1231
       $1230
       $123F  <--- IRQ here
       $123E
        ...
     
     
     Any write to the reset reg ($F000) will copy the 16-bit reload value into the IRQ counter (full 16 bits are
     copied, regardless of current 'S' value).
     
     Any write to $F000 or $F001 will acknowledge the IRQ.
    

  


## See also

  * [NES mapper list](http://www.romhacking.net/documents/362/) by Disch
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.
  * Sound IC D7756 [description](http://www.cpu-world.com/Support/7/77.html) and [datasheet](http://bootgod.dyndns.org:7777/downloads/UPD7755_UPD7756_UPD7757_UPD7758.pdf)
  * Sound IC BBS topics [762](http://forums.nesdev.org/viewtopic.php?t=762) and [4030](http://forums.nesdev.org/viewtopic.php?p=32572#p32572)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [Expansion audio](Category_Expansion_audio.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
