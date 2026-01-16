# EXP pins

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/EXP_pins) | View [other pages](Special_AllPages.xhtml#EXP_pins)

The EXP pins are expansion pins present on the 72-pin NES cartridge connector and which connect to the expansion port on the bottom of the NES-001 console. These pins were not used by any officially released consumer device, but are used by various unlicensed devices and the [FamicomBox](FamicomBox.xhtml "FamicomBox") hotel console. 

## Contents

  * 1 Pin summary
  * 2 Pin notes
  * 3 Devices
    * 3.1 Broke Studio discrete mapper cartridges
    * 3.2 Broke Studio MMC mapper cartridges
    * 3.3 Broke Studio Rainbow mapper cartridges
    * 3.4 CopyNES
    * 3.5 Expansion Port Sound Module (EPSM)
    * 3.6 ExROM (MMC5)
    * 3.7 Extended NES I/O (ENIO)
    * 3.8 Famicom-to-NES adapter
    * 3.9 FamicomBox
    * 3.10 INL cartridges
    * 3.11 INL Expansion Audio Dongle Slim
    * 3.12 Muramasa NES FDS
    * 3.13 NES Hub
    * 3.14 NES modem
    * 3.15 NES-21G-CPU-72P
    * 3.16 PowerPak
    * 3.17 PowerPak Lite
    * 3.18 Simple Famicom Expansion Audio Module (SFEAM)



## Pin summary

Device  | EXP0  | EXP1  | EXP2  | EXP3  | EXP4  | EXP5  | EXP6  | EXP7  | EXP8  | EXP9  | Type   
---|---|---|---|---|---|---|---|---|---|---|---  
Broke Studio discrete mapper cartridges | ✓  |  |  |  |  | ✓  |  |  |  |  | Cartridge   
Broke Studio MMC mapper cartridges |  |  |  |  |  | ✓  | ♪  |  |  | ♪  | Cartridge   
Broke Studio Rainbow mapper cartridges |  |  |  |  |  | ✓  | ♪  |  |  | ♪  | Cartridge   
CopyNES | ✓  |  |  |  |  |  |  |  |  |  | Console   
[Everdrive N8](Everdrive_N8.xhtml "Everdrive N8") |  |  |  |  |  |  | ♪  |  |  |  | Cartridge   
[Everdrive N8 Pro](Everdrive_N8_Pro.xhtml "Everdrive N8 Pro") | ~  | ~  | ~  | ~  | ~  | ~  | ♪  | ~  | ~  | ~  | Cartridge   
Expansion Port Sound Module (EPSM) |  | ✓  | ♪  | ✓  | ✓  |  | ♪  | ✓  | ✓  | ♪  | Expansion   
ExROM (MMC5) |  |  |  |  |  | ✓  | ♪  |  |  |  | Cartridge   
Extended NES I/O (ENIO) | ~  | ~  | ~  | ~  | ~  | ✓  | ♪  | ✓  | ✓  | ✓  | Expansion   
Famicom-to-NES adapter |  |  | ♪*  |  |  |  | ♪*  |  |  |  | Cartridge   
FamicomBox | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | Console   
INL cartridges | ✓*  | ✓*  | ✓*  | ✓*  | ✓*  | ✓*  | ♪  |  |  | ♪  | Cartridge   
INL Expansion Audio Dongle Slim |  |  |  |  |  |  | ♪  |  |  | ~  | Expansion   
Muramasa NES FDS | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ♪  | ✓  | ✓  | ✓  | Expansion   
NES Hub | ~  | ~  | ~♪  | ~  | ~  | ~  | ~♪  | ~  | ~  | ~♪  | Expansion   
NES-21G-CPU-72P | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | ✓  | Cartridge   
PowerPak | ✓  | ~  | ~  | ~  | ~  | ~  | ♪  | ~  | ~  |  | Cartridge   
PowerPak Lite | ✓  |  |  |  |  |  |  |  |  |  | Cartridge   
Simple Famicom Expansion Audio Module (SFEAM) |  |  | ♪*  |  |  |  | ♪  |  |  | ♪*  | Expansion   
[TinyNES](https://www.nesdev.org/w/index.php?title=TinyNES&action=edit&redlink=1 "TinyNES \(page does not exist\)") |  |  |  |  |  |  | ♪  |  |  |  | Console   
  
✓ : This pin is used.  
♪ : This pin is used for expansion audio output (cartridge) or input (expansion device or console).  
* : This pin is used by some, but not all such devices.  
~ : This pin is connected, but lacks a specific function. 

## Pin notes

Pin  | Pin exists on:  | Notes   
---|---|---  
NES-001  | NES-101   
EXP0  | ✓  | ✓  | Some modern cartridges connect EXP0 to ground, likely because of the presence of a ground pin on the Famicom's cartridge connector in the region where the EXP pins are placed on the NES' connector. This provides no benefit because consoles don't put ground there, and it adds risk because a console or expansion port signal could be unexpectedly shorted to ground. For example, inserting such a cartridge into the FamicomBox can short 5V to ground through this pin, causing damage to the cartridge or console.   
EXP1  | ✓  | ✓  |   
EXP2  | ✓  |  | EXP2 is used by some Famicom-to-NES adapters for expansion audio, but no cartridges are known to use it for this purpose.   
EXP3  | ✓  |  |   
EXP4  | ✓  | ✓  |   
EXP5  | ✓  |  |   
EXP6  | ✓  |  | EXP6 has become the standard console input for expansion audio and is used by flash carts, modern homebrew, and modern Famicom-to-NES adapters.   
EXP7  | ✓  | ✓  |   
EXP8  | ✓  | ✓  |   
EXP9  | ✓  | ✓  | Because EXP6 is not present on the NES-101, EXP9 is often used as a secondary audio output so the NES-101 can be modified to use it.   
  
**Note** : On the NES-101, the EXP pins that don't exist are not even present on the cartridge slot; that is, there is no metal making contact with those cartridge pins, preventing their use. The other EXP pins connect to the board, but are not routed anywhere, so they are available for console modifications. 

## Devices

### Broke Studio discrete mapper cartridges

Broke Studio's NROM, BNROM, and UNROM512 boards use EXP0 and EXP5 for programming and ROM and CIC, respectively. 
    
    
       Programmer     |      Cart       |     Programmer
                       -----------------
                             ...
                      |20 EXP4
                      |19 EXP3   EXP5 55| <- ATtiny13 AVR reset
                      |18 EXP2   EXP6 54|
                      |17 EXP1   EXP7 53|
       PRG-ROM /WE -> |16 EXP0   EXP8 52|
                                 EXP9 51|
                              ...
    

### Broke Studio MMC mapper cartridges

Broke Studio's MMC board uses EXP5 for programming the CIC and outputs expansion audio on both EXP6 and EXP9. These audio outputs are tied together. 
    
    
     Programmer or                           Programmer or
     Expansion port   |      Cart       |    Expansion Port
                       -----------------
                             ...
                      |20 EXP4
                      |19 EXP3   EXP5 55| <- ATtiny13 AVR reset
                      |18 EXP2   EXP6 54| -> audio
                      |17 EXP1   EXP7 53|
                      |16 EXP0   EXP8 52|
                                 EXP9 51| -> audio
                              ...
    

### Broke Studio Rainbow mapper cartridges

Broke Studio's Rainbow mapper board uses EXP5 for programming the CIC and is capable of outputting _different_ expansion audio on each of EXP6 and EXP9. The Rainbow mapper itself produces just one audio stream, which software can choose to output on either or both of these pins. However, the cartridge pins connect to separate FPGA pins, and so another mapper implemented on the same board could output unique audio on each. 
    
    
     Programmer or                           Programmer or
     Expansion port   |      Cart       |    Expansion Port
                       -----------------
                             ...
                      |20 EXP4
                      |19 EXP3   EXP5 55| <- ATtiny13 AVR reset
                      |18 EXP2   EXP6 54| -> audio 1
                      |17 EXP1   EXP7 53|
                      |16 EXP0   EXP8 52|
                                 EXP9 51| -> audio 2
                              ...
    

### CopyNES

The [CopyNES](CopyNES.xhtml "CopyNES") allows the 2A03 to control EXP0. It's used to write to RAM carts, such as the PowerPak Lite. 

### Expansion Port Sound Module (EPSM)

The [EPSM](Expansion_Port_Sound_Module.xhtml "EPSM") can be written through either a cart-agnostic universal access mode or a mapper-specific mode, where the cartridge decodes the address and passes chip enable and address bits via EXP pins. It also accepts expansion audio input on EXP2, EXP6, and EXP9, to support as wide a variety of cartridges as possible. 
    
    
    Expansion Port    |      Cart       |    Expansion Port
                       -----------------
                              ...
           EPSM A1 <- |20 EXP4
         EPSM /CE2 <- |19 EXP3   EXP5 55|
             audio <- |18 EXP2   EXP6 54| -> audio
         EPSM /CE1 <- |17 EXP1   EXP7 53| -> EPSM A0
                      |16 EXP0   EXP8 52| -> EPSM CE3
                                 EXP9 51| -> audio
                              ...
    

### ExROM (MMC5)

ExROM boards, using [MMC5](MMC5.xhtml "MMC5"), are configured for expansion audio output on EXP6, though the boards must also have the appropriate resistors and capacitors populated for this to function. EXP5, which is pulled low in the cart, is used as a PRG-RAM read disable. 
    
    
    Expansion Port    |      Cart       |    Expansion Port
                       -----------------
                              ...
                      |20 EXP4
                      |19 EXP3   EXP5 55| <- PRG-RAM /OE
                      |18 EXP2   EXP6 54| -> audio
                      |17 EXP1   EXP7 53|
                      |16 EXP0   EXP8 52|
                                 EXP9 51|
                              ...
    

### Extended NES I/O (ENIO)

The [ENIO](Extended_NES_I_O.xhtml "ENIO") CPU board can be accessed through either a cart-agnostic compatibility mode or a direct addressing mode, where the cartridge decodes the address and passes R/W and /CE to the ENIO via EXP pins. EXP5 and EXP7-9 are passed to the CPU board, handling R/W, /CE, and presumably other currently-undocumented functionality. EXP6 is used as an expansion audio input. EXP0-4 are routed to an unpopulated header for expansion use. 
    
    
          Expansion Port    |      Cart       |    Expansion Port
                             -----------------
                                    ...
    ENIO J4 header pin 7 ?? |20 EXP4
    ENIO J4 header pin 8 ?? |19 EXP3   EXP5 55| ?? unknown (ENIO CPU board)
    ENIO J4 header pin 5 ?? |18 EXP2   EXP6 54| -> audio
    ENIO J4 header pin 6 ?? |17 EXP1   EXP7 53| -> ENIO R/W
    ENIO J4 header pin 3 ?? |16 EXP0   EXP8 52| -> ENIO /CE
                                       EXP9 51| ?? unknown (ENIO CPU board)
                                    ...
    

### Famicom-to-NES adapter

Some Famicom-to-NES adapters connect expansion audio to an EXP pin. If connected, modern adapters typically use EXP6, though EXP2 has been observed due to its proximity to the audio-to-RF pin on the Famicom cartridge connector. 

### FamicomBox

The [FamicomBox](FamicomBox.xhtml "FamicomBox") uses the EXP pins primarily to indicate slot ID to the cartridge's 3198 CIC. 9 of the pins are tied to some combination of ground and 5V, which can cause damage to non-FamicomBox cartridges that use EXP pins. One additional pin provides A15, which is not used by any contemporary game. Unlike the NES-001, the FamicomBox does not route any of these pins to any external port for expansion use. 
    
    
     FamicomBox    |      Cart       |    FamicomBox
                    -----------------
                           ...
        CPU A15 -> |20 EXP4
    /SlotIndex3 -> |19 EXP3   EXP5 55| -- GND
    /SlotIndex2 -> |18 EXP2   EXP6 54| -- GND
    /SlotIndex1 -> |17 EXP1   EXP7 53| -- GND
    /SlotIndex0 -> |16 EXP0   EXP8 52| -- +5V
                              EXP9 51| -- +5V
                           ...
    

### INL cartridges

Infinite NES Lives cartridges use EXP pins for various purposes, though not universally across all boards. EXP6 and EXP9 are used to output expansion audio to the console. Newer boards use EXP0 as PRG-ROM /WE for writing the cartridges, with a pullup on the board. Some boards use EXP0-3 (and EXP4 on dual-CPLD boards) for a JTAG interface. When an ATtiny13 is used for the CIC, EXP5 acts as AVR reset. The ATtiny13 is configured for high voltage power supply (HVPS) programming at 12v. 
    
    
     Programmer or                           Programmer or
     Expansion port   |      Cart       |    Expansion Port
                       -----------------
                              ...
                      |20 EXP4
                      |19 EXP3   EXP5 55|
                      |18 EXP2   EXP6 54| -> audio
                      |17 EXP1   EXP7 53|
       PRG-ROM /WE -> |16 EXP0   EXP8 52|
                                 EXP9 51| -> audio
                              ...
    
    
    
     Programmer or                           Programmer or
     Expansion port   |      Cart       |    Expansion Port
                       -----------------
                             ...
    CPLD2 JTAG TCK -> |20 EXP4
    CPLD1 JTAG TCK -> |19 EXP3   EXP5 55| <- ATtiny13 AVR reset
          JTAG TMS -> |18 EXP2   EXP6 54| -> audio
          JTAG TDI -> |17 EXP1   EXP7 53|
          JTAG TDO -> |16 EXP0   EXP8 52|
                                 EXP9 51| -> audio
                              ...
    

### INL Expansion Audio Dongle Slim

This expansion port adapter enables expansion audio input on EXP6. While it only has pins on EXP6 and audio mix input by default, pins can be added to it on EXP9 and audio output, though they aren't routed anywhere on the board. Because EXP6 is the default for expansion audio output and the NES-001 is the only console with this expansion port, connecting to EXP9 is unlikely to be useful. 

### Muramasa NES FDS

Because the NES-001 doesn't easily allow a cartridge to connect with another device via an external cable, Muramasa's NES FDS connects the RAM adapter cartridge and disk drive via the EXP pins. 
    
    
                Expansion Port    |      Cart       |    Expansion Port
                                   -----------------
                                          ...
                    serial out <- |20 EXP4
                (R/W) $4034.W2 <- |19 EXP3   EXP5 55| <- serial in
     (transfer reset) $4035.W1 <- |18 EXP2   EXP6 54| -> audio
    (write protected) $4032.R2 -> |17 EXP1   EXP7 53| -> $4025.W0 (motor control)
        (disk /ready) $4032.R1 -> |16 EXP0   EXP8 52| <- $4032.R0 (disk /inserted)
                                             EXP9 51| <- battery
                                          ...
    

### NES Hub

The [NES Hub](https://www.nesdev.org/w/index.php?title=NES_Hub&action=edit&redlink=1 "NES Hub \(page does not exist\)") can be configured with DIP switches to use any combination of EXP2, EXP6, and EXP9 for expansion audio. It also exposes all 10 EXP signals on a mini DisplayPort connector for use by other devices. Note that the pins are different on the source (NES Hub) and destination (EXP addon) side; the pinout below uses source/destination for pins. 
    
    
                                 Expansion Port    |      Cart       |    Expansion Port
                                                    -----------------
                                                           ...
                            EXP addon pin 11/15 ?? |20 EXP4
                            EXP addon pin 09/17 ?? |19 EXP3   EXP5 55| ?? EXP addon pin 03/12
    EXP addon pin 10/05, and (optionally) audio <? |18 EXP2   EXP6 54| ?> EXP addon pin 01/08, and (optionally) audio
                            EXP addon pin 12/03 ?? |17 EXP1   EXP7 53| ?? EXP addon pin 05/10
                            EXP addon pin 15/11 ?? |16 EXP0   EXP8 52| ?? EXP addon pin 06/06
                                                              EXP9 51| ?> EXP addon pin 04/04, and (optionally) audio
                                                           ...
    

### NES modem

The NES modem is an unreleased official expansion device that would have allowed online access. It is highly likely this device uses the EXP pins, but no specifics are known. 

### NES-21G-CPU-72P

The [NES-21G-CPU-72P](User_Lidnariq_NES_21G_CPU_72P.xhtml "User:Lidnariq/NES-21G-CPU-72P") board is used in some Nintendo test cartridges and uses the EXP pins to interact with an unknown device. The last value written to $6000 is outputted over 8 EXP pins, with the high bit also signaling whether the M2 and the system clock inputs are not functioning. EXP6 indicates when $6000 is being read, and it is speculated that this would cause the expansion device to drive the latched data onto the CPU data lines. EXP5 is used, but its effect is currently unknown; it may control the PRG-ROM's /OE. 
    
    
    Expansion Port    |      Cart       |    Expansion Port
                       -----------------
                              ...
          $6000.R4 <- |20 EXP4
          $6000.R3 <- |19 EXP3   EXP5 55| <- unknown
          $6000.R2 <- |18 EXP2   EXP6 54| -> $6000 /read
          $6000.R1 <- |17 EXP1   EXP7 53| -> $6000.R7 OR clocks bad
          $6000.R0 <- |16 EXP0   EXP8 52| -> $6000.R6
                                 EXP9 51| -> $6000.R5
                              ...
    

### PowerPak

The [PowerPak](PowerPak.xhtml "PowerPak") uses EXP0 to allow the CopyNES to program the boot ROM. EXP1-8 are connected to the FPGA and can be used, but only EXP6 has been (for expansion audio output). EXP9 is not connected, and some users have bridged it with EXP6 to enable expansion audio on NES-101 consoles. 
    
    
      Expansion Port    |      Cart       |    Expansion Port
                         -----------------
                                ...
         FPGA pin 22 ?? |20 EXP4
         FPGA pin 20 ?? |19 EXP3   EXP5 55| ?? FPGA pin 19
         FPGA pin 27 ?? |18 EXP2   EXP6 54| -> audio (FPGA pin 23)
         FPGA pin 12 ?? |17 EXP1   EXP7 53| ?? FPGA pin 94
    boot ROM program -> |16 EXP0   EXP8 52| ?? FPGA pin 41
                                   EXP9 51|
                                ...
    

### PowerPak Lite

The [PowerPak Lite](https://www.nesdev.org/w/index.php?title=PowerPak_Lite&action=edit&redlink=1 "PowerPak Lite \(page does not exist\)") is programmed with settings for the current game by a CopyNES via EXP0. 

### Simple Famicom Expansion Audio Module (SFEAM)

This expansion port adapter enables expansion audio input on EXP2, EXP6, or EXP9. It comes with a resistor populated only for EXP6 audio, leaving EXP2 and EXP9 disconnected by default. 
