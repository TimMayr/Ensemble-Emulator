# Sunsoft FME-7

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sunsoft_FME-7) | View [other pages](Special_AllPages.xhtml#Sunsoft_FME_7)

**FME-7**   
**Sunsoft 5A/5B**

**Company** | Sunsoft   
---|---  
**Games** | [9 in NesCartDB](https://nescartdb.com/search/advanced?ines=69)  
**Complexity** | ASIC   
**Boards** | JLROM, JSROM,  
NES-BTR, others   
**Pinout** | [Sunsoft 5 pinout](Sunsoft_5_pinout.xhtml "Sunsoft 5 pinout")  
**PRG ROM capacity** | 512K (FME-7)  
256K (5A/5B)   
**PRG ROM window** | 8K×4 + 8K fixed   
**PRG RAM capacity** | 512K (FME-7)   
**PRG RAM window** | 8K   
**CHR capacity** | 256K   
**CHR window** | 1Kx8   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H, V, or 1, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | CPU cycle counter   
**Audio** | [5B only](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio")  
**iNES[mappers](Mapper.xhtml "Mapper")** | [069](Sunsoft_FME_7.xhtml "INES Mapper 069")  
  
The Sunsoft FME-7 is a mapper IC used by Sunsoft in several of its games. It is nearly identical to the **Sunsoft 5A** and **Sunsoft 5B** mapper chips used only in Famicom games, with the 5B notably having expansion audio (see [Sunsoft 5B audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio")). 

The FME-7, 5A and 5B are grouped together as **iNES Mapper 69**. 

Both the Sunsoft 5B and FME-7 exist as a 44 pin TQFP chip: [diagram](Sunsoft_5_pinout.xhtml "Sunsoft 5 pinout")

In Europe, boards using the FME-7 were labeled as [JSROM and JLROM](JxROM.xhtml "JxROM"). The FME-7 mapper was used in only one game released in the US, _Batman: Return of the Joker_. Many Japanese releases by Sunsoft used the FME-7: _Gimmick!_ , _Hebereke_ , _Gremlins 2_ (but not in the US version), _Barcode World_ , and others. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Command Register ($8000-$9FFF)
    * 2.2 Parameter Register ($A000-$BFFF)
  * 3 Commands
    * 3.1 CHR Bank 0-7 ($0-7)
    * 3.2 PRG Bank 0 ($8)
    * 3.3 PRG Bank 1-3 ($9-B)
    * 3.4 Name Table Mirroring ($C)
    * 3.5 IRQ Control ($D)
    * 3.6 IRQ Counter Low Byte ($E)
    * 3.7 IRQ Counter High Byte ($F)
  * 4 IRQ Operation
    * 4.1 How to Use the IRQ Generator
  * 5 References
  * 6 See also



## Banks

  * CPU $6000-$7FFF: 8 KB Bankable PRG ROM or PRG RAM
  * CPU $8000-$9FFF: 8 KB Bankable PRG ROM
  * CPU $A000-$BFFF: 8 KB Bankable PRG ROM
  * CPU $C000-$DFFF: 8 KB Bankable PRG ROM
  * CPU $E000-$FFFF: 8 KB PRG ROM, fixed to the last bank of ROM
  * PPU $0000-$03FF: 1 KB Bankable CHR ROM
  * PPU $0400-$07FF: 1 KB Bankable CHR ROM
  * PPU $0800-$0BFF: 1 KB Bankable CHR ROM
  * PPU $0C00-$0FFF: 1 KB Bankable CHR ROM
  * PPU $1000-$13FF: 1 KB Bankable CHR ROM
  * PPU $1400-$17FF: 1 KB Bankable CHR ROM
  * PPU $1800-$1BFF: 1 KB Bankable CHR ROM
  * PPU $1C00-$1FFF: 1 KB Bankable CHR ROM



## Registers

Configuration of the FME-7 is accomplished by first writing the command number to the Command Register at $8000-9FFF, then writing the command's parameter byte to the Parameter Register at $A000-BFFF. 

There are 16 commands: 

  * **$0-7** control CHR banking
  * **$8-B** control PRG banking
  * **$C** controls nametable mirroring
  * **$D-F** controls IRQ



On the 5B variant, there are two additional registers at $C000-DFFF and $E000-FFFF that control the audio expansion. See: [Sunsoft 5B audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio")

### Command Register ($8000-$9FFF)
    
    
    7  bit  0
    ---- ----
    .... CCCC
         ||||
         ++++- The command number to invoke when writing to the Parameter Register
    

### Parameter Register ($A000-$BFFF)
    
    
    7  bit  0
    ---- ----
    PPPP PPPP
    |||| ||||
    ++++-++++- The parameter to use for this command. Writing to this register invokes the command in the Command Register.
    

## Commands

### CHR Bank 0-7 ($0-7)
    
    
    7  bit  0
    ---- ----
    BBBB BBBB
    |||| ||||
    ++++-++++- The bank number to select for the specified bank.
    
    
    
    Bank $0 - PPU $0000-$03FF
    Bank $1 - PPU $0400-$07FF
    Bank $2 - PPU $0800-$0BFF
    Bank $3 - PPU $0C00-$0FFF
    Bank $4 - PPU $1000-$13FF
    Bank $5 - PPU $1400-$17FF
    Bank $6 - PPU $1800-$1BFF
    Bank $7 - PPU $1C00-$1FFF
    

### PRG Bank 0 ($8)
    
    
    7  bit  0
    ---- ----
    ERbB BBBB
    |||| ||||
    ||++-++++- The bank number to select at CPU $6000 - $7FFF
    |+------- RAM / ROM Select Bit
    |         0 = PRG ROM
    |         1 = PRG RAM
    +-------- RAM Enable Bit ([6264](6264_static_RAM.xhtml "6264") +CE line)
              0 = PRG RAM Disabled
              1 = PRG RAM Enabled
    

The FME-7 has up to 6 bits for PRG banking (512 KiB), though this was never used in a game. The 5A and 5B, however, support only 5 (256 KiB)—hence the lowercase 'b' above. The extra address line is instead an audio expansion line, or unused. 

It is [confirmed](http://forums.nesdev.org/viewtopic.php?f=9&t=12467) that the FME-7 outputs the bank number during accesses to $6000-$7FFF even if RAM is enabled. Though Sunsoft never took advantage of this, it would allow making a cartridge that bank switches up to 256 KiB of PRG RAM. The FME-7 mapper in Loopy's [PowerPak](PowerPak.xhtml "PowerPak") mappers, for example, supports 32 KiB. 

Open bus occurs if the RAM / ROM Select Bit is 1 (RAM selected), but the RAM Enable Bit is 0 (disabled), i.e. any value in the range $40-$7F. This is a limited form of WRAM write protection on power-up. 

NOTE: the enable bit is NOT functional with 2048×8, 32768×8, and 524288×8 RAMs, because those RAMs don't have a +CE input. 

There is a [tentative report](http://forums.nesdev.org/viewtopic.php?p=105129#p105129) that [not all games honor some or any of the bits in this register](http://forums.nesdev.org/viewtopic.php?p=105193#p105193). Corroboration is needed before any action is taken. 

### PRG Bank 1-3 ($9-B)
    
    
    7  bit  0
    ---- ----
    ..bB BBBB
      || ||||
      ++-++++- The bank number to select for the specified bank.
    
    
    
    Bank $9 - CPU $8000-$9FFF
    Bank $A - CPU $A000-$BFFF
    Bank $B - CPU $C000-$DFFF
    

The FME-7 has up to 6 bits for PRG banking, but the 5A and 5B support only 5. 

### Name Table Mirroring ($C)

These values are the same as [MMC1](MMC1.xhtml "MMC1") mirroring modes with the MSB inverted. 
    
    
    7  bit  0
    ---- ----
    .... ..MM
           ||
           ++- Mirroring Mode
                0 = Vertical
                1 = Horizontal
                2 = One Screen Mirroring from $2000 ("1ScA")
                3 = One Screen Mirroring from $2400 ("1ScB")
    

### IRQ Control ($D)
    
    
    7  bit  0
    ---- ----
    C... ...T
    |       |
    |       +- IRQ Enable
    |           0 = Do not generate IRQs
    |           1 = Do generate IRQs
    +-------- IRQ Counter Enable
                0 = Disable Counter Decrement
                1 = Enable Counter Decrement
    

All writes to this register acknowledge an active IRQ.[1] It is not yet known what will happen if this register is written to at the same time as an IRQ would have been generated. 

### IRQ Counter Low Byte ($E)
    
    
    7  bit  0
    ---- ----
    LLLL LLLL
    |||| ||||
    ++++-++++- The low eight bits of the IRQ counter
    

### IRQ Counter High Byte ($F)
    
    
    7  bit  0
    ---- ----
    HHHH HHHH
    |||| ||||
    ++++-++++- The high eight bits of the IRQ counter
    

Writes to the IRQ counter registers directly set the lower or upper eight bits of the counter. Unlike on MMC3, there is no separate reload latch. 

## IRQ Operation

The IRQ feature of FME-7 is a CPU cycle counting IRQ generator. When enabled the 16-bit IRQ counter is decremented once per CPU cycle. When the IRQ counter is decremented from $0000 to $FFFF an IRQ is generated. The IRQ line is held low until it is acknowledged. 

### How to Use the IRQ Generator

  1. Set the counter to the desired number of cycles minus one.
  2. Enable the IRQ generator by turning on both the IRQ Enable and IRQ Counter Enable flags of the IRQ Control command.
  3. Within the IRQ handler, write to the IRQ Control command to acknowledge the IRQ.
  4. Optional: Go back to Step 1 for the next IRQ.



## References

  1. ↑ Test performed in 2015 by Oliveira using [IRQ acknowledge test ROM on NESdev BBS](http://forums.nesdev.org/viewtopic.php?p=142243#p142243)



## See also

  * [Sunsoft Mapper](http://nesdev.org/sunsoft.txt) by goroh, translated by Sgt. Bowhack.
  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with large PRG RAM](Category_Mappers_with_large_PRG_RAM.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
