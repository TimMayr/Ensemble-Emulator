# Cartridge connector

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Cartridge_connector) | View [other pages](Special_AllPages.xhtml#Cartridge_connector)

## Contents

  * 1 Pinout of 60-pin Famicom consoles and cartridges
  * 2 Pinout of 72-pin NES consoles and cartridges
  * 3 Additional pinout notes
  * 4 Comparison
  * 5 Signal descriptions



## Pinout of 60-pin Famicom consoles and cartridges

This diagram represents a top-down view looking directly into the connector. Pins 01–30 are on the label side of the cartridge, left to right. 

The pitch, or pin spacing, of this connector is 2.5 _4_ mm. This corresponds to 0.1 inch. 
    
    
      (front)                 (back)
      Famicom    | Cart  |    Famicom
                  -------
          GND -- |01   31| -> CartridgePresent or 5V supply, depending
      CPU A11 -> |02   32| <- M2
      CPU A10 -> |03   33| <- CPU A12
       CPU A9 -> |04   34| <- CPU A13
       CPU A8 -> |05   35| <- CPU A14
       CPU A7 -> |06   36| <> CPU D7
       CPU A6 -> |07   37| <> CPU D6
       CPU A5 -> |08   38| <> CPU D5
       CPU A4 -> |09   39| <> CPU D4
       CPU A3 -> |10   40| <> CPU D3
       CPU A2 -> |11   41| <> CPU D2
       CPU A1 -> |12   42| <> CPU D1
       CPU A0 -> |13   43| <> CPU D0
      CPU R/W -> |14   44| <- /ROMSEL (/A15 + /M2)
         /IRQ <- |15   45| <- Audio from 2A03
          GND -- |16   46| -> Audio to RF
      PPU /RD -> |17   47| <- PPU /WR
    CIRAM A10 <- |18   48| -> CIRAM /CE
       PPU A6 -> |19   49| <- PPU /A13
       PPU A5 -> |20   50| <- PPU A7
       PPU A4 -> |21   51| <- PPU A8
       PPU A3 -> |22   52| <- PPU A9
       PPU A2 -> |23   53| <- PPU A10
       PPU A1 -> |24   54| <- PPU A11
       PPU A0 -> |25   55| <- PPU A12
       PPU D0 <> |26   56| <- PPU A13
       PPU D1 <> |27   57| <> PPU D7
       PPU D2 <> |28   58| <> PPU D6
       PPU D3 <> |29   59| <> PPU D5
          +5V -- |30   60| <> PPU D4
                  -------
    

  * pin 31: On certain revisions of the RF modulator board, pin 31 connects only to the TV/Game switch on the RF modulator board, meaning that the RF modulator is only allowed to send video if a cartridge connects pin 30 to pin 31. On other RF modulator board revisions, pin 31 is connected to the normal 5V supply.



## Pinout of 72-pin NES consoles and cartridges

This diagram represents a top-down view looking directly into the connector. On a front-loader, pins 01–36 are the top side of the connector. Pins 36–01 are on the label side of the cartridge, left to right. 

The pitch, or pin spacing, of this connector is 2.5 _0_ mm. This does _NOT_ correspond to 0.1 inch. 

![](../wiki-images/Ambox_content.png) |  **PPU A10 and PPU A11 (pins 63 and 62) are in reverse order (i.e. not sequential/linear) on the NES!**  
---|---  
![](../wiki-images/Ambox_content.png) |  **The pitch, or pin spacing, of this connector is 2.5 _0_ mm. This does _NOT_ correspond to 0.1 inch. Typical female board edge connectors are not compatible.**  
---|---  
      
    
     (front/top)           (back/bottom)
          NES    | Cart  |    NES
                  -------
          +5V -- |36   72| -- GND
     CIC toMB <- |35   71| <- CIC CLK
    CIC toPak -> |34   70| <- CIC +RST
       PPU D3 <> |33   69| <> PPU D4
       PPU D2 <> |32   68| <> PPU D5
       PPU D1 <> |31   67| <> PPU D6
       PPU D0 <> |30   66| <> PPU D7
       PPU A0 -> |29   65| <- PPU A13
       PPU A1 -> |28   64| <- PPU A12
       PPU A2 -> |27   63| <- PPU A10
       PPU A3 -> |26   62| <- PPU A11
       PPU A4 -> |25   61| <- PPU A9
       PPU A5 -> |24   60| <- PPU A8
       PPU A6 -> |23   59| <- PPU A7
    CIRAM A10 <- |22   58| <- PPU /A13
      PPU /RD -> |21   57| -> CIRAM /CE
        EXP 4    |20   56| <- PPU /WR
        EXP 3    |19   55|    EXP 5
        EXP 2    |18   54|    EXP 6
        EXP 1    |17   53|    EXP 7
        EXP 0    |16   52|    EXP 8
         /IRQ <- |15   51|    EXP 9
      CPU R/W -> |14   50| <- /ROMSEL (/A15 + /M2)
       CPU A0 -> |13   49| <> CPU D0
       CPU A1 -> |12   48| <> CPU D1
       CPU A2 -> |11   47| <> CPU D2
       CPU A3 -> |10   46| <> CPU D3
       CPU A4 -> |09   45| <> CPU D4
       CPU A5 -> |08   44| <> CPU D5
       CPU A6 -> |07   43| <> CPU D6
       CPU A7 -> |06   42| <> CPU D7
       CPU A8 -> |05   41| <- CPU A14
       CPU A9 -> |04   40| <- CPU A13
      CPU A10 -> |03   39| <- CPU A12
      CPU A11 -> |02   38| <- M2
          GND -- |01   37| <- SYSTEM CLK
                  -------
    

## Additional pinout notes

  * For the Famicom: most chips and components appear on the opposite side of the PCB from the label in Famicom cartridges. (Nintendo boards follow this convention, but third party boards vary.)
  * For the NES: most chips and components appear on the label side of the PCB in NES cartridges.
  * Active-Low signals are indicated by a / (slash) symbol.
  * The NES and Famicom connectors have a similar arrangement; the connector on the NES is mostly a mirror image of the Famicom's.
  * Most cartridge PCBs made by Nintendo are numbered the same way as indicated in these diagrams.



## Comparison

Pin(s) | 60-pin  
Famicom | 72-pin  
NES   
---|---|---  
+5V, GND  | ✓  | ✓   
CPU A0-A14, D0-D7, R/W, M2, /ROMSEL, /IRQ   
PPU A0-A13, D0-D7, /RD, /WR, /A13   
CIRAM A10, /CE   
[Audio in/out](Category_Expansion_audio.xhtml "Category:Expansion audio") | ✓  | ✗   
[CIC pins](CIC_lockout_chip.xhtml "CIC lockout chip") | ✗  | ✓   
[EXP 0-9](Expansion_port.xhtml#NES "Expansion port")  
SYSTEM CLK   
  
## Signal descriptions

![](../wiki-images/Ambox_notice.png) |  **Unless otherwise specified, these signals are from the console's viewpoint. So an input is a signal driven by the cartridge to the console, and an output is a signal driven by the console to the cartridge connector.**  
---|---  
  
  * **+5V** : 5V Power supply from the main voltage regulator.
  * **GND** : 0V power supply.
  * **SYSTEM CLK** : Main oscillator frequency output. It is only available on 72-pin connectors, and its speed varies between NTSC (21MHz) and PAL (27MHz) machines.
  * **M2** : Also called PHI2 (φ2) in official docs (however, see the [CPU M2 and CLK description](CPU_pinout.xhtml#Signal_description "CPU pin out and signal description") for additional details). This is the CPU clock output. When this signal is high, this means only that the CPU bus address is in a stable state. For both reads and writes, data is only guaranteed or required to be valid at the falling edge of this signal.
  * **CPU R/W** : The Read/Write signal output from the CPU. This signal is high during CPU reads and low during CPU writes (switches from one mode to another only when M2 is low).
  * **CPU A0..A14** : Also called just A0..A14 in official docs, or CPU A0..A14 (to not confuse with address outputs of [mappers](Mapper.xhtml "Mapper") sharing the same number). This is the CPU address bus. It is stable when M2 is high. Note that A15 exists, but is not directly available on the connector.
  * **CPU D0..D7** : Also called just D0..D7 in official docs, or CPU D0..D7. This is the CPU bidirectional data bus. It goes high impedance on reads, allowing external memory chips to place their data here.
  * **/ROMSEL** : This pin outputs the logical NAND of M2 and CPU A15. It is low when the CPU reads or writes to `$8000`–`$FFFF` and when the address is stable, allowing to enable ROM chips directly. [Advanced mappers](Category_ASIC_mappers.xhtml "Category:ASIC mappers") use more logic between this pin and the actual PRG /CE (to avoid [bus conflicts](Bus_conflict.xhtml "Bus conflict"), for example). Using this signal is the only way to determine the state of A15, so it's needed for any mappers doing any address decoding.
  * **/IRQ** : Interrupt request input. Pull low to trigger an interrupt to the CPU. There is an internal pull-up resistor in the NES/Famicom, so it can be left floating if interrupts aren't used. NES hardware can safely pull the pin high or low, but [PlayChoice-10](https://www.nesdev.org/w/index.php?title=PlayChoice-10&action=edit&redlink=1 "PlayChoice-10 \(page does not exist\)") modules must treat it as an open-collector input.
  * **Audio from 2A03** : Audio output from the 2A03's sound generation hardware, already amplified. Only exists with 60-pins connectors.
  * **Audio to RF** : Usually just tied to the audio from 2A03. This one goes directly to the sound output of the console. This allows cartridges to mix audio with their own audio sources. This is not directly present on 72-pins connectors.
  * **EXP0..9** : These connect to the [expansion port](Expansion_port.xhtml#NES "Expansion port") on the bottom of the NES-001 and have no predefined meaning, so they can be used by any cartridge and expansion device pair for whatever purpose. EXP6 has become the standard for expansion audio. See [EXP pins](EXP_pins.xhtml "EXP pins") for detailed pin usage.
  * **PPU /WR** : Also called /WE in official docs. This signal is low when the PPU is writing. On its falling edge, the address and data are stable.
  * **PPU /RD** : Also called /RD in official docs. This signal is low when the PPU is reading. On its falling edge, the address is stable, and the data should be stable until its rising edge.
  * **PPU A0..A13** : Also called PA0..13 in official docs. This is the PPU's address bus. Most boards tie PA13 directly to the /CE of CHR ROM or CHR RAM to map it into [pattern table](PPU_pattern_tables.xhtml "PPU pattern tables") space (`$0000`–`$1FFF`) without any extra logic.
  * **PPU D0..D7** : Also called PD0..7 in official documentation. This is the PPU's bidirectional data bus. Goes high impedance when PPU /RD goes low allowing memory devices to place their data here.
  * **PPU /A13** : The inverted form of PPU A13. Typically used to map [nametables](PPU_nametables.xhtml "PPU nametables") and [attribute tables](PPU_attribute_tables.xhtml "PPU attribute tables") to `$2000`–`$3FFF`.
  * **CIRAM /CE** : Also called VRAM /CS. This signal is used as an input to enable the internal 2k of VRAM (used for name table and attribute tables typically, but could be made for another use). This signal is usually directly connected with PPU /A13, but carts using their own RAM for name table and attribute tables will have their own logic implemented.
  * **CIRAM A10** : Also called VRAM A10. This is the 1k bank selection input for internal VRAM. This is used to control how the name tables are banked; in other words, this selects [nametable mirroring](Mirroring.xhtml#Nametable_Mirroring "Mirroring"). Connect to PPU A10 for vertical mirroring or PPU A11 for horizontal mirroring. Connect it to a software operated latch to allow bank switching of two separate name tables in single-screen mirroring (as in [AxROM](AxROM.xhtml "AxROM")). Many mappers have software operated mirroring selection: they multiplex PPU A10 and PPU A11 into this pin, selected by a latch.
  * **CIC +RST** and **CIC CLK** : On the top-loading NES-101, these two are connected to +5V and PPU D4 respectively. The other two CIC pins float.



Categories: [Pinouts](Category_Pinouts.xhtml)
