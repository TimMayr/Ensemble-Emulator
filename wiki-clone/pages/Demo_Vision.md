# Demo Vision

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Demo_Vision) | View [other pages](Special_AllPages.xhtml#Demo_Vision)

The Demo Vision, also branded as **Demo Boy II** , is a specialized NES console used in Game Boy kiosks. It does not have a cartridge slot nor controllers. Instead, an attached Game Boy supplies video and audio to the system. 

This system has not yet been thoroughly reverse engineered. 

## Contents

  * 1 Design
  * 2 Registers
    * 2.1 $4016 write
    * 2.2 $4016 read
  * 3 IRQ
  * 4 DIP switches
    * 4.1 Software
  * 5 Firmware
    * 5.1 Power-on
    * 5.2 Unused code
  * 6 Pinouts
    * 6.1 Power connector (P1)
    * 6.2 Game Boy connector (P2)
      * 6.2.1 Game Boy (DMG) connections
      * 6.2.2 Game Boy Pocket (MGB) connections
  * 7 Hardware configurations



## Design

The Demo Vision uses two pairs of standard, 8 KiB CHR-RAM (32 KiB total) to hold Game Boy screen data. Each pair is used as a screen buffer. The active pair is read by the PPU to render the screen, while Game Boy screen data is written into the inactive pair. Their roles can be swapped at any time by firmware software running on the NES CPU. An additional 8 KiB of CHR-ROM is present, used to hold the screen border graphics. 

An MMC5 or MMC5A mapper is built into the system, allowing any of the active 24 KiB of CHR data to be used anywhere on the screen. The active screen buffer is available as the first 16 KiB of CHR, followed immediately by the CHR-ROM. The inactive screen buffer is not mapped. 

The firmware is contained in 8 KiB of PRG-ROM. This software sets up the mapper, nametables, attributes, and palettes for displaying the Game Boy screen data and border, and is responsible for controlling /reset in the Game Boy unit, which it enables after completing setup. The firmware is also responsible for switching screen buffers. This is done in an interrupt handler, but rather than using NMIs at the NES frame rate, an IRQ at the Game Boy frame rate is used, ensuring the active buffer always contains a complete Game Boy frame. However, there is still visible tearing because the swap can happen at any time in the NES frame. 

The CPU address space features 8 KiB of RAM at $0000-1FFF, rather than the standard 2 KiB. 

There are 6 DIP switches available for software use. The firmware uses switches 1-3 for a gameplay timer. If configured with a time, the firmware will reset the Game Boy after the timer expires by toggling its /reset input. The other 3 are unused. 

Demo Vision systems were available in 6 variations: domestic, Europe, and UK, each in single and dual varieties. Domestic versions use an NTSC CPU and PPU and Europe versions use a PAL CPU and PPU. The UK variant has not been documented. The dual systems are believed to use two identical Demo Vision boards with no logical connections between them except some kind of shared 2-pin reset cable. 

While initially available with just the original Game Boy, kiosks could later be used with Game Boy Pockets. Functionally, these differ in the behavior of the handheld's power switch. The power switch on the Game Boy controls power for the handheld as normal, while the Game Boy Pocket's switch controls power for just the screen, allowing the kiosk to continue to function with the handheld's switch in the off position. 

## Registers

### $4016 write
    
    
    7  bit  0
    ---- ----
    .... ..BP
           ||
           |+- Game Boy /reset (0 = hold in reset, 1 = run)
           +-- Active screen buffer
    

  * Game Boy /reset should be set to 1 once the system is configured and ready. The Game Boy can be reset by clearing this to 0 for at least 2 PPU frames and then setting it back to 1.
  * Active screen buffer swaps which 16 KiB of CHR-RAM is accessible to the PPU as the first 16 KiB of CHR and which is being filled with Game Boy screen data. It is not known if the active CHR-RAM can be written by the PPU.



### $4016 read
    
    
    7  bit  0
    ---- ----
    ..65 4321
      || ||||
      ++-++++- DIP switches 6-1 (0 = off/up, 1 = on/down)
    

## IRQ

The CPU /IRQ is believed to be asserted at some point during every Game Boy vblank. It is asserted for a limited time and does not require an ack. The firmware software's IRQ handler includes a 1281 cycle wait loop, and the handler takes 1322 cycles in total, which should be taken as an upper bound on the IRQ length. 

## DIP switches

There are 6 DIP switches. They are software-readable and have no other hardware function. They read as 0 in the off/up position and 1 in the on/down position. 

### Software

  * 1: Gameplay time
  * 2: Gameplay time
  * 3: Gameplay time
  * 4: Unused
  * 5: Unused
  * 6: Unused

SW3  | SW2  | SW1  | Gameplay Time   
---|---|---|---  
Demo Boy II  | Demo Vision   
0  | 0  | 0  | 1 minute  | 1 minute   
0  | 0  | 1  | 3 minutes  | 3 minutes   
0  | 1  | 0  | 5 minutes  | 5 minutes   
0  | 1  | 1  | 7 minutes  | 7 minutes   
1  | 0  | 0  | 12 minutes  | 12 minutes   
1  | 0  | 1  | 18374 seconds*  | Unlimited   
1  | 1  | 0  | Unlimited  | Unlimited   
1  | 1  | 1  | 65481 seconds*  | Unlimited   
  
*Note: The Demo Boy II revision A firmware allows the timer to run for switch values %101 and %111, but does not define timer values for these settings. Instead, the code following the timer value table is interpreted as timer values. 

If the Demo Vision firmware detects that switches 1-3 have changed, it resets the Game Boy and sets the timer with the new time. The Demo Boy II rev A firmware only reads the DIP switch value on reset. 

## Firmware

### Power-on

The Demo Boy II revision A firmware requires one or two resets to reliably enter stable operation, and Demo Boy II documentation instructs that 2 resets should be performed. This issue is fixed in the Demo Vision firmware. The Demo Vision firmware duplicates the reset code, running it twice with no other significant changes compared to the Demo Boy II. The reset code disables rendering, waits for at least 4 frames, sets up the palettes, nametables, attributes, MMC5, scroll, and timer, releases the Game Boy from reset, and enables rendering. It is not currently known why executing this twice prevents the problem. 

The observed behavior is that onscreen graphics are corrupted, either displaying a blank border and garbage in the Game Boy screen area, or showing correct graphics but with nametable corruption exhibiting a checkboard pattern. It is suspected that MMC5 ExRAM is being initialized properly, but CIRAM is not. 

### Unused code

The Demo Boy II revision A firmware contains unused code that is incompatible with the finalized hardware. A joypad-reading function is present that targets standard controllers, but keeps OUT1 set, suggesting that OUT1 may have originally controlled Game Boy reset. Another function reads Game Boy screen pixel data from $8000-A3FF, separates it into bitplanes, and copies it into CHR-RAM, but in the finalized hardware, all of this is automated, there are two screen buffers, and the Game Boy screen data is not believed to be mapped into the CPU address space. 

## Pinouts

### Power connector (P1)
    
    
    Demo Vision
       +---+  
       | 2 | -- GND
       | 1 | -- +5V
       +---+
    

### Game Boy connector (P2)
    
    
              Demo Vision
               +-------+
               |       |
          S -> |01   02| <- CPL
         ST -> |03   04| <- CP 
        LD1 ->  05   06  <- LD0
    speaker -> |07   08| -> /reset
        +5V -- |09   10| -- GND
               |       |
               +-------+
    

Pinout is as viewed into the port, pin 1 on the top right. Note that while this is keyed the same on both sides, the PCB partially blocks the keyhole on the bottom side, preventing the cable from being fully inserted upside-down. 

#### Game Boy (DMG) connections
    
    
    cable |  | Game Boy
    --------------------------
       01 |<-| P3.12 (S)
       02 |<-| P3.18 (CPL)
       03 |<-| P3.17 (ST)
       04 |<-| P3.14 (CP)
       05 |<-| P3.15 (LD1)
       06 |<-| P3.16 (LD0)
       07 |<-| P3.20 (speaker)
       08 |->| SW1.3 (/reset)
       09 |--| SW1.1 (+5V)
       10 |--| P3.01 (GND)
    

#### Game Boy Pocket (MGB) connections
    
    
    cable |  | Game Boy Pocket
    --------------------------
       01 |<-| S
       02 |<-| CPL
       03 |<-| ST
       04 |<-| P2-CP
       05 |<-| LD1
       06 |<-| P2-LD0
       07 |<-| U3.3 (speaker)
       08 |->| P1.30 RES
       09 |--| SW1-VCC
       10 |--| P1.32 GND
    

## Hardware configurations

Demo Boy II: 

  * Board: 
    * Rev A (green PCB)
    * Rev B (green PCB)
  * Chipsets: 
    * RP2A03G / RP2C02G-0
    * RP2A07A / RP2C07-0 (Europe)



Demo Vision: 

  * Board: 
    * Rev B (blue PCB)
  * Chipsets: 
    * RP2A03G / RP2C02G-0 (domestic)
    * RP2A07A / RP2C07-0 (Europe)



The Demo Boy II rev B and Demo Vision boards are suspected to be the same aside from color. 
