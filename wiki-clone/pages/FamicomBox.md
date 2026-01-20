# FamicomBox

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FamicomBox) | View [other pages](Special_AllPages.xhtml#FamicomBox)

The Nintendo FamicomBox (SSS-CDS), also released as the Sharp **FamicomStation** (AN-560), is a commercial Famicom console with 16 cartridge slots, intended for use in hotels. One internal slot runs the menu software, while the 15 other slots are externally-visible and can be selected by the user through software. These slots take NES-form-factor 72-pin cartridges, but with a modified EXP pinout to support the system's unique 3198 lockout chip. In addition to free-play, play time can be purchased with coins, unlocked with a key, or tracked via a pay-to-play CATV interface. 

The FamicomBox hardware differs significantly from standard consoles. It adds 14 KiB of extra CPU RAM and various hardware registers controlling features such as a reset-based exception system. However, it also has significant limitations that impact its compatibility with the NES library, such as restricting cartridges to just the upper half of the address space, not connecting some of the cartridge pins, and allowing inactive cartridges to interfere with the active one. 

The menu software will refuse to run any game that neither matches a checksum in the menu's game database nor has a valid-enough [Nintendo header](Nintendo_header.xhtml "Nintendo header"). 

![](../wiki-images/Ambox_content.png) |  **The FamicomBox power supply frequently fails with high-voltage output, which can cause significant system-wide damage. It is strongly recommended that original power supplies be replaced with modern ones that support 100-240V input.**  
---|---  
![](../wiki-images/Ambox_content.png) |  **Because of the FamicomBox's different EXP pinout, inserting standard NES cartridges or Famicom adapters that use EXP pins into an unmodified FamicomBox may cause damage to the cartridge or console by shorting to 5V or ground.**  
---|---  
  
## Contents

  * 1 Physical description
  * 2 CPU address space
  * 3 Clock rates
  * 4 Features
    * 4.1 Slot selection
      * 4.1.1 Cartridge compatibility
    * 4.2 Reset
    * 4.3 TV and Game modes
    * 4.4 Exceptions
    * 4.5 Zapper detection
    * 4.6 40% brightness
    * 4.7 Coin timer and near-end flash
    * 4.8 CATV interface
  * 5 Registers
    * 5.1 Joypad out ($4016W)
    * 5.2 Joypad 1 in ($4016R)
    * 5.3 Joypad 2 in ($4017R)
    * 5.4 Exception enable ($5000W)
    * 5.5 Exception flags ($5000R)
    * 5.6 Payment ($5001W)
    * 5.7 LED and Memory protect ($5002W)
    * 5.8 DIP switches ($5002R)
    * 5.9 8-bit timer ($5003W)
    * 5.10 Keyswitch ($5003R)
    * 5.11 Slot select ($5004W)
    * 5.12 25P expand input ($5004R)
    * 5.13 Miscellaneous output ($5005W)
    * 5.14 Expansion input ($5005R)
    * 5.15 25P expand output ($5006W)
    * 5.16 Expansion input ($5006R)
    * 5.17 Expansion output ($5007W)
    * 5.18 Miscellaneous input ($5007R)
  * 6 DIP switch settings
    * 6.1 Software settings
    * 6.2 Hardware settings
  * 7 Pinouts (external)
    * 7.1 Cartridge connector
    * 7.2 15P expand (RP-1)
    * 7.3 25P expand (RP-2)
    * 7.4 CATV interface terminal (RP-3)
    * 7.5 DC in connector (RP-5)
    * 7.6 Expansion connector (P4)
    * 7.7 Joypad port 1 (FP-2)
    * 7.8 Joypad port 2 (FP-3)
    * 7.9 Joypad port 3 (FP-4)
    * 7.10 Coin box connector (FP-7)
    * 7.11 Coin box power connector (P8)
  * 8 Pinouts (internal)
    * 8.1 SSS-SOB connector (P1)
    * 8.2 SSS-SOB connector (P2)
    * 8.3 SSS-SOB connector (P3)
    * 8.4 RF modulator connector (P5)
    * 8.5 SSS-Interface connector (P6 / RP-4)
    * 8.6 SSS-REL connector (P7 / FP-1)
    * 8.7 SSS-LED connector (P19)
    * 8.8 SSS-LED connector (P20)
    * 8.9 SSS-LED connector (P21)
    * 8.10 SSS-SW connector (FP-5)
    * 8.11 Keyswitch (FP-6)
    * 8.12 3198A CIC lockout
    * 8.13 3199A coin timer
  * 9 Schematic errata
  * 10 Hardware configurations



## Physical description

The front of the FamicomBox has 3 columns of 5 cartridge slots each, with end labels visible through plastic windows. These are indexed 1-15 top to bottom, left to right. Each slot has an LED to the left. In the bottom center from left to right are a TV/GAME LED (illuminated in game mode), TV/GAME button, reset button, and keyswitch that can be rotated between 6 positions. Three controllers attach through the lower left. 

The front panel is closed via a tubular lock on each of the top left and top right. With the panel open, the 15 cartridge slots can be accessed. On the lower left is a board with 3 joypad ports, the third one used for the Zapper. This board also has a service button for adding credits. 

An optional coin box attaches to the right side of the console. It has a 100 yen coin slot, a coin drawer secured by a tubular lock, and an LED that indicates play time and flashes when play time is low. 

The left side of the console has a holder for the controllers and Zapper. 

The back of the console features various ports. On the left are input and output coaxial RF ports, a switch for channel 1 or 2, and composite AV ports for video and audio. Under these is a panel covering a 50-pin expansion edge connector. To the right are a DB-15 Famicom expansion port, DB-25 expansion port, and 8-pin CATV connector. The CATV connector is used for remote billing. To the right of these are a 5V DC center-negative power input and 10 DIP switches. Further right is a power supply that plugs into the wall, outputs 5V DC center negative via an external cable, has 2 AC output plugs for other devices, and a fuse. 

## CPU address space

  * $0000-1FFF: 8 KiB RAM
  * $2000-3FFF: PPU registers
  * $4000-401F: APU registers
  * $4020-4FFF: Unused
  * $5000-5FFF: FamicomBox registers
  * $6000-7FFF: 8 KiB RAM
  * $8000-FFFF: Cartridge space



The FamicomBox's CPU address space differs from that of the [standard console](CPU_memory_map.xhtml "CPU memory map"), adding RAM and registers and reducing space for cartridge mapping. Unlike normal systems, the FamicomBox cuts cartridges off from the CPU data lines when the CPU is accessing under $8000, preventing cartridges from mapping in the unused region. 

## Clock rates

The FamicomBox uses a 21.47727 MHz system clock. All other clocks are derived from this one. These are as follows: 

  * 3.579545 MHz: This divides the system clock by 3*2 and is referred to here as 3.58 MHz. It is used by the 3198 CIC and 3199 coin timer.
  * 873.9123 Hz: This divides the system clock by 3*(2^13) and is referred to here as 874 Hz. It is used to generate the near-end flash beep sound.
  * 6.827440 Hz: This divides the system clock by 3*(2^20) and is referred to here as 6.83 Hz. It is used by the 6.83 Hz period exception and the 8-bit timer.
  * 3.413720 Hz: This divides the system clock by 3*(2^21) and is referred to here as 3.41 Hz. It is used for the slot LED flash rate.
  * 0.8534300 Hz: This divides the system clock by 3*(2^23) and is referred to here as 0.85 Hz. It is used by the watchdog counter.



## Features

### Slot selection

The FamicomBox has 16 cartridge slots, separated into 4 "columns" containing slots 0, 1-5, 6-10, and 11-15. The slot and column are selected via $5004W. This selection controls multiplexing of various cartridge signals. Slot multiplexing is done on /ROMSEL to only enable the target slot's PRG-ROM, on PPU A13 so only the target slot sees that a PPU access is in the $0000-1FFF range, and on CIRAM A10 to allow the target slot to control nametable mirroring. Column multiplexing is present to combat fanout, ensuring signals are sufficiently strong, and is applied to the CPU and PPU data lines. 

Despite this multiplexing, inactive slots in the system are still connected enough that they may respond to accesses, potentially interfering with the active slot. This multiplexing, however, was sufficient for the FamicomBox's limited selection of games. 

#### Cartridge compatibility

See: Cartridge connector pinout

Cartridges are selected by multiplexing certain pins by slot, but some are by column, most aren't muxed at all, and some signals aren't even present on the slots. This leads to significant compatibility limitations. The following signals encounter problems: 

  * **/IRQ** : Most mappers that use /IRQ drive the pin at all times (despite it being intended to be open collector/drain), which interferes with IRQs from any other slot.
  * **M2** : Unlike /ROMSEL, M2 is not muxed. Cartridges that map under $8000 determine the value of CPU A15 using /ROMSEL and M2. Because /ROMSEL is held high for inactive slots, these cartridges will see any CPU access to $8000-FFFF as an access to $0000-7FFF, which may cause unintended mapper register or PRG-RAM accesses. For reads, this can cause bus conflicts with other slots. The CPU data lines are muxed by column, so conflicts will occur within a single column. On the active column, any such cartridges can cause bus conflicts with the active slot, corrupting the value seen by the CPU. These unintended accesses will also occur when explicitly accessing addresses under $8000, but because columns are isolated from the CPU data lines during such accesses, the cartridges cannot see nor influence the value on the CPU side.
  * **CPU D7..0** : The CPU data lines are only passed through when /ROMSEL is asserted, so cartridges cannot map nor snoop below $8000. Games that use PRG-RAM at $6000-7FFF will still work because the console provides RAM there, but it isn't battery-backed and any stateful RAM on the cartridge will be corrupted by writes (see M2 above). When disabled, it is suspected the SN74ALS245's that isolate the columns maintain the current line levels, effectively creating an open bus that does not decay.
  * **PPU D7..0** : The PPU data lines are only passed through when PPU /A13 is asserted, so cartridges cannot map nor snoop $2000 or above. This prevents cartridge nametable RAM or ROM. When disabled, it is suspected the SN74ALS245's that isolate the columns maintain the current line levels, effectively creating an open bus that does not decay.
  * **CIRAM /CE** : PPU /A13 is directly connected by the console to CIRAM /CE, so cartridges cannot disable this RAM for accesses at $2000 or above nor map CIRAM under $2000. This prevents cartridge nametable RAM or ROM and use of CIRAM as pattern table RAM.
  * **PPU A12** : Some cartridges place a capacitor on PPU A12, which can slow transitions enough to interfere with the active slot.
  * **PPU /A13** : This pin is not connected on the FamicomBox. While it's normally only used for controlling CIRAM /CE, both MMC5 and Sunsoft 4 mappers take PPU /A13 as an input instead of PPU A13.
  * **PPU /RD** and **PPU /WR** : All cartridges see that a PPU read or write is occurring, but for non-selected slots, PPU A13 multiplexing makes the address appear as though it is a nametable address, which is usually ignored. However, cartridges with nametable RAM or ROM may respond to reads and can cause bus conflicts in the same column, and cartridges with nametable RAM may be written by writes.
  * **EXP4** : This is used for CPU A15, which could cause issues for cartridges configured for an NES-001 expansion device.
  * **EXP0..3** , **EXP5..9** : These are tied to ground or +5V and can cause physical damage. Examples of susceptible cartridges include those with an opamp on EXP6 or which connect EXP0 to ground (to match the Famicom ground pin at this relative location).



### Reset

When the FamicomBox is reset, the CPU and PPU are both reset and the $5002W, $5004W, and $5005W registers are cleared. The system can be reset in a number of ways. While it does have a reset button, this button does not directly control reset, but instead uses the exception system. Reset occurs under any of the following conditions: 

  * The active slot provided incorrect CIC data, causing the host 3198 CIC to do a single reset pulse.
  * An exception occurred, resetting the host 3198 CIC for 1.76 ms. While in reset, the CIC's outputs are pulled down, activating its host /reset output.
  * The console is in TV Mode. The console can be switched back to Game Mode, releasing it from reset, by pressing the TV/GAME button. TV Mode is activated in the following ways: 
    * The TV/GAME button is pressed while in Game Mode.
    * CATV Billing Clear is asserted.



### TV and Game modes

The FamicomBox offers TV and Game modes. In Game Mode, the FamicomBox operates as normal, generating video and audio that it outputs through its RF modulator. In TV Mode, the FamicomBox is held in reset and its RF modulator passes its coaxial input through to its coaxial output. The FamicomBox powers up into Game Mode. It is switched from Game Mode to TV Mode by pressing the TV/GAME button or asserting CATV Billing Clear. It is returned to Game Mode by pressing the TV/GAME button. When entering TV Mode, the TV Mode latch on $5007R D0 is set, allowing menu software after returning to Game Mode to detect that the reset was caused by TV Mode. 

According to the manual's schematic, the FamicomBox was intended to allow the menu software to disable the TV/GAME button via $5005W /D4 (1 = disable) and enter TV mode via $5005W D5, but this functionality is not present on any known revision of the SSS-CPU board. 

### Exceptions

See: Exception enable ($5000W), Exception flags ($5000R)

The FamicomBox features 8 kinds of exceptions. When the console transitions from having 0 to 1 active exceptions (enabled exceptions with conditions met), the console is reset by holding the 3198 CIC in reset for 1.76 ms and the $5000R register becomes held in the latched state, latching the current active exception state if it was unlatched. This exception behavior then cannot happen again until the number of active exceptions becomes 0 (all active exceptions become inactive either by being disabled or having their conditions no longer met). When all exceptions are inactive, reading $5000R unlatches it, but still provides the latched value on that read. 

The exceptions are as follows: 

  * **6.83 Hz period expiration** : This ensures the internal slot is running FamicomBox-aware software. The condition is true while the 6.83 Hz clock is low. Unlike the other exceptions that can be toggled, this one is enabled at power-on and will repeatedly reset the system unless disabled.
  * **8-bit timer expiration** : This is used to run a game in attract mode for a limited amount of time. The condition is true while the upper 4 bits of the $5003W timer are 0. This timer is clocked by the inverse of the 6.83 Hz clock.
  * **Joypad button press** : This exception is used to detect button presses for exiting attract mode. The condition is true while joypads 1 and 2 are enabled ($5005W D6), joypad 1 or 2's D0 is asserted, and that joypad hasn't been read 8 or more times since the last strobe. This can be true even if a joypad is not currently being read because it is still outputting on D0. Note that the read count applies to the joypad port, not the register, so swapping joypads ($5005W D7) will also swap the read counts.
  * **Keyswitch rotation** : This exception is used to detect when the keyswitch is rotated to a new position. The condition is true for 13.4 ms after moving from an invalid angle to a valid position (that is, when the state transitions from all bits 0 to any bits being 1). This can also be checked through $5007R /D1.
  * **Credit transition** : This exception triggers when transitioning either way between having and not having credit. This is controlled by the 3199 coin timer and is a pulse.
  * **Reset button press** : This is true while the reset button on the front panel is pressed. When disabled, the reset button has no effect.
  * **Joypad watchdog** : This exception recovers if the software has locked up. It is implemented using a 4-bit up counter clocked at 0.85 Hz that is cleared on read of either joypad. The condition is true while the counter is 15. Unlike other exceptions, this one cannot be disabled.
  * **CATV device disconnection** : This exception triggers if the CATV connection is lost. The condition is true while the CATV Operation On latch (CATV pin 1) is high or open, indicating a CATV device is not connected.



### Zapper detection

Joypad port 3 is intended for use with the Zapper. The ground pin of this port can be disconnected from ground via $5005W D2, disabling the device. When this is done, the ground line is weakly pulled low through a 100k ohm resistor. The value on the ground line can be read via $5007R D2. If the Zapper is not connected, this will read as 0 because of that pulldown. However, the Zapper has a pullup effect on ground of about 10k ohm, stronger than the pulldown, which will cause the line to read as 1 slightly more than one frame after disconnecting ground. Therefore, if one disconnects ground and waits at least 2 frames, the presence of the Zapper can be detected using the ground line value. 

### 40% brightness

The RF modulator can be configured to reduce the screen's brightness. This is enabled via $5005W D3 or by the 3199 coin timer during near-end flash. While the effect is called 40%, it has been measured as reducing brightness to about 45% overall. If voltages are taken relative to sync tip rather than blanking, then resulting brightness can be calculated with the function `dim(bright) = 8.3906 * bright**(0.62567)`, with colorburst H being about 53% brightness, color $00 about 48%, color $10 about 46%, and color $20 about 45%. This effect should be multiplicative with the about-75% brightness resulting from using all 3 of the PPU's color emphasis bits. 

### Coin timer and near-end flash

When using coins for payment, the 3199 coin timer tracks the remaining time, signals that time is low, and sends an exception pulse when transitioning between having and not having time. The timer is a Sharp SM59x 4-bit CPU like the CIC, but its ROM has not been fully dumped and its behavior is not fully understood. 

When time is running low, the 3199 coin timer outputs a near-end flash (/NEF) signal. This alternates between active and inactive until either time runs out or more coins are added. When active, this triggers 3 different low-time alerts: 

  * The coin box LED is disabled.
  * If DIP switch 9 is off, an 874 Hz beep sound is mixed into the console's audio output.
  * If it's not already enabled via $5005W D3, the screen brightness is reduced using the RF modulator's 40% brightness feature.



The rate at which near-end flash alternates is believed to be controlled by the coin timer's NMI/2 input. _240p Test Suite_ has been observed slowing the rate at which it alternates, but the cause of this is not yet known. 

### CATV interface

See: CATV interface terminal (RP-3)

The CATV interface provides a pay-to-play billing mechanism, tracking playtime via an external computer system for later payment. Signals on the CATV pins are active low. The FamicomBox detects a CATV connection via the Operation On latch on CATV pin 1, which is tied to the ground input on pin 4. An exception can be generated when this line is no longer driven low, indicating a loss of connection. This state can also be read by the menu software via $5007R D4. 

The FamicomBox contains a relay that tracks whether the user has provided billing authorization. This relay controls CATV pin 3, and its state can be read by the menu software via $5007R /D5. Menu software is able to set the relay to the authorized state by setting $5005W D0 to 1 for at least 75 ms (5 frames). This bit should then be cleared. If the user attempts to start a game and billing is not yet authorized, the menu software should display a screen requesting authorization and then set the relay if authorized by the user. Even if billing is authorized, it is expected that the menu software not allow games to be played while the Operation On latch is inactive, and should instead enter attract mode. 

The relay is reset back to the not-authorized state when a Billing Clear pulse is received via CATV pin 6. This pulse should last for 75 ms. This pulse (of any duration) also places the system into TV Mode. It is expected to be sent when the Operation On latch becomes active, but may also be sent anytime, even while the Operation On latch is inactive. 

When billing is authorized and the Operation On latch is active, the menu software should allow games to be played. While a game is running, the menu software should indicate so via the Game Active signal on $5001W D6, which is sent over CATV pin 7. When exiting the game, Game Active should be cleared. The external system can track the amount of playtime for billing using this Game Active signal. 

The CATV interface also contains an Energized signal on CATV pin 8 indicating the FamicomBox is functional, controlled via $5001W D7. The menu software only sets this if it successfully detects the presence of the Zapper on joypad port 3. This signal can be used by the external system to detect that the FamicomBox is either not running or its Zapper has been vandalized. 

## Registers

### Joypad out ($4016W)

This is wired the same as in an AV Famicom, but OUT0 is not connected to joypad port 3. 

### Joypad 1 in ($4016R)
    
    
    Mask: $FFFF
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    |||| |||+- Joypad port 1 D0
    |||| ||+-- 15P expand pin 13
    |||| |+--- Expansion connector (P4) pin 44
    |||| +---- Joypad port 1 D3
    |||+------ Joypad port 1 D4
    ||+------- Expansion connector (P4) pin 19
    |+-------- Expansion connector (P4) pin 45
    +--------- Expansion connector (P4) pin 20
    
    Note: All bits are active low.
    Note: Any bit not driven by an input device is pulled high (reads as 0).
    

Because all joypad read bits are driven, games such as _Paperboy_ that depend on open bus will not work. 

### Joypad 2 in ($4017R)
    
    
    Mask: $FFFF
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    |||| |||+- Joypad port 2 D0, and 15P expand pin 8
    |||| ||+-- 15P expand pin 7
    |||| |+--- 15P expand pin 6
    |||| +---- Joypad port 2 D3, joypad port 3 D3, and 15P expand pin 5
    |||+------ Joypad port 2 D4, joypad port 3 D4, and 15P expand pin 4
    ||+------- Expansion connector (P4) pin 17
    |+-------- Expansion connector (P4) pin 43
    +--------- Expansion connector (P4) pin 18
    
    Note: All bits are active low.
    Note: Any bit not driven by an input device is pulled high (reads as 0).
    

D3 and D4 on joypad ports 2 and 3 can be disabled using DIP switch 10. 

### Exception enable ($5000W)
    
    
    Mask: $F007
    Power-on value: $00
    
    7  bit  0
    ---- ----
    V.RC KJTP
    | || ||||
    | || |||+- 6.83 Hz period expiration (0 = enable)
    | || ||+-- 8-bit timer expiration (1 = enable)
    | || |+--- Joypad button press (1 = enable)
    | || +---- Keyswitch rotation (1 = enable)
    | |+------ Credit transition (1 = enable)
    | +------- Reset button press (1 = enable)
    +--------- CATV device disconnect (1 = enable)
    

Note that the watchdog exception cannot be disabled. 

See Exceptions for details. 

### Exception flags ($5000R)
    
    
    Mask: $F007
    Power-on state: either unlatched, or latched and probably $FF
    
    7  bit  0
    ---- ----
    VWRC KJTP
    |||| ||||
    |||| |||+- 6.83 Hz period expired
    |||| ||+-- 8-bit timer expired
    |||| |+--- Joypad button pressed
    |||| +---- Keyswitch rotated
    |||+------ Credit transitioned
    ||+------- Reset button pressed
    |+-------- Joypad watchdog triggered
    +--------- CATV device disconnected
    
    Note: All bits are active low.
    

See Exceptions for details. 

### Payment ($5001W)
    
    
    Mask: $F007
    Power-on value: $00
    
    7  bit  0
    ---- ----
    EACC CCCC
    |||| ||||
    |||| |||+- Coin timer (3199) R0.0 (TS0)
    |||| ||+-- Coin timer (3199) R0.1 (TS1)
    |||| |+--- Coin timer (3199) R0.2 (TS2)
    |||| +---- Coin timer (3199) R0.3 (CC0)
    |||+------ Coin timer (3199) R1.3 (CC1)
    ||+------- Coin timer (3199) R2.1 (CH)
    |+-------- CATV Game Active (1 = active). Inverse of CATV pin 7.
    +--------- CATV Energized (1 = energized). Inverse of CATV pin 8.
    

  * TS2-0 appear to be related to time per coin. These are set by the menu software to %001 for the 10 minute DIP switch 2 setting and %010 for the 15 minute setting. See 3199A coin timer pinout for more details on these and other coin timer pins.
  * CH is hypothesized as telling the coin timer to run the timer.
  * CATV Energized is set by menu software only if the Zapper is detected via $5007R /D2. See Zapper detection and CATV interface for details.



### LED and Memory protect ($5002W)
    
    
    Mask: $F007
    Reset value: $00
    
    7  bit  0
    ---- ----
    FMMM LLLL
    |||| ||||
    |||| ++++- LED select (0 = none)
    |+++------ $0000-1FFF memory protect:
    |             0 = $0000-1FFF read-only
    |             1 = $0800-1FFF read-only
    |             2 = $1000-1FFF read-only
    |             3 = $1800-1FFF read-only
    |           4-7 = all writable
    +--------- Flash LED at 3.41 Hz (1 = enable)
    

This register controls the 15 LEDs adjacent to the 15 game cartridge slots. Only one can be lit at a time. 

### DIP switches ($5002R)
    
    
    Mask: $F007
    
    7  bit  0
    ---- ----
    8765 4321
    |||| ||||
    ++++-++++- Switches 8-1 (left = 0, right = 1)
    

Behavior is software-defined. See DIP switch settings for details. 

### 8-bit timer ($5003W)
    
    
    Mask: $F007
    Power-on value: Probably $FF
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    ++++-++++- Timer value
    

This is a down counter that decrements using the inverse of the 6.83 Hz clock. The 8-bit timer exception is triggered when the timer's upper 4 bits are 0. Because of this, the value written should be 15 more than desired. The timer should be set before enabling the exception. 

### Keyswitch ($5003R)
    
    
    Mask: $F007
    
    7  bit  0
    ---- ----
    NAKK KKKK
    |||| ||||
    ||++-++++- Keyswitch position
    |+-------- Coin timer (3199) ACT
    +--------- Coin timer (3199) NE
    

  * The keyswitch has 6 positions, each corresponding to one bit in this register. A bit returns 1 if the keyswitch is in that position. All bits are 0 while the keyswitch is rotated between positions. The behavior triggered by each position is software-defined.
  * NE is not used by contemporary menu software.



### Slot select ($5004W)
    
    
    Mask: $F007
    Reset value: $00
    
    7  bit  0
    ---- ----
    .RCC SSSS
     ||| ||||
     ||| ++++- Slot select
     |++------ Column select:
     |          0: Slot 0
     |          1: Slots 1-5
     |          2: Slots 6-10
     |          3: Slots 11-15
     +-------- Unmap $5000-5FFF registers (1 = unmap)
    

  * The slot select and column select control multiplexing of certain cartridge connector pins. See Slot selection and Cartridge connector for details.
  * The unmap feature disables all of the register enables, but does not disable the /$5000-5FFF signal on expansion connector (P4) pin 40. Note that once unmapped, registers stay unmapped until reset.



### 25P expand input ($5004R)
    
    
    Mask: $F007
    Power-on value: Unknown
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    |||| |||+- DB-25 pin 2
    |||| ||+-- DB-25 pin 15
    |||| |+--- DB-25 pin 3
    |||| +---- DB-25 pin 16
    |||+------ DB-25 pin 4
    ||+------- DB-25 pin 17
    |+-------- DB-25 pin 5
    +--------- DB-25 pin 18
    
    Note: All bits inverted
    

### Miscellaneous output ($5005W)
    
    
    Mask: $F007
    Reset value: $00
    
    7  bit  0
    ---- ----
    SJO. DZCB
    |||  ||||
    |||  |||+- Set billing relay to authorized (1 = set)
    |||  ||+-- Coin timer reset (1 = reset)
    |||  |+--- Enable joypad port 3 (1 = enable)
    |||  +---- Enable "40%" screen brightness (0 = enable)
    ||+------- $5007R /D7 and expansion connector (P4) pin 23
    |+-------- Disable joypads 1 and 2 (1 = disable)
    +--------- Swap joypads 1 and 2 (1 = swap)
    

  * The billing relay controls the output of the Billing Authorized latch on CATV connector pin 3. This should be set for at least 75 ms to reliably set the relay. Billing can only be authorized by software and can only be revoked by the Billing Clear input on CATV connector pin 6. See CATV interface for details.
  * The coin timer will be held in reset as long as coin timer reset is 1.
  * The joypad port 3 enable controls the port's ground line. When disabled, the Zapper will not function and can be detected. See Zapper detection for details.
  * 40% brightness reduces the screen brightness. See 40% brightness for details.
  * When joypads 1 and 2 are disabled, the joypads can no longer receive joypad /OE or send D0. They still receive OUT0.
  * Swapping joypads 1 and 2 swaps joypad /OE and D0 between joypads. It is used by the menu software to allow the controller that selected the game to be joypad 1.



### Expansion input ($5005R)
    
    
    Mask: $F007
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    ++++-++++- Open bus
    

The enable for this register is outputted over expansion connector (P4) pin 28. A response can be sent over the CPU data lines on the same connector. 

### 25P expand output ($5006W)
    
    
    Mask: $F007
    Power-on value: Probably $FF
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    |||| |||+- 25P expand pin 6
    |||| ||+-- 25P expand pin 15
    |||| |+--- 25P expand pin 7
    |||| +---- 25P expand pin 16
    |||+------ 25P expand pin 8
    ||+------- 25P expand pin 17
    |+-------- 25P expand pin 9
    +--------- 25P expand pin 18
    

### Expansion input ($5006R)
    
    
    Mask: $F007
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    ++++-++++- Open bus
    

The enable for this register is outputted over expansion connector (P4) pin 27. A response can be sent over the CPU data lines on the same connector. 

### Expansion output ($5007W)
    
    
    Mask: $F007
    
    Unimplemented
    

The enable for this register is outputted over expansion connector (P4) pin 2. The data is sent over the CPU data lines on the same connector. 

### Miscellaneous input ($5007R)
    
    
    Mask: $F007
    
    7  bit  0
    ---- ----
    IEBC EZKT
    |||| ||||
    |||| |||+- TV mode latch (0 = game, 1 = TV). Cleared on read.
    |||| ||+-- Keyswitch rotated (0 = true)
    |||| |+--- Joypad port 3 ground value (1 = device present)
    |||| +---- Inverse of expansion connector (P4) pin 21
    |||+------ CATV device connected (1 = connected). Inverse of CATV pin 1, Operation On latch.
    ||+------- Billing relay state (0 = authorized)
    |+-------- Inverse of expansion connector (P4) pin 22
    +--------- $5005W /D5
    

  * The TV mode latch is set when entering TV mode and can be used by the software to detect that a reset was caused by TV mode. See TV and Game modes for details.
  * Keyswitch rotated will show as 0 for 13.4 ms after the keyswitch enters a position (keyswitch position transitioned from all bits 0 to any bits being 1). This is the same state used as the condition for the keyswitch rotation exception, just inverted.
  * When joypad port 3 ground is disabled, this bit can be used to detect a device. See Zapper detection for details.



## DIP switch settings

Switches are 0 to the left and 1 to the right. 

### Software settings

  * 1: Operation mode (0 = game, 1 = self check)
  * 2: Play time per coin (0 = 10 minutes, 1 = 15 minutes)
  * 3: Unused
  * 4: FamicomBox title screen attract time (0 = 6 seconds, 1 = 11 seconds)
  * 5: Game demo time
  * 6: Game demo time
  * 7: Billing mode
  * 8: Billing mode

SW6 | SW5 | Game demo time | Timer value   
---|---|---|---  
0 | 0 | 10 seconds | $54   
0 | 1 | 15 seconds | $77   
1 | 0 | 20 seconds | $9A   
1 | 1 | 5 seconds | $31   
SW8 | SW7 | Billing mode   
---|---|---  
0 | 0 | Key   
0 | 1 | CATV   
1 | 0 | Coin   
1 | 1 | Free play   
  
### Hardware settings

  * 9: Disable near-end flash beeping (1 = disable). (P6 pin 1)
  * 10: Disable joypad 2 D3 and D4 on joypad ports 2 and 3 (1 = disable). This disables the Zapper input. This does not affect the 15P expand inputs. (P6 pin 36)



## Pinouts (external)

### Cartridge connector
    
    
       FamicomBox  |   Cart    |  FamicomBox
                    -----------
            +5V -- |36       72| -- GND
       CIC toMB <- |35       71| <- **CIC CLK (3.58 MHz)**
      CIC toPak -> |34       70| <- CIC +RST
         PPU D3 <> |33 **C**   **C** 69| <> PPU D4
         PPU D2 <> |32 **C**   **C** 68| <> PPU D5
         PPU D1 <> |31 **C**   **C** 67| <> PPU D6
         PPU D0 <> |30 **C**   **C** 66| <> PPU D7
         PPU A0 -> |29     **S** 65| <- PPU A13
         PPU A1 -> |28       64| <- PPU A12
         PPU A2 -> |27       63| <- PPU A10
         PPU A3 -> |26       62| <- PPU A11
         PPU A4 -> |25       61| <- PPU A9
         PPU A5 -> |24       60| <- PPU A8
         PPU A6 -> |23       59| <- PPU A7
      CIRAM A10 <- |22 **S**     58| -- **NC**
        PPU /RD -> |21       57| -- **NC**
        **CPU A15** -> |20       56| <- PPU /WR
    **/SlotIndex3** -> |19       55| -- **GND**
    **/SlotIndex2** -> |18       54| -- **GND**
    **/SlotIndex1** -> |17       53| -- **GND**
    **/SlotIndex0** -> |16       52| -- **+5V**
           /IRQ <- |15       51| -- **+5V**
        CPU R/W -> |14     **S** 50| <- /ROMSEL (/A15 + /M2)
         CPU A0 -> |13     **C** 49| <> CPU D0
         CPU A1 -> |12     **C** 48| <> CPU D1
         CPU A2 -> |11     **C** 47| <> CPU D2
         CPU A3 -> |10     **C** 46| <> CPU D3
         CPU A4 -> |09     **C** 45| <> CPU D4
         CPU A5 -> |08     **C** 44| <> CPU D5
         CPU A6 -> |07     **C** 43| <> CPU D6
         CPU A7 -> |06     **C** 42| <> CPU D7
         CPU A8 -> |05       41| <- CPU A14
         CPU A9 -> |04       40| <- CPU A13
        CPU A10 -> |03       39| <- CPU A12
        CPU A11 -> |02       38| <- M2
            GND -- |01       37| -- **NC**
                    -----------
    

Notes: 

  * Signals marked in **bold** differ compared to a standard [NES cartridge connector](Cartridge_connector.xhtml#Pinout_of_72-pin_NES_consoles_and_cartridges "Cartridge connector").
  * Signals marked with **S** are multiplexed and only sent to the selected slot. (See Slot select ($5004W))
  * Signals marked with **C** are multiplexed and only sent to the selected column. (See Slot select ($5004W)) 
    * Column-muxed CPU lines are only enabled when /ROMSEL is asserted.
    * Column-muxed PPU lines are only enabled when /PA13 is asserted.
  * All non-multiplexed signals are sent to all 16 cartridge slots.
  * The FamicomBox leaves system clock (pin 37), CIRAM /CE (pin 57), and PPU /A13 (pin 58) not connected.



Signal descriptions: 

  * **/SlotIndex3..0** : These are fixed to +5V or GND to produce an index matching the ones used in register $5004W. /SlotIndex0 on slot 2 and /SlotIndex1 on slot 4 are floating instead of grounded.



### 15P expand (RP-1)

This is a [Famicom expansion port](Expansion_port.xhtml#Famicom "Expansion port"). The SOUND output is 'audio to RF', with everything already mixed in. 

### 25P expand (RP-2)
    
    
               SSS-Interface
                   DB-25
                   /\
                  |   \
           +5V -- | 01  \
                  |   14 | ?? /IRQ
    $5004R /D0 <- | 02   |
                  |   15 | -> $5004R /D1
    $5004R /D2 <- | 03   |
                  |   16 | -> $5004R /D3
    $5004R /D4 <- | 04   |
                  |   17 | -> $5004R /D5
    $5004R /D6 <- | 05   |
                  |   18 | -> $5004R /D7
     $5006W D0 -> | 06   |
                  |   19 | <- $5006W D1
     $5006W D2 -> | 07   |
                  |   20 | <- $5006W D3
     $5006W D4 -> | 08   |
                  |   21 | <- $5006W D5
     $5006W D6 -> | 09   |
                  |   22 | <- $5006W D7
           GND -- | 10   |
                  |   23 | -- GND
           GND -- | 11   |
                  |   24 | -> M2
           GND -- | 12   |
                  |   25 | -- GND
           GND -- | 13  /
                  |   /
                   \/
    

### CATV interface terminal (RP-3)
    
    
    SSS-Interface
         8| -> Energized
         7| -> Game Active
         6| <- Billing Clear
         5| -> +5v (not used)
       +-4| <- GND
       | 3| -> Billing Authorized latch
       | 2| <- +6.3v
       +-1| <> Operation On latch
    
    All signals active low.
    

Pins 1 and 4 are jumpered together. 

See CATV interface for details. 

**Signal descriptions** : 

  * **Operation On latch** : This is tied to the pin 4 GND input by the FamicomBox and thus can be used by both the FamicomBox and external system to detect a connection. If this is not asserted, the FamicomBox can assume that billing is no longer being tracked, so it can trigger an exception and the menu software can prevent further play until asserted again.
  * **Billing Authorized latch** : This outputs the state of the Billing Authorized relay.
  * **GND** : This is only used for the Operation On latch and thus can be used as a meaningful input to prevent the FamicomBox from allowing gameplay.
  * **Billing Clear** : This clears the Billing Authorized relay and puts the console into TV mode. A 75 ms pulse is required to reliably clear the relay.
  * **Game Active** : This indicates a game is currently being played. It is controlled by the menu software. The external system can use this for billing.
  * **Energized** : This indicates the FamicomBox is operating correctly. It is controlled by the menu software. It requires both system operation and successful detection of a Zapper.



### DC in connector (RP-5)
    
    
    SSS-Interface
        +---+
        | 1 | -- GND
        | 2 | -- +5V
        | 3 | -- GND
        +---+
    

### Expansion connector (P4)
    
    
                       |SSS-CPU|
                        -------
                +5V -- |01   26| -> $5007W /enable
                +5V -- |02   27| -> $5006R /OE
                +5V -- |03   28| -> $5005R /OE
                 M2 <- |04   29| -- +5V
    audio mix input -> |05   30| -- +5V
                +5V -- |06   31| -- +5V
             CPU A6 <- |07   32| -> CPU A7
             CPU A4 <- |08   33| -> CPU A5
             CPU A2 <- |09   34| -> CPU A3
             CPU A0 <- |10   35| -> CPU A1
             CPU D1 <> |11   36| <> CPU D0
             CPU D3 <> |12   37| <> CPU D2
             CPU D5 <> |13   38| <> CPU D4
             CPU D7 <> |14   39| <> CPU D6
            CPU R/W <- |15   40| -> /$5000-5FFF
    CPU audio out 1 <- |16   41| ?? /IRQ
        joypad 2 D5 -> |17   42| -> CPU audio out 2
        joypad 2 D7 -> |18   43| <- joypad 2 D6
        joypad 1 D5 -> |19   44| <- joypad 1 D2
        joypad 1 D7 -> |20   45| <- joypad 1 D6
         $5007R /D3 -> |21   46| -- GND
         $5007R /D6 -> |22   47| -- GND
          $5005W D5 <- |23   48| -- GND
                GND -- |24   49| -- GND
                GND -- |25   50| -- GND
                        -------
    

### Joypad port 1 (FP-2)
    
    
                   SSS-REL
                     -
             GND -- |1 \
    joypad 1 /OE <- |2 5| -- +5V
            OUT0 <- |3 6| <- joypad 1 D3
     joypad 1 D0 -> |4 7| <- joypad 1 D4
                     ---
    

### Joypad port 2 (FP-3)
    
    
                   SSS-REL
                     -
             GND -- |1 \
    joypad 2 /OE <- |2 5| -- +5V
            OUT0 <- |3 6| <- joypad 2 D3
     joypad 2 D0 -> |4 7| <- joypad 2 D4
                     ---
    

  * Joypad 2 D3 and D4 can be disabled via DIP switch 10.



### Joypad port 3 (FP-4)
    
    
                        SSS-REL
                          -
    joypad port 3 GND -- |1 \
                   NC -- |2 5| -- +5V
                 OUT0 <- |3 6| <- joypad 2 D3
                   NC -- |4 7| <- joypad 2 D4
                          ---
    

  * This port is intended for use with the Zapper.
  * Joypad port 3 GND is the ground signal controlled by $5005W D2. When this is disabled, it instead has a weak pulldown, and the line is expected to be pulled up by the attached controller.
  * Joypad 2 D3 and D4 can be disabled via DIP switch 10.



### Coin box connector (FP-7)
    
    
    SSS-REL
       1 <- /coin switch
       2 -- GND
       3 -> coin LED
    

Signal descriptions: 

  * **coin LED** : This is controlled by the 3199 coin timer's /NEF and /ACT outputs. If /ACT is 0, then LED is /NEF, and otherwise, LED is 0.



### Coin box power connector (P8)
    
    
    SSS-REL
       1 -- +5V
       2 -- GND
    

If using coin box model CTB-370S, 5V is needed for the credit counter. The coin box's 5V input comes from this connector, and its ground input is tied to both FP-7 and P8. With the counterless CTB-370N coin box, this connector is not used. 

## Pinouts (internal)

### SSS-SOB connector (P1)
    
    
                 SSS-SOB  |SSS-CPU|  SSS-SOB
                           -------
                   +5V -- |01   37| -- +5V
                   +5V -- |02   38| -> PPU A13[15]
           /ROMSEL[15] <- |03   39| -> PPU A13[14]
           /ROMSEL[14] <- |04   40| -> PPU A13[13]
           /ROMSEL[13] <- |05   41| -> PPU A13[12]
           /ROMSEL[12] <- |06   42| -> PPU A13[11]
           /ROMSEL[11] <- |07   43| -> PPU A13[10]
           /ROMSEL[10] <- |08   44| -> PPU A13[9]
            /ROMSEL[9] <- |09   45| -> PPU A13[8]
            /ROMSEL[8] <- |10   46| -> PPU A13[7]
            /ROMSEL[7] <- |11   47| -> PPU A13[6]
            /ROMSEL[6] <- |12   48| -> PPU A13[5]
            /ROMSEL[5] <- |13   49| -> PPU A13[4]
            /ROMSEL[4] <- |14   50| -> PPU A13[3]
            /ROMSEL[3] <- |15   51| -> PPU A13[2]
            /ROMSEL[2] <- |16   52| -> PPU A13[1]
            /ROMSEL[1] <- |17   53| -> PPU /WR
               PPU /RD <- |18   54| -> CPU R/W
    CIC CLK (3.58 MHz) <- |19   55| -> guest CIC reset
             CIC toPak <- |20   56| -> PPU A0
               PPU A12 <- |21   57| -> PPU A1
               PPU A10 <- |22   58| -> PPU A2
               PPU A11 <- |23   59| -> PPU A3
                PPU A9 <- |24   60| -> PPU A4
                PPU A8 <- |25   61| -> PPU A5
                PPU A7 <- |26   62| -> PPU A6
                CPU A0 <- |27   63| -> CPU A1
                CPU A2 <- |28   64| -> CPU A3
                CPU A4 <- |29   65| -> CPU A5
                CPU A6 <- |30   66| -> CPU A7
               CPU A14 <- |31   67| -> CPU A8
               CPU A13 <- |32   68| -> CPU A9
               CPU A12 <- |33   69| -> CPU A10
                    M2 <- |34   70| -> CPU A11
              CIC toMB -> |35   71| -- GND
                   GND -- |36   72| -- GND
                           -------
    

Numbers in brackets indicate slot or column indexes for multiplexed signals. The signals here that are duplicated on P3 are behind a separate ALS541 buffer. 

### SSS-SOB connector (P2)
    
    
            SSS-SOB  |SSS-CPU|  SSS-SOB
                      -------
              +5V -- |01   37| -- +5V
              +5V -- |02   38| -- +5V
    CIRAM A10[15] -> |03   39| <- CIRAM A10[14]
    CIRAM A10[13] -> |04   40| <- CIRAM A10[12]
    CIRAM A10[11] -> |05   41| <- CIRAM A10[10]
     CIRAM A10[9] -> |06   42| <- CIRAM A10[8]
     CIRAM A10[7] -> |07   43| <- CIRAM A10[6]
     CIRAM A10[5] -> |08   44| <- CIRAM A10[4]
     CIRAM A10[3] -> |09   45| <- CIRAM A10[2]
     CIRAM A10[1] -> |10   46| <> PPU D4[3]
        PPU D3[3] <> |11   47| <> PPU D5[3]
        PPU D2[3] <> |12   48| <> PPU D6[3]
        PPU D1[3] <> |13   49| <> PPU D7[3]
        PPU D0[3] <> |14   50| <> CPU D0[3]
        CPU D1[3] <> |15   51| <> CPU D2[3]
        CPU D3[3] <> |16   52| <> CPU D4[3]
        CPU D5[3] <> |17   53| <> CPU D6[3]
        CPU D7[3] <> |18   54| <> PPU D4[2]
        PPU D3[2] <> |19   55| <> PPU D5[2]
        PPU D2[2] <> |20   56| <> PPU D6[2]
        PPU D1[2] <> |21   57| <> PPU D7[2]
        PPU D0[2] <> |22   58| <> CPU D0[2]
        CPU D1[2] <> |23   59| <> CPU D2[2]
        CPU D3[2] <> |24   60| <> CPU D4[2]
        CPU D5[2] <> |25   61| <> CPU D6[2]
        CPU D7[2] <> |26   62| <> PPU D4[1]
        PPU D3[1] <> |27   63| <> PPU D5[1]
        PPU D2[1] <> |28   64| <> PPU D6[1]
        PPU D1[1] <> |29   65| <> PPU D7[1]
        PPU D0[1] <> |30   66| <> CPU D0[1]
        CPU D1[1] <> |31   67| <> CPU D2[1]
        CPU D3[1] <> |32   68| <> CPU D4[1]
        CPU D5[1] <> |33   69| <> CPU D6[1]
        CPU D7[1] <> |34   70| -- GND
              GND -- |35   71| -- GND
              GND -- |36   72| -- GND
                      -------
    

Numbers in brackets indicate slot or column indexes for multiplexed signals. 

### SSS-SOB connector (P3)
    
    
      SSS-SOB  |SSS-CPU|  SSS-SOB
                -------
        +5V -- |01   37| -- +5V 
        +5V -- |02   38| -- +5V
    LED /S3 <- |03   39| -- +5V
    LED /S4 <- |04   40| -- +5V
    LED /S5 <- |05   41| -- +5V
    LED /S6 <- |06   42| -- +5V
    LED /S7 <- |07   43| -- +5V
        GND -- |08   44| -- GND
        GND -- |09   45| -- GND
    LED /S0 <- |10   46| -- GND
    LED /S1 <- |11   47| -- GND
    LED /S2 <- |12   48| -- GND
     LED C0 <- |13   49| -- GND
     LED C1 <- |14   50| -- GND
        GND -- |15   51| -- GND
        GND -- |16   52| -- GND
        GND -- |17   53| -- GND
       /IRQ -> |18   54| -> PPU /WR
    PPU /RD <- |19   55| -> CPU R/W
     PPU A0 <- |20   56| -> PPU A12
     PPU A1 <- |21   57| -> PPU A11
     PPU A2 <- |22   58| -> PPU A10
     PPU A3 <- |23   59| -> PPU A9
     PPU A4 <- |24   60| -> PPU A8
     PPU A5 <- |25   61| -> PPU A7
     PPU A6 <- |26   62| -> CPU A0
     CPU A1 <- |27   63| -> CPU A2
     CPU A3 <- |28   64| -> CPU A4
     CPU A5 <- |29   65| -> CPU A6
     CPU A7 <- |30   66| -> CPU A14
     CPU A8 <- |31   67| -> CPU A13
     CPU A9 <- |32   68| -> CPU A12
    CPU A10 <- |33   69| -> M2
    CPU A11 <- |34   70| -- GND
    CPU A15 <- |35   71| -- GND
        GND -- |36   72| -- GND
                -------
    

Numbers in brackets indicate slot or column indexes for multiplexed signals. The signals here that are duplicated on P1 are behind a separate ALS541 buffer. 

Signal descriptions: 

  * **LED /S0..7** : These select a slot LED. Only one of these is asserted at a time (unless flashing is enabled, which disables all of them half the time). The active line is selected by $5002W D2..0.
  * **LED C0..1** : These select a slot LED channel and have opposite values. LC1 is $5002W D3 and LC0 the inverse.



### RF modulator connector (P5)
    
    
    SSS-CPU
     +---+
     | 1 | -> TV/GAME
     | 2 | -- +5V
     | 3 | -> VIDEO
     | 4 | -> 40% on
     | 5 | -- GND
     | 6 | -> audio to RF
     | 7 | -- GND
     +---+
    

Signal descriptions: 

  * **TV/GAME** : When high, passes RF input through to RF output. The RF modulator pulls this low (via pin 5).
  * **40% on** : When high, reduces screen brightness to approximately 45%.



### SSS-Interface connector (P6 / RP-4)
    
    
     SSS-Interface  |SSS-CPU|  SSS-Interface
                     -------
             +5V -- |01   31| -- GND
             +5V -- |02   32| -- GND
             +5V -- |03   33| -- GND
             +5V -- |04   34| -- GND
             +5V -- |05   35| -- GND
     joypad 1 D0 xx |06   36| <- joypad ports 2 and 3 D3 and D4 disable
      $5002R /D7 -> |07   37| <- $5002R /D6
      $5002R /D5 -> |08   38| <- $5002R /D4
      $5002R /D3 -> |09   39| <- $5002R /D2
      $5002R /D1 -> |10   40| <- $5002R /D0
    joypad 1 /OE <- |11   41| xx joypad 2 D0
     joypad 1 D1 <- |12   42| <- joypad 2 D1
            OUT0 <- |13   43| <- joypad 2 D2
            OUT1 <- |14   44| xx joypad 2 D3
            OUT2 <- |15   45| xx joypad 2 D4
    joypad 2 /OE <- |16   46| ?? /IRQ
              M2 <- |17   47| -> audio to RF
    beep disable <- |18   48| -- GND
       $5006W D7 <- |19   49| -- GND
       $5006W D5 <- |20   50| -> $5006W D6
       $5006W D3 <- |21   51| -> $5006W D4
       $5006W D1 <- |22   52| -> $5006W D2
      $5004R /D7 -> |23   53| <- $5004W D0
      $5004R /D5 -> |24   54| <- $5004R /D6
      $5004R /D3 -> |25   55| <- $5004R /D4
      $5004R /D1 -> |26   56| <- $5004R /D2
      CATV pin 7 -> |27   57| <- $5004R /D0
      CATV pin 6 -> |28   58| <- CATV pin 8
      CATV pin 3 -> |29   59| -- GND
      CATV pin 1 -> |30   60| <- CATV pin 2
                     -------
    

Signal descriptions: 

  * **beep disable** : This disables the near-end flash beeping controlled by the 3199 coin timer. SSS-Interface connects this to DIP switch 9.
  * **joypad ports 2 and 3 D3 and D4 disable** : This disables D3 and D4 just on joypad ports 2 and 3. They can still be used over the Famicom expansion port. SSS-Interface connects this to DIP switch 10.
  * **audio to RF** : This is the same audio sent to the RF modulator (with everything mixed in), but here is used for the Famicom expansion port.
  * **CATV pin 6..8** : These are referred to in the schematic as CA5..7.



### SSS-REL connector (P7 / FP-1)
    
    
                    SSS-REL  |SSS-CPU|  SSS-REL
                              -------
                      +5V -- |01   14| -> OUT0
                      +5V -- |02   15| <- joypad 2 D4
              TV/GAME LED <- |03   16| <- joypad 2 D3
           TV/GAME button -> |04   17| <- joypad 1 D3
            /reset button -> |05   18| <- joypad 1 D4
    TV/GAME button enable <- |06   19| <- /keyswitch position 0
                 coin LED <- |07   20| <- /keyswitch position 1
                    /COIN -> |08   21| <- /keyswitch position 2
        joypad port 3 GND <> |09   22| <- /keyswitch position 3
              joypad 1 D0 -> |10   23| <- /keyswitch position 4
              joypad 2 D0 -> |11   24| <- /keyswitch position 9
             joypad 1 /OE <- |12   25| -- GND
             joypad 2 /OE <- |13   26| -- GND
                              --------
    

This connector differs from the SSS-CPU schematic on manual page 48 in size, signal pin numbers, and signal names. The connector as shown on page 49 is accurate (except that pins 15 and 16 are reversed). 

Signal descriptions: 

  * **TV/GAME LED** : This is used to indicate game mode (high to light LED).
  * **TV/GAME button enable** : This is connected to +5V on the SSS-CPU (but incorrectly shown on the schematic as controlled by a register).
  * **/COIN** : This connects to the 3199 coin timer and is driven low by either the coin box's coin switch, or by the SSS-REL's service button.
  * **joypad port 3 GND** : This is driven low by SSS-CPU to enable joypad port 3, or instead weakly pulled low so the Zapper can pull it high, in which case it's an input to the SSS-CPU.
  * **/keyswitch position 0..4, 9** : These are pulled high on the SSS-CPU.



### SSS-LED connector (P19)
    
    
    SSS-SOB  SSS-LED
     +---+
     | 1 | -> LED C0
     | 2 | -> LED C0
     | 3 | -> LED /S5
     | 4 | -> LED /S4
     | 5 | -> LED /S3
     | 6 | -> LED /S2
     | 7 | -> LED /S1
     +---+
    

### SSS-LED connector (P20)
    
    
    SSS-SOB  SSS-LED
     +---+
     | 1 | -> LED C0
     | 2 | -> LED C1
     | 3 | -> LED /S2
     | 4 | -> LED /S1
     | 5 | -> LED /S0
     | 6 | -> LED /S7
     | 7 | -> LED /S6
     +---+
    

LED C0 enables LED /S6..7 and LED C1 enables LED /S0..2. 

### SSS-LED connector (P21)
    
    
    SSS-SOB  SSS-LED
     +---+
     | 1 | -> LED C1
     | 2 | -> LED C1
     | 3 | -> LED /S7
     | 4 | -> LED /S6
     | 5 | -> LED /S5
     | 6 | -> LED /S4
     | 7 | -> LED /S3
     +---+
    

### SSS-SW connector (FP-5)
    
    
    SSS-REL
     +---+
     | 1 | -- GND
     | 2 | <- /reset button
     | 3 | <- TV/GAME button
     | 4 | -> TV/GAME button enable
     | 5 | -> TV/GAME LED
     +---+
    

### Keyswitch (FP-6)
    
    
    SSS-REL
     +---+
     | 1 | <- /keyswitch position 0
     | 2 | <- /keyswitch position 1
     | 3 | <- /keyswitch position 2
     | 4 | <- /keyswitch position 3
     | 5 | <- /keyswitch position 4
     | 6 | -- GND
     | 7 | <- /keyswitch position 9
     +---+
    

### 3198A CIC lockout
    
    
                        -------------------
           Data out <- | 1  R0.0    VDD 16 | -- +5V
            Data in -> | 2  R0.1   R2.2 15 | <- Slot select D3 ($5004W D3)
               Seed -> | 3  R0.2   R2.1 14 | <- Slot select D2 ($5004W D2)
         Host/Guest -> | 4  R0.3   R2.0 13 | -- NC
                 NC -- | 5  R3.3   R1.3 12 | <- Slot select D1 ($5004W D1)
    /3.58 MHz clock -> | 6  CL1    R1.2 11 | <- Slot select D0 ($5004W D0)
    Exception reset -> | 7  ACL    R1.1 10 | -> Guest CIC reset
                GND -- | 8  GND    R1.0  9 | -> /Host reset
                        -------------------
    

  * When the 3198 triggers a host reset, it does so just once, allowing the menu software to regain control of the system and skip over any slot that may have triggered the reset.
  * On guest CICs, the slot select D3..0 inputs use SlotIndex3..0.



### 3199A coin timer
    
    
                                -------------------
            TS0 ($5001W D0) -> | 1  R0.0    VDD 16 | -- +5V
            TS1 ($5001W D1) -> | 2  R0.1   R2.2 15 | <- /COIN (/coin switch)
            TS2 ($5001W D2) -> | 3  R0.2   R2.1 14 | <- CH ($5001W D5)
            CC0 ($5001W D3) -> | 4  R0.3   R2.0 13 | -> /NEF (/near-end flash)
    ACTE (active exception) <- | 5  R3.3   R1.3 12 | <- CC1 ($5001W D4)
            /3.58 MHz clock -> | 6  CL1    R1.2 11 | <- NMI/2
                POC (reset) -> | 7  ACL    R1.1 10 | -> /NE (/near end)
                        GND -- | 8  GND    R1.0  9 | -> /ACT (/active)
                                -------------------
    

Signal descriptions: 

  * **ACTE** : This controls the credit transition exception. It is believed to be a pulse when switching either way between having and not having credit.
  * **/ACT** : This indicates the coin timer is active (has play time credit). The menu software uses this to know there is credit.
  * **/NE** : This indicates low time remaining.
  * **/NEF** : This controls the low-time beeping, screen dimming, and coin-box LED flashing.
  * **NMI/2** : This alternates every NMI. It appears to control the /NEF timing.
  * **TS1-0** : These control time per coin. %01 sets 10 minutes and %10 sets 15 minutes. It is not known what other configurations do.
  * **TS2** : Behavior is unknown, but this is set to 0 by the menu software when setting TS1-0 and may be another time-per-coin control.
  * **CH** : Behavior is unknown, but according to the partial ROM dump, this has some effect when the coin timer sets /ACT to 0 and /NE to 1. Menu software sets this to 1 when configuring time-per-coin. Speculation is that this may tell the coin timer to run the timer ("charge").



## Schematic errata

  * P7 should be a 26-pin connector, not a 40-pin connector. All of the signals shown on the 40-pin version are still present, but numbering and naming differ slightly.
  * P7 $4017 D3 and D4 pin names should be swapped.
  * P4 pin 10 should be labeled A0.
  * FP-1 pin 1 should be labeled GND.
  * The LS3 input to LC0 and LC1 on P3 pins 13 and 14 should be labeled LE3.
  * The 4040 12-bit counter for dividing the clock should have a tap on Q9, producing a 0.85 Hz clock. The watchdog should be clocked by this.
  * For the PA13 signal coming out of the ALS541 by the PPU, /PA13 goes to the TM2215 CIRAM as pictured, but the signal to the LS139 and ALS138's should be PA13 (does not go through the inverter).
  * Instead of $5005W D4 controlling P7 pin 6 (TV/GAME switch enable), $5005W D4 should be not connected and the switch enable should be tied to +5V on the main board.
  * Instead of $5005W D5 acting as a TV mode enable by connecting to the TV/GAME mode SR latch, it should instead connect to $5007R /D7 and expansion connector (P4) pin 23. This makes pin 23 an output.



## Hardware configurations

  * Main board: 
    * SSS-CPU-02
    * SSS-CPU-03 
      * The PCB traces are wired so that the joypad button press exception's two 4021 shift registers, which implement the read counts, are each used for the wrong joypad. This was fixed at the factory by cutting the joypad /OE traces between the HC257 muxer and shift registers and adding wires to swap the connections.
    * SSS-CPU-04
  * Chipsets: 
    * RP2A03E / RP2C02E-0
    * RP2A03G / RP2C02G-0



Categories: [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Pinouts](Category_Pinouts.xhtml)
