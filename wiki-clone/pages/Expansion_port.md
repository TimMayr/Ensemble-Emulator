# Expansion port

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Expansion_port) | View [other pages](Special_AllPages.xhtml#Expansion_port)

Both the NES and Famicom have expansion ports that allow peripheral devices to be connected to the system. 

See also: [Input devices](Input_devices.xhtml "Input devices")

## Contents

  * 1 Famicom
    * 1.1 Pinout
    * 1.2 Signal descriptions
  * 2 NES
    * 2.1 Pinout
    * 2.2 Signal notes



## Famicom

The Famicom has a 15-pin (male) port on the front edge of the console. 

Because its two default controllers were not removable like the NES, [peripheral devices](Input_devices.xhtml "Input devices") had to be attached through this expansion port, rather than through a [controller port](Controller_port_pinout.xhtml "Controller port pinout") as on the NES. 

This was commonly used for third party controllers, usually as a substitute for the built-in controllers, but sometimes also as a 3rd and 4th player. 

### Pinout
    
    
           (top)    Famicom    (bottom)
                   Male DA-15
                     /\
                    |   \
    joypad 2 /D0 ?? | 08  \
                    |   15 | -- +5V
    joypad 2 /D1 -> | 07   |
                    |   14 | -> /OE for joypad 1 ($4016 read strobe)
    joypad 2 /D2 -> | 06   |
                    |   13 | <- joypad 1 /D1
    joypad 2 /D3 -> | 05   |
                    |   12 | -> OUT0 ($4016 write data, bit 0, strobe on pads)
    joypad 2 /D4 -> | 04   |
                    |   11 | -> OUT1 ($4016 write data, bit 1)
            /IRQ ?? | 03   |
                    |   10 | -> OUT2 ($4016 write data, bit 2)
           SOUND <- | 02   |
                    |   09 | -> /OE for joypad 2 ($4017 read strobe)
             Gnd -- | 01  /
                    |   /
                     \/
    

### Signal descriptions

  * **Joypad 1 /D1** , **Joypad 2 /D0-/D4** : Joypad data lines, which are inverted before reaching the CPU. Joypad 1 /D1 and joypad 2 /D1-/D4 are exclusively inputs, but on the RF Famicom, Twin Famicom, and Famicom Titler, joypad 2 /D0 is supplied by the permanently-connected player 2 controller, making it an output. In contrast, the AV Famicom features user-accessible controller ports and thus detachable controllers, allowing joypad 2 /D0 to potentially be an input. At least one expansion port device, the Multi Adapter AX-1, expects joypad 2 /D0 to be an output.
  * **Joypad 1 /OE** , **Joypad 2 /OE** : Output enables, asserted when reading from $4016 for joypad 1 and $4017 for joypad 2. Joypads are permitted to send input values at any time and often use /OE just as a clock to advance a shift register. Internally, the console uses /OE to know when to put the joypad input onto the CPU data bus.
  * **OUT2-0** : Joypad outputs from the CPU matching the values written to $4016 D2-D0. These are updated every APU cycle (every 2 CPU cycles).
  * **/IRQ** : The direction of this signal depends on the cartridge being used. Some cartridges use a push/pull /IRQ driver, which doesn't permit anything else to disagree, preventing input on this pin. Otherwise, it can be used as an input.
  * **SOUND** : Analog audio output. In the RF Famicom, this is before expansion audio is mixed in. In the AV Famicom, it is after. It is possible to use this for audio input, but is inadvisable; there is no single way to mix in audio that is compatible with all consoles and all cartridges, and in most cases, the voltage must be carefully balanced to mix linearly with the signal output by the console's hex inverter.



## NES

The NES has a 48-pin card edge located on the underside of the NES, beneath a plastic tab which must be cut or broken to expose the connector. The connector is exceptionally thick (2.6mm), thicker than standard PCB thicknesses. The port containing the connector is slightly keyed in the front-side corners. 

Because the NES had [controller ports](Controller_port_pinout.xhtml "Controller port pinout") on the front that allowed different devices to be plugged in, the expansion port was a kind of "back up plan" for Nintendo that was never used commercially. 

### Pinout
    
    
                                  (back)       NES       (front)
                                            +-------\
                                     +5V -- |01   48| -- +5V
                                     Gnd -- |02   47| -- Gnd
                         Audio mix input -> |03   46| -- NC
                                    /NMI <> |04   45| -> OUT2 ($4016 write data, bit 2)
                                     A15 <- |05   44| -> OUT1 ($4016 write data, bit 1)
                                    EXP9 ?? |06   43| -> OUT0 ($4016 write data, bit 0, strobe on sticks)
                                    EXP8 ?? |07   42| ?? EXP0
                                    EXP7 ?? |08   41| ?? EXP1
                                    EXP6 ?? |09   40| ?? EXP2
                                    EXP5 ?? |10   39| ?? EXP3
    ($4017 read strobe) /OE for joypad 2 <- |11   38| ?? EXP4
                            joypad 1 /D1 -> |12   37| -> /OE for joypad 1 ($4016 read strobe)
                            joypad 1 /D3 xx |13   36| xx joypad 1 /D4
                                    /IRQ <> |14   35| xx joypad 1 /D0
                            joypad 2 /D2 -> |15   34| -> duplicate of pin 37
                            joypad 2 /D3 xx |16   33| <- joypad 1 /D2
                     duplicate of pin 11 <- |17   32| <> CPU D0
                            joypad 2 /D4 xx |18   31| <> CPU D1
                            joypad 2 /D0 xx |19   30| <> CPU D2
                            joypad 2 /D1 -> |20   29| <> CPU D3
                               Video out <- |21   28| <> CPU D4
                         Amplified audio <- |22   27| <> CPU D5
           unregulated power adapter vdd -- |23   26| <> CPU D6
                         4.00MHz CIC CLK <- |24   25| <> CPU D7
                                            +-------/
    

### Signal notes

  * All joypad input lines /D0-/D4 are logically inverted before reaching the CPU. A high signal will be read as a 0 and vice versa.
  * `xx` in above pinout: Joypad 1 and 2 /D0, /D3, and /D4 are available as an input if no peripheral is connected to the corresponding [joystick port](Controller_port_pinout.xhtml "Controller port pinout") that uses those bits: 
    * e.g. /D0 is unavailable if a [Standard controller](Standard_controller.xhtml "Standard controller") or [Four score](Four_player_adapters.xhtml "Four score") is plugged in, and
    * /D3 and /D4 are unavailable if a [Zapper](Zapper.xhtml "Zapper"), [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller"), or [Power Pad](Power_Pad.xhtml "Power Pad") is plugged in.
  * /NMI is open-collector.
  * /IRQ depends on the cartridge—most ASICs seem to use a push-pull driver instead of relying on the pull-up resistor inside the console. 
    * Because of this, a series 1kΩ resistor should be included to safely use the /IRQ signal in the expansion port.
    * This resistor is enough to logically overcome the internal 10kΩ pull-up, and will also limit any ASIC's output-high current to 5mA if your expansion device tries to drive it low at the same time.
  * See [EXP pins](EXP_pins.xhtml "EXP pins") for notes about the ten EXP pins.
  * See [Standard controller](Standard_controller.xhtml "Standard controller") and [Controller port pinout](Controller_port_pinout.xhtml "Controller port pinout") for more information about controller connections.



Categories: [Pinouts](Category_Pinouts.xhtml)
