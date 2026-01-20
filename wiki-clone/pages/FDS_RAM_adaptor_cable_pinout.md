# FDS RAM adaptor cable pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FDS_RAM_adaptor_cable_pinout) | View [other pages](Special_AllPages.xhtml#FDS_RAM_adaptor_cable_pinout)

The [FDS](Family_Computer_Disk_System.xhtml "FDS") RAM adapter requires a cable to be connected to the disk drive (physical or emulated) in order to facilitate disk transfers. 

## Contents

  * 1 Diagram
  * 2 Pin meanings
  * 3 Signal Descriptions
    * 3.1 Pin 1 (Output) /write ($4025.2)
    * 3.2 Pin 3 (Output) /scan media ($4025.0)
    * 3.3 Pin 5 (Output) write data
    * 3.4 Pin 6 (Input) motor on, battery good ($4033.7)
    * 3.5 Pin 7 (Input) /writable media ($4032.2)
    * 3.6 Pin 9 (Input) read data
    * 3.7 Pin 10 (Input) /media set ($4032.0)
    * 3.8 Pin 11 (Input) /ready ($4032.1)
    * 3.9 Pin 12 (Output) /stop motor ($4025.1)
  * 4 Notes



## Diagram

Open-end view of the RAM adapter's disk drive connector. 
    
    
     ████████████████████████████████
     ████████████████████████████████
     ███  1   3   5   7   9   11  ███
     ███                          ███
     ███  2   4   6   8   10  12  ███
     \██████████████████████████████/
      \████████████████████████████/
    

## Pin meanings

pin # | 2C33 register | *[2C33 pin](RP2C33_pinout.xhtml "RP2C33 pinout") | *RAM pins | I/O | signal description   
---|---|---|---|---|---  
1 | $4025.2 | 50 | 5 (green) | Output | /write   
2 | - | 64 | 12 (cyan) | Output | VCC (+5VDC)   
3 | $4025.0 | 48 | 6 (blue) | Output | /scan media   
4 | - | 32 | 1 (brown) | Output | VEE (ground)   
5 | $4024[1] | 52 | 3 (orange) | Output | write data   
6 | $4033.7 | 37 | 11 (pink) | Input | motor on/battery good   
7 | $4032.2 | 47 | 8 (grey) | Input | /writable media   
8 | - | - | - | Input | motor power†   
9 | $4031[1] | 51 | 4 (yellow) | Input | read data   
10 | $4032.0 | 45 | 10 (black) | Input | /media set   
11 | $4032.1 | 46 | 9 (white) | Input | /ready   
12 | $4025.1 | 49 | 7 (violet) | Output | /stop motor   
      
    
    notes on symbols
    ----------------
    I/O: input/output
    / : Indicates a signal which is active on a low (0) condition.
    * :	These are corresponding pinouts for the 2C33 I/O chip, and the other  end  
    of the RAM adaptor cable, which both are located inside the RAM adaptor.
    † :	The RAM adaptor does not use this signal (there is no wire in the cable 
    to carry the signal). An electronically controlled 5-volt power supply 
    inside the disk drive unit generates the power that appears here. This power 
    is also shared with the drive's internal electric motor. Therefore, the 
    motor only comes on when there is voltage on this pin.
    

## Signal Descriptions

### Pin 1 (Output) /write ($4025.2)

While active, this signal indicates that data appearing on the "write data" signal pin is to be written to the storage media. 

### Pin 3 (Output) /scan media ($4025.0)

While inactive, this instructs the storage media pointer to be reset (and stay reset) at the beginning of the media. When active, the media pointer is to be advanced at a constant rate, and data progressively transferred to/from the media (via the media pointer). 

### Pin 5 (Output) write data

This is the serial data the RAM adaptor issues to be written to the storage media on the "/write" condition. 

### Pin 6 (Input) motor on, battery good ($4033.7)

Applicable mostly to the FDS disk drive unit only, after the RAM adaptor issues a "/scan media" signal, it will check the status of this input to see if the disk drive motor has turned on. If this input is found to be inactive, the RAM adaptor interprets this as the disk drive's batteries having failed. Essentially, this signal's operation is identical to the above mentioned "motor power" signal, except that this is a TTL signal version of it. 

### Pin 7 (Input) /writable media ($4032.2)

When active, this signal indicates to the RAM adaptor that the current media is not write protected. 

### Pin 9 (Input) read data

When "/scan media" is active, data that is progressively read off the storage media (via the media pointer) is expected to appear here. 

### Pin 10 (Input) /media set ($4032.0)

When active, this signal indicates the presence of valid storage media. 

### Pin 11 (Input) /ready ($4032.1)

Applicable mostly to the FDS disk drive unit only, the falling edge of this signal would indicate to the RAM adaptor that the disk drive has acknowledged the "/scan media" signal, and the disk drive head is currently at the beginning of the disk (most outer track). While this signal remains active, this indicates that the disk head is advancing across the disk's surface, and appropriate data can be transferred to/from the disk. This signal would then go inactive if the head advances to the end of the disk (most inner track), or the "/scan media" signal goes inactive. 

### Pin 12 (Output) /stop motor ($4025.1)

Applicable mostly to the FDS disk drive unit only, the falling edge of this signal would instruct the drive to stop its motor (and therefore end the current scan of the disk). 

## Notes

  1. ↑ 1.0 1.1 Parallel/serial conversion is done via a pair of shift registers inside 2C33



Categories: [Pinouts](Category_Pinouts.xhtml)
