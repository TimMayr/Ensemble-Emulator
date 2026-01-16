# SNES controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/SNES_controller) | View [other pages](Special_AllPages.xhtml#SNES_controller)

[![](../wiki-images/SNES-Controller-Flat.jpg)](File_SNES_Controller_Flat_jpg.xhtml)

[](File_SNES_Controller_Flat_jpg.xhtml "Enlarge")

Standard SNES controller

The **Super NES Controller** (SHVC-005, SNS-005, SNS-102) is very similar to the NES's [standard controller](Standard_controller.xhtml "Standard controller"), with a collection of digital inputs that are latched and read in series. With a [suitable adapter](Controller_port_pinout.xhtml "Controller port pinout"), reading the SNES controller is like reading an NES controller with 4 extra buttons. 

The 16 bit report will read, in order: 
    
    
     0 - B
     1 - Y
     2 - Select
     3 - Start
     4 - Up
     5 - Down
     6 - Left
     7 - Right
    
    
    
     8 - A
     9 - X
    10 - L
    11 - R
    12 - 0
    13 - 0
    14 - 0
    15 - 0
    

Note that the first 8 values map directly to the original NES controller's 8 inputs (SNES **B** = NES **A** , and SNES **Y** = NES **B**). The [Virtual Boy controller](Virtual_Boy_controller.xhtml "Virtual Boy controller") likewise returns button states in nearly the same order, with the right Control Pad corresponding to ABXY on the Super NES. 

The last 4 bits will always read 0 from a standard SNES controller. Other values here may indicate other devices,[1] such as a [mouse](Mouse.xhtml "Mouse") or a Virtual Boy controller. 

After the 16 bit report, subsequent bits will read as 1. 

## Games

An incomplete list of NES games with special support for SNES controllers: 

  * _Spook-O-Tron_
  * _Nova the Squirrel_[2]
  * _Full Quiet_[3]



## See also

  * [Controller port pinout](Controller_port_pinout.xhtml "Controller port pinout")
  * [Controller reading](Controller_reading.xhtml "Controller reading")



## References

    

  * [Standard controller](https://snes.nesdev.org/wiki/Standard_controller) at SNESdev Wiki



  1. ↑ [FullSNES:](http://problemkaputt.de/fullsnes.htm#snescontrollershardwareidcodes) SNES Controller Hardware ID Codes
  2. ↑ [Nova the Squirrel (WIP)](https://forums.nesdev.org/viewtopic.php?p=198552#p198552) \- forum post discussing SNES controller support.
  3. ↑ [Tweet by E.C. Myers](https://twitter.com/ecmyers/status/1600574149092589569)



Categories: [Controllers](Category_Controllers.xhtml), [Super NES](Category_Super_NES.xhtml)
