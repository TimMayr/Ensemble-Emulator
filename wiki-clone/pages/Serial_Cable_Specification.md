# Serial Cable Specification

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Serial_Cable_Specification) | View [other pages](Special_AllPages.xhtml#Serial_Cable_Specification)

The RS-232 serial protocol allows two devices to communicate over just a few wires. This page describes a serial cable connection scheme for the NES and Famicom that can use the second controller port or expansion port. 

## Contents

  * 1 Cable connections
    * 1.1 One-way cable
    * 1.2 Two-way cable
  * 2 Software interface
  * 3 Port pinouts
    * 3.1 NES controller port
    * 3.2 Famicom expansion port
    * 3.3 NES expansion port
  * 4 Design rationale



# Cable connections

Each of the two serial signals may be connected to any one of the listed pins. 

Serial | NES 2nd controller | Famicom expansion port | NES expansion port   
---|---|---|---  
TX | D0 (data) or D4 | 2nd controller D4 | 2nd controller D2   
RX & /CTS | OUT0 (strobe) | OUT0 | OUT0   
  
TX, RX, and /CTS are TTL-level signals, **not** RS-232-level signals. TX and RX use +5V for Mark, 0V for Space. **Do not** connect the NES directly to RS-232; see [Serial Cable Construction](Serial_Cable_Construction.xhtml "Serial Cable Construction") for proper connection. 

## One-way cable

A one-way cable connects TX to the second controller D0 or D4. On the NES, D0 and D4 are available on the second controller port. On the NES and Famicom, second controller D4 is available on the expansion port. 

Any other devices connected to D0, D1, D2 or D4 must drive the line high whenever OUT0 (strobe) is high, as is done when nothing is connected or a controller/Zapper is connected and no buttons are being held. 

## Two-way cable

A two-way cable additionally connects OUT0 to RX & /CTS. On the NES, OUT0 (strobe) is available on the second controller port. On the NES and Famicom, OUT0 is available on the expansion port. 

# Software interface

The TX line is available inverted as the logical OR of bits D0, D1, D2, and D4 of $4017. If zero, TX is high (Mark state). Bits D1 and D2 are included because future serial devices might connect to them. 

The RX & /CTS pair is controlled by $4016 D0. /CTS might not be supported by a given cable, in which case flow control can't be done. Software should write zero bits for D2, D3, and D4, in case they are used for other things. Also, at some point D1 may be used by some devices for serial output (since it avoids junk bytes when reading the controllers), so software should output serial data on bits 0 _and_ 1 of $4016. 

# Port pinouts

D0 (data) and D4 are inputs for second controller ($4017). OUT0 (strobe) is from the CPU ($4016). For clarity, unused pins are not numbered or labeled. 

## NES controller port
    
    
          _
     GND |1\ 
         |.5| +5V
    OUT0 |3.|
      D0 |47| D4
         '--'
    

## Famicom expansion port

Male connector. 
    
    
      GND          D4
    -----------------------------------
    \  1   .   .   4   .   .   .   .  /
     \                               /
      \  .   .   .  12   .   .  15  /
       `---------------------------'
                   OUT0        +5V
    

## NES expansion port
    
    
              ------- 
             |  ___  | 
         +5V | 1  48 | +5V
         GND | 2  47 | GND
             | [   ] |
             | [   ] |
             | [   ] |
             | [  43 | OUT0
             | [   ] |
             | [   ] |
             | [   ] |
             | [   ] |
             | [   ] |
    (back)   | [   ] |   (front of NES)
             | [   ] |
             | [   ] |
          D2 | 15  ] |
             | [   ] |
             | [   ] |
             | [   ] |
             | [   ] |
             | [   ] |
             | [   ] |
             | [   ] |
             | [   ] |
             | [   ] |
             |  ---  |
              -------
    

# Design rationale

**TX connected to four bits:** The NES controller port exposes D0, D3, and D4. The expansion ports of NES and Famicom expose D0-D4. The second Famicom controller is hardwired to D0, so we need something in addition to D0. D3 and D4 are available on both, but many NES controller cables don't connect either one. D2 is available on both, but it requires connecting to the NES expansion port, for which the connector is rare/non-existent. External Famicom controllers typically use D1, and might not connect to other data bits. So we use D0, D1, D2, and D4. 

D0 allows use of the common NES controller cable. D1 allows use with common Famicom external controller cables. D4 allows modification of a NES extension cable to pass through D0 to a controller, and connect serial at the same time. The Zapper connects to D3 and D4 and has the trigger connected to D4, so it won't generate any false data unless pressed. D2 allows connection via the expansion connector, without disturbing any controllers on NES/Famicom, even the Zapper. 

In software, it's simplest to support just D0, since it can be shifted into carry easily. Once it must support multiple bits, any combination of bits becomes just as easy as any other, since it's set by a bit mask to an AND instruction. 

**Active-high TX and RX:** A serial cable should be as easy to construct as possible. The FTDI TTL-232R USB-to-RS-232 cable outputs TTL levels, allowing construction of a serial cable with no soldering, just splicing it to a NES controller cable. The MAX232 also uses these levels. Further, simple transistor inverters match these as well. Since RX and /CTS are connected together, and CTS is normally non-inverted, we get the inverted /CTS. 
