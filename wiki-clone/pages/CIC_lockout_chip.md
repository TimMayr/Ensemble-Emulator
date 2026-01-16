# CIC lockout chip

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CIC_lockout_chip) | View [other pages](Special_AllPages.xhtml#CIC_lockout_chip)

The frontloading NES has a **CIC Lockout Chip** , a microcontroller that performs a proprietary handshake, as an anticompetitive measure. Famicom and toploading NES consoles do not contain this chip. The abbreviation CIC is short for "Checking Integrated Circuit" according to Nintendo's patents. 

## Contents

  * 1 Overview
  * 2 Disabling
    * 2.1 Two-wire method
    * 2.2 Pin 4 method
  * 3 Defeating
  * 4 Pinout
  * 5 Reference



## Overview

Both the lockout chip inside the NES and the one on the cartridge are the same IC. The one inside the NES acts as a lock and the one in the Cart a key. The difference is how they are hooked up. The system is wired so that the output of one CIC is connected to the input of other and vice versa. LOCK/KEY is pulled to +5V inside the NES and grounded on the Cart. Both share the same 4MHZ clock on pin 6. The RESET pin on the key is connected to SLAVE CIC RESET on the lock. The lock's RESET pin is connected to the system reset bus. This can be demonstrated by inserting a game with the system already on. The NES will not work until you press the reset button which will reset the lock CIC, which in turn resets the key. /CPU & PPU RESET is not connected on the key, on the lock it is connected to the CPU and PPU reset pins. Pins 11-15 are grounded on both CIC's in an NES; these are actually used in multi-game systems so that multiple CICs may be addressed within one system. Finally VCC goes to +5V. 

Once the system comes out of POR the Lock sends the appropriate reset and initialization signals to the key. The key then returns the correct response, otherwise the lock will pull the /CPU & PPU RESET line low with a 1Hz square wave. Since both share the same clock and the lock is able to reset the key, both CIC's stay in sync with each other. 

## Disabling

### Two-wire method

Using just two wires, the CIC can be held in reset and the console's reset button can directly control the CPU and PPU's /RESET inputs, instead. Compared to the pin 4 method, this has the advantage of being reversible while still being fairly simple to do. It is done by connecting the lock CIC's pin 7 (CIC RESET in) to the 74HCU04 inverter's pin 1 (oscillator), and the 74HCU04 inverter's pin 2 to the lock CIC's pin 9 (CPU and PPU /RESET in). 

### Pin 4 method

When communication between the two CICs fails, it is only the lock, not the key, the asserts its reset output. Therefore, configuring the CIC in the console as a key prevents it from resetting. This can be done by disconnecting the console CIC's pin 4 from the board. Optionally, this disconnected pin can be tied to ground, but the CIC has internal pulldowns that make the pin read as 0 even if left floating. 

## Defeating

Boards made by [Camerica](https://en.wikipedia.org/wiki/Camerica "wikipedia:Camerica"), [Color Dreams](https://en.wikipedia.org/wiki/Color_Dreams "wikipedia:Color Dreams"), [AVE](https://en.wikipedia.org/wiki/American_Video_Entertainment "wikipedia:American Video Entertainment"), and [AGCI](https://en.wikipedia.org/wiki/American_Game_Cartridges "wikipedia:American Game Cartridges"), as well as older Famicom to NES adapters, contain a charge pump to create waveforms involving -5V, which freezes the CIC. Later runs of frontloading NES consoles have diodes to protect against out-of-spec voltages on the CIC data pins, but not on the reset pins. 

In late 2006, Tengen's "Rabbit" chip was completely reverse-engineered and a [PIC](https://en.wikipedia.org/wiki/PIC_microcontroller "wikipedia:PIC microcontroller")-based [clone](http://forums.nesdev.org/viewtopic.php?t=1219) was successfully made. 

## Pinout

  * [CIC lockout chip pinout](CIC_lockout_chip_pinout.xhtml "CIC lockout chip pinout")



## Reference

  * The CIC is explained in US patents 4,799,635 and 5,070,479.
  * A clone cic is patented in US patent 5,004,232 which doesn't look like it should actually work unless it inadvertently stuns the CIC. A chip based on this patent was used by [AVE](https://www.nesdev.org/w/index.php?title=AVE&action=edit&redlink=1 "AVE \(page does not exist\)").


