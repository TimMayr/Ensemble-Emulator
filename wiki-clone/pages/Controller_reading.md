# Controller reading

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Controller_reading) | View [other pages](Special_AllPages.xhtml#Controller_reading)

NES and Famicom controllers are operated through a register interface that is connected to the [controller port](Controller_port_pinout.xhtml "Controller port pinout") and [expansion port](Expansion_port.xhtml "Expansion port"), as well as hardwired controllers on the Famicom. 

For most [input devices](Input_devices.xhtml "Input devices") a standard procedure is used for reading input: 

  1. Write 1 to $4016 to signal the controller to poll its input
  2. Write 0 to $4016 to finish the poll
  3. Read polled data one bit at a time from $4016 or $4017



## Contents

  * 1 $4016 Write
  * 2 $4016 / $4017 Read
    * 2.1 Clock timing
    * 2.2 DPCM conflict
      * 2.2.1 Multiple Read Solution
      * 2.2.2 Synchronized OAM Solution
    * 2.3 Unconnected data lines and open bus
  * 3 See Also
  * 4 References



## $4016 Write
    
    
    7  bit  0
    ---- ----
    xxxx xEES
          |||
          ||+- Controller port latch bit
          ++-- Expansion port latch bits
    

The low 3 bits written to this register will be latched and held. Its output will be continuously available on the OUT line of the [controller port](Controller_port_pinout.xhtml "Controller port pinout"), and the [expansion port](Expansion_port.xhtml "Expansion port"). 

Other bits are ignored. 

On the [standard controller](Standard_controller.xhtml "Standard controller") this is connected to the parallel/serial control of a 4021 8-bit shift register. Writing 1 to $4016 causes the register to fill its parallel inputs from the buttons currently held. Writing 0 to $4016 returns it to serial mode, waiting to be read out one bit at a time. Most other [input devices](Input_devices.xhtml "Input devices") operate in a similar way. 

## $4016 / $4017 Read
    
    
    7  bit  0
    ---- ----
    xxxD DDDD
    |||+-++++- Input data lines D4 D3 D2 D1 D0
    +++------- Open bus
    

Reading from this register causes a clock pulse to be sent to the [controller port](Controller_port_pinout.xhtml "Controller port pinout") CLK line on one controller, and one bit will be read from the connected input lines. $4016 reads only from controller port 1, and $4017 reads only from controller port 2. The read value is inverted: a high signal from the data line will read as 0, and a low signal will read as 1. 

For most devices it is necessary to read several times from these registers to collect multiple output bits from the device. 

The specific use of each data line depends on the [input device](Input_devices.xhtml "Input devices") connected. For the standard controller and [Zapper](Zapper.xhtml "Zapper") which commonly came with the NES/Famicom: 

Line | Used by   
---|---  
D0 | NES standard controller, Famicom hardwired controller   
D1 | Famicom expansion port controller   
D2 | Famicom microphone (controller 2 only)   
D3 | Zapper light sense   
D4 | Zapper trigger   
  
The NES [controller port](Controller_port_pinout.xhtml "Controller port pinout") makes only D0, D3 and D4 available for peripherals. The Famicom hardwired controllers connect to D0, and $4016.D2 (microphone) only. Some of the other lines can be connected through the [expansion port](Expansion_port.xhtml "Expansion port") on the Famicom. The NES expansion port was never used commercially, but connects all 5 data lines to both ports. 

The [Four Score](Four_player_adapters.xhtml#Four_Score "Four player adapters") multiplayer adapter for NES only passes D0 from the connected controllers. 

### Clock timing

The CLK line for controller port is _R/W nand (ADDRESS == $4016/$4017)_ (i.e., CLK is low only when reading $4016/$4017, since R/W high means read). When this transitions from high to low, the buffer inside the NES latches the output of the controller data lines, and when it transitions from low to high, the shift register in the controller shifts one bit.[1]

### DPCM conflict

Using DPCM sample playback while trying to read the controller can cause problems because of a [bug in its hardware implementation](Errata.xhtml#APU_DMC "Errata"). 

If the [DMC](APU_DMC.xhtml "APU DMC") DMA is running, and happens to start a read in the same cycle that the CPU is trying to read from $4016 or $4017, the values read will become invalid. Since the address bus will change for one cycle, the shift register will see an extra rising clock edge (a "double clock"), and the shift register will drop a bit out. The program will see this as a **bit deletion** from the serial data. Not correcting for this results in spurious presses. On the [standard controller](Standard_controller.xhtml "Standard controller") this is most often seen as a right-press as a trailing 1 bit takes the place of the 8th bit of the report (right). 

This glitch is fixed in the 2A07 CPU used in the PAL NES. 

This detail is poorly represented in emulators.[2] Because it is not normally a compatibility issue, many emulators do not simulate this glitch at all. 

#### Multiple Read Solution

The standard solution used in most games using DMC will read the controller multiple times and compare the results to avoid this problem. This is not suitable for controllers that can't be reread, such as the [Super NES Mouse](Super_NES_Mouse.xhtml "Super NES Mouse"). 

See: [Controller reading code](Controller_reading_code.xhtml "Controller reading code")

#### Synchronized OAM Solution

Because OAM DMA synchronizes the CPU and APU such that reads on an "even" CPU cycle never overlap a glitch, a program on an NTSC NES can miss all the glitches by triggering an OAM DMA as the last thing in vblank just before reading the controller, so long as all the reads are spaced an even number of cycles apart.[3] If the execution time of the read routine can span more than one DPCM sample fetch, additional care must be taken to align all CPU writes.[4]

Because this is a relatively new discovery, many current emulated implementations of the DMC glitch may be inadequate for testing this technique.[5] In Mesen 2, set a breakpoint on reading $4016-$4017 with `IsDma` as the condition. Hardware testing is recommended as well. 

See: [Controller reading code: DPCM Safety using OAM DMA](Controller_reading_code.xhtml#DPCM_Safety_using_OAM_DMA "Controller reading code")

### Unconnected data lines and open bus

The behaviour of unconnected lines is more complicated. For the most part, unconnected lines are either pulled to 0, or exhibit [open bus](Open_bus_behavior.xhtml "Open bus") behaviour. 

| NES-001  
$4016/$4017 | NES-101  
$4016/$4017 | Famicom  
$4016 | Famicom  
$4017   
---|---|---|---|---  
D0  | 0 if not connected | 0 if not connected[6] | 0 if not connected[6]  
D1  | always 0 | 0 if not connected | 0 if not connected   
D2  | always 0 | open bus[7] | microphone on controller 2 | 0 if not connected   
D3  | 0 if not connected | open bus | 0 if not connected   
D4  | 0 if not connected | open bus | 0 if not connected   
D5  | open bus   
D6   
D7   
  
## See Also

  * [Controller reading code](Controller_reading_code.xhtml "Controller reading code")
  * [Standard controller](Standard_controller.xhtml "Standard controller")
  * [Input devices](Input_devices.xhtml "Input devices")



## References

  1. ↑ [Forum post:](https://forums.nesdev.org/viewtopic.php?t=4116) DPCM generates extra $4016 read pulse
  2. ↑ <http://forums.nesdev.org/viewtopic.php?p=231604#p231604>
  3. ↑ [Forum post:](https://forums.nesdev.org/viewtopic.php?p=171971) Rahsennor's OAM-synchronized controller read
  4. ↑ [Forum post:](https://forums.nesdev.org/viewtopic.php?p=231604#p231604) demonstration of how ROL instruction affects alignment for OAM DMA synchronized controller reading.
  5. ↑ [Forum post:](https://forums.nesdev.org/viewtopic.php?f=2&t=14319&start=15#p172194) as of May 2016, Nintendulator and Nestopia do not accurately emulate OAM-synchronized controller reading.
  6. ↑ 6.0 6.1 Famicom controllers are not user-replacable, however they can be easily disconnected (or even replaced with external connectors) after disassembling the console. In that case, $4016/$4017 D0 reads 0.
  7. ↑ [Forum post](https://forums.nesdev.org/viewtopic.php?p=143759#p143759): Riding the open bus (NES-101 D2 behaviour)


