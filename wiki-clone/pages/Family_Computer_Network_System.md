# Family Computer Network System

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Family_Computer_Network_System) | View [other pages](Special_AllPages.xhtml#Family_Computer_Network_System)

## Contents

  * 1 System Overview
  * 2 LH5323M1 Kanji Graphic ROM
  * 3 Expansion Audio
  * 4 Disk Drive Support
  * 5 Memory Map
    * 5.1 Data Bus Behavior
  * 6 Known Registers
  * 7 RF5A18 Internal 65C02 CPU
    * 7.1 CPU2 Memory Map
      * 7.1.1 RF5A18 Pin 26 Low (default)
      * 7.1.2 RF5A18 Pin 26 High
    * 7.2 CPU2 Known Registers
  * 8 Communication Between Famicom and CPU2
    * 8.1 CPU2 Commands
      * 8.1.1 Commands Read by the Famicom from CPU2
      * 8.1.2 Commands Written by the Famicom to CPU2
      * 8.1.3 CPU2 Commands with CRC key bytes
      * 8.1.4 Connection Status Byte
  * 9 Pinouts
    * 9.1 RF5C66 Mapper and Disk Drive Controller
    * 9.2 RF5A18 CPU2 / Modem Controller
    * 9.3 LH5323M1 Kanji Graphic ROM
    * 9.4 8633 Famicom Network System CIC Key Chip
    * 9.5 8634A Tsuushin Card CIC Lock Chip
    * 9.6 8kByte CHR RAM
    * 9.7 8kByte W-RAM
    * 9.8 8kByte CPU2 RAM
    * 9.9 P4: Modem Module Edge Connector
    * 9.10 P2: Tsuushin Card Connector
    * 9.11 P3: Expansion Connector
    * 9.12 P5: Expansion Connector
  * 10 See Also



# System Overview

The Famicom Network System is a complicated device with its own memory mapping system and internal CPU. The RF5C66 chip provides the main mapper functionality, delegating its own registers at $40A0, RF5A18 Famicom registers at $40D0, an internal Kanji ROM at $5000, an internal 8kByte W-RAM at $6000. It also controls the bank of a built-in 16 KiB CHR RAM (using two 8 KiB CHR RAM chips). 

The RF5A18 contains CPU2, which is a 65** _C_** 02 processor with its own independent CPU clock. It has a built-in 4kByte ROM. This chip is responsible for controlling the modem communications. It communicates with the Famicom CPU through four bidirectional registers at $40Dx. 

The Famicom Network System plugs into the Famicom through its cartridge connector and provides the user a ZIF style slot to insert a _tsuushin card_. The card is similar to a normal cartridge but does not have access to any PPU signals. Commercial cards are observed to have their own MMC1 memory mapper, which does not interfere with any of the registers of the Famicom Network System. CPU R/W and the CPU data bus are routed through the RF5C66 chip before making it to the card and other internal hardware. It effectively blocks the Famicom Network System from driving the data bus for certain regions of memory, and possibly also is intended to act as a bidirectional buffer / signal conditioner. It is also used for blocking writes when the lid switch is open or when the CIC fails. Older revisions of Famicom Network System also buffered the Famicom address bus with 74HC541 chips, so it is plausible that signal conditioning was a concern. 

# LH5323M1 Kanji Graphic ROM

The LH5323M1 is a 256kByte graphics ROM containing primarily Kanji data that is mapped at $5000-5FFF. Each index in this range is a 32-byte space containing 16x16 1bpp graphics, usually for a single character, and each read automatically advances to the next byte in the sequence. There are 2 128kByte banks, and the low bank is default at power-on. Writing 1 to $40B0.0 selects the high bank. Reading from $40B0 resets to the beginning of the 32-byte sequence. Writing $40B0 does not reset the sequence however. No values written to $40B0 have been observed to arbitrarily change or reset the position in the sequence. Commercial software has been observed to use throwaway reads when accessing data that does not start at the beginning of the 32-byte area. 

The Kanji ROM chip is connected directly to the non-buffered Famicom CPU data bus. Writes in the range $5000-5FFF do activate Kanji ROM /CE and are subject to bus conflict. 

# Expansion Audio

The Famicom Network System does have expansion audio capabilities. The Famicom audio is routed through the modem module, but nowhere directly to either of the large ASICs. Dial tones have been observed through the television speakers. It is unlikely but unknown if there are other possible sources of sound. 

# Disk Drive Support

According to a block diagram with potentially dubious origins, the RF5C66 chip contains a disk drive controller. Similar design in several ways to the Famicom Disk System, it is suspected that a disk drive can be connected to the expansion port and controlled by the RF5C66. Since this feature was never used, it is unknown how to use or activate it, or even if that feature is fully implemented. The original FDS has a large DRAM that is not present as a discrete chip in the Famicom Network System. It is unknown if such a DRAM could be already integrated into the RF5C66, or could be attached externally and simply not populated, or if a special tsuushin card was to be constructed containing this RAM. All original FDS registers are notably absent and all discovered registers start immediately after where the FDS registers would normally be. There is no obvious path to produce FDS expansion audio. This remains a mystery presently. 

One possibility if the RF5C66 follows a similar pinout to the FDS RP2C33: 
    
    
            79 / -> Exp P3-2  (reg unknown) Serial Out (observed 95.95kHz)
           78 / <- Exp P3-3  (reg unknown) Serial In
          77 / -> Exp P3-4  (!$40A4.2)    Read / Write
         76 / -> Exp P3-5  (!$40A4.1)    Reset Transfer Timing
        75 / -> Exp P3-6  ($40A3.0)     Turn on Motor
       74 / <- Exp P3-7  ($40A5.2)     Write Protect
      73 / <- Exp P3-8  ($40A5.1)     Disk Not Ready
     72 / <- Exp P3-9  ($40A5.0)     Disk Missing
    71 / <- Exp P3-11 ($40A5.7)     Battery Health
    

# Memory Map
    
    
    +================+ $0000 - Famicom Internam RAM
    | Famicom        |
    | Internal RAM   |
    +----------------+ $0800
    |   (Mirrors of  |
    |   $0000-$07FF) |
    +================+ $2000 - Famicom PPU Registers
    | Famicom PPU    |
    | Registers      |
    +----------------+ $2008
    |   (Mirrors of  |
    |   $2000-$2007) |
    +================+ $4000 - Famicom APU, IO, and Test Registers
    | FC APU and IO  |
    | Registers      |
    +----------------+ $4018
    | FC Test Mode   |
    | Registers      |
    +----------------+ $4020
    |   (Open Bus)   |
    +================+ $40A0 - Famicom Network System Internal Registers
    | Famicom Modem  |
    | RF5C66         |
    | Registers      |
    +----------------+ $40D0
    | Famicom Modem  |
    | RF5A18 (CPU2)  |
    | Registers      |
    +----------------+ $40D8
    |   (Mirror of   |
    |   $40D0-$40D7) |
    +----------------+ $40E0
    | Unused Device  |
    | Registers      |
    | (Open Bus)     |
    +----------------+ $40F0
    |   (Open Bus)   |
    +----------------+ $4100
    |   (Open Bus)   |
    +----------------+ $41A0
    |   (Mirror of   |
    |   $40A0-$40FF) |
    +----------------+ $4200
    |   (Mirrors of  |
    |   $4100-$41FF) |
    +================+ $5000 - Famicom Network System Internal Kanji ROM
    | FNS            |
    | LH5323M1       |
    | Kanji ROM      |
    +================+ $6000 - Famicom Network System Internal RAM and Tsuushin Card RAM
    | FNS            |
    | Internal RAM   |
    +================+ $8000 - Tsuushin Card ROM Space
    | Tsuushin Card  |
    | Space          |
    |                |
    +================+ $10000
    

## Data Bus Behavior

The CPU data bus is buffered in both directions through the RF5C66 chip before making it to the tsuushin card and other internal hardware, with the exception of the Kanji graphics ROM and the RF5C66's own registers, which sit directly on the CPU data bus. It appears that the buffer goes from CPU Data Bus to Card Data Bus, or Card Data Bus to CPU Data Bus only. There has not been a high-impedance state observed when testing each possible address in read and write directions on a standalone RF5C66 chip. The CPU Data Bus and Card Data Bus were always observed to be equal despite attaching opposite pull-up and pull-down resistors. 

When RF5C66 pin 29 is driven low by the master CIC chip's pin 11, it forces Card R/W always high, thereby preventing most RAM and register writes in the Famicom Network System. Potentially, the data bus buffer has an additional behavior in this mode in order to prevent bus conflicts. The lid switch also imposes such a behavior when the lid is open, but it does not appear that the RF5C66 has a way of knowing when the switch is open, so the bus conflicts do not appear to be prevented in that case. 

The propagation delay of the CPU data bus buffer and CPU R/W buffer are measured to be about 16 nsec. 

Address Range | Buffer Direction When Reading | Buffer Direction When Writing | Notes   
---|---|---|---  
$0x00-$4x1F  | CPU -> Card | CPU -> Card | Famicom internal RAM and registers   
$4x20-$4x9F  | CPU -> Card | CPU -> Card | (unknown; [FDS](Family_Computer_Disk_System.xhtml "FDS") registers exactly fit here)   
$4xA0-$4xCF  | CPU -> Card | CPU -> Card | RF5C66 registers   
$4xD0-$4xDF  | CPU <\- Card | CPU -> Card | CPU 2 registers, /CE at RF5C66 pin 42   
$4xE0-$4xEF  | CPU <\- Card | CPU -> Card | Unused device registers1, /CE at RF5C66 pin 41   
$4xF0-$4xFF  | CPU -> Card | CPU -> Card | (unknown)   
$5x00-$5xFF  | CPU -> Card | CPU -> Card2 | Kanji graphic ROM, /CE at RF5C66 pin 50   
$6x00-$7xFF  | CPU <\- Card | CPU -> Card | Famicom Network System internal RAM, /CE at RF5C66 pin 40   
$8x00-$FxFF  | CPU <\- Card | CPU -> Card | Tsuushin card, /CE is /ROMSEL   
  
1 Address range $4xE0-$4xEF was meant for a second device similar to CPU 2. Since that device doesn't exist, a tsuushin card could theoretically exploit those 256 addresses for its own purposes; for example, its own read/write registers. 

2 Writes here are subject to bus conflict on the CPU data bus. 

# Known Registers

Note: All registers available to the Famicom ignore address bits 8-11 because those bits are not physically connected to the RF5C66. Therefore, register $4xA0 has mirrors that exist at $40A0, $41A0 ... $4FA0. For simplicity, this page shows all registers as the $40xx mirror. 

Address  | Read  
Has  
Effect  | Read  
Has  
Data  | Write  | Owner  | Function   
---|---|---|---|---|---  
$40A0  | Unknown  | No  | Unknown  | RF5C66  | No evidence has been found that this register does or does not exist. It is possible that it is a write-only register that is not used by any known Famicom Network System card. No reads or writes to this register were ever observed to have any effect. However, its apparent absence is suspicious enough to have this placeholder here. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

| Reference Data   
---  
  
  * Bench test found open bus when reading this register.

  
$40A1  | Unknown  | Yes  | Unknown  | RF5C66  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ++++++++-- Bits exist but function is unknown
    

| Reference Data   
---  
  
  * Bench test observed value $FF with pull-downs.
  * Test code writing values $01,02,04,08,10,20,40,80 always read back $FF after each write.

  
$40A2  | Yes  | Yes  | Unknown  | RF5C66  | IRQ Acknowledge, similar to FDS register $4030 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    |||||||+-- Timer Interrupt (1: an IRQ occurred)
    ||||||+--- Bit exists but function is unknown
    ||||++---- (unlikely to exist)
    ++++------ Bits exist but function is unknown
    

  * Reading this register acknowledges /IRQ. 
    * Observed inconsistent behavior acknowledging, possibly suggesting multiple IRQ sources.

| Reference Data   
---  
  
  * Bench test observed value $20 with pull-downs, $2C with pull-ups.
  * Test code writing values $01,02,04,08,10,20,40,80 always read back $20 after each write.
  * JRA-PAT: 
    * Reads this register with a BIT op-code right before CLI op-code.
    * Reads this register and makes a decision based on D7.
  * Super Mario Club reads this register with BIT op-code.

  
$40A3  | Unknown  | No  | Yes  | RF5C66  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- EXP 6 = $40A3.0 (POR value = 0)
    +++++++--- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Various values written to this register affect the behavior of pulses on pin 79.

| Reference Data   
---  
  
  * Bench test found open bus when reading this register.
  * JRA-PAT writes $2F to this register and appears to keep a RAM copy at $15.
  * Super Mario Club writes $2F to this register and appears to keep a RAM copy at $15.

  
$40A4  | Unknown  | No  | Yes  | RF5C66  | Expansion Port Control 
    
    
    Write
    76543210
    |||||||+-- (unknown)
    ||||||+--- EXP 5 = !($40A4.1) (POR value = 0)
    |||||+---- EXP 4 = !($40A4.2) (POR value = 0)
    +++++----- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Note: Reading test should be repeated with pull-ups and pull-downs on expansion pins.
  * This register has not been observed read or written to by any commercial software.

| Reference Data   
---  
  
  * Bench test found open bus when reading this register.

  
$40A5  | Unknown  | Yes  | Unknown  | RF5C66  | Expansion Port Input Data 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    |||||||+-- Input value of EXP 9
    ||||||+--- Input value of EXP 8
    |||||+---- Input value of EXP 7
    |++++----- (unlikely to exist)
    +--------- Input value of EXP 11
    

| Reference Data   
---  
  
  * Bench test observed value $00 with pull-downs, $78 with pull-ups.

  
$40A6  | Unknown  | Yes  | Yes  | RF5C66  | M2 Cycle Counter LSB, similar to FDS register $4020 
    
    
    Write
    76543210
    ++++++++-- Cycle counter reload value (LSB)
    
    Read
    76543210
    ++++++++-- Cycle counter present value (LSB)
    

  * Writing to this register writes to the cycle counter reload value.
  * Reading this register gives the present value of the counter.
  * Writing any value to $40A8 resets the counter to the reload value.
  * This value counts down.
  * When the value reaches $0000, the next count rolls over to $FFFF or auto-reloads depending on $40A8.0.

  
$40A7  | Unknown  | Yes  | Yes  | RF5C66  | M2 Cycle Counter MSB, similar to FDS register $4021 
    
    
    Write
    76543210
    ++++++++-- Cycle counter reload value (MSB)
    
    Read
    76543210
    ++++++++-- Cycle counter present value (MSB)
    

  * Refer to description in $40A6, this register being the MSB portion of the counter.

  
$40A8  | Unknown  | No  | Yes  | RF5C66  | IRQ Control, similar to FDS register $4022 
    
    
    Write
    76543210
    |||||||+-- IRQ Repeat Flag
    ||||||+--- IRQ Enable
    ++++++---- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Writing anything to this register resets the cycle counter to the reload value.
  * Observed writing $02 makes /IRQ go low, $00 makes /IRQ go high. 
    * Reading $40A2 with /IRQ low acknowledges it back high.
    * Acknowledging with $40A2 first before writing $02 here prevents IRQ immediately going low.
  * Observed rollover of cycle counter to $FFFF with repeat flag = 0 and auto-reload when flag = 1.

| Reference Data   
---  
  
  * Bench test found open bus when reading this register.
  * JRA-PAT: 
    * Writes $00 to this register.
    * Writes $00 again later, potentially connected to RAM $4F.
    * Later, right after having written $25 to $40A7 and $20 to $40A6, writes $02 to this register.
    * Has these various sequences hard-coded: 
      * $25->$40A7, $20->$40A6, $02->$40A8
      * $1C->$40A7, $10->$40A6, $02->$40A8
      * $03->$40A7, $19->$40A6, $02->$40A8
      * $06->$40A7, $F1->$40A6, $02->$40A8
  * Super Mario Club: 
    * Writes $00 to this register.
    * Later, right after having written $24 to $40A7 and $F8 to $40A6, writes $02 to this register.

  
$40A9  | Yes  | Yes  | Unknown  | RF5C66  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ++++++++-- Bits exist but function is unknown
    

  * Bench test observed value $00 with pull-ups.
  * When driving RF5C66 pin 45 high, this 8-bit value changes.
  * Pin 45 high causes the value to change continuously, as if counting cycles.
  * The value does not appear to match $40A7 or $40A6, though further testing is required to say that for sure.
  * Reading this register causes its contents to be loaded into register $40AA.

  
$40AA  | No  | Yes  | Unknown  | RF5C66  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ++++++++-- Bits exist but function is unknown
    

  * This register maintains the most recent value that was read from $40A9.

| Reference Data   
---  
  
  * Bench test observed value $00 with pull-ups.

  
$40AB  | Yes  | No  | Yes  | RF5C66  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Reading this register resets the value of $40A9 back to $00.

| Reference Data   
---  
  
  * Bench test found open bus when reading this register.
  * JRA-PAT writes $00 to this register.
  * Super Mario Club writes $00 to this register.

  
$40AC  | Yes  | No  | Unknown  | RF5C66  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Reading this register prevents timed toggle on RF5C66-69 (see notes in pinout).

| Reference Data   
---  
  
  * Bench test: 
    * Observed $00 with pull-downs and $FF with pull-ups (totally open bus).
  * JRA-PAT reads this register with a BIT op-code and throws away the result.
  * Super Mario Club reads this register with a BIT op-code.

  
$40AD  | Unknown  | Yes  | Yes  | RF5C66  | Mirroring Control 
    
    
    Write
    76543210
    |+++++++-- (unknown)
    +--------- Mirroring (POR value = 0)
                 0 = Vertical Mirroring (CIRAM A10 = PPU A10)
                 1 = Horizontal Mirroring (CIRAM A10 = PPU A11)
    
    Read
    76543210
    |+++++++-- (unlikely to exist)
    +--------- Present value of CIRAM A10
    

| Reference Data   
---  
  
  * Bench test observed $00 with pull-downs, $7F with pull-ups.
  * JRA-PAT writes $00 to this register.
  * Super Mario Club writes $80 to this register.

  
$40AE  | Unknown  | No  | Yes  | RF5C66  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Built-in RAM /CE control (POR value = 1)
    |||||||      1 = Built-in RAM /CE enabled to go low for reads and writes in the range $6000-7FFF.
    |||||||          Pin 5C66.28 = 1 at all address ranges.  (This pin normally n/c.)
    |||||||      0 = Built-in RAM /CE is always high, preventing all reads and writes of the built-in RAM.
    |||||||          Pin 5C66.28 = 0 at all address ranges.  (This pin normally n/c.)
    +++++++--- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Refer also to $40C0.0 for built-in RAM enabling.

| Reference Data   
---  
  
  * Bench test found open bus when reading this register.
  * JRA-PAT: 
    * Writes $00 to this register.
    * Later writes $01 to this register.
  * Super Mario Club: 
    * Writes $00 to this register.
    * Later writes $01 to this register.

  
$40B0  | Yes  | No  | Yes  | RF5C66  | Kanji Graphic ROM Control 
    
    
    Write
    76543210
    |||||||+-- Kanji ROM Bank Select (POR value = 0)
    +++++++--- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * All reads reset the Kanji auto-increment counter.
  * D0 is written with 0 or 1 in Kanji graphics-loading code depending on the Kanji character index, changing the Kanji bank.
  * For an unknown reason, RF5C66 registers can't be written on test setups. They only work in a real Famicom so far.

| Reference Data   
---  
  
  * Bench test found open bus when reading this register.
  * Is read with BIT and results discarded before reading Kanji data out of $5000-5FFF.
  * Super Mario Club: 
    * Stores X to this register after incrementing X (observed value $20).
    * Reads this register and throws away the value read.

  
$40B1  | Unknown  | Yes  | Yes  | RF5C66  | Modem Control 
    
    
    Write
    76543210
    |||||||+-- Modem Module pin 29 = $40B1.0, rises slowly, goes low fast (POR value = 1)
    ||||||+--- Modem Module pin 32 = $40B1.1, rises slowly, goes low fast (POR value = 1)
    |||||+---- Modem Module pin 31 = $40B1.2, rises slowly, goes low fast (POR value = 1)
    ||||+----- Pin 5C66.68: CPU2 Reset on new revision Famicom Network Systems (POR value = 1)
    ||||         CPU2 /Reset = !($40B1.3)
    ||||         CPU2 runs when $40B1.3 = 0.
    ||||         See also $40C0.2.
    |||+------ Exp 15 = $40B1.4, rises slowly, goes low fast (POR value = 1)
    ||+------- Exp 14 = $40B1.5, rises slowly, goes low fast (POR value = 1)
    |+-------- Exp 13 = $40B1.6, rises slowly, goes low fast (POR value = 1)
    +--------- Exp 12 = $40B1.7, rises slowly, goes low fast (POR value = 1)
    
    Read
    76543210
    |||||||+-- Input value of Modem Module pin 29
    ||||||+--- Input value of Modem Module pin 32
    |||||+---- Input value of Modem Module pin 31
    ||||+----- Input value of 5C66 pin 63 (normally n/c)
    |||+------ Input value of EXP 15
    ||+------- Input value of EXP 14
    |+-------- Input value of EXP 13
    +--------- Input value of EXP 12
    

| Reference Data   
---  
  
  * Bench test observed value $FF with pull-downs.
  * JRA-PAT: 
    * Writes $F7 to this register and appears to keep a RAM copy at $17.
    * Later writes the value from $17, ORed with #$08. (Bit 3 being set to 1.)
    * Also writes the value from $17, ANDed with #$F7. (Bit 3 being set to 0.)
  * Super Mario Club: 
    * Writes $F7 to this register and appears to keep a RAM copy at $17.
    * Later writes the value from $17, ORed with #$08. (Bit 3 being set to 1.)

  
$40C0  | Unknown  | Yes  | Yes  | RF5C66  | CIC Status, CHR Bank, and RAM Control 
    
    
    Write
    76543210
    |||||||+-- Pin 5C66.35 = $40C0.0 (POR value = 0)
    |||||||      RAM +CE Enable (1 = enabled, 0 = disabled)
    ||||||+--- Pin 5C66.36 = $40C0.1 (POR value = 0)
    ||||||       (This pin normally n/c)
    |||||+---- Pin 5C66.37 = $40C0.2 (POR value = 0)
    |||||        Old Revision Famicom Network System: CPU2 /Reset (This pin n/c on newer revisions)
    |||||        CPU2 runs on old revisions when $40C0.2 = 1.
    |||||        See also $40B1.3.
    ||||+----- Pin 5C66.38 = $40C0.3 (POR value = 0)
    ||||         CHR RAM Bank Select (selects between two 8 KiB CHR RAM chips)
    ++++------ (unknown)
    
    Read
    76543210
    |||||||+-- Input value of pin 5C66.31:
    |||||||      From FNS CIC (Key) pin 10, or jumpered to GND on models without a CIC chip.
    ||||||+--- Input value of pin 5C66.32:
    ||||||       From FNS CIC (Key) pin 15, or jumpered to GND on models without a CIC chip.
    |||||+---- Input value of pin 5C66.33:
    |||||        CPU2 /Reset fed back in for all models, regardless which model-specific 5C66 pin is sending it out.
    ||||+----- Input value of pin 5C66.34:
    ||||         Selected CHR RAM Bank, fed back in from 5C66.38.
    |+++------ (unlikely to exist)
    +--------- Input value of pin 5C66.29:
                 /CPU R/W Inhibit, from FNS CIC (Key) pin 11, or floating up to logic high on models without a CIC chip.
                 1 = Writes to card, W-RAM, and $40Dx are allowed
                 0 = Blocked (CPU R/W is overriden high, i.e. "read" for these writes)
    

  * RAM +CE always reflects bit 0 of this register regardless of address space. 
    * Refer also to $40AE.0 for built-in RAM enabling.
  * All examined software waits for D7 = 1 at initialization.
  * D7 is controlled by the CIC key chip, if present. If the CIC fails authentication, it latches to 0 until a power cycle.
  * D1 and D0 always observed low in all cases, regardless if the model does not have CIC key chip, or does have the chip and authentication passes or fails. 
    * It is unknown in what scenario that the CIC chip could generate logic high on these pins.
    * Theories: 
      * These pins may reflect detection of different lock CIC chips in the tsuushin card, for different regions or "disk drive mode" or something like that.
      * Transient role at power-on, indicating when the CIC is ready.
      * Showing internal error state of the CIC.

| Reference Data   
---  
  
  * Bench test observed value $00 with pull-downs, $70 with pull-ups.
  * JRA-PAT: 
    * Writes to this register from what appears to be a RAM copy at $18 ORed with #$01.
    * Also writes from $18 ANDed with #$FE.
  * Super Mario Club: 
    * Writes $00 to this register and appears to keep a RAM copy at $18.
    * Later writes the value from $18, ANDed with #$FB. (Bit 2 being set to 0.)
    * Reads this register and makes a decision using D7.

  
$40D0  | Unknown  | Yes  | Yes  | RF5A18  | Famicom CPU <-> CPU2 Interface, Data Byte 0 
    
    
    Write
    76543210
    ++++++++-- 8-bit value written here can be read by CPU2 from register $4123.
    
    Read
    76543210
    ++++++++-- 8-bit value read here was written by CPU2 to register $4123.
    

| Reference Data   
---  
  
  * Super Mario Club: 
    * Does multiple writes of various values to this register when opening a connection.
    * Reads this register when closing a modem connection, which reports value $80 and stores it at $701.
  * Writing a value to this register does not cause the read value to change. 
    * This shows that Famicom -> CPU2 and CPU2 -> Famicom directions each use separate physical registers.

  
$40D1  | Unknown  | Yes  | Yes  | RF5A18  | Famicom CPU <-> CPU2 Interface, Data Byte 1 
    
    
    Write
    76543210
    ++++++++-- 8-bit value written here can be read by CPU2 from register $4124.
    
    Read
    76543210
    ++++++++-- 8-bit value read here was written by CPU2 to register $4124.
    

| Reference Data   
---  
  
  * Super Mario Club: 
    * Does multiple writes of various values to this register when opening a connection.
    * Reads this register when closing a modem connection, which reports value $01 and stores it at $702.
  * Writing a value to this register does not cause the read value to change. 
    * This shows that Famicom -> CPU2 and CPU2 -> Famicom directions each use separate physical registers.

  
$40D2  | Unknown  | Yes  | Yes  | RF5A18  | Famicom CPU <-> CPU2 Interface, Data Byte 2 
    
    
    Write
    76543210
    ++++++++-- 8-bit value written here can be read by CPU2 from register $4125.
    
    Read
    76543210
    ++++++++-- 8-bit value read here was written by CPU2 to register $4125.
    

| Reference Data   
---  
  
  * Super Mario Club: 
    * Does multiple writes of various values to this register when opening a connection.
    * Reads this register when closing a modem connection, which reports value $00 and stores it at $703.
  * Writing a value to this register does not cause the read value to change. 
    * This shows that Famicom -> CPU2 and CPU2 -> Famicom directions each use separate physical registers.

  
$40D3  | Unknown  | Yes  | Yes  | RF5A18  | Famicom CPU <-> CPU2 Interface, Data Acknowledge 
    
    
    Write
    76543210
    |||+++++-- (unknown)
    +++------- 3-bit value written here can be read by CPU2 from register $4122.
    
    Read
    76543210
    |||+++++-- (unlikely to exist)
    +++------- 3-bit value read here was written by CPU2 to register $4122.
    

| Reference Data   
---  
  
  * JRA-PAT writes $FF to this register and appears to keep a RAM copy at $19.
  * Super Mario Club: 
    * Writes $FF to this register and appears to keep a RAM copy at $19.
    * Does a BIT operation on this register and makes decisions based on D7 and D6. 
      * Probably the same: Reads this register once per frame, which reports value $E0.
    * Also writes value $BF to this register, not in connection with $19.
    * Does multiple writes of various values to this register when opening and closing a connection.
  * Bits 4,3,2,1,0 follow pull-ups and downs on the data bus.
  * Writing a value to this register does not cause the read value to change. 
    * This shows that Famicom -> CPU2 and CPU2 -> Famicom directions each use separate physical registers.

  
$40D4  | Unknown  | Yes  | Yes  | RF5A18  | Tone Rx and Expansion I/O Control 
    
    
    Write
    76543210
    |||||||+-- RF5A18 Pin 65 (Exp 17) = $40D4.0 (open-drain)
    ||||||+--- RF5A18 Pin 67 (Exp 19) = $40D4.1 (push-pull)
    |||||+---- RF5A18 Pin 66 (Exp 18) = $40D4.2 (open-drain)
    +++++----- (unknown)
    
    Read
    76543210
    |||||||+-- $40D4.0 = I/O value of RF5A18 Pin 65 (Exp 17)
    ||||||+--- $40D4.1 = output value of RF5A18 Pin 67 (Exp 19)
    |||||+---- $40D4.2 = I/O value of RF5A18 Pin 66 (Exp 18)
    ||||+----- $40D4.3 = input value of RF5A18 Pin 69 (Tone Rx D1)
    |||+------ $40D4.4 = input value of RF5A18 Pin 70 (Tone Rx D2)
    ||+------- $40D4.5 = input value of RF5A18 Pin 71 (Tone Rx D4)
    |+-------- $40D4.6 = input value of RF5A18 Pin 72 (Tone Rx D8)
    +--------- $40D4.7 = input value of RF5A18 Pin 73 (Tone Rx DV)
    

| Reference Data   
---  
  
  * JRA-PAT writes $FF to this register and appears to keep a RAM copy at $1A.
  * Super Mario Club writes $FF to this register and appears to keep a RAM copy at $1A.
  * Bits 7,6,5,4,3 read back as always 0 when writing inverting values to these bits.
  * Bits 2,1,0 read back with the same value that was written.

  
$40D5  | Unknown  | Yes  | Unknown  | RF5A18  | Clocks 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ||++++++-- Bits exist but function is unknown
    ++-------- (unlikely to exist)
    

  * The purpose is unknown but it could potentially serve to generate random numbers. 
    * The Famicom CPU and CPU2 have separate clocks and are asynchronous, making the value read especially random.
  * Bit 5 is a 0.05% duty square wave, measured 1.200kHz (19.6608MHz CPU2 crystal / 16384) 
    * Negative pulse width measures 406 nsec, the same as period of bit 2.
  * Bit 4 is a 50% duty square wave, depending on $4114.1 and $4114.0: 
    * (0,0) measured 19.20kHz (19.6608MHz CPU2 crystal / 1024)
    * (0,1) measured 38.40kHz (19.6608MHz CPU2 crystal / 512)
    * (1,0) measured 76.80kHz (19.6608MHz CPU2 crystal / 256)
    * (1,1) measured 153.6kHz (19.6608MHz CPU2 crystal / 128)
  * Bit 3 is a 50% duty square wave, measured 307.3kHz (19.6608MHz CPU2 crystal / 64)
  * Bit 2 is a 50% duty square wave, measured 2.451MHz (19.6608MHz CPU2 crystal / 8)
  * Bit 1 is a 50% duty square wave, measured 4.902MHz (19.6608MHz CPU2 crystal / 4) 
    * This bit toggles at the same rate as RF5A18 pin 42, though there is a 129.9 degree phase difference.
  * Bit 0 is a 50% duty square wave, measured 9.804MHz (19.6608MHz CPU2 crystal / 2)
  * The phasing of these signals do not line up nicely but they do stay in perfect sync with CPU2 M2.

| Reference Data   
---  
  
  * $40Dx registers can be monitored in real-time by statically giving the RF5A18 the bus signals for a read from this register. 
    * M2 = 1, Card R/W (i.e. buffered CPU R/W) = 1, Famicom /CE = 0, CPU A0,A1,A2 = x,x,x
    * Unimplemented bits will latch the Famicom data bus's last value when toggling M2.
  * Bits 7,6 follow pull-ups and downs on the data bus.

  
$40D6  | Unknown  | Yes  | Unknown  | RF5A18  | UART Status 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    |||||||+-- Bit exists but function is unknown
    ||||||+--- $40D6.1 = !($4112.0) read by CPU2.  (0 = $4110 UART Rx buffer has a byte ready to be read.)
    |||||+---- $40D6.2 = !($4112.1) read by CPU2.  (0 = $4110 UART Tx buffer can be written.)
    ||||+----- $40D6.3 = !($4113.1 OR $4113.2) written by CPU2.
    |||+------ $40D6.4 = !($4113.7) written by CPU2.  (1 = Transmit repeat.)
    ||+------- $40D6.5 = !($4113.6) written by CPU2.
    ++-------- (unlikely to exist)
    

$40D6.2 previously observed to equal !($4113.7 AND $4112.1) written by CPU2. It was later discovered that this bit also follows the $4112.1 read value (Tx buffer ready), which makes a lot more sense being beside $40D6.1 (Rx buffer ready). The previous observation can be explained because writing to those CPU2 bits both likely changed the availability of the Tx buffer.  | Reference Data   
---  
  
  * Bits 7,6 follow pull-ups and downs on the data bus.
  * Bit 0 only ever observed with value 1 regardless of various techniques writing to lots of CPU2 registers.
  * Writing a value to this register does not _seem to_ cause its own read value to change.

  
  
# RF5A18 Internal 65C02 CPU

The RF5A18 contains its own CPU, termed "CPU2" on this page. It is a 65C02 supporting bitwise set/clear/branch instructions. Note that CPU2 has its own parallel execution with its own address and data busses that are not available to the Famicom's CPU. CPU2 also has its own clock source, so it does not execute synchronously with the Famicom CPU. CPU2 clock speed is 2.4576 MHz (19.6608 MHz crystal / 8). This section describes CPU2's own memory mapping and its own internal registers. 

CPU2 /Reset is controlled 2 different ways depending on the revision of Famicom Network System. To support all revisions when enabling CPU2, both of these bits should be written: 

  * $40B1.3 = 0 (new revisions Famicom Network System, and old revisions with J1 closed)
  * $40C0.2 = 1 (old revisions Famicom Network System with J2 closed)



## CPU2 Memory Map

### RF5A18 Pin 26 Low (default)
    
    
    +================+ $0000
    | CPU2 RAM       |
    | (U6)           |
    +================+ $2000
    |   (Returns     |
    |   last fetch)  |
    +================+ $4100
    | CPU2 Control   |
    | Registers      |
    +================+ $4140
    |   (Returns     |
    |   last fetch)  |
    +================+ $C000
    | External ROM   |
    | (not used)     |
    +================+ $E000
    | RF5A18         |
    | Internal ROM   |
    +================+ $10000
    

### RF5A18 Pin 26 High
    
    
    +================+ $0000
    | CPU2 RAM       |
    | (U6)           |
    +================+ $2000
    |   (Returns     |
    |   last fetch)  |
    +================+ $4100
    | CPU2 Control   |
    | Registers      |
    +================+ $4140
    |   (Returns     |
    |   last fetch)  |
    +================+ $C000
    |                |
    |                |
    | External ROM   |
    |                |
    |                |
    +================+ $10000
    

## CPU2 Known Registers

Address  | Read  
Has  
Effect  | Read  
Has  
Data  | Write  | Function   
---|---|---|---|---  
$4100  | No  | No  | Yes  | NMI Timer 1 Period, low byte. 
    
    
    Write
    76543210
    ++++++++-- NMI timer 1 period low byte.
    
    Read
    76543210
    ++++++++-- (does not exist)
    

  * Time (seconds) * 1200 = 16-bit timer 1 register value. 
    * 1 count = 2048 CPU cycles.
  * This is the LSB of the 16-bit period. Refer to $4101 for the MSB.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.
  * When tone dialing, the digit length is $0078 (~100ms) and the inter-digit gap is $0046 (~60ms).
  * Slow pulse dialing uses on-hook pulses of $004f (~67ms) and off-hook gaps of $0027 (~33ms) and the inter-digit gap is $031e (~666ms).
  * Fast pulse dialing uses on-hook pulses of $0027 (~33ms) and off-hook gaps of $0013 (~17ms) and the inter-digit gap is $027e (~533ms). 
    * The times are approximate and assume a 1200Hz clock source, but the actual clock source is unknown. 
      * Theory: 19.6608MHz RF5A18 crystal / 2^14 = 1200Hz.

  
$4101  | No  | No  | Yes  | NMI Timer 1 Period, high byte. 
    
    
    Write
    76543210
    ++++++++-- NMI timer 1 period high byte.
    
    Read
    76543210
    ++++++++-- (does not exist)
    

  * Time (seconds) * 1200 = 16-bit timer 1 register value. 
    * 1 count = 2048 CPU cycles.
  * This is the MSB of the 16-bit period. Refer to $4100 for the LSB.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.

  
$4102  | No  | No  | Yes  | NMI Timer 1 Restart. 
    
    
    Write
    76543210
    |||||||+-- $4102.0 = timer 1 loop.  1 = Loop, 0 = One-Shot
    ||||||+--- $4102.1 = timer 1 restart.  1 = Restart.
    ++++++---- (unknown)
    
    Read
    76543210
    ++++++++-- (does not exist)
    

  * When setting bit 1 = 1, timer 1 restarts with the period specified in $4100, $4101.
  * The values of $4100, $4101 are not affected by running timer 1 and need not be rewritten each time.
  * NMI will be generated when timer 1 expires if $412F.0 = 1.
  * The behavior is untested when setting bits 0 or 1 = 1 while timer 1 is already running.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.
  * ROM $E136 writes value #$02 to this address.
  * ROM $F3D6 writes value #$00 to this address (initialization).
  * ROM $F840 writes value #$00 to this address (Famicom command $61).

  
$4103  | Yes  | Yes  | Unknown  | NMI Timer 1 Acknowledge. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    |||||||+-- $4103.0 = Timer 1 NMI flag.  1 = Timer 1 NMI triggered.
    +++++++--- (does not exist)
    

  * Reading from this register likely acknowledges timer 1 NMI.
  * The read value of $4103.0 reflects the timer 1 NMI flag, but does not reflect external NMI via pin 29.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $40. ($41 XOR $40 = $01 existence mask)
    * Value $40 from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $BE. ($BF XOR $BE = $01 existence mask)
    * $01 OR $01 = $01 accumulated existence mask.
  * ROM $E825 reads from this register at the beginning of the NMI handler and throws away the value.
  * ROM $F3D9 reads from this register during initialization and throws away the value.

  
$4104  | No  | No  | Yes  | IRQ Timer 2 Period, low byte. 
    
    
    Write
    76543210
    ++++++++-- IRQ timer 2 period low byte.
    
    Read
    76543210
    ++++++++-- (does not exist)
    

  * Time (seconds) * 2,457,600 = 16-bit timer 2 register value. 
    * 1 count = 1 CPU cycle.
  * This is the LSB of the 16-bit period. Refer to $4105 for the MSB

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.
  * ROM $F415 writes value #$FD to this address.

  
$4105  | No  | No  | Yes  | IRQ Timer 2 Period, high byte. 
    
    
    Write
    76543210
    ++++++++-- IRQ Timer 2 Period, high byte.
    
    Read
    76543210
    ++++++++-- (does not exist)
    

  * Time (seconds) * 2,457,600 = 16-bit timer 2 register value. 
    * 1 count = 1 CPU cycle.
  * This is the MSB of the 16-bit period. Refer to $4104 for the LSB

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.
  * ROM $F410 writes value #$2F to this address.

  
$4106  | No  | No  | Yes  | IRQ Timer 2 Restart. 
    
    
    Write
    76543210
    |||||||+-- $4106.0 = timer 2 loop.  1 = Loop, 0 = One-Shot
    ||||||+--- $4106.1 = timer 2 restart.  1 = Restart.
    ++++++---- (unknown)
    
    Read
    76543210
    ++++++++-- (does not exist)
    

  * Theory: When setting bit 1 = 1, timer 2 restarts with the period specified in $4100, $4101.
  * Theory: The values of $4104, $4105 are not affected by running timer 2 and need not be rewritten each time.
  * IRQ will be generated when timer 2 expires if $412F.6 = 1.
  * The behavior is untested when setting bits 0 or 1 = 1 while timer 2 is already running.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.
  * ROM $F3DE writes value #$00 to this address.
  * ROM $F41A writes value #$03 to this address.

  
$4107  | Yes  | Yes  | Unknown  | IRQ Timer 2 Acknowledge. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    |||||||+-- (unlikely to exist)
    ||||||+--- Likely to be the Timer 2 IRQ flag
    ++++++---- (unlikely to exist)
    

  * Reading from this register likely acknowledges timer 2 IRQ.
  * $4107.1 is probably the timer 2 IRQ flag.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 and $40 from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $BD. ($BF XOR $BD = $02 existence mask)
  * ROM $E78A reads from this register directly after seeing IRQ flag $412F.6 and throws away the value.
  * ROM $F3E1 reads from this register during initialization and throws away the value.

  
$4110  | Yes  | Yes  | Yes  | UART Rx/Tx Data Buffer 
    
    
    Write
    76543210
    ++++++++-- UART Tx Data Byte
                 Bits 7:0 in 8-bit mode, bits 6:0 in 7-bit mode
    
    Read
    76543210
    ++++++++-- UART Rx Data Byte
                 Bits 7:0 in 8-bit mode, bits 6:0 in 7-bit mode
    

  * Writing a byte to this register automatically clears $4112.1 read value and triggers the byte to be sent on UART Tx.
  * Reading a byte from this register automatically clears $4112.0 read value, to be triggered high again next byte received on UART Rx.
  * This UART is connected to the MSM6827L TXD and RXD pins.
  * The byte is ready to read here when the IRQ flag $4112.0 is high.
  * The byte is ready to be written here when flag $4112.1 is high.
  * Writes to $4110 are specifically prevented by built-in ROM in modes 0 and 5.
  * Bench testing confirmed reads and writes of this register correspond to data read by UART Rx and written to UART Tx. 
    * In 7-bit mode, Rx bit 7 (unused in this case) observed to be 0 but not confirmed always to be 0.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $FF. ($41 XOR $FF = $BE existence mask)
    * Value $41 from previous fetch also observed changed to $00. ($41 XOR $00 = $41 existence mask)
    * Value $40 from previous fetch observed changed to $00. ($40 XOR $00 = $40 existence mask)
    * Value $BF from previous fetch observed changed to $00. ($BF XOR $00 = $BF existence mask)
    * $BE OR $41 OR $40 OR $BF = $FF accumulated existence mask.
  * ROM $E69B reads from this address inside of an IRQ handler, seemingly caused by its own IRQ, with flag at $4112.0.
  * ROM $E72F writes to this address inside of an IRQ handler, which may have been caused by timer 2.
  * ROM $E754 writes to this address inside of an IRQ handler, which may have been caused by timer 2.

  
$4111  | Unknown  | Yes  | Yes  | UART Configuration 
    
    
    Write
    76543210
    |||||||+-- $4111.0 = UART Rx Enable (1 = enabled)
    ||||||+--- $4111.1 = UART Tx Enable (1 = enabled)
    |||||+---- $4111.2 = Baudrate Scaler (0 = multiply baudrate x4)
    ||||+----- $4111.3 = Data bits (0 = 7 bits, 1 = 8 bits)
    |||+------ $4111.4 = Tx Stop bits (0 = 1 stop bit, 1 = 2 stop bits)
    ||+------- $4111.5 = Parity bit (0 = don't use, 1 = append parity bit)
    |+-------- $4111.6 = Parity type (0 = odd parity, 1 = even parity)
    +--------- $4111.7 = Send Tx Break (1 = force UART Tx low (break), 0 = normal)
    
    Read
    76543210
    ++++++++-- Reads back the value written.
    

  * See register $4114 for baudrate table.
  * Confirmed write bits 2,3,5,6 apply to both Rx and Tx.
  * Write bit $4111.4 confirmed not to enforce 2 stop bits for UART Rx. 
    * Consecutive Rx bytes with 1 stop bit separation can still be received when $4111.4 = 1.
  * Write bit $4111.7 requires manual operation to send a break character and has no effect on Rx operation.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $00. ($41 XOR $00 = $41 existence mask)
    * Value $40 from previous fetch observed changed to $00. ($40 XOR $00 = $40 existence mask)
    * Value $BF from previous fetch observed changed to $00. ($BF XOR $00 = $BF existence mask)
    * $41 OR $40 OR $BF = $FF accumulated existence mask.
  * The built-in ROM function at $E1BC determines the UART settings to be written to this register. 
    * Clears bits 0,1,7.
    * Sets bit 2 (slow baudrate scaler).
    * Figures out bits 3,4,5,6 (UART bits/parity settings).
    * Returns that $4111 value in accumulator A.
    * Also sets baudrate to 1200 bits/sec by writing to $4114 before returning.
  * ROM $E151 runs $E1BC, sets bits [1,0] = 0,0, and writes it to this register.
  * ROM $ECE3 runs $E1BC, sets bits [1,0] = 1,1, and writes it to this register.
  * ROM $ED20 runs $E1BC, sets bits [1,0] = 0,1, and writes it to this register.
  * ROM $ED4E reads from this address, OR's it with #$03 (i.e. sets bits [1,0] = 1,1), and writes it back to this register.
  * ROM $EE83 runs $E1BC, sets bits [1,0] = 0,1, and writes it to this register.
  * ROM $F0E8 reads from this address, AND's it with #$FE (i.e. clears bit 7), and writes it back to this register.
  * ROM $F2DC runs $E1BC, sets bits [1,0] = 0,1, and writes it to this register.

  
$4112  | Unknown  | Yes  | Yes  | UART Status 
    
    
    Write
    76543210
    |||||||+-- Bit exists but function is unknown
    ||||||+--- $4112.1: Tx Silence
    ||||||       0 = Set UART Tx directly high (idle state), 1 = Allow sending data.
    ||||||       This bit figures into $40D6.2 read by the Famicom. (See register $40D6.)
    |+++++---- (unknown)
    +--------- $4112.7 = $4110 UART Rx Data IRQ Acknowledge, 1 = Acknowledge IRQ.
    
    Read
    76543210
    |||||||+-- $4112.0 = $4110 UART Rx Buffer Ready Flag (1 = $4110 buffer has a byte ready to be read.)
    ||||||+--- $4112.1 = $4110 UART Tx Buffer Ready Flag (1 = $4110 buffer can be written.)
    |||||+---- $4112.2 = UART Tx Idle, 1 = Idle, 0 = Active.
    ||||+----- Bit exists but function is unknown
    |||+------ $4112.4 = Rx Parity Error. (1 = error detected)
    ||+------- $4112.5 = Rx Framing Error. (1 = error detected)
    |+-------- $4112.6 = Rx Break Received. (1 = break detected)
    +--------- Bit exists but function is unknown
    

  * Error and Break bits persist through $4110 operations, but can be cleared by writing 1 to $4112.7.
  * Found $4112.4 read became 1 when setting in parity mode ($4111.5 = 1) and injecting a wrong parity bit. 
    * Confirmed this flag not getting set anymore with the same test when changing parity even/odd (i.e. inverted $4111.6).
  * $4112.6 = 1 (break) comes accompanied with $4112.5 = 1 (framing error).

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $04. ($41 XOR $04 = $45 existence mask)
    * Value $40 from previous fetch observed changed to $04. ($40 XOR $04 = $44 existence mask)
    * Value $BF from previous fetch observed changed to $04. ($BF XOR $04 = $BB existence mask)
    * $45 OR $44 OR $BB = $FF accumulated existence mask.
  * ROM implements a shadow read register at $0011.
  * ROM implements a shadow write register at $0012.
  * ROM $E159 writes value #$80 to this address when disconnecting.
  * ROM $E4D8 reads from this address in the IRQ handler and uses bit 0.
  * ROM $E4F9 writes to this address in the IRQ handler after having set bit 7 in its shadow register. 
    * If $4112.0 read = 1, it writes a 1 to $4112.7.
  * ROM $ECDB writes value #$81 to this address.
  * ROM $ED18 writes value #$81 to this address.
  * ROM $ED4B writes value #$81 to this address.
  * ROM $EE7B writes value #$81 to this address.
  * ROM $F2D4 writes value #$81 to this address.


  * ROM $E4F2 uses read bit 0.
  * ROM $E67C uses read bit 5.
  * ROM $E68E uses read bit 4.
  * ROM $E703 uses read bit 1.

  
$4113  | Unknown  | Yes  | Yes  | UART Configuration 
    
    
    Write
    76543210
    |||||||+-- (unknown)
    ||||||+--- $4113.1 figures into $40D6.3 read by the Famicom. (See register $40D6.)
    |||||+---- $4113.2 figures into $40D6.3 read by the Famicom. (See register $40D6.)
    ||+++----- (unknown)
    |+-------- $4113.6 figures into $40D6.5 read by the Famicom. (See register $40D6.)
    +--------- $4113.7: Tx Repeat
                 1 = Send once on pin 90 (UART Tx), 0 = Send repeatedly
                 This figures into $40D6.2 and $40D6.4 read by the Famicom. (See register $40D6.)
    
    Read
    76543210
    ++++++++-- Bits exist but function is unknown
    

| Reference Data   
---  
  
  * Writing to Bit 4 appeared to have an effect in some tests, apparently sending only 1 bit on the UART Tx, but this was not able to be reproduced in later tests.
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $10. ($41 XOR $10 = $51 existence mask)
    * Value $40 from previous fetch observed changed to $10. ($40 XOR $10 = $50 existence mask)
    * Value $04 from previous fetch observed changed to $10. ($04 XOR $10 = $14 existence mask)
    * Value $BF from previous fetch observed changed to $10. ($BF XOR $10 = $AF existence mask)
    * $51 OR $50 OR $14 OR $AF = $FF accumulated existence mask.
  * ROM implements a shadow write register at $0013.
  * ROM $E161 writes value #$00 to this address.
  * ROM $ECD3 writes value #$C0 to this address.
  * ROM $ED11 writes value #$40 to this address.
  * ROM $ED44 writes value #$C0 to this address.
  * ROM $EE73 writes value #$40 to this address.
  * ROM $F2CD writes value #$40 to this address.
  * Bits 7 and 6 differ in these writes, suggesting that they exist.

  
$4114  | Unknown  | No  | Yes  | UART Baudrate 
    
    
    Write
    76543210
    ||||||++-- UART Baud Rate (Rx and Tx)
    ||||||       This is a clock divider that also affects frequency seen on $40D5.4.
    ++++++---- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Baud Rate Selection:

| $4111.2  | $4114.1  | $4114.0  | Baudrate   
---|---|---|---  
0  | 0  | 0  | 1200   
0  | 0  | 1  | 2400   
0  | 1  | 0  | 4800   
0  | 1  | 1  | 9600   
1  | 0  | 0  | 300   
1  | 0  | 1  | 600   
1  | 1  | 0  | 1200   
1  | 1  | 1  | 2400   
Reference Data   
---  
  
  * $40D5.4 shows these frequencies depending on $4114 bits 1 and 0: 
    * (0,0) measured 19.20kHz (19.6608MHz CPU2 crystal / 1024)
    * (0,1) measured 38.40kHz (19.6608MHz CPU2 crystal / 512)
    * (1,0) measured 76.80kHz (19.6608MHz CPU2 crystal / 256)
    * (1,1) measured 153.6kHz (19.6608MHz CPU2 crystal / 128)
  * Note: $40D5.4 is not affected by $4111.2.
  * Read Bit Existence: 
    * Values $41, $40, $FF, and $BF from previous fetch observed unaffected.
  * ROM $E1DE writes value #$02 to this address.

  
$4120  | Unknown  | Yes  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Pin 39 (MSM6827L AD0) = $4120.0
    ||||||+--- Pin 38 (MSM6827L AD1) = $4120.1
    |||||+---- Pin 37 (n/c) = $4120.2
    ||||+----- Pin 36 (MSM6827L /RD) = $4120.3
    |||+------ Pin 35 (MSM6827L /WR) = $4120.4
    ||+------- Pin 34 (MSM6827L EXCLK) = $4120.5
    |+-------- Pin 32 (MSM6827L Data): Direction = $4120.6: 1 = input, 0 = output (refer to $4121.0)
    +--------- (unknown)
    
    Read
    76543210
    |+++++++-- Bits exist but function is unknown
    +--------- $4120.7 = Pin 33 (MSM6827L /INT)
    

  * When pin 32 is set as input, it is unknown if or how the value can be read.
  * Pin 32 floats in input mode, regardless of the value written to $4121.0.
  * When pin 32 is set as output, it is a push-pull output.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $FF. ($41 XOR $FF = $BE existence mask)
    * Value $40 from previous fetch observed changed to $FF. ($40 XOR $FF = $BF existence mask)
    * Value $BF from previous fetch observed changed to $FF. ($BF XOR $FF = $40 existence mask)
    * $BE OR $BF OR $40 = $FF accumulated existence mask.
  * Sequence: 
    * ROM $E3BC writes value #$38 to this address.
    * ROM $E3C7 writes value #$38 & #$DF = #$18 to this address.
    * ROM $E3CC writes value #$18 | #$20 = #$38 to this address.
    * ROM $E3D4 writes value #$38 & #$DF = #$18 to this address.
    * ROM $E3D9 writes value #$18 & #$EF = #$08 to this address.
    * ROM $E3DE writes value #$08 | #$10 = #$18 to this address.
    * ROM $E3E3 writes value #$58 to this address.
  * ROM $E3E9 writes value #$5A to this address.
  * ROM $E3EE writes value #$52 to this address.
  * ROM $E3F5 writes value #$72 to this address.
  * ROM $E3FA writes value #$52 to this address.
  * ROM $E402 writes value #$72 to this address.

  
$4121  | Unknown  | Yes  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Pin 32 (MSM6827L Data) = $4121.0 when set as output (refer to $4120.6)
    +++++++--- (unknown)
    
    Read
    76543210
    ++++++++-- Bits exist but function is unknown
    

  * Reading $4121.0 reflects the value written to $4121.0 when $4120.6 is set as output.
  * Surprisingly, reading $4121.0 is always 1 when $4120.6 is set as input regardless of: 
    * Driving pin 32 high or low.
    * Writing 1 or 0 to $4121.0.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $FF. ($41 XOR $FF = $BE existence mask)
    * Value $40 from previous fetch observed changed to $FF. ($40 XOR $FF = $BF existence mask)
    * Value $FF from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $FF. ($BF XOR $FF = $40 existence mask)
    * $BE OR $BF OR $40 = $FF accumulated existence mask.
  * ROM $E3BF writes to this address.
  * ROM $E405 reads this address.

  
$4122  | Unknown  | Yes  | Yes  | Famicom CPU <-> CPU2 Interface, Data Acknowledge 
    
    
    Write
    76543210
    |||+++++-- (unlikely to exist)
    +++------- 3-bit value written here can be read by Famicom CPU from register $40D3.
    
    Read
    76543210
    |||+++++-- (unlikely to exist)
    +++------- 3-bit value read here was written by Famicom CPU to register $40D3.
    

  * Read and write directions are represented by separate physical registers.
  * The read value can only be affected by the Famicom CPU.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $E1. ($41 XOR $E1 = $A0 existence mask)
    * Value $40 from previous fetch observed changed to $E0. ($40 XOR $E0 = $A0 existence mask)
    * Value $FF from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $FF. ($BF XOR $FF = $40 existence mask)
    * $A0 OR $A0 OR $40 = $E0 accumulated existence mask.
  * ROM $F3E6 writes value #$FF to this address.
  * ROM $FA70 writes value #$E0 to this address.
  * ROM $FA78 writes value #$E0 to this address.
  * ROM $FA89 reads this address using BIT operation and uses BMI, indicating bit 7 used.
  * ROM $FAA7 reads this address using BIT operation and uses BPL, indicating bit 7 used.
  * ROM $FAB4 reads this address and checks if bits 7 and 6 both = 1.
  * ROM $FABF writes #$40 to this address, reads it right back, and checks if bits 7 and 6 both = 1.
  * ROM $FACF writes value #$20 to this address.
  * ROM $FAF3 writes value #$A0 to this address.
  * ROM $FB00 writes value #$20 to this address.
  * ROM $FB0D writes value #$A0 to this address.
  * ROM $FB15 writes value #$E0 to this address.
  * ROM $FB1D writes value #$20 to this address.
  * ROM $FB25 writes value #$E0 to this address.
  * ROM $FB8F writes value #$E0 to this address.
  * ROM $FC02 writes value #$E0 to this address.
  * ROM $FC13 reads this address using BIT operation and uses BMI, indicating bit 7 used.
  * ROM $FC2D reads this address using BIT operation and uses BPL, indicating bit 7 used.
  * ROM $FC3A reads this address and checks if bits 7 and 6 both = 1.
  * ROM $FC43 writes value #$60 to this address.
  * ROM $FC69 writes value #$E0 to this address.
  * ROM $FC76 writes value #$60 to this address.
  * ROM $FC83 writes value #$E0 to this address.
  * ROM $FC8D reads this address using BIT operation and uses BPL and BVC, indicating bits 7 and 6 are used.
  * ROM $FC96 writes value #$E0 to this address.
  * ROM $FCA4 writes value #$60 to this address.

  
$4123  | Unknown  | Yes  | Yes  | Famicom CPU <-> CPU2 Interface, Data Byte 0 
    
    
    Write
    76543210
    ++++++++-- 8-bit value written here can be read by Famicom CPU from register $40D0.
    
    Read
    76543210
    ++++++++-- 8-bit value read here was written by Famicom CPU to register $40D0.
    

  * Read and write directions are represented by separate physical registers.
  * The read value can only be affected by the Famicom CPU.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $FF. ($41 XOR $FF = $BE existence mask)
    * Value $40 from previous fetch observed changed to $FF. ($40 XOR $FF = $BF existence mask)
    * Value $FF from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $FF. ($BF XOR $FF = $40 existence mask)
    * $BE OR $BF OR $40 = $FF accumulated existence mask.
  * ROM $F3E9 writes value #$FF to this address.
  * ROM $FAD9 writes to this address.
  * ROM $FB32 writes to this address.
  * ROM $FC49 reads from this address.
  * ROM $FCAE reads from this address.

  
$4124  | Unknown  | Yes  | Yes  | Famicom CPU <-> CPU2 Interface, Data Byte 1 
    
    
    Write
    76543210
    ++++++++-- 8-bit value written here can be read by Famicom CPU from register $40D1.
    
    Read
    76543210
    ++++++++-- 8-bit value read here was written by Famicom CPU to register $40D1.
    

  * Read and write directions are represented by separate physical registers.
  * The read value can only be affected by the Famicom CPU.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $FF. ($41 XOR $FF = $BE existence mask)
    * Value $40 from previous fetch observed changed to $FF. ($40 XOR $FF = $BF existence mask)
    * Value $FF from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $FF. ($BF XOR $FF = $40 existence mask)
    * $BE OR $BF OR $40 = $FF accumulated existence mask.
  * ROM $F3EC writes value #$FF to this address.
  * ROM $FADF writes to this address.
  * ROM $FB38 writes to this address.
  * ROM $FC55 reads from this address and checks if it equals $00, indicating all bits exist.
  * ROM $FCB5 reads from this address.

  
$4125  | Unknown  | Yes  | Yes  | Famicom CPU <-> CPU2 Interface, Data Byte 2 
    
    
    Write
    76543210
    ++++++++-- 8-bit value written here can be read by Famicom CPU from register $40D2.
    
    Read
    76543210
    ++++++++-- 8-bit value read here was written by Famicom CPU to register $40D2.
    

  * Read and write directions are represented by separate physical registers.
  * The read value can only be affected by the Famicom CPU.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $FF. ($41 XOR $FF = $BE existence mask)
    * Value $40 from previous fetch observed changed to $FF. ($40 XOR $FF = $BF existence mask)
    * Value $FF from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $FF. ($BF XOR $FF = $40 existence mask)
    * $BE OR $BF OR $40 = $FF accumulated existence mask.
  * ROM $F3EF writes value #$FF to this address.
  * ROM $FAE7 writes to this address.
  * ROM $FB3E writes to this address.
  * ROM $FC4F reads from this address.
  * ROM $FCBC reads from this address.

  
$4126  | Unknown  | Yes  | Unknown  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    |||||||+-- $4126.0 = Pin 47 (+5V)
    ||||||+--- $4126.1 = !(Pin 48) (+5V)
    |||||+---- $4126.2 = Pin 49 (from 5C66-69)
    ||||+----- $4126.3 = Pin 51 (Switch SW1-2)
    |||+------ $4126.4 = Pin 52 (Switch SW1-4)
    ||+------- $4126.5 = Pin 53 (Modem P4-25)
    |+-------- $4126.6 = Pin 54 (Modem P4-28)
    +--------- $4126.7 = Pin 55 (Modem P4-23)
    
    

  * SW1 selects between pulse and DTMF dialing.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $FB. ($41 XOR $FB = $BA existence mask)
    * Value $41 from previous fetch also observed changed to $FF. ($41 XOR $FF = $BE existence mask)
    * Value $40 from previous fetch observed changed to $FF. ($40 XOR $FF = $BF existence mask)
    * Value $FF from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $FF. ($BF XOR $FF = $40 existence mask)
    * $BA OR $BE OR $BF OR $40 = $FF accumulated existence mask.
  * ROM $E482 reads this address and checks Bit 2.
  * ROM $E7BC reads this address and checks Bits 7 and 6.
  * ROM $E886 reads this address and checks Bits 7 and 6.
  * ROM $E8DC reads this address.
  * ROM $E8FC reads this address and checks Bits 7 and 6.
  * ROM $E9C0 reads this address.
  * ROM $EBAB reads this address and checks Bits 7 and 6.
  * ROM $F1B6 reads this address.
  * ROM $F4F2 reads this address.

  
$4127  | Unknown  | No  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Pin 56 MSM6827L +Reset = $4127.0
    ||||||+--- Pin 57 /Red LED = $4127.1
    |||||+---- Pin 58 /Green LED = $4127.2
    ||||+----- Pin 59 (n/c) = $4127.3
    |||+------ Pin 60 /Phone Off Hook = $4127.4
    ||+------- Pin 61 /DTMF Output Enable = $4127.5
    |+-------- Pin 62 /Phone Audio Enable = $4127.6
    +--------- Pin 63 (Modem P4-19) = $4127.7
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Pin 60 observed low corresponding to off hook and pulse dialing pulses.
  * Pin 61 observed low for the duration of DTMF numbers being dialed, but not in pulse mode.
  * Pin 62 observed low corresponding to modem sounds coming through the TV speakers.
  * Pin 63 observed always high.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, $FF, and $BF from previous fetch observed unaffected.
  * ROM implements a shadow write register at $0014.
  * ROM $E237 writes the shadow register $0014 to this register, modified bit 6 (/Phone Audio Enable).
  * ROM $E24D writes the shadow register $0014 to this register, modified bit 7 (Modem P4-19).
  * ROM $E263 writes the shadow register $0014 to this register, modified bit 4 (/Phone Off Hook).
  * ROM $E27D writes the shadow register $0014 to this register, modified bit 1 (/Red LED).
  * ROM $E29A writes the shadow register $0014 to this register, modified bit 2 (/Green LED).
  * ROM $E3B0 writes the shadow register $0014 to this register, modified bit 5 (/DTMF Output Enable).
  * ROM $E987 writes the shadow register $0014 to this register, modified bit 4 (/Phone Off Hook).
  * ROM $EA83 writes the shadow register $0014 to this register, modified bit 4 (/Phone Off Hook).
  * ROM $F3BF writes value #$FF directly to this register, then clears bit 0 with value #$FE via the shadow register $0014.

  
$4128  | Unknown  | Yes  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Pin 68 (Tone Rx GT) = $4128.0
    +++++++--- (unknown)
    
    Read
    76543210
    |||||+++-- (unlikely to exist)
    ||||+----- $4128.3 = Pin 69 (Tone Rx D1)
    |||+------ $4128.4 = Pin 70 (Tone Rx D2)
    ||+------- $4128.5 = Pin 71 (Tone Rx D4)
    |+-------- $4128.6 = Pin 72 (Tone Rx D8)
    +--------- $4128.7 = Pin 73 (Tone Rx DV)
    

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $01. ($41 XOR $01 = $40 existence mask)
    * Value $41 from previous fetch also observed changed to $F9. ($41 XOR $F9 = $B8 existence mask)
    * Value $40 from previous fetch observed changed to $F8. ($40 XOR $F8 = $B8 existence mask)
    * Value $FF from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $FF. ($BF XOR $FF = $40 existence mask)
    * $40 OR $B8 OR $B8 OR $40 = $F8 accumulated existence mask.
  * ROM $E599 reads from this address.
  * ROM $E5BB reads from this address and uses bits 3,4,5,6.
  * ROM $F3F7 writes value #$00 to this address.

  
$4129  | Unknown  | Unknown  | Yes  | P5 Expansion Port 
    
    
    Write
    76543210
    ||||++++-- Data nybble written to device attached to P5 connector.
    ||++------ Used for sequencing writes to the device.
    ++-------- (unknown)
    
    Read
    76543210
    ||++++++-- Controlled by device attached to P5 connector, though the ROM code never reads it.
    ++-------- Not used
    

  * The data bus floats when reading this register, presumably to be driven by a device connected to P5.
  * Pin 23 is a /CE low when writing to this register (untested for reads).
  * Data bits 6 and 7 are not available in the P5 connector.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $FF with pull-ups on data bus.
    * Value $41 from previous fetch observed changed to $00 with pull-downs on data bus.
    * Value $41 from previous fetch observed changed to $BF, when data bus configured with pull-ups and -downs to make $BF.
    * Value $40 from previous fetch observed changed to $BF, when data bus configured with pull-ups and -downs to make $BF.
    * Value $FF from previous fetch observed changed to $BF, when data bus configured with pull-ups and -downs to make $BF.
  * Sequence: 
    * ROM $E4B6 writes a value to this address.
    * ROM $E4BB writes previous value again, OR'd with #$10.
    * ROM $E4C0 writes previous value again, AND'd with #$2F.

  
$412F  | Unknown  | Yes  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- $412F.0 = Timer 1 NMI enable.  1 = Enabled.
    ||||+++--- (unknown)
    |||+------ $412F.4 = Pin 33 (MSM6827L /INT) IRQ enable. 1 = Enabled.
    ||+------- $412F.5 = Pin 73 (Tone Rx DV) IRQ enable.  1 = Enabled.
    |+-------- $412F.6 = Timer 2 IRQ enable.  1 = Enabled.
    +--------- $412F.7 = UART Rx IRQ enable.  1 = Enabled.
    
    Read
    76543210
    |||||||+-- Bit exists but function is unknown
    |||||++--- (unlikely to exist)
    ||||+----- Bit exists but function is unknown
    |||+------ $412F.4 = !(Pin 33) (MSM6827L /INT)
    ||+------- $412F.5 is set when IRQ is triggered by writing 1 to $412F.5.
    ||           This is probably the Tone Rx DV interrupt flag.
    |+-------- $412F.6 is an IRQ flag.  1 = IRQ pending.
    +--------- Bit exists but function is unknown
    

  * ROM tends to clear $412F.0 immediately after disabling interrupts with SEI, and sets it right before enabling with CLI. 
    * The ROM is apparently disabling/enabling both IRQs and the timer NMI this way.
  * None of the write bits in this register prevent external NMI generated from pin 29 low.
  * None of the write bits prevent exteral IRQ generated from pin 28 low either.
  * Pin 33 (MSM6827L /INT) low triggers interrupt.
  * Pin 73 (Tone Rx DV) high triggers interrupt.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $08. ($41 XOR $08 = $49 existence mask)
    * Value $41 from previous fetch also observed changed to $20. ($41 XOR $20 = $61 existence mask)
    * Value $40 from previous fetch observed changed to $20. ($40 XOR $20 = $60 existence mask)
    * Value $BF from previous fetch observed changed to $26. ($BF XOR $26 = $99 existence mask)
    * $49 OR $61 OR $60 OR $99 = $F9 accumulated existence mask.
  * ROM implements a shadow read register at $0015.
  * ROM implements a shadow write register at $0016.
  * ROM $E31C clears bit 0 of this register.
  * ROM $E33B writes to this address, setting bit 0 to 1.
  * ROM $E4D3 reads from this address in the IRQ handler and uses bit 6.
  * ROM $F377 writes value #$00 to this address.
  * ROM $F40A writes value #$C1 to this address.
  * ROM $F4E1 writes value #$00 to this address.
  * ROM $F4E9 writes the shadow register to this address.
  * ROM $F99C writes value #$00 to this address.
  * Bits 7, 6, and 0 differ in values written, suggesting that they exist.

  
$4130  | Unknown  | No  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Serial bit to be written
    +++++++--- (unknown, unlikely to exist)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * The buffer at RAM $0330 is written to this register 1 bit at a time, seemingly using only bit 0 of the register.
  * The most significant bit of each byte is sent first.
  * 40 bytes are sent from $0330, then only the 6 most significant bits of the 41st byte. 
    * A total of 326 bits are sent to this register.
  * Note: CPU2 commands $10 and $11 fill this buffer.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, $26, and $BF from previous fetch observed unaffected.
  * ROM $FF2D writes to this address.
  * ROM $FF40 writes to this address.

  
$4131  | Unknown  | No  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Serial bit to be written
    +++++++--- (unknown, unlikely to exist)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * The buffer at RAM $0360 is written to this register 1 bit at a time, seemingly using only bit 0 of the register.
  * The most significant bit of each byte is sent first.
  * 40 bytes are sent from $0360, then only the 6 most significant bits of the 41st byte. 
    * A total of 326 bits are sent to this register.
  * Note: CPU2 commands $10, $11, and $6A fill this buffer.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.
  * ROM $FF51 writes to this address.
  * ROM $FF64 writes to this address.

  
$4132  | Unknown  | No  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Serial bit to be written
    +++++++--- (unknown, unlikely to exist)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * The buffer at RAM $0390 is written to this register 1 bit at a time, seemingly using only bit 0 of the register.
  * The most significant bit of each byte is sent first.
  * 40 bytes are sent from $0390, then only the 6 most significant bits of the 41st byte. 
    * A total of 326 bits are sent to this register.
  * Note: CPU2 commands $10, $11, and $6A fill this buffer.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.
  * ROM $FF75 writes to this address.
  * ROM $FF88 writes to this address.

  
$4133  | Unknown  | No  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    |||||||+-- Serial bit to be written
    +++++++--- (unknown, unlikely to exist)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * The buffer at RAM $0320 is written to this register 1 bit at a time, seemingly using only bit 0 of the register.
  * The most significant bit of each byte is sent first.
  * 4 bytes are sent from $0320, then only the 4 most significant bits of the 5th byte. 
    * A total of 36 bits are sent to this register.
  * Note: CPU2 commands $10 and $11 fill this buffer.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BF from previous fetch observed unaffected.
  * ROM $FF99 writes to this address.
  * ROM $FFAC writes to this address.

  
$4134  | Yes  | Yes  | Unknown  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    |||||||+-- Bit exists but function is unknown
    +++++++--- (unlikely to exist)
    

  * This register has serial data that can be read 1 bit at a time from $4134.0. 
    * Without any other register reads/writes, the next bit is provided the next time this register is read.
  * In each byte read, the most significant bit comes first.
  * CPU2 built-in ROM reads 40 bytes this way and stores them starting at RAM $03C0. 
    * Then 6 additional bits are read and stored as the 41st byte bits 7:2, and 0 for bits 1:0.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Value $41 from previous fetch observed changed to $40. ($41 XOR $40 = $01 existence mask)
    * Value $40 from previous fetch observed unaffected.
    * Value $BF from previous fetch observed changed to $BE. ($BF XOR $BE = $01 existence mask)
    * $01 OR $01 = $01 accumulated existence mask.
  * ROM $FFB8 reads this address and uses bit 0
  * ROM $FFCD reads this address and uses bit 0

  
$4135  | Unknown  | Yes  | Unknown  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    |||||||+-- $4135.0 = $4134 Serial Data Ready (1 = ready)
    +++++++--- (unlikely to exist)
    

  * If $4135.0 = 1, Built-in ROM proceeds to read the serial data from $4134.
  * Note: Built-in ROM only treats this register as a flag and does not treat it as serial data.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BE from previous fetch observed unaffected.
    * Value $41 from previous fetch also observed changed to $40. ($41 XOR $40 = $01 existence mask)
    * Value $BF from previous fetch observed changed to $BE. ($BF XOR $BE = $01 existence mask)
    * $01 OR $01 = $01 accumulated existence mask.
  * ROM $FD35 reads this address and uses bit 0.

  
$4136  | Unknown  | No  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Built-in ROM writes #$00 to $4137, then writes serial data to $4130, $4131, $4132, $4133, then writes #$00 to this register.
  * Theory: This is a "start" flag that starts using new data from $4130-4133 when the flag is cleared.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BE from previous fetch observed unaffected.
  * ROM $F74A writes value #$00 to this address.

  
$4137  | Unknown  | No  | Yes  | Unknown Function. 
    
    
    Write
    76543210
    ++++++++-- (unknown)
    
    Read
    76543210
    ++++++++-- (unlikely to exist)
    

  * Built-in ROM writes #$00 to this register, then writes serial data to $4130, $4131, $4132, $4133, then writes #$00 to $4136.
  * Theory: This is a "clear" flag that resets write bit counters and/or clears old data from $4130-4133 when the flag is cleared.

| Reference Data   
---  
  
  * Read Bit Existence: 
    * Values $41, $40, and $BE from previous fetch observed unaffected.
  * ROM $F3FA writes value #$00 to this address.
  * ROM $F73B writes value #$00 to this address.

  
  
# Communication Between Famicom and CPU2

Registers $40D0, $40D1, $40D2, and $40D3 are used for communication between Famicom and CPU2. One would tend to expect the Famicom to receive the same value from any of these four registers if read back right after writing. However, each register is actually a _separate_ register in each direction. The Famicom only controls the value read by CPU2, and CPU2 only controls the value read by the Famicom. It may be that CPU2 echoes the value back in some cases, but don't be fooled. 

Data packets are sent in both directions between the Famicom and CPU2 using these registers. The data flow is controlled by status and acknowledge flags in $40D3, and data is sent 3 bytes at a time using registers $40D0, $40D1, and $40D2. When CPU2 receives each 3-byte chunk, it buffers it in its RAM starting at address $0401 until the full message has been received. The maximum message length is at most 255 bytes, possibly less. 

Each message starts with a command byte, followed by a byte count. The byte after the byte count is not used and not counted towards the byte count in most commands. There are a total of 25 command bytes, which are stored in a lookup table at CPU2 ROM address $FB52. The index of this lookup table corresponds to the index of a function pointer table at address $FBB2, and a command mode support bitfield table at address $FB6B. This is how CPU2 efficiently directs to a unique message handler function for each command byte. The mode bitfield checks against the mode byte at $0051. It is probably used for enforcing the correct sequence of commands, as the command handlers themselves seem to be a major contributor changing the mode. It seems there are 6 possible modes: 0, 1, 2, 3, 4, and 5, though command $03 is set to support modes 6 and 7 as well. The mode may actually represent a global state machine, such as 0 = disconnected, 1 = dialing, etc. 

## CPU2 Commands

### Commands Read by the Famicom from CPU2

Command  
Byte  | Description   
---|---  
$80  | This command is received by the Famicom in response to writing to command $00. (See next section for details.)   
$81  | Unknown function. 
    
    
    [$81] [count=$01] [$00] [connection status]

  * It is unknown what events trigger this command to be sent to the Famicom.
  * [connection status] comes from RAM $0052, see section below for enumerated values.
  * This response can only happen in mode 3 via NMI.
  * Theory: This command is a response to writing to command $01.

  
$82  | Unknown function. 
    
    
    [$82] [count=$01] [$00] [connection status]

  * It is unknown what events trigger this command to be sent to the Famicom.
  * [connection status] comes from RAM $0052, see section below for enumerated values.

Response: 

  * [parameter] $00 means success, all other values indicate an error. 
    * When successful, the mode changes to $02.
    * When unsuccessful, the mode changes to $00.
  * [parameter] comes from RAM $0052
  * This response can only happen in mode 4 via NMI.
  * Theory: This command is a response to writing to command $02.

  
$83  | This command is received by the Famicom in response to writing to command $03. (See next section for details.)   
$90  | This command is received by the Famicom in response to writing to command $10. (See next section for details.)   
$91  | This command is received by the Famicom in response to writing to command $11. (See next section for details.)   
$92  | This command is received by the Famicom in response to writing to command $12. (See next section for details.)   
$C0  | UART Rx Packet There are 2 different parts of the CPU2 code that can send this message: 
    
    
    [$C0] [count=N] [parameter] [...N bytes...]

  * This command can only be sent to the Famicom in mode 2.
  * [parameter] comes from RAM $0024.
  * [...N bytes...] are read from register $4110, most recent reads first, 1 byte read from $4110 per IRQ.
  * Count N does not include the parameter byte.
  * This is serial data received from the Oki MSM6827L integrated modem chip.
  * This response can only happen in mode 2 via IRQ.

  

    
    
    [$C0] [count=$01] [$00] [parameter]

  * It is unknown what events trigger this command to be sent to the Famicom.
  * The modem abruptly disconnects when sending this command.
  * [parameter] can be #$00 or #$01.
  * This response can only happen in mode 2 via IRQ.

  
$C1  | Tone Rx Data Packet. When the packet only contains 1 nybble: 
    
    
    [$C1] [count=$01] [$80] [tone Rx nybble]

  * [tone Rx nybble] is register ($4128 read value >> 3) & 0x0F.

  
When there is more than 1 nybble in the packet: 
    
    
    [$C1] [count=N] [parameter] [tone Rx bytes]

  * [parameter] is either $00 or $01 based on the value of RAM $007A. 
    * Theory: [parameter] may indicate an even or odd number of nybbles in the packet.
  * Each Tone Rx byte is packed with 2 nybbles.
  * Count N does not include the parameter byte.

  
$E1  | This command indicates that CPU2 had a failure receiving a command from the Famicom. Command from Famicom that failed due to invalid command byte, or a command not supported in the present mode: 
    
    
    Message from Famicom:
    [invalid command] [count] [byte 0] [byte 1] ...
    
    Response back to Famicom:
    [$E1] [count=$03] [?] [$00] [invalid command] [byte 1]

  * [?] byte appears to be open to sending an artifact from the previous response.
  * Observed JRA-PAT rev 05 read this response after attempting to write to command $01 after a failed connection. 
    * E1 03 (00) 00 01 01

  
Command from Famicom that failed due to invalid byte count: 
    
    
    Message from Famicom:
    [command] [invalid count] [byte 0] [byte 1] ...
    
    Response back to Famicom:
    [$E1] [count=$03] [$00] [$01] [command] [byte 1]  
  
$F0  | This command is received by the Famicom in response to writing to command $7C. (See next section for details.)   
  
### Commands Written by the Famicom to CPU2

Note that most descriptions are _incomplete_. 

Command  
Byte  | CPU2  
Handler  
Address  | Byte  
Count  
Expected  | Modes  
Supported  | Description   
---|---|---|---|---  
$00  | $F43A  | >= 0  
<= 60  | 0  | Hook And Dialing Sequence Configuration 
    
    
    Message Bytes:
    [$00] [count=N] [xx] [...N bytes command string...]
    
    Response:
    [$80] [count=$01] [$00] [status byte]

  * [xx] byte is ignored.
  * All observed software uses this command when opening a connection.
  * Changes to Command Mode 1 when the full message has been received from the Famicom.
  * [command string] is a series of one-byte commands executed in Command Mode 1. 
    * $00-$1F: ignored
    * $20: wait up to 3 seconds for call progress (dial) tone to stop 
      * Fails with status $03 if the tone never stops
    * $21: switch to DTMF dialing
    * $22: switch to normal (10pps) pulse dialing
    * $23: switch to fast (20pps) pulse dialing
    * $24: turn on audio output
    * $25: turn off audio output
    * $26: disconnect phone line (on hook)
    * $27: connect phone line (off hook)
    * $28-$2F: ignored
    * $30: dial 0 (10 pulses)
    * $31-$39: dial 1-9 (1-9 pulses)
    * $3A: dial D (no pulses)
    * $3B: dial * (11 pulses)
    * $3C: dial # (12 pulses)
    * $3D: dial A (13 pulses)
    * $3E: dial B (14 pulses)
    * $3F: dial C (15 pulses)
    * $40-7F: currently unknown, but not ignored
    * $80-FF: delay 0.2-25.6 seconds (in 200ms increments)

Response: 

  * [connection status] comes from RAM $0052, see section below for enumerated values. 
    * When successful, the mode changes to $02.
    * When unsuccessful, the mode changes to $00.
  * This response can only happen in mode 1 via NMI.

| Reference Data   
---  
  
  * Starts NMI timer 1 with period $0006 (5 msec), one-shot: 
    * Writes #$06 to $4100
    * Writes #$00 to $4101
    * Writes #$02 to $4102
  * Observed Messages (unused bytes in parentheses): 
    * Super Mario Club rev 09: 
      * 00 13 (00) 42 31 20 36 33 57 30 36 30 37 38 31 34 34 31 33 25 67 25 (A2 01)
      * "...B1 63W0607814413%g%¢."
      * Note: The first '%' is definitely not a '$' in this one.
    * Heart no Benrikun Mini rev 01 A: 
      * 00 13 (00) 42 31 20 36 33 57 30 36 30 33 31 33 35 38 35 35 24 67 25 (37 31)
      * "...B1 63W0603135855$g%71"
    * Okasan no Famicom Trade rev 01: 
      * 00 13 (00) 42 31 20 36 33 57 30 36 30 33 31 33 33 33 39 30 24 67 25 (00 00)
      * "...B1 63W0603133390$g%.."
    * JRA-PAT rev 05: 
      * 00 0F (00) 88 30 42 30 20 36 34 32 35 36 34 36 36 36 24
      * "...ˆ0B0 642564666$"
    * JRA-PAT rev 05, with phone number manually set to 6666666666 
      * 00 0F (00) 42 36 20 36 36 36 36 36 36 36 36 36 24 67 25
      * "...B6 666666666$g%"
    * PiT Motorboat Race rev 02, with phone number manually set to 1234567890: 
      * 00 0E (00) 24 42 31 20 32 33 34 35 36 37 38 39 30 00 (00)
      * "...$B1 234567890.."

  


  * Okasan no Famicom Trade rev 01 responses observed: 
    * 80 01 00 01 after failure due to no phone line connected, reported Error 4001.
    * 80 01 00 03 after failure due to dial tone not going away, reported Error 4003.
    * 80 01 00 06 after failure due to server never answering, reported Error 4006.
  * PiT Motorboat Race rev 02 responses observed: 
    * 80 01 00 01 after failure due to no phone line connected.
    * 80 01 00 0D after failure due to server never answering.
  * JRA-PAT rev 05 responses observed: 
    * 80 01 00 01 after failure due to no phone line connected, reported Error 390010019999.
    * 80 01 00 03 after failure due to dial tone not going away, reported Error 390030019999.
    * 80 01 00 04 after failure due to server never answering, reported Error 390040019999.
  * The response [parameter] byte comes from RAM $0052.

  
$01  | $F46C  | 1  | 2  | Unknown Function. 
    
    
    Message Bytes:
    [$01] [count=$01] [xx] [parameter]

  * [xx] byte is ignored.
  * [parameter] Values: 
    * $00 = Write #$00 to $0031
    * $01-FF = Write #$02 to $0031
  * JRA-PAT and PiT Motorboat Race use this command when closing a connection.
  * Observed Messages (unused bytes in parentheses): 
    * JRA-PAT rev 05: 
      * 01 01 (00) 01 (30 42)
    * PiT Motorboat Race rev 02: 
      * 01 01 (00) 01 (02 31)
  * Checks if the byte received is 00 and if so, acts differently.
  * Starts NMI timer 1 with period $0006 (5 msec), one-shot: 
    * Writes #$06 to $4100
    * Writes #$00 to $4101
    * Writes #$02 to $4102
  * Changes to Command Mode 3 when complete.
  * Theory: This command generates response command $81 (see previous section).

  
$02  | $F48A  | 2  | 0  | Unknown Function. 
    
    
    Message Bytes:
    [$02] [count=$02] [xx] [parameter 0] [parameter 1]

  * [xx] byte is ignored.
  * [parameter 0] gets written to $002E.
  * [parameter 1] gets written to $002B and $002C.
  * 9 zero-page addresses get zeroed out; seems to be initializing something.
  * Starts NMI timer 1 with period $0006 (5 msec), one-shot: 
    * Writes #$06 to $4100
    * Writes #$00 to $4101
    * Writes #$02 to $4102
  * Changes to Command Mode 4 when complete.
  * Theory: This command generates response command $82 (see previous section).

  
$03  | $F4BB  | 0  | 0,1,2,3,  
4,5,6,7  | System Revision and Status Information 
    
    
    Message Bytes:
    [$03] [count=$00]
    
    Response:
    [$83] [count=$0A] [$00] [ROM revision] [ROM checksum MSB] [ROM checksum LSB] [byte 4] ... [byte 10]

  * Writes to $4120.
  * Writes $412F shadow register to $412F (update only, no apparent modification).
  * Response: 
    * [byte 1] = Comes from ROM address $FFF9, which seems to be a ROM revision byte. Observed value $03.
    * [byte 2], [byte 3] = 16-bit ROM checksum. Computed at bootup with the function located at $FD7D. It adds each byte of ROM.
    * [byte 4] = Value read from register $4121 & #$3F. If RAM $0023 is not equal to #$04, it also sets bit 7.
    * [byte 5] = Value read from register $4126.
    * [byte 6] = Value most recently written to $4127.
    * [byte 7] = Present mode.
    * [byte 8] = Number of 256-byte data blocks presently queued up in RAM range $1300-$1FFF.
    * [byte 9] = Value of RAM $0076.
    * [byte 10] = Value of RAM $0077.
  * Observed Messages (unused bytes in parentheses): 
    * Super Mario Club rev 09 at power-on: 
      * Write: 03 00 (00)
      * Read: 83 0A 00 03 0C AF A1 FB FE 00 0D 00 00
    * JRA-PAT rev 05 and 06 at power-on: 
      * Write: 03 00 (00)
      * Read: 83 0A 00 03 0C AF 81 FB FE 00 0D 00 00

  
$10  | $F71A  | 40  | 0,2,5  | Unknown $413x Function. 
    
    
    Message Bytes:
    [$10] [count=$28] [parameter] [...40 bytes...]
    
    Response:
    [$90] [count=$28] [response parameter] [...40 bytes...]

  * This command is similar to command $11, except using pre-defined data from ROM instead of the additional chunks in command $11.
  * [parameter] is used but does not count towards the byte count.
  * [parameter] value #$00 - #$0F: 
    * Pre-defined ROM data gets copied from tables.
    * 40 bytes from ROM are copied starting at RAM address $0360. ROM table index using parameter bits 2,1,0. 
      * $0388 (i.e. 41st byte) set to #$00.
      * See Register $4131.
    * 40 bytes from ROM are copied starting at RAM address $0390. ROM table index using parameter bits 1,0. 
      * $03B8 (i.e. 41st byte) set to #$00.
      * See Register $4132.
    * 40 bytes from the command are copied starting at RAM address $0330. 
      * $0358 (i.e. 41st byte) set to #$00.
      * $0330 (i.e. 1st byte) set to #$00.
      * See Register $4130.
    * 5 static bytes from ROM are copied starting at RAM address $0320: 01 40 50 00 60 
      * See Register $4133.
  * [parameter] value #$10: 
    * Skips the $0360 and $0390 ROM copy routines if RAM $0055 == #$00.
    * If $0055 is not #$00, it aborts the command.
  * Writes #$00 to $4137
  * Writes the 4 data chunks shifting 1 bit at a time to $4130, $4131, $4132, $4133
  * Writes #$00 to $4136
  * For the response, the count has not been confirmed to include [response parameter] or not.
  * The response comes from RAM starting at $03C0. (See register $4134.)

| ROM Data   
---  
Data Loaded to $0330 (which later gets written to register $4130): (These are the 40 bytes supplied by the Famicom to this command, with 1st byte and 41st byte set to $00.)   
Data Loaded to $0360 (which later gets written to register $4131):  | Parameter Bits  | Data   
---|---  
xxxxx000  | 00000c6bbc41351b1e0fa372963f7af704415090ee9a139ff87c60ca610dec66f74bd5b992c8868300   
xxxxx001  | 0000027473468ebd1f161331ad0e5512ba56487f7b30037dac78ed9c51b308b6d8c597ca8b59cc1300   
xxxxx010  | 000001c89b4630b518cbc342a45282fa0908773df56830150ec1955c9ec1abbd1100d4d878604afd00   
xxxxx011  | 00000939c25f1859304000e9fdab927959a49d1acdd136da4ddb2b551b2bc942a36077d1d09dbc3500   
xxxxx100  | 3cfed7ad4cd7435c3544144572d1d0e83d1859864bf8c7412cac015ced49a1cabce13fbd6262af0b00   
xxxxx101  | 8aac02df739c8c0a2572493bd95755450b06ea0d77f1f08bc665636ed3cf317cb89f6f301ef03a2f00   
xxxxx110  | 39b07c4f12614dadb4eb214af3a6a0dc046b47d10c184b04d16c91d9eeb5e0c21a052529738c6f0100   
xxxxx111  | 36a6a31fa79ba49a9f1d1985c640ad9a6c79a4195d06ece8726fc006e1d347efa5d129cdb79017b900   
Data Loaded to $0390 (which later gets written to register $4132):  | Parameter Bits  | Data   
---|---  
xxxxxx00  | 882a14099f3f32b2419d0b09237a252ece6534274700628043a6df86657a84c0c4463fa72b49ad4900   
xxxxxx01  | bbb57552f19d8a71d6ed68fd037188b700eab270071acedc624ff4c64b53ed1c4e1046ed89e775fd00   
xxxxxx10  | cfb8e78ec54b81a1100f02f524acf0e05a45cc36a98f161c115a41082abb587ecabad5453511877100   
xxxxxx11  | aa189432166db82500fffbb1107ced2469b12f0959b263c63a0332bb4a8e6ea378a0a47150cb296100   
Data Loaded to $0320 (which later gets written to register $4133):  | Parameter Bits  | Data   
---|---  
xxxxxxxx  | 0140500060   
$11  | $F75C  | 127  | 0,2,5  | Unknown $413x Function. 
    
    
    Message Bytes:
    [$11] [count=$7F] [...5 byte chunk...] [...41 byte chunk 0...]
                      [...41 byte chunk 1...] [...41 byte chunk 2...]
    
    Response:
    [$91] [count=$29] [response parameter] [...41 bytes...]

  * Note that the first byte of the 5 byte chunk is not ignored in this command like it is in most others, though it still does not count towards the byte count.
  * The 5 byte chunk gets written to RAM starting at $0320 with lots of bit shifting and byte swapping. (See Register $4133.)
  * The 41 byte chunk 0 gets copied to RAM starting at $0330. (See Register $4130.)
  * The 41 byte chunk 1 gets copied to RAM starting at $0360. (See Register $4131.)
  * The 41 byte chunk 2 gets copied to RAM starting at $0390. (See Register $4132.)
  * Shares the remaining code in common with command $10.
  * For the response, the count has not been confirmed to include [response parameter] or not.
  * The response comes from RAM starting at $03C0. (See register $4134.)

  
$12  | $F773  | >= 3  
<= 252  | 0,2,5  | CRC Calculator 
    
    
    Message Bytes:
    [$12] [count=N] [initial] [polynomial 1] [polynomial 2] [...N-2 bytes...]
    
    Response:
    [$92] [count=$02] [$00] [CRC 2] [CRC 1]

  * The [initial] byte does not count towards the byte count.
  * See next section for information about CRC keys.
  * Doesn't seem to write to any registers.

  
$40  | $F7AF  | >= 1  
<= 252  | 2  | Unknown Function. 
    
    
    Message Bytes:
    [$40] [count=N] [xx] [...N-1 bytes...]

  * [xx] byte is ignored, but _does_ count towards the byte count.
  * Copies the N-1 bytes data directly into memory starting at the location +2 pointed to by $0000:0001.
  * Copies the byte count to the location +1 pointed to by $0000:0001.
  * Writes value $FF directly to the location pointed to by $0000:0001. 
    * Not known what sets up that pointer.
  * This command is tricky to disassemble because it actually does copy the [xx] byte, then writes over it with the byte count, ultimately not using it.
  * Theory: This command sends a UART Tx Packet to the Oki MSM6827L, and connected with response command $C0.

  
$41  | $F7D0  | >= 1  
<= 100  | 5  | Unknown Function. 
    
    
    Message Bytes:
    [$41] [count=N] [xx] [...N bytes...]

  * [xx] byte is ignored and does not count towards the byte count.
  * Writes ((Byte count + 2) * 2) to the location +1 pointed to by $0000:0001.
  * Writes value $00 to the location +2 pointed to by $0000:0001.
  * Writes value $0F to the location +3 pointed to by $0000:0001.
  * For each of N bytes: 
    * Writes upper nybble of the byte (>> 4) to the next byte pointed to by $0000:0001.
    * Writes the lower nybble of the byte (& $0F) to the next byte pointed to by $0000:0001.
  * Appends the value of $007C similarly as 2 nybbles at the end.
  * Writes value $FF directly to the location pointed to by $0000:0001. 
    * Not known what sets up that pointer.
  * Theory: This command is something to do with the Tone Rx chip, and connected with response command $C1.

  
$60  | $F829  | 6  | 0  | Unknown Function. 
    
    
    Message Bytes:
    [$60] [count=$06] [...6 data bytes...]

  * Note that the first data byte is not ignored in this command, and it _does_ count towards the byte count.
  * The 6 data bytes written by this command get copied to CPU2 RAM starting at address $84.
  * PiT Motorboat Race uses this command when opening a connection.
  * Observed Messages (unused bytes in parentheses): 
    * PiT Motorboat Race rev 02: 
      * 60 06 00 01 80 00 00 00 (00)

  
$61  | $F839  | 0  | 1,3,4  | Disconnect 
    
    
    Message Bytes:
    [$61] [count=$00]

  * This command abruptly disconnects the modem.
  * Writes #$00 to register $4102. (Stops NMI timer 1.)
  * Changes mode to 0 when complete.

  
$62  | $F849  | 1  | 0,2,5  | Unknown Function. 
    
    
    Message Bytes:
    [$62] [count=$01] [xx] [parameter]

  * [xx] byte is ignored.
  * Disables interrupts while executing this command.
  * Valid values of the [parameter] byte are $00 and $01. 
    * Other values abort the command.
  * Appears to write to $4120

  
$63  | $F887  | 0  | 0,1,2,3,  
4,5  | NOP Command 
    
    
    Message Bytes:
    [$63] [count=$00]

  * This command only validates the byte count = #$00 and is then complete.

  
$64  | $F88D  | 1  | 0,2,5  | Modem Off Hook Control 
    
    
    Message Bytes:
    [$64] [count=$01] [xx] [Modem Off Hook mode]

  * [xx] byte is ignored.
  * [Modem Off Hook mode] Values: 
    * $00 = Don't update but save mode as $00.
    * $01 = Set $4127.4 (/Modem Off Hook) = 0 (off hook). (Also stores #$00 to $0049)
    * $02-FF = Set $4127.4 (/Modem Off Hook) = 1 (hang up). (Also stores #$00 to $0049)
    * Value gets saved to $004E.
  * PiT Motorboat Race uses this command when closing a connection.
  * Observed Messages (unused bytes in parentheses): 
    * PiT Motorboat Race rev 02: 
      * 64 01 (00) 02 (A9 31)
  * Note: This description is fairly complete.

  
$65  | $F89D  | 2  | 0,2,5  | Modem Audio Enable and P4-19 Control 
    
    
    Message Bytes:
    [$65] [count=$02] [xx] [Modem Audio Enable mode] [P4-19 mode]

  * [xx] byte is ignored.
  * [Modem Audio Enable mode] Values: 
    * $00 = Don't update but save mode as $00.
    * $01 = Set $4127.6 (/Modem Audio Enable) = 0 (enabled to TV speakers).
    * $02-FF = Set $4127.6 (/Modem Audio Enable) = 1 (disabled).
    * Value gets saved to $004F.
  * [P4-19 mode] Values: 
    * $00 = Don't update but save mode as $00.
    * $01 = Set $4127.7 (Modem P4-19) = 0.
    * $02-FF = Set $4127.7 (Modem P4-19) = 1.
    * Value gets saved to $0050.
  * Heart no Benrikun Mini rev 01 A uses this command when closing a connection.
  * Observed Messages (unused bytes in parentheses): 
    * Heart no Benrikun Mini rev 01 A: 
      * 65 02 (00) 02 00 (20)
  * Note: This description is fairly complete.

  
$66  | $F8B7  | 2  | 0,2,5  | LED Control 
    
    
    Message Bytes:
    [$66] [count=$02] [xx] [red LED mode] [green LED mode]

  * [xx] byte is ignored.
  * [red LED mode] Values: 
    * $00 = Don't update but save mode as $00.
    * $01 = Set Red LED on.
    * $02-7F = Set Red LED off.
    * $80-FF = Set Red LED to be controlled automatically by network status.
    * Value gets saved to $004C.
  * [green LED mode] Values: 
    * $00 = Don't update but save mode as $00.
    * $01 = Set Green LED on.
    * $02-7F = Set Green LED off.
    * $80-FF = Set Green LED to be controlled automatically by network status.
    * Value gets saved to $004D.
  * PiT Motorboat Race uses this command when closing a connection.
  * Observed Messages (unused bytes in parentheses): 
    * PiT Motorboat Race rev 02: 
      * 66 02 (00) 02 02 (31)
  * Note: This description is fairly complete.

  
$67  | $F8D1  | 2  | 0,5  | Unknown Function. 
    
    
    Message Bytes:
    [$67] [count=$02] [xx] [parameter 0] [parameter 1]

  * [xx] byte is ignored.
  * If [parameter 0] is $00: 
    * Abruptly disconnects the modem.
    * Zeroes out RAM $0076 and $0077.
    * Changes to mode 0 when complete.
  * Else if [parameter 0] is not $00: 
    * Writes [parameter 0] to $74.
    * Writes [parameter 1] to $75.
    * Refreshes Green LED based on its mode.
    * Refreshes P4-27 (Phone Off Hook) based on its mode.
    * Zeroes out RAM $76,77,37,39,38.
    * Refreshes P4-21 (Phone Audio Enable) based on its mode.
    * Zeroes out all queued up outgoing modem Tx messages at RAM $1300-1FFF. 
      * Each message has space $100 and number of queued messages is at RAM $0100.
    * Zeroes out Tone Rx incoming data buffer at RAM $0500-05FF.
    * Changes to mode 5 when complete.

  
$68  | $F912  | 1  | 0,5  | Unknown Function. 
    
    
    Message Bytes:
    [$68] [count=$01] [xx] [parameter]

  * [xx] byte is ignored.
  * The code has 3 different paths depending on parameter. 
    * [parameter] < #$10
    * [parameter] == #$FF
    * All other values of [parameter]

  
$69  | $F951  | 10  | 0  | Unknown Function. 
    
    
    Message Bytes:
    [$69] [count=$0A] [xx] [...10 data bytes...]

  * [xx] byte is ignored.
  * Observed Messages (unused bytes in parentheses): 
    * PiT Motorboat Race rev 02 when opening a connection: 
      * 69 0A (00) 00 04 14 1D 28 80 00 30 26 21 (00 00)
    * JRA-PAT rev 05 and 06 at power-on: 
      * 69 0A (00) 00 04 14 1D 28 30 00 30 26 21 (00 00)
  * The 10 data bytes are copied to CPU2 RAM starting at $8A.
  * If the 6th data byte (location $8F) is > #$FC, it gets overwritten by #$01.

  
$6A  | $F96F  | 80  | 0,2,5  | Unknown $413x Function. 
    
    
    Message Bytes:
    [$6A] [count=$50] [xx] [...40 byte chunk 0...] [...40 byte chunk 1...]

  * [xx] byte is ignored.
  * Copies the 40 byte chunk 0 to RAM starting at $0360. (See Register $4131.)
  * Copies the negative (2's complement) of each byte in 40 byte chunk 1 to RAM starting at $0390. (See Register $4132.)
  * Writes #$00 to $0388 and $03B8. (i.e. 41st byte of each.)
  * Writes #$10 to $0055.
  * Observations: 
    * No updates were made to buffers at $0320, $0330 from this command. (i.e. register $4133, $4130 data)
    * The command does not appear to initiate any operations to the $413x registers.
  * Note: This description is fairly complete.

  
$7B  | $F994  | No  
Check  | 0,1,2,3,  
4,5  | Software Reset 
    
    
    Message Bytes:
    [$7B] [count=xx] [xx]

  * Does not care about any bytes beyond the command byte.
  * Refreshes the green LED.
  * Disables interrupts.
  * Writes #$00 to $412F.
  * Sorts through some memory comparing values then generates a software reset. 
    * Stores #$09 as the reset type code into $00FF.

  
$7C  | $F9AC  | 5  | 0,1,2,3,  
4,5  | Arbitrary Memory Read 
    
    
    Message Bytes:
    [$7C] [count=$05] [xx] [CRC 1] [CRC 2] [address MSB] [address LSB] [read count N]
    
    Response:
    [$F0] [read count N] [$00] ...N bytes from memory...

  * [xx] byte does not count towards the byte count.
  * See next section for information about CRC bytes.
  * If [address MSB] >= #$80, exits early with no response. (Prevent ROM dump?? :) )
  * If [read count N] >= #$39, exits early with no response.

  
$7D  | $F9D9  | >= 5  | 0,1,2,3,  
4,5  | Arbitrary Memory Write 
    
    
    Message Bytes:
    [$7D] [count=N] [xx] [CRC 1] [CRC 2] [address MSB] [address LSB] [...N-4 bytes...]

  * [xx] byte does not count towards the byte count.
  * See next section for information about CRC bytes.
  * count - 4 is the number of bytes to write to memory.
  * Writes $00 to $006E.
  * Super Mario Club rev 09 uses this command at the welcome screen. 
    * 7D 1A (00) 07 60 01 04 09 01 06 E0 16 A2 90 A5 29 F0 06 C9 FF D0 04 A2 01 86 29 4C 20 F4 (30)
    * CRC bytes: 07 60
    * Starting address: $0104 (farthest reaches of the stack)

| Disassembly   
---  
      
    
           .org $0104
           .dd2 T0109   ;Wrote over Main Super-loop Function Pointer, now executes code below.
           .dd2 TE006   ;Wrote over IRQ Handler Function Pointer, now immediately exits IRQ.
           .dd1 $16     ;Satisfy periodic checksum of bytes $102-109.
          
    T0109  ldx  #$90    ;Arbitrary code inserted.
           lda  $29
           beq  L0115
           cmp  #$ff
           bne  L0117
           ldx  #$01
    L0115  stx  $29
    L0117  jmp  $f420   ;Continue to original main super-loop.
    
           ; Code Below Existing In CPU2 ROM:
           .org $e006
    TE006  jmp  TE502
          
           .org $e502
    TE502  ply          ;Return from IRQ.
           plx
           pla
           rti  
  
  * JRA-PAT rev 05 and 06 use this command at power-on. 
    * 7D 29 (00) EC AD 01 02 1A 01 09 01 06 E0 DB A2 90 A5 29 F0 06 C9 FF D0 04 A2 01 86 29 4C 20 F4 AD 32 00 C9 05 D0 03 EE 32 00 4C 39 E8 (00)
    * CRC bytes: EC AD
    * Starting address: $0102 (farthest reaches of the stack)

Disassembly   
---  
      
    
           .org $0102
           .dd2 T011A   ;Wrote over NMI function pointer, now executes code below.
           .dd2 T0109   ;Wrote over Main Super-loop Function Pointer, now executes code below.
           .dd2 TE006   ;Wrote over IRQ Handler Function Pointer, now immediately exits IRQ.
           .dd1 $db     ;Satisfy periodic checksum of bytes $102-109.
          
    T0109  ldx  #$90    ;Arbitrary code inserted.
           lda  $29
           beq  L0115
           cmp  #$ff
           bne  L0117
           ldx  #$01
    L0115  stx  $29
    L0117  jmp  $f420   ;Continue to original main super-loop.
          
    T011A  lda  $0032   ;Arbitrary NMI handler code inserted.
           cmp  #$05
           bne  L0124
           inc  $0032
    L0124  jmp  TE839
          
           ; Code Below Existing In CPU2 ROM:
           .org $e006
    TE006  jmp  TE502
          
           .org $e502
    TE502  ply          ;Return from IRQ.
           plx
           pla
           rti
          
           .org $e839
    TE839  ply          ;Return from NMI.
           plx
           pla
           rti  
  
$7E  | $F9FE  | 5  | 0,1,2,3,  
4,5  | Unknown Function. 
    
    
    Message Bytes:
    [$7E] [count=$05] [xx] [CRC 1] [CRC 2] [parameter 0] [parameter 1] [parameter 2]

  * [xx] byte does not count towards the byte count.
  * See next section for information about CRC bytes.
  * Writes a lot of stuff to $4120 and $4121
  * Writes to $4127 bit 5 (/DTMF Output Enable)

  
$7F  | $FA16  | 5  | 0,1,2,3,  
4,5  | Unknown Function. 
    
    
    Message Bytes:
    [$7F] [count=$05] [parameter 0] [CRC 1] [CRC 2] [parameter 1] [parameter 2] [parameter 3]

  * [parameter 0] byte does not count towards the byte count.
  * See next section for information about CRC bytes.
  * If [parameter 0] is $00-7F, write value $0C out the P5 Expansion Port.
  * If [parameter 0] is $80-FF, dump RAM $0240-$0280 out the P5 Expansion Port. 
    * The P5 Expansion port sequences out 1 nybble at a time.
  * [parameter 1] gets written to $00B7.
  * [parameter 2] gets written to $00B6.
  * [parameter 3] gets written to $00B8.
  * [parameter 0] gets written to $00B5.

  
  
### CPU2 Commands with CRC key bytes

CPU2 command $12 calculates the CRC-16 of its message using [polynomial 1] and [polynomial 2] as the CRC polynomial, with [initial] placed in both bytes of the initial value, and performs the calculation LSB-first or "reflected". This command requires the polynomial to be specified in reciprocal form. For example, to calculate the standard CRC-16 with polynomial $8005, [polynomial 1] would be set to $40, [polynomial 2] would be set to $03, and [initial] would be set to $00. 

CPU2 Commands $7C, $7D, $7E, and $7F use CRC key bytes [CRC 1] and [CRC 2] to validate the message. These commands calculate a CRC-16 across the message backwards, starting from the last byte and ending at [CRC 1]. The result must be 0 or the command is ignored. The CRC-16 polynomial is $8385 and the initial value is $35AC. The calculation is done LSB-first, or "reflected". 

### Connection Status Byte

Response commands $80, $81, and $82 send an enumerated value indicating the result when attempting to make a telephone connection. These enumerations are used clearly in the CPU2 ROM: 00, 01, 02, 03, 04, 05, 06, 07, 08, 0B, 0C, 0D, 0E. CPU2 ROM keeps track of this value at RAM location $0052. $00 indicates a successful connection, and all other values indicate a failure. Super Mario Club shows these failures with error code 40xx, xx directly reflecting this byte. Super Mario Club's manual gives troubleshooting information for most of these values. That information was used to create the table below. 

Status  
Byte  | Error  
Code  | Description   
---|---|---  
$00  | (n/a)  | Connection was successful.   
$01  | 4001  | No dial tone was detected.  | Reference Data   
---  
Super Mario Club Manual:  | おもな原因 (Main Cause) | 対処方法 (Workaround)   
---|---  
電話線がはずれています。  
The telephone line is off.  | 

  * 通信アダプタの電話回線の差し込みへ電話線を接続してください。
  * Connect the telephone line to the telephone line plug of the communication adapter.

  


  * 回線テストのテスト1(29ページ) を行ってください。
  * Perform Test 1 (page 29) of the line test.

  
既に電話が使われています。  
The phone is already in use.  |  通話が終わるまで待ってください。  
Please wait until the call ends.   
ホー厶テレホン用のモジュラーコンセントなどにテレホンスイッチを取りつけていませんか?  
Is the telephone switch attached to a modular outlet for the Ho 厶 telephone?  |  テレホンスイッチがNTT の局線に直接取りつけられるように工事を行ってください。  
Make sure that the telephone switch can be attached directly to the NTT station line.   
  
Kangyou Sumimaru no Famicom Trade Manual: 

原 因•対応方法 (Cause • Countermeasure)   
---  
電話線は正常に接続されていますか？  
Is the telephone line connected normally?  
  
自宅の電話が使用中ではありませんか？  
Is your home phone in use?   
$02  | 4002  | An incoming phone call is being received.  | Reference Data   
---  
Super Mario Club Manual:  | おもな原因 (Main Cause) | 対処方法 (Workaround)   
---|---  
電話がかかってきました。  
I got a call.  |  受話器をお取りください。  
Please pick up the handset.   
  
Kangyou Sumimaru no Famicom Trade Manual: 

原 因•対応方法 (Cause • Countermeasure)   
---  
もう一度ご利用ください。  
Please use it again.   
$03  | 4003  | Pulse vs. Tone setting is incorrect.  | Reference Data   
---  
Super Mario Club Manual:  | おもな原因 (Main Cause) | 対処方法 (Workaround)   
---|---  
プッシュ回線とダイヤル回線の設定を誤っています。  
The push line and dial line settings are incorrect.  |  通信アダプタ裏の切換スイッチを回線に合わせてください。  
Set the changeover switch on the back of the communication adapter to match the line.  
  
(番号をかけた時にプッシュ回線の場合はピ・ポ・パと鳴りますのでPBにセットしてください。それ以外はダイヤル回線ですので、20にセットしてください。 

  * 部の地域では 10 にセットしていただく場合があります。)  


(If you use a push line when you dial a number, you will hear a beep, so set it to PB. Other than that, it is a dial line, so set it to 20. 

  * Set it to 10 in some areas. You may be asked to do so.)

  
テレホンスイッチを新電電アダプタ等の後に取りつけていませんか?  
Is the telephone switch installed after the new electric adapter, etc.?  |  テレホンスイッチを新電電アダプタより前(NTTの局線側)に取りつけてください。  
Install the telephone switch in front of the new electric adapter (NTT station line side).   
  
Kangyou Sumimaru no Famicom Trade Manual: 

原 因•対応方法 (Cause • Countermeasure)   
---  
ご利用電話の種類の設定が誤っています。  
The phone type setting is incorrect.  
  
通信アダプタ裏の電話回線切換スイッチをセットしください。  
Set the telephone line selector switch on the back of the communication adapter.  
  
任天堂の通信アダプタセット取扱説明書をご覧下さい。  
Please refer to the Nintendo communication adapter set instruction manual.   
$04  | 4004  | "0" Outgoing call setting is incorrect.  | Reference Data   
---  
Super Mario Club Manual:  | おもな原因 (Main Cause) | 対処方法 (Workaround)   
---|---  
0発信の設定が誤っています。  
The 0 outgoing call setting is incorrect.  
  
(0発信電話なのに0発信の設定がなされていません。 又は直通電話なのに0発信の設定がされています。)  
(0 outgoing call is not set even though it is a 0 outgoing call, or 0 outgoing call is set even though it is a direct call.)  |  登録内容を見直してください。(訂正する場合は、28ページをご参照ください。)  
Please review the registered contents. (Refer to page 28 for corrections.)  
  
  
テレホンスイッチの取りつけが誤っています  
Wrong installation of telephone switch  |  通信機器の接続 (7,8ページ) をご確認ください。  
Check the connection of communication devices (pages 7 and 8).   
  
Kangyou Sumimaru no Famicom Trade Manual: 

原 因•対応方法 (Cause • Countermeasure)   
---  
電話回線は利用可能となっていますか？  
Is the telephone line available?  
  
ご利用の電話は「ゼロ発信」ではありませんか？  
Isn't your phone "zero call"?   
$05  | 4005  | DDX-TP connection completed earlier than expected.  | Reference Data   
---  
Super Mario Club Manual:  | おもな原因 (Main Cause) | 対処方法 (Workaround)   
---|---  
DDX-TPの開通 (工事) が完了していないと思われます。  
It seems that the opening (construction) of DDX-TP has not been completed.  
  
| 

  * 回線テストのテスト2 (29ページ) を行ってください。(DDX-TP網IDの場合)
  * Perform Test 2 (page 29) of the line test. (For DDX-TP network ID)

  


  * NTTの113番に電話して確認してください。
  * Please call NTT 113 to confirm.

  
テレホンスイッチの取りつけが誤っています。  
The telephone switch is installed incorrectly.  |  通信機器の接続(7, 8ページ) をご確認ぐださい。  
Check the connection of communication devices (pages 7 and 8).   
  
Kangyou Sumimaru no Famicom Trade Manual: 

原 因•対応方法 (Cause • Countermeasure)   
---  
  
  * 113に電話して、NTTにDDXの工事完了を確認して下さい。
  * Call 113 to confirm with NTT that the DDX construction has been completed.

  
$06  | 4006  | Unknown.   
$07  | 4007  | Unknown.   
$08  | 4008  | Server was busy, disconnected, or rejected the password.  | Reference Data   
---  
Super Mario Club Manual:  | おもな原因 (Main Cause) | 対処方法 (Workaround)   
---|---  
センター側が混雑しています。  
The center side is crowded.  |  しばらく経ってから再度お試しください。  
Please try again after a while.   
センターが停止しています。  
The center is stopped.  |  サービス時間を確認してください。  
Please check the service time.   
DDX-TP の開通 (工事) が完了していないと思われます。  
It seems that the opening (construction) of DDX-TP has not been completed.  | 

  * 回線テストのテスト2(29ページ) を行ってください。(DDX- TP綱IDの場合)
  * Perform Test 2 (page 29) of the line test. (For DDX-TP rope ID)

  


  * NTTの113番に電話して確認してください。
  * Please call NTT 113 to confirm.

  
テレホンスイッチの取りつけが誤っています。  
The telephone switch is installed incorrectly.  |  通信機器の接続(7,8ページ) をご確認ください。  
Check the connection of communication devices (pages 7 and 8).   
DDX-TPパスワードで、登録に誤りがあります。  
There is an error in the registration of the DDX-TP password.  |  パスワード番号とパスワード登録電話番号をご確認ください。  
Please check your password number and password registration phone number.   
  
Kangyou Sumimaru no Famicom Trade Manual:  
Same as 4005.   
  
$09  | 4009  | Server was busy.  | Reference Data   
---  
Super Mario Club Manual:  
Same as 4008.  
  
Kangyou Sumimaru no Famicom Trade Manual:  | 原 因•対応方法 (Cause • Countermeasure)   
---  
相手方が話中です。しばらくして、再度かけ返して下さい。  
The other party is busy. Please call back again after a while.  
  
  
$0A  | 400A  | DDX-TP registration incorrect or server disconnected.  | Reference Data   
---  
Super Mario Club Manual:  | おもな原因 (Main Cause) | 対処方法 (Workaround)   
---|---  
DDX-TPの登録内容に誤りがあります。  
There is an error in the registered contents of DDX-TP.  |  NTTの113番に電話して登録内容の確認をしてください。  
Please call NTT 113 to confirm your registration details.   
センターが停止しています。  
The center is stopped.  |  サービス時間を確認してください。(15ページ)  
Please check the service time. (Page 15)   
  
Kangyou Sumimaru no Famicom Trade Manual: 

原 因•対応方法 (Cause • Countermeasure)   
---  
再度、お試し下さい。  
Please try again.  
  
頻繁に発生する場合にはファミコントレード事務局へお電話下さい。  
If it occurs frequently, please call the NES Trade Secretariat.   
$0B  | 400B   
$0C  | 400C   
$0D  | 400D  | Problem with telephone line connection.  | Reference Data   
---  
Super Mario Club Manual:  | おもな原因 (Main Cause) | 対処方法 (Workaround)   
---|---  
通信アダプタから電話線がはずれました。  
The telephone line has been disconnected from the communication adapter.  |  通信アダプタと電話線の接続を確認してください。  
Check the connection between the communication adapter and the telephone line.   
$0E  | 400E  | Problem with telephone line connection or server.  | Reference Data   
---  
Super Mario Club Manual:  
Same as 400D.  
  
Kangyou Sumimaru no Famicom Trade Manual:  
Same as 400A.   
  
# Pinouts

## RF5C66 Mapper and Disk Drive Controller
    
    
                                                           _____
                                                          /     \
                                               CPU A0 -> / 1 100 \ -- +5Vcc
                                              CPU A1 -> / 2    99 \ -- n/c
                                             CPU A2 -> / 3      98 \ <> CPU D0
                                            CPU A3 -> / 4        97 \ <> CPU D1
                                           CPU A4 -> / 5          96 \ <> CPU D2
                                          CPU A5 -> / 6            95 \ <> CPU D3
                                         CPU A6 -> / 7              94 \ <> CPU D4
                                        CPU A7 -> / 8                93 \ <> CPU D5
                                      CPU A12 -> / 9                  92 \ <> CPU D6
                                     CPU A13 -> / 10                   91 \ <> CPU D7
                                    CPU A14 -> / 11                     90 \ -- GND
                                   /ROMSEL -> / 12                       89 \ <> Card D0
                                  CPU R/W -> / 13                         88 \ <> Card D1
                                      M2 -> / 14                           87 \ <> Card D2
              P6-1 Lid Switch, Card R/W <- / 15                             86 \ <> Card D3
              (20k resistor to 5Vcc) ? -> / 16                               85 \ <> Card D4
                                 /IRQ <- / 17                                 84 \ <> Card D5
                               +5Vcc -- / 18                                   83 \ <> Card D6
                                n/c -- / 19                                     82 \ <> Card D7
                  21.47727MHz Xtal -- / 20                                       81 \ -- +5Vcc
                             Xtal -- / 21                                            \
                             n/c -- / 22                                     O       /
                            GND -- / 23                                          80 / -- n/c
            (n/c) Xtal Osc Out <- / 24                                          79 / -> Exp P3-2
                          n/c -- / 25                                          78 / <- Exp P3-3
       ToneRx Xin, CIC Clock <- / 26              Nintendo RF5C66             77 / -> Exp P3-4
                        n/c -- / 27       Package QFP-100, 0.65mm pitch      76 / -> Exp P3-5
             (n/c) $40AE.0 <- / 28                                          75 / -> Exp P3-6
     CIC /CPU R/W Inhibit -> / 29                Mapper and                74 / <- Exp P3-7
          Key CIC-12 (?) <- / 30           Disk Drive Controller          73 / <- Exp P3-8
                           /       O                                     72 / <- Exp P3-9
                           \                                            71 / <- Exp P3-11
          Key CIC-10 (?) -> \ 31                                       70 / -- GND
           Key CIC-15 (?) -> \ 32                                     69 / -> 5A18-49
               CPU2 /Reset -> \ 33                                   68 / -> CPU2 /Reset (new rev)
        CHR RAM /CE (input) -> \ 34                                 67 / <> Exp P3-12           Orientation:
                     RAM +CE <- \ 35                               66 / <> Exp P3-13            --------------------
                (n/c) $40C0.1 <- \ 36                             65 / <> Exp P3-14                 80         51
     CPU2 /Reset (old rev: J2) <- \ 37                           64 / <> Exp P3-15                   |         |
                    CHR RAM /CE <- \ 38                         63 / <- $40B1.3 (n/c)               .-----------.
                             GND -- \ 39                       62 / <> Modem P4-31               81-|O Nintendo |-50
    Built-in RAM /CE ($6000-7FFF) <- \ 40                     61 / <> Modem P4-32                   |  RF5C66   |
          (n/c) ? /CE ($4xE0-4xEF) <- \ 41                   60 / <> Modem P4-29                100-|  GCD 4R  O|-31
           5A18-85 /CE ($4xD0-4xDF) <- \ 42                 59 / -- +5Vcc                           \-----------'
                             (GND) ? -> \ 43               58 / -- n/c                               |         |
                              (GND) ? -> \ 44             57 / -> Kanji ROM A17                     01         30
                               (GND) ? -> \ 45           56 / -> Kanji ROM A4
                                (GND) ? -> \ 46         55 / -> Kanji ROM A3          Legend:
                               CIRAM A10 <- \ 47       54 / -> Kanji ROM A2           ------------------------------
                                  PPU A11 -> \ 48     53 / -> Kanji ROM A1            --[RF5C66]-- Power
                                   PPU A10 -> \ 49   52 / -> Kanji ROM A0             ->[RF5C66]<- RF5C66 input
                 Kanji ROM /CE ($5000-5FFF) <- \ 50 51 / -- n/c                       <-[RF5C66]-> RF5C66 output
                                                \     /                               <>[RF5C66]<> Bidirectional
                                                 \   /                                    f      Famicom connection
                                                  \ /                                     r      ROM chip connection
                                                   V                                      R      RAM chip connection
    Notes:
    - +5Vcc pins 18, 59, 81, 100 are all connected together internally.
    - GND pins 23, 39, 70, 90 are all connected together internally.
    - 43, 44, 45, 46 are GND on the PCB, but have internal protection diodes from GND, suggesting they are logic pins.
      - Pins 45-46, when pulled high, causes oscillation on pin 56.
    - 24, 28, 36, 37, 41, 63 are n/c on the PCB, but function as noted.
    - /CE Pins 40, 41, 42, 50 behaviors:
      - Pin 40 (Built-in RAM /CE) activates low in range $6000-7FFF, regardless of CPU R/W, but only when M2 is high.
      - Pin 41 (Unknown /CE) always activates low in range $4xE0-4xEF, regardless of CPU R/W and M2.
      - Pin 42 (RF5A18 CPU2 /CE) always activates low in range $4xD0-4xDF, regardless of CPU R/W and M2.
      - Pin 50 (Kanji ROM /CE) activates low in range $5000-5FFF, regardless of CPU R/W.  (It appears to go low only when M2 is high but not specifically proven.)
    - Pin 29:
      - If the FNS has a CIC chip, CIC-11 drives this pin high after 24.8msec, and remains high as long as the CIC authentication is successful.  There is a low-pass filter in this case.
      - If the FNS does not have a CIC chip, the pin floats high.  There may be a pull-up resistor somewhere.  The delay to go high (if any) has not been measured.
      - When this pin is low, it resets pins 52-57 low and possibly lots of other things.
      - Card R/W is always high when this pin is low.
    - Pin 31 (CIC-10) and Pin 32 (CIC-15):
      - Both are jumpered direct to GND if the FNS does not have a CIC chip.
      - If it does have a CIC chip, these signals are both always low with or without tsuushin card inserted.
      - In no observed case are these signals ever high.
    - Pin 16 Pull-up of 20k to 5V is also required in order to avoid triggering reset.
    - Pin 16 seems to be related to pin 29.  With pin 29 floating and pin 16 pulled high at power on, the chip runs for 5 seconds, then enters reset.
    - Tested 10k instead of 20k (per original PCB) on pin 16, found no difference in time or function.
    - Pin 69 has a high pulse of 11.9085 usec at any time that register $4xAC has not been read for 12.4892 seconds.
      - Each additional 12.4892 seconds generates another pulse.
      - It has very repeatable precision, at least 6 figures on each.
      - It is not synchronized to M2 or any other inputs.
      - Note that 12.4892 sec * 21.47727 MHz = 2^28, with an error of 0.075%. (Nominal would be 12.4986 sec.)
      - Note that 11.9085 usec * 21.47727 MHz = 2^8, with an error of 0.093%. (Nominal would be 11.9196 usec.)
    - Pins 52-56 drive the address pins of the Kanji ROM.  (See notes below the LH5323M1 pinout.)
    - Pin 15 (Card R/W) is a non-inverted buffer of CPU R/W.  This signal connects through the lid switch.
    - Pin 26 puts out a 3.58 MHz square wave, ~50% duty.  This corresponds to 21.47727 MHz / 6.
    - Pin 79 (Exp 2) puts out a 95.95 kHz square wave, 93.7% duty.  Clock source unknown.
      - Note that this seems similar to FDS serial bitrate.
      - Standalone chip can get into a 341.2 kHz mode when touching pin 80, though pulling 80 high or low doesn't correlate.
      - Either frequency, the negative pulse width is 650 nsec.
    - Pins 71-79 appear strikingly similar to an FDS interface.
    - CIRAM A10 follows PPU A10 by default.
    

## RF5A18 CPU2 / Modem Controller
    
    
                                                         _____
                                                        /     \
                                            CPU2 A0 <- / 1 100 \ -- GND
                                           CPU2 A1 <- / 2    99 \ <> CPU2 D0         
                                          CPU2 A2 <- / 3      98 \ <> CPU2 D1         
                                         CPU2 A3 <- / 4        97 \ <> CPU2 D2         
                                        CPU2 A4 <- / 5          96 \ <> CPU2 D3         
                                       CPU2 A5 <- / 6            95 \ <> CPU2 D4         
                                      CPU2 A6 <- / 7              94 \ <> CPU2 D5         
                                     CPU2 A7 <- / 8                93 \ <> CPU2 D6         
                                      +5Vcc -- / 9                  92 \ <> CPU2 D7         
                                   CPU2 A8 <- / 10                   91 \ -- +5Vcc
                                  CPU2 A9 <- / 11                     90 \ -> UART Tx (MSM6827L TXD)
                                CPU2 A10 <- / 12                       89 \ <- UART Rx (MSM6827L RXD)
                               CPU2 A11 <- / 13                         88 \ <- CPU A2
                              CPU2 A12 <- / 14                           87 \ <- CPU A1
                       (n/c) CPU2 A13 <- / 15                             86 \ <- CPU A0
                      (n/c) CPU2 A14 <- / 16                               85 \ <- /CE (5C66-42)
                     (n/c) CPU2 A15 <- / 17                                 84 \ <- P6-1 Lid Switch, Card R/W
                               GND -- / 18                                   83 \ <- M2
       (2.4576 MHz) (n/c) CPU2 M2 <- / 19                                     82 \ <> Card D7
                        CPU2 R/W <- / 20                                       81 \ <> Card D6
           RAM /CE ($0000-1FFF) <- / 21                                            \
    (n/c) ROM /CE ($C000-xFFF) <- / 22                                     O       /
          P5 /CE ($4129 Only) <- / 23                                          80 / <> Card D5
           (GND) CPU2 +Reset -> / 24                                          79 / <> Card D4
                    (GND) ? -> / 25                                          78 / -- GND
      /Internal ROM Enable -> / 26             Nintendo RF5A18              77 / <> Card D3
    (5C66-68) CPU2 /Reset -> / 27      Package QFP-100, 0.65mm pitch       76 / <> Card D2
      (10k up) CPU2 /IRQ -> / 28                                          75 / <> Card D1
     (10k up) CPU2 /NMI -> / 29             Modem Controller             74 / <> Card D0
                   n/c -- / 30                   CPU2                   73 / <- Tone Rx DV
                         /       O                                     72 / <- Tone Rx D8
                         \                                            71 / <- Tone Rx D4
                 +5Vcc -- \ 31                                       70 / <- Tone Rx D2
          MSM6827L DATA <> \ 32                                     69 / <- Tone Rx D1
           MSM6827L /INT -> \ 33                                   68 / -> Tone Rx GT
             MSM6827L /RD <- \ 34                                 67 / -> Exp P3-19
              MSM6827L /WR <- \ 35                               66 / <> Exp P3-18
             MSM6827L EXCLK <- \ 36                             65 / <> Exp P3-17
               (n/c) $4120.2 <- \ 37                           64 / -- +5Vcc                  Orientation:
                 MSM6827L AD1 <- \ 38                         63 / -> Modem P4-19             --------------------
                  MSM6827L AD0 <- \ 39                       62 / -> /Phone Audio Enable          80         51
                  (n/c) CPU2 D0 <- \ 40                     61 / -> /DTMF Output Enable            |         |
                         (n/c) ? <- \ 41                   60 / -> /Phone Off Hook                .-----------.
           (4.9152 MHz) Exp P3-16 <- \ 42                 59 / -> $4127.3 (n/c)                81-|O  RF5A18  |-50
                               GND -- \ 43               58 / -> /Green LED                       |  Nintendo |
                    19.6608MHz Xtal -- \ 44             57 / -> /Red LED                      100-|  GCD 8C  O|-31
                          1k to Xtal -- \ 45           56 / -> MSM6827L +Reset                    \-----------'
                                  GND -- \ 46         55 / <- Audio from phone line                |         |
                       (+5Vcc) $4126.0 -> \ 47       54 / <- Modem P4-28                          01         30
                     (+5Vcc) !($4126.1) -> \ 48     53 / <- Modem P4-25             
                                 5C66-69 -> \ 49   52 / <- Switch SW1-4             Legend:
                                      n/c -- \ 50 51 / <- Switch SW1-2              ------------------------------
                                              \     /                               --[RF5A18]-- Power, n/a
                                               \   /                                ->[RF5A18]<- RF5A18 input
                                                \ /                                 <-[RF5A18]-> RF5A18 output
                                                 V                                  <>[RF5A18]<> Bidirectional
    Notes:
    - This chip contains its very own 65C02 CPU, with built-in ROM.
    - +5Vcc pins 9, 31, 64, 91 are all connected together internally.
    - GND pins 18, 43, 46, 78, 100 are all connected together internally.
    - 24, 26 are GND on the PCB, but function as noted.
    - 25 is GND on the PCB, but has internal protection diode from GND, suggesting it is a logic pin.
    - 47, 48 are +5Vcc on the PCB, but function as noted.
    - 15, 16, 17, 19, 22, 37, 40, 59 are n/c on the PCB, but function as noted.
    - 41 is n/c on the PCB, but has protection diode from GND, suggesting it may have a function.
    - Pin 42 (Exp 16) puts out a 4.92 MHz square wave, ~50% duty.  This is 19.6608 MHz / 4.
    - CPU2 /Reset comes from RF5C66 pin 68 on new revisions and selectable with J1, J2 on old revisions:
      - J2 closed = RF5C66 pin 37 (default)
      - J1 closed = RF5C66 pin 68
    - Pin 24 prevents CPU2 functioning when held high at power-on.  If the pin is then driven low, the reset vector is then fetched after that.
      - Pin 24 can be freely used as a +reset at any time this way.
    - Pin 25 low at any time causes address bus to go to $FFFF and data bus shows a toggle on bits 2,5,6,7: period 208.7 usec, low for 7.93 usec.
      - Other data bits always low.
      - Shortly after applying power, the toggle has a lot of variations for a period of about 1.5 seconds, including a 225 msec gap where the bits are low.
      - The mentioned data bits all appear to have the same data.
      - Held low at power-on will fetch the reset vector later when driven high.
      - Held high at power-on, driven low later, enters the data bus toggle mode but:
        - Does not appear to fetch the reset vector when driven high after that.
        - Does execute code, possibly resuming from where it left off.
    - When pin 26 is set low (default case: This pin is tied directly to GND on the PCB):
      - Internal ROM is enabled in CPU2 range $E000-FFFF.
      - Open Bus in CPU2 range $C000-DFFF.
      - Pin 22 (ROM /CE) is enabled low in range $C000-DFFF
      - This mode allows ROM expansion at $C000-DFFF, with internal ROM (and its vector table) in place.
    - When pin 26 is set high:
      - Internal ROM is disabled, leaving open bus in CPU2 range $E000-FFFF.
      - Open Bus in CPU2 range $C000-DFFF.
      - Pin 22 (ROM /CE) is enabled low in the entire range $C000-FFFF.
      - This mode allows a totally custom external CPU2 ROM with its own interrupt vector table.
    

## LH5323M1 Kanji Graphic ROM
    
    
                                  _____  Note: Flat spot does not correspond to pin 1.
                                 /     \
                         n/c -- / 12 11 \ -- n/c
               (5C66-52) A0 -> / 13   10 \ -- n/c
                    CPU D0 <> / 14      9 \ <- A1 (5C66-53)
                   CPU D1 <> / 15        8 \ <- A2 (5C66-54)
                  CPU D2 <> / 16          7 \ <- A3 (5C66-55)
                    GND -- / 17            6 \ -- GND
                CPU D3 <> / 18              5 \ <- A5 (CPU A0)
               CPU D4 <> / 19                4 \ -- n/c
              CPU D5 <> / 20                  3 \ <- A6 (CPU A1)
             CPU D6 <> / 21                    2 \ <- A7 (CPU A2)
            CPU D7 <> / 22  Nintendo LH5323M1   1 \ -- n/c
                     /        Package QFP-44       \
                     \         0.8mm pitch         /
               n/c -- \ 23                     44 / <- A8 (CPU A3)
                n/c -- \ 24   Kanji Graphic   43 / <- A13 (CPU A8)
           (GND) /OE -- \ 25       ROM       42 / <- A16 (CPU A11)
         (CPU A6) A11 -> \ 26               41 / <- A4 (5C66-56)         Orientation:
         (5C66-50) /CE -> \ 27             40 / -- n/c                   --------------------
                    GND -- \ 28           39 / -- n/c                        33         23
            (CPU A7) A12 -> \ 29         38 / -- +5Vcc                        |         |
             (CPU A5) A10 -> \ 30       37 / <- A17 (5C66-57 Bankswitch)     .-----------.
                       n/c -- \ 31     36 / <- A15 (CPU A10)              34-| Nintendo O|-22
                        n/c -- \ 32   35 / -- n/c                            |  CCR-01   |
                 (CPU A4) A9 -> \ 33 34 / <- A14 (CPU A9)                    | LH5323M1  |
                                 \     /                                  44-|O 9528 D   |-12
                                  \   /                                      '-----------/
                                   \ /                                        |         |
                                    V                                        01         11
                                    
    Notes:
    - 6 & 28 are connected together internally.
    - 17 has no measurable connection to 6 & 28.
    - All logic pins have protection diode from pin 17, suggesting this is the true GND.
    - Pin 25 also appears as a logic pin with respect to pin 17.
    - When pins 25 and 27 are both driven low, the data bus becomes an output.  Otherwise it is hi-z.
    - Pins 13, 9, 8, 7, 41, 37 are controlled by the RF5C66.
      - Pins 13, 9, 8, 7, 41 are controlled with auto-increment function.
      - The value of these pins increments each M2 falling edge when the CPU is in range $5000-5FFF.
      - Pin 37 is a bankswitch, controlled by register $40B0.0
      - At reset and when reading from register $40B0, these pins reset to 0.
      - The conditions resetting or maintaining the bankswitch pin to 1 are still unknown.
    

## 8633 Famicom Network System CIC Key Chip

Unlike the NES console, the Famicom Network System appears to have the CIC key. 
    
    
                                     _______   _______
                                     |      \_/      |
     (To Card CIC Pin 2) Data Out <- | 1          18 | -- +5Vcc
    (From Card CIC Pin 1) Data In -> | 2  O       17 | -- n/c
                              n/c -- | 3   8633   16 | -- n/c
                              n/c -- | 4          15 | -> ? (5C66-32) always observed low
                              n/c -- | 5    CIC   14 | -- n/c
                              n/c -- | 6    Key   13 | -- n/c
                            Clock -> | 7          12 | <- ? (5C66-30)
    (From Card CIC Pin 11) +Reset -> | 8    U8    11 | -> /CPU R/W Inhibit (5C66-29)
                              GND -- | 9          10 | -> ? (5C66-31) always observed low
                                     |_______________|
    

  * When the CIC key drives pin 11 low, this stops operation of the Famicom Network System by means of holding Card R/W high.
  * The clock is 3.58 MHz, coming from RF5C66 pin 26.



## 8634A Tsuushin Card CIC Lock Chip

Unlike the NES cartridge, the tsuushin card appears to have the CIC lock. 
    
    
                                     _______   _______
                                     |      \_/      |
      (To FNS CIC Pin 2) Data Out <- | 1          18 | -- +5Vcc
     (From FNS CIC Pin 1) Data In -> | 2  O       17 | -- n/c
                              n/c -- | 3   8634A  16 | ?? GND
                              n/c -- | 4          15 | -- n/c
                              n/c -- | 5    CIC   14 | -- n/c
                              n/c -- | 6   Lock   13 | ?? +5V
                            Clock -> | 7          12 | ?? Card-33, n/c in Famicom Network System
               (Cap to 5V) +Reset -> | 8          11 | -> CIC Key +Reset (To FNS CIC Pin 8)
                              GND -- | 9          10 | -- n/c
                                     |_______________|
    

  * +Reset is connected with a ceramic capacitor to 5V. This gives a momentary positive pulse at power-on.
  * The clock is 3.58 MHz, coming from RF5C66 pin 26.
  * Note: Some assumptions made on CIC chips based on similarity to F411A from Super NES.



## 8kByte CHR RAM
    
    
                       _______   _______
                       |      \_/      |
               n/c? -- | 1          28 | -- +5Vcc
            PPU A12 -> | 2  O       27 | <- PPU /WR
             PPU A7 -> | 3          26 | <- +CE: U3=RF5C66 34/38, U4=PPU /A13
             PPU A6 -> | 4          25 | <- PPU A8
             PPU A5 -> | 5  LH5268  24 | <- PPU A9
             PPU A4 -> | 6    CHR   23 | <- PPU A11
             PPU A3 -> | 7    RAM   22 | <- /OE: PPU /RD
             PPU A2 -> | 8   U3/U4  21 | <- PPU A10
             PPU A1 -> | 9          20 | <- /CE: U3=PPU A13, U4=RF5C66 34/38
             PPU A0 -> | 10         19 | <> PPU D7
             PPU D0 <> | 11         18 | <> PPU D6
             PPU D1 <> | 12         17 | <> PPU D5
             PPU D2 <> | 13         16 | <> PPU D4
                GND -- | 14         15 | <> PPU D3
                       |_______________|
    

## 8kByte W-RAM
    
    
                       _______   _______
                       |      \_/      |
               n/c? -- | 1          28 | -- +5Vcc
            CPU A12 -> | 2  O       27 | <- /WR: Card R/W (P6-2 Lid Switch)
             CPU A7 -> | 3          26 | <- +CE: RAM +CE
             CPU A6 -> | 4          25 | <- CPU A8
             CPU A5 -> | 5  LH5268  24 | <- CPU A9
             CPU A4 -> | 6 Built-in 23 | <- CPU A11
             CPU A3 -> | 7  W-RAM   22 | <- /OE: GND
             CPU A2 -> | 8    U5    21 | <- Card A10
             CPU A1 -> | 9          20 | <- /CE: Built-in RAM /CE
             CPU A0 -> | 10         19 | <> Card D7
            Card D0 <> | 11         18 | <> Card D6
            Card D1 <> | 12         17 | <> Card D5
            Card D2 <> | 13         16 | <> Card D4
                GND -- | 14         15 | <> Card D3
                       |_______________|
    

## 8kByte CPU2 RAM
    
    
                       _______   _______
                       |      \_/      |
               n/c? -- | 1          28 | -- +5Vcc
           CPU2 A12 -> | 2  O       27 | <- /WR: CPU2 R/W
            CPU2 A7 -> | 3          26 | <- +CE: +5Vcc
            CPU2 A6 -> | 4          25 | <- CPU2 A8
            CPU2 A5 -> | 5  LH5268  24 | <- CPU2 A9
            CPU2 A4 -> | 6   CPU2   23 | <- CPU2 A11
            CPU2 A3 -> | 7    RAM   22 | <- /OE: GND
            CPU2 A2 -> | 8    U6    21 | <- CPU2 A10
            CPU2 A1 -> | 9          20 | <- /CE: CPU2 RAM /CE
            CPU2 A0 -> | 10         19 | <> CPU2 D7
            CPU2 D0 <> | 11         18 | <> CPU2 D6
            CPU2 D1 <> | 12         17 | <> CPU2 D5
            CPU2 D2 <> | 13         16 | <> CPU2 D4
                GND -- | 14         15 | <> CPU2 D3
                       |_______________|
    

## P4: Modem Module Edge Connector
    
    
           Famicom     | Modem  |    Famicom
        Network System | Module | Network System
                       __________
                       |        |
              +5Vcc -- | 1   19 | <- 5A18-63
    MSM6827L +Reset -> | 2   20 | <- Tone Rx GT
       MSM6827L AD0 <> | 3   21 | <- /Phone Audio Enable
                GND -- | 4   22 | -> Tone Rx DV
       MSM6827L AD1 <> | 5   23 | -> 5A18-55, Audio from phone line
       MSM6827L RXD <- | 6   24 | <- Tone Rx Xin, from 5C66-26
      MSM6827L DATA <- | 7   25 | -> 5A18-53
       MSM6827L TXD -> | 8   26 | -- GND
       MSM6827L /WR -> | 9   27 | <- /Phone Off Hook
     MSM6827L EXCLK -> | 10  28 | -> 5A18-54
       MSM6827L /RD -> | 11  29 | <> 5C66-60
              +5Vcc -- | 12  30 | <- /DTMF Output Enable
         Tone Rx D1 <- | 13  31 | <> 5C66-62           __________________________
      MSM6827L /INT <- | 14  32 | <> 5C66-61           | Modem Module           |
              +5Vcc -- | 15  33 | <- Audio from 2A03   | Orientation            |
         Tone Rx D2 <- | 16  34 | -> Audio to RF       |                        |
         Tone Rx D8 <- | 17  35 | -- GND               |    19 _____________ 36 |
         Tone Rx D4 <- | 18  36 | -- GND               |     1 |___________| 18 |
                       |________|                      |________________________|
     
    Note: The modem module uses modem chip Oki MSM6827L and Dual Tone Receiver MC14LC5436P.
    

## P2: Tsuushin Card Connector

Note that the tsuushin card may appear to have a metric 1mm pin pitch, but in fact it has an imperial 0.040" (40 thousandths) pin pitch. 
    
    
    Card |  | Famicom Network System
    -----+--+-------------------------
      1  |--| +5Vcc
      2  |--| +5Vcc
      3  |??| n/c in JRA-PAT card, n/c in FNS
      4  |??| n/c in JRA-PAT card, FNS has 10k pull-up only.
      5  |<>| Card D0
      6  |<>| Card D1
      7  |<>| Card D2
      8  |<>| Card D3
      9  |<>| Card D4
     10  |<>| Card D5
     11  |<>| Card D6
     12  |<>| Card D7
     13  |<-| Card R/W (P6-2 Lid Switch)
     14  |<-| M2
     15  |<-| /ROMSEL
     16  |<-| CPU A0
     17  |<-| CPU A1
     18  |<-| CPU A2
     19  |<-| CPU A3
     20  |<-| CPU A4
     21  |<-| CPU A5
         |  |
         |  |
     22  |<-| CPU A6
     23  |<-| CPU A7
     24  |<-| CPU A8
     25  |<-| CPU A9
     26  |<-| CPU A10
     27  |<-| CPU A11
     28  |<-| CPU A12
     29  |<-| CPU A13
     30  |<-| CPU A14
     31  |??| n/c in JRA-PAT card, FNS has 10k pull-up only.
     32  |??| n/c in JRA-PAT card, FNS has 10k pull-up only.
     33  |??| connected to Card Lock CIC-12 in JRA-PAT, n/c in FNS
     34  |??| n/c in JRA-PAT card, n/c in FNS
     35  |->| CIC Key Reset (Card Lock CIC-11 -> FNS Key CIC-8)
     36  |->| CIC Lock-to-Key Data (Card Lock CIC-1 -> FNS Key CIC-2)
     37  |<-| CIC Key-to-Lock Data (Card Lock CIC-2 <- FNS Key CIC-1)
     38  |<-| CIC Clock (3.58 MHz, from 5C66.26)
     39  |??| n/c in JRA-PAT card, FNS has 10k pull-up only.
     40  |<-| RAM +CE (n/c in JRA-PAT card)
     41  |--| GND
     42  |--| GND
    

## P3: Expansion Connector
    
    
                     Outside    |  FNS  |    Outside                   _____________________
                                _________                             / Orientation        /|
                                |       |                            /____________________/ |
                        /IRQ -> | 1  20 | -- +5Vcc                   |o|_| ==      |_||_|o|/
    (95.94kHz Clock) 5C66-79 <- | 2  19 | -> 5A18-67                 \_  _  _ _||_  _  _ _/|
                     5C66-78 -> | 3  18 | -> 5A18-66                  |-| |    || HVC-050| |
                     5C66-77 <- | 4  17 | -> 5A18-65                  |-|_|    ||        | |
                     5C66-76 <- | 5  16 | -> 5A18-42 (4.92MHz Clock)  |        ||        | |
                     5C66-75 <- | 6  15 | <> 5C66-64                  |  20 __/__\__ 11  | |
                                |       |                             |o  1 |______| 10 o| |
                     5C66-74 -> | 7  14 | <> 5C66-65                  | ________________ | |
                     5C66-73 -> | 8  13 | <> 5C66-66                  |/_______________/|| |
                     5C66-72 -> | 9  12 | <> 5C66-67                  ||______________|/ | |
                         GND -- | 10 11 | <- 5C66-71                  |                  | |
                                |_______|                             |      ______      | |
                                                                      |o    | |    |    o| |
                                                                      \_____|/     |_____|/
    

## P5: Expansion Connector

Note: This connector only exists on old revisions of Famicom Network System. Expansion P5 /CE is activated low specifically at CPU2 address $4129. 
    
    
    Outside |  | Famicom Network System
    --------+--+-------------------------
        9   |--| GND
        8   |<>| CPU2 D5
        7   |<>| CPU2 D4
        6   |<>| CPU2 D3
        5   |<>| CPU2 D2
        4   |<>| CPU2 D1
        3   |<>| CPU2 D0
        2   |<-| Expansion P5 /CE
        1   |--| +5V
    

# See Also

  * [https://forums.nesdev.org/viewtopic.php?f=9&t=18530](https://forums.nesdev.org/viewtopic.php?f=9&t=18530)
  * <https://niwanetwork.org/wiki/Tsuushin_Cartridge>
  * <https://niwanetwork.org/wiki/List_of_Famicom_Network_System_software>



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [Expansion audio](Category_Expansion_audio.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with IRQs](Category_Mappers_with_IRQs.xhtml), [Pinouts](Category_Pinouts.xhtml)
