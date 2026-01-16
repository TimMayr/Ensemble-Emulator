# NES 2.0 Mapper 561

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_561) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_561)

**NES 2.0 Mapper 561** denotes ROM image files extracted from disk images for the _Bung Super Game Doctor 2M_ and _4M_ [RAM cartridges](RAM_cartridge.xhtml "RAM cartridge"). They represent games whose [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Bung_Super_Game_Doctor_disks "Game Doctor/Magic Card FDS Format") denotes a Super Game Doctor disk (byte $0 bit 7 set, byte $7!=$00). A clone of the [Magic Card 2M](Super_Magic_Card.xhtml "Super Magic Card"), it differs in several aspects. 

As with [mapper 6](INES_Mapper_006.xhtml "INES Mapper 006"), the submapper field denotes the initial latch-based banking mode (0-7). 

## Contents

  * 1 Banks
  * 2 Trainer
  * 3 Banking Modes
    * 3.1 Latch-based modes
      * 3.1.1 0: UNROM
      * 3.1.2 1: UN1ROM+CHRSW
      * 3.1.3 2: UOROM
      * 3.1.4 3: Reverse UOROM+CHRSW
      * 3.1.5 4: GNROM
      * 3.1.6 5: CNROM-256
      * 3.1.7 6: Custom #1
      * 3.1.8 7: Custom #2
    * 3.2 4M PRG banking mode
  * 4 Interrupts
  * 5 Registers
    * 5.1 1M banking mode ($42FC-$42FF, write-only)
    * 5.2 4M PRG banking mode ($43FE-$43FF, write-only)
    * 5.3 FDS Write Data ($4024)
    * 5.4 FDS Control ($4025)
    * 5.5 IRQ Counter LSB/MSB ($4100/$4101, write-only)
  * 6 Notes



# Banks

  * CPU $6000-$7FFF: 8 KiB WRAM
  * CPU $8000-$FFFF: 32 KiB window into 256 KiB (Super Game Doctor 2M)/512 KiB (Super Game Doctor 4M) PRG memory
  * PPU $0000-$1FFF: 8 KiB window into 32 KiB CHR memory



# Trainer

Loadable to any RAM offset and of arbitrary size, Super Game Doctor trainers cannot use the [iNES format's trainer bit](INES.xhtml#Trainer "INES"). Instead, its data is included as NES 2.0 [Misc. ROM](NES_2_0.xhtml#Miscellaneous_ROM_Area "NES 2.0") data: 

  * Misc. ROM offset 0: Trainer load address (16-bit little endian)
  * Misc. ROM offset 2: JSR address of pre-reset-handler initialization routine, or $0000 if none (16-bit little endian)
  * Misc. ROM offset 4+: Trainer data



# Banking Modes

## Latch-based modes

  * A bus-conflict-free latch at CPU $8000-$FFFF switches PRG and CHR banks.
  * The 1M Mode Register selects the latch-based mode.
  * The latch is only active when PRG memory is write-protected.
  * There are no CNROM-128 or NROM-256 modes; BIOS runs such games in CNROM-256 mode by duplicating CHR banks.



### 0: [UNROM](UxROM.xhtml "INES Mapper 002")
    
    
    D~[.... .PPP]
             +++- CPU $8000-$BFFF: switchable 16 KiB PRG bank #0-#7 via PPP
                  CPU $C000-$FFFF: fixed 16 KiB PRG bank #7
                  PPU $0000-$1FFF: fixed 8 KiB CHR bank #0, writable
    

### 1: [UN1ROM](INES_Mapper_094.xhtml "INES Mapper 094")+CHRSW
    
    
    D~[..BB BBCC]
         || ||++- PPU $0000-$1FFF: switchable 8 KiB CHR bank #0-#3 via CC, writable
         ++-++--- CPU $8000-$BFFF: switchable 16 KiB PRG bank #0-#15 via PPP
                  CPU $C000-$FFFF: fixed 16 KiB PRG bank #7
    

UN1ROM+CHRSW is used for games that originally were [SKROM](SxROM.xhtml "SKROM") with 128 KiB PRG and 128 KiB CHR data. The original 128 KiB CHR data are stored in the second 128 KiB of PRG address space, with a trainer program copying the most-recentl CHR data into 32 KiB CHR memory. 

### 2: [UOROM](UxROM.xhtml "INES Mapper 002")
    
    
    D~[.... PPPP]
            ++++- CPU $8000-$BFFF: switchable 16 KiB PRG bank #0-#15 via PPPP
                  CPU $C000-$FFFF: fixed 16 KiB PRG bank #15
                  PPU $0000-$1FFF: fixed 8 KiB CHR bank #0, writable
    

With no other means of masking PRG addresses, UNROM vs. UOROM are explicitly differentiated. 

### 3: [Reverse UOROM](INES_Mapper_097.xhtml "INES Mapper 097")+CHRSW
    
    
    D~[..CC PPPP]
         || ++++- CPU $C000-$FFFF: switchable 16 KiB PRG bank #0-#15 via PPPP
         ||       CPU $8000-$7FFF: fixed 16 KiB PRG bank #15
         ++------ PPU $0000-$1FFF: switchable 8 KiB CHR bank #0-#3 via CC, writable
         
    

### 4: [GNROM](GxROM.xhtml "GNROM")
    
    
    D~[..PP ..CC]
         ||   ++- PPU $0000-$1FFF: switchable 8 KiB CHR bank #0-#3 via CC, write-protected
         ++------ CPU $8000-$FFFF: switchable 32 KiB PRG bank #0-#3 via PP
    

### 5: [CNROM-256](CNROM.xhtml "INES Mapper 003")
    
    
    D~[.... ..CC]
              ++- PPU $0000-$1FFF: switchable 8 KiB CHR bank #0-#3 via CC, write-protected
                  CPU $8000-$FFFF: fixed 32 KiB PRG bank #3
    

### 6: Custom #1
    
    
    D~[pppp PPPP]
       |||| ++++- CPU $8000-$9FFF: switchable 8 KiB PRG bank #0-15 via PPPP
       ++++------ CPU $A000-$BFFF: switchable 8 KiB PRG bank #0-15 via pppp
                  CPU $C000-$FFFF: fixed 16 bit PRG bank #7
                  PPU $0000-$1FFF: fixed 8 KiB CHR bank #0-3 at last-selected value in another mode
    

This mode is used by a few games that originally ran on the [Namcot 118](INES_Mapper_206.xhtml "INES Mapper 206") ASIC. 

### 7: Custom #2
    
    
    D~[ppp1 PPP0]
       |||| ++++- CPU $8000-$9FFF: switchable 8 KiB PRG bank #0-14 via PPP0
       ++++------ CPU $A000-$BFFF: switchable 8 KiB PRG bank #1-15 via ppp1
                  CPU $C000-$FFFF: fixed 16 bit PRG bank #7
                  PPU $0000-$1FFF: fixed 8 KiB CHR bank #0-3 at last-selected value in another mode
    

## 4M PRG banking mode

  * Four registers at $8000-$9FFF/$A000-$BFFF/$C000-$DFFF/$E000-$FFFF switch four 8 KiB PRG banks and the common 8 KiB CHR bank. They overlap with the latch at $8000-$FFFF and accept values even when the 4M PRG banking mode itself is not active.
  * 4M PRG banking mode is enabled by the 4M Mode Register. When active, it overrides the Latch-based mode in all aspects except CHR memory protection, which is still decided by the 1M Mode Register.
  * The entire 512 KiB can be reached if available.


    
    
    D~[PPPP PPCC]
       |||| ||++- PPU $0000-$1FFF: switchable 8 KiB CHR bank #0-#3 via CC in $8000-$FFFF
       ++++-++--- CPU $8000-$9FFF: switchable 8 KiB PRG Bank #0-31 via PPPPPP in $8000-$9FFF
                  CPU $A000-$BFFF: switchable 8 KiB PRG Bank #0-31 via PPPPPP in $A000-$BFFF
                  CPU $C000-$DFFF: switchable 8 KiB PRG Bank #0-31 via PPPPPP in $C000-$DFFF
                  CPU $E000-$FFFF: switchable 8 KiB PRG Bank #0-31 via PPPPPP in $E000-$FFFF
    

# Interrupts

  * The Magic Card 1M/2M and Bung Game Doctor 1M cannot generate an IRQ themselves. They did not relay the M2 signal to the FDS RAM Adapter, so its IRQ counter could not be used either.
  * The FDS RAM Adapter's Disk Data IRQ however is based on its own clock source. It can and was used by a few Super Game Doctor titles that were converted from Magic Card 1M/2M disks for frame timing.
  * The Super Game Doctor 2M/4M add a cycle-based IRQ counter of its own.



# Registers

## 1M banking mode ($42FC-$42FF, write-only)
    
    
    A~FEDC BA98 7654 3210  D~7654 3210
      -------------------    ---------
      0100 0010 1111 11bM    BBBM ....
                       |+----|||+------ Set nametable mirroring type
                       |     |||         0: One-screen, page 0
                       |     |||         1: One-screen, page 1
                       |     |||         2: Vertical
                       |     |||         3: Horizontal
                       +-----|||------- 0: PRG memory is writeable, latch is disabled
                             |||        1: PRG memory is write-protected, latch is enabled
                             +++------- Select latch-based banking mode
    

## 4M PRG banking mode ($43FE-$43FF, write-only)
    
    
    A~FEDC BA98 7654 3210  D~7654 3210
      -------------------    ---------
      0100 0011 1111 111M    .... ..CC
                        |           ++- switchable 8 KiB CHR bank #0-#3 via CC
                        |
                        +- 4M PRG banking mode select
                            0: Enable
                            1: Disable
    

The CC bits are mirrors of those in the latch at $8000-$FFFF. 

## FDS Write Data ($4024)

[This register](Family_Computer_Disk_System.xhtml#Write_data_register_.28.244024.29 "Family Computer Disk System") is not part of the RAM cartridge, but part of the FDS RAM adapter that originally attached to it. Because the original Game Doctor 1M had no IRQ counter of its own, a few games abuse the FDS Disk Data IRQ for frame timing and write any value to this register to acknowledge a pending IRQ. 

## FDS Control ($4025)

[This register](Family_Computer_Disk_System.xhtml#FDS_Control_.28.244025.29 "Family Computer Disk System") is not part of the RAM cartridge, but part of the FDS RAM adapter that originally attached to it. Because the original Game Doctor 1M had no IRQ counter of its own, a few games abuse the FDS Disk Data IRQ for frame timing. If bit 7 is set, the FDS RAM adapter will generate IRQs every 1,792 cycles of the 21.4772 MHz master clock, or after every 149+1/3 CPU cycles. 

## IRQ Counter LSB/MSB ($4100/$4101, write-only)

  * This upwards-counting signed 16-bit counter is increased on every M2 rise and raises an IRQ when flipping from $FFFF to $0000.
  * Counting and IRQ generation are enabled by writing a non-zero value to $4101, and are both disabled and an IRQ acknowledged by writing a zero value to $4101.



# Notes

  * The Venus Game Converter 2M is a fully-similar clone of the Super Game Doctor 2M.



Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
