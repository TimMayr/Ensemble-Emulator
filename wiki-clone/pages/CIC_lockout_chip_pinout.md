# CIC lockout chip pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CIC_lockout_chip_pinout) | View [other pages](Special_AllPages.xhtml#CIC_lockout_chip_pinout)

## NES [CIC lockout chip](CIC_lockout_chip.xhtml "CIC lockout chip")
    
    
                     ----_----
     Data Out 01 <-x|P0.0  Vcc|--- 16 +5V
     Data In  02 x->|P0.1 P2.2|x-x 15 Gnd
     Seed     03 x->|P0.2 P2.1|x-x 14 Gnd
     Lock/Key 04 x->|P0.3 P2.0|x-x 13 Gnd
     N/C      05 x- |Xout P1.3|<-x 12 Gnd/Reset speed B
     Clk in   06  ->|Xin  P1.2|<-x 11 Gnd/Reset speed A
     Reset    07  ->|Rset P1.1|x-> 10 Slave CIC reset
     Gnd      08 ---|Gnd  P1.0|x-> 09 /Host reset
                     ---------
    
    P0.x = I/O port 0
    P1.x = I/O port 1
    P2.x = I/O port 2
    Xin  = Clock Input
    Xout = Clock Output
    Rset = Reset
    Vcc  = Input voltage
    Gnd  = Ground
    ->|  = input
    <-|  = output
    -x|  = unused as input
    x-|  = unused as output
    ---  = Neither input or output
    
    The CIC is a primitive 4-bit microcontroller. It contains the following registers:
    
    +-+         +-------+  +-------+-------+-------+-------+
    |C|         |   A   |  |       |       |       |       |
    +-+         +-+-+-+-+  +- - - - - - - - - - - - - - - -+
                |   X   |  |       |       |       |       |
            +---+-+-+-+-+  +- - - - - - - - - - - - - - - -+
            |     P     |  |       |       |       |       |
            | PH|   PL  |  +- - - - - - - - - - - - - - - -+
    +-------+-+-+-+-+-+-+  |       |       |       |       |
    |         IC        |  +- - - - - - - -R- - - - - - - -+
    +-+-+-+-+-+-+-+-+-+-+  |       |       |       |       |
    |                   |  +- - - - - - - - - - - - - - - -+
    +- - - - - - - - - -+  |       |       |       |       |
    |                   |  +- - - - - - - - - - - - - - - -+
    +- - - - -S- - - - -+  |       |       |       |       |
    |                   |  +- - - - - - - - - - - - - - - -+
    +- - - - - - - - - -+  |       |       |       |       |
    |                   |  +- - - - - - - - - - - - - - - -+
    +-+-+-+-+-+-+-+-+-+-+
    
    A  = 4-bit Accumulator
    C  = Carry flag
    X  = 4-bit General register
    P  = Pointer, used for memory access
    PH = Upper 2-bits of P
    PL = Lower 4-bits of P, used for I/O
    IC = Instruction counter, to save some space; it counts in a polynominal manner instead of linear manner
    S  = Stack for the IC register
    R  = 32 nibbles of RAM
    There are also 512 (768 for the 3195A) bytes of ROM, where the executable code is stored.
    

## Kevtris' CIClone Lockout chip pinout
    
    
                              ,---_---.
                     +5V 1 ---|01   08|-- 8 GND
                     CLK 2 x->|02   07|<-x /Force NTSC
     Lockout functioning 3 <-x|03   06|x-> Data Out
                 Data In 4 x->|04   05|<-x Reset
                              `-------'
    

Lockout functioning (3)
    This signal goes high when the lockout chip successfully completes 64 frames. The "lockout functioning" pin is only for debug use. Do not rely on it as some form of cartridge power up reset. Due to toploaders lacking the 4MHz clock, this pin will float or do odd things in those systems. Cutting pin 4 of the lockout chip on a frontloader will cause the pin never to go high.
/Force NTSC (7)
    Pulling this pin low forces the chip into NTSC only (3193 only) mode. The three PAL modes are not usable. Floating (disconnecting) this pin allows the chip to try all 4 regions.

## krikzz's AVR ATtiny13 Lockout chip pinout
    
    
                              ATtiny13A
                              ,---_---.
                        nc -x |1     8| --- VCC 
              (NES-71) clk x->|2     7| <-x rst (NES-70)
                       led <-x|3     6| <-x din (NES-34)
                       GND ---|4     5| x-> dout(NES-35)
                              `-------'
    

The pin "led" is low when the lockout is functioning normally, high when the chip is trying to change region. 

Categories: [Integrated circuits](Category_Integrated_circuits.xhtml), [Pinouts](Category_Pinouts.xhtml)
