# Controller port pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Controller_port_pinout) | View [other pages](Special_AllPages.xhtml#Controller_port_pinout)

## Contents

  * 1 Pinout
  * 2 Protection Diodes
  * 3 Super NES
  * 4 Notes
  * 5 Hardware design
  * 6 See Also
  * 7 References



## Pinout
    
    
                                +----------< $4017.D3          
                                | +--------< D0                
                                | | +------> OUT0             +------------------ GND
                                | | | +----> CLK              | +--------------<> AUDIO
                                | | | | +--< $4017.D4         | |           +---< D0
            .-                  | | | | |                     | |           |
     GND -- |O\              +-------------+               +-------------------+
     CLK <- |OO\ -- +5V       \ O O O O O /                 \ O O O O O O O O /
     OUT <- |OO| <- D3         \ O O O O /                   \ O O O O O O O /
      D0 -> |OO| <- D4          +-------+                     +-------------+
            '--'                 |   |                         |     |     |
                                 |   +------ GND               |     |     +---- +5V
                                 +---------- +5V               |     +---------: OUT0
                                                               +---------------> CLK
         NES port           9 pin famiclone port               15 pin famiclone port     
    	 
     * Directions are relative to the jack on the from of console
     * $4017.D3/$4017.D4 are available only at port 2 in consoles equipped only with 2x9pin ports
     * AUDIO might not always be available
    

The 15-pin connector is a subset of the standard Famicom [expansion port](Expansion_port.xhtml "Expansion port"). 

Official [standard controllers](Standard_controller.xhtml "Standard controller") usually use a standard coloring for their wires: 

Signal  | NES | Famicom player 1 | Famicom player 2   
---|---|---|---  
+5V  | **White** | **White** | **Blue**  
OUT  | **Orange** | **Orange** | **Yellow**  
D0  | **Red** | **Yellow** | **Orange**  
GND  | **Brown** | **Brown** | **Red**  
CLK  | **Yellow** | **Red** | **White**  
D2  | \---  | \---  | **Brown**  
  
## Protection Diodes

Some PAL region systems have a set of diodes on the inside of the controller port which make it incompatible with NTSC controllers.[1]
    
    
    +5V --|>|-- jack
     D3 --|>|-- jack
     D4 --|>|-- jack
     D0 --|>|-- jack
    OUT --|<|-- jack
    CLK --|<|-- jack
    

With these diodes, the OUT and CLK lines have to be pulled high by the controller, or else the controller can't receive these signals. See: [Standard controller: PAL](Standard_controller.xhtml#PAL "Standard controller")

These diodes are also present on the ports of the [Four Score](Four_player_adapters.xhtml "Four Score") (NESE-034) accessory for this region. 

## Super NES

The [FC Twin](https://en.wikipedia.org/wiki/FC_Twin "wikipedia:FC Twin") NES/SNES combo clone uses Super NES controllers, whose pinout is as follows:[2]
    
    
     1 [oooo|ooo) 7  1:+5V  2:Clk  3:Out  4:D0  5:D1  6: I/O  7:Gnd
    

An adapter to use Super NES controllers on an NES (or NES controllers on an FC Twin) could be constructed as follows (leaving D3 and D4 unconnected): 
    
    
            .-
     GND -- |**7** \
     CLK <- |**21** \ -- +5V
     OUT <- |**3** o| -- (D3)
      D0 -> |**4** o| -- (D4)
            '--'
    

  1. Buy two controller extension cables, one for NES and one for Super NES, and cut them apart. Strip the cut ends to reveal a small amount of bare wire.
  2. Using [inline splice technique](http://www.instructables.com/id/Master-a-perfect-inline-wire-splice-everytime/?ALLSTEPS), wrap each wire from one cable with the corresponding wire from the other cable.
  3. With solder and a soldering iron, glue each wrapped pair together.
  4. With electrical tape, wrap each joint to insulate it from the other wires.
  5. Apply heat shrink around the whole assembly.



## Notes

  * The signal read by the CPU is logically inverted from the signal input on the D0-4 lines. A low voltage on D0 will be read as a 1 bit from $4016/4017.
  * CLK will be low during reads from the CPU, then immediately return to high. This rising edge transition is used to clock the shift register inside the [standard controller](Standard_controller.xhtml "Standard controller").
  * OUT is a signal latched and held from the last CPU write to $4016:0. For standard controller reads, the program will write a 1 to load the shift register, then return to 0 before reading the results.



## Hardware design

  * Famicom and NES controllers use single [4021](4021.xhtml "4021") 8-step shift register for serial transmission (pressed button is shorted to GND by low-resistance rubber pad; VCC pull-ups are inside 4021)


    
    
          +-------
          | -----+
     BTN--+----- +-- GND
          | -----+
          +-------
    

  * Cascading input is connected to GND, which results in returning 1 for reads greater than 8 (in contrary to most blob pads, where it returns 0).


  * Rare model of non-blob IQ502 famiclone joypad uses [UM6582](UM6582_pinout.xhtml "UM6582 pinout") chip which aggregates both serial encoder and square wave generator (for turbo buttons)


  * Most famiclone pads use glob chip which integrates both serial encoder and turbo generator


  * Pressing TURBOA (or TURBOB) causes short of A (or B) to the square wave generated by chip (or blob).


    
    
               TURBO A
                ----
     TURBO >----'  '--+
                      |
                ----  |
       GND -----'  '--+----> !A
                 A
                 
    

  * Some joypads contains also buttons C and TURBO C (whose acts as pressing buttons A and B together); The SMD contact for that button contains three signals


  * SNES joypads contain two 4021 or two WR545 chips chained together


    
    
               ,---v---.                 WR545#1  WR545#2  
           d0->|01   14|--VCC         d0  B       A
         DOUT<-|02   13|<-d1          d1  Y       X
           d4->|03   12|<-d2          d2  SEL     TL
           d5->|04   11|<-d3          d3  ST      TR
           d6->|05   10|<-DIN         d4  U       -
           d7->|06   09|<-CLOCK       d5  D       -
          GND--|07   08|<-STROBE      d6  L       -
               +-------+              d7  R       -
                WR545
    

## See Also

  * [Standard controller](Standard_controller.xhtml "Standard controller")
  * [Expansion port](Expansion_port.xhtml "Expansion port")



## References

  1. ↑ [Forum post](https://forums.nesdev.org/viewtopic.php?p=238272#p238272): explaining PAL controller diodes and their function.
  2. ↑ [superfamicom.org](https://wiki.superfamicom.org/schematics-ports-and-pinouts): Schematics, Ports, and Pinouts.



Categories: [Controllers](Category_Controllers.xhtml), [Pinouts](Category_Pinouts.xhtml)
