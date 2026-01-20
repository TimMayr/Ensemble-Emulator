# NES 2.0 Mapper 562

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_562) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_562)

**NES 2.0 Mapper 562** denotes ROM image files extracted from disk images for the _Venus Turbo Game Doctor 4+_ , _6+_ and _6M_ [RAM cartridges](RAM_cartridge.xhtml "RAM cartridge"). They represent games whose [Doctor Header file](Game_Doctor_Magic_Card_FDS_Format.xhtml#Venus_Turbo_Game_Doctor_disks "Game Doctor/Magic Card FDS Format") denotes a Turbo Game Doctor disk (byte $0 bit 7 clear). A clone of the [Magic Card 2M](Super_Magic_Card.xhtml "Super Magic Card"), it differs in several aspects. 

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
    * 3.2 2M PRG banking mode
    * 3.3 4M PRG banking mode
    * 3.4 1 KiB CHR banking mode
  * 4 Interrupts
  * 5 Registers
    * 5.1 1M banking mode ($42FC-$42FF, write-only)
    * 5.2 2M PRG banking mode ($43FE-$43FF, write-only)
    * 5.3 Turbo Game Doctor mode ($4411, read/write)
    * 5.4 IRQ Counter Target LSB/MSB ($440D/$440C, write-only)
    * 5.5 IRQ Counter Value LSB/MSB ($440D/$440C, read-only)
    * 5.6 Current CHR Bank ($4420, read-only)



# Banks

  * CPU $4800-$4FFF: BIOS offsets $0800-$0FFF. Some locations checked by games to lock out competing RAM cartridges from Bung and FFE.
  * CPU $6000-$7FFF: 8 KiB WRAM
  * CPU $8000-$FFFF: 32 KiB window into 256 KiB (Turbo Game Doctor 6+)/512 KiB (Turbo Game Doctor 4+/6M) PRG memory
  * PPU $0000-$1FFF: 8 KiB window into 32 KiB (Turbo Game Doctor 4+)/256 KiB (Turbo Game Doctor 6+/6M) CHR memory



# Trainer

Loadable to any RAM offset and of arbitrary size, Turbo Game Doctor trainers cannot use the [iNES format's trainer bit](INES.xhtml#Trainer "INES"). Instead, its data is included as NES 2.0 [Misc. ROM](NES_2_0.xhtml#Miscellaneous_ROM_Area "NES 2.0") data: 

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
    

## 2M PRG banking mode

  * Four registers at $8000-$9FFF/$A000-$BFFF/$C000-$DFFF/$E000-$FFFF switch four 8 KiB PRG banks and the common 8 KiB CHR bank. They overlap with the latch at $8000-$FFFF and accept values even when the 2M PRG banking mode itself is not active.
  * 2M PRG banking mode is enabled by the 2M Mode Register. When active, it overrides the Latch-based mode in all aspects except CHR memory protection, which is still decided by the 1M Mode Register.
  * Only 256 KiB can be reached.
  * Only the four lowest address bits can be switched on an 8 KiB level; the 128 KiB bit is switched for the entire $8000-$FFFF range via $43FE.


    
    
    D~[..PP PPCC]
         || ||++- PPU $0000-$1FFF: switchable 8 KiB CHR bank #0-#3 via CC in $8000-$FFFF
         ++-++--- CPU $8000-$9FFF: switchable 8 KiB PRG Bank #0-31 via PPPP in $8000-$9FFF (LSB) and p (MSB) in $43FE
                  CPU $A000-$BFFF: switchable 8 KiB PRG Bank #0-31 via PPPP in $A000-$BFFF (LSB) and p (MSB) in $43FE
                  CPU $C000-$DFFF: switchable 8 KiB PRG Bank #0-31 via PPPP in $C000-$DFFF (LSB) and p (MSB) in $43FE
                  CPU $E000-$FFFF: switchable 8 KiB PRG Bank #0-31 via PPPP in $E000-$FFFF (LSB) and p (MSB) in $43FE
    

## 4M PRG banking mode

  * Identical in operation to 2M PRG Banking Mode, except that all six bits from the four registers at $8000-$9FFF/$A000-$BFFF/$C000-$DFFF/$E000-$FFFF are used.
  * 4M PRG banking mode is enabled by the TGD Mode Register. When active, it overrides the Latch-based mode in all aspects except CHR memory protection, which is still decided by the 1M Mode Register, and the 2M PRG banking mode.
  * The entire 512 KiB can be reached if available.


    
    
    D~[PPPP PPCC]
       |||| ||++- PPU $0000-$1FFF: switchable 8 KiB CHR bank #0-#3 via CC in $8000-$FFFF
       ++++-++--- CPU $8000-$9FFF: switchable 8 KiB PRG Bank #0-63 via PPPPPP in $8000-$9FFF
                  CPU $A000-$BFFF: switchable 8 KiB PRG Bank #0-63 via PPPPPP in $A000-$BFFF
                  CPU $C000-$DFFF: switchable 8 KiB PRG Bank #0-63 via PPPPPP in $C000-$DFFF
                  CPU $E000-$FFFF: switchable 8 KiB PRG Bank #0-63 via PPPPPP in $E000-$FFFF
    

## 1 KiB CHR banking mode

  * Eight registers at $4400-$4407 switch eight 1 KiB CHR banks. The registers are readable as well.
  * 1 KiB CHR banking mode is enabled via the TGD Mode Register. register. When active, it overrides the CC bits that selected 8 KiB banks from 32 KiB.
  * Although the Turbo Game Doctor 4+ only has 32 KiB of CHR memory, 1 KiB CHR banking mode still exists, allowing the 32 KiB to be banked with finer granularity than on previous Bung and FFE models.


    
    
    D~[CCCC CCCC]
       ++++-++++- PPU $0000-$03FF: switchable 1 KiB CHR Bank #0-255 via CCCCCCC in $4400
                  PPU $0400-$07FF: switchable 1 KiB CHR Bank #0-255 via CCCCCCC in $4401
                  PPU $0800-$0BFF: switchable 1 KiB CHR Bank #0-255 via CCCCCCC in $4402
                  PPU $0C00-$0FFF: switchable 1 KiB CHR Bank #0-255 via CCCCCCC in $4403
                  PPU $1000-$13FF: switchable 1 KiB CHR Bank #0-255 via CCCCCCC in $4404
                  PPU $1400-$17FF: switchable 1 KiB CHR Bank #0-255 via CCCCCCC in $4405
                  PPU $1800-$1BFF: switchable 1 KiB CHR Bank #0-255 via CCCCCCC in $4406
                  PPU $1C00-$1FFF: switchable 1 KiB CHR Bank #0-255 via CCCCCCC in $4407
    

# Interrupts

The Turbo Game Doctors have a cycle-based IRQ counter that operates in an unusual fashion. After resetting the counter at the start of the NMI handler, a game writes the target value at which an IRQ is to be generated. If a second IRQ is desired further down the frame, only a new target value is written while keeping the counter running. This approach avoids accumulating timing errors from delays and jitter. 

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
    

## 2M PRG banking mode ($43FE-$43FF, write-only)
    
    
    A~FEDC BA98 7654 3210  D~7654 3210
      -------------------    ---------
      0100 0011 1111 111M    .p.. ..CC
                        |     |     ++- switchable 8 KiB CHR bank #0-#3 via CC
                        |     +-------- PRG A17 for CPU $8000-$FFFF
                        +- 2M PRG banking mode select
                            0: Enable
                            1: Disable
    

The CC bits are mirrors of those in the latch at $8000-$FFFF. 

## Turbo Game Doctor mode ($4411, read/write)
    
    
    D~7654 3210
      ---------
      41p. GAWW
      |||  ||++- CPU $6000-$7FFF: switchable 8 KiB WRAM bank #0-3 via WW.
      |||  ||    Only usable in floppy disk load mode, and ignored in gameplay mode.
      |||  |+--- Map WRAM in floppy disk load mode. Ignored in gameplay mode.
      |||  +---- Game Saver attachment is active. Related to real-time saving.
      ||+------- PRG A18 in floppy disk mode. Ignored in gameplay mode.
      |+-------- 1=1 KiB CHR banking mode enabled
      +--------- 1=4M PRG banking mode enabled
    

## IRQ Counter Target LSB/MSB ($440D/$440C, write-only)

  * This upwards-counting signed 16-bit counter, when negative, is increased on every M2 or PA12 rise and raises an IRQ when reaching the target value, continuing to count until it reaches $FFFF.
  * Writing to either register acknowledges a pending IRQ.
  * The counter is reset and disabled by writing a positive value to $440C.



## IRQ Counter Value LSB/MSB ($440D/$440C, read-only)

The current counter value can be read from the same addresses that the target counter value is written to. 

## Current CHR Bank ($4420, read-only)

The value of the current 1 KiB CHR bank register can be read and seen changing as the PPU renders the screen. The BIOS uses this in test mode to determine correct operation, and some games check the result to lock out competing products from Bung and FFE. 

Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
