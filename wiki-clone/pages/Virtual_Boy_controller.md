# Virtual Boy controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Virtual_Boy_controller) | View [other pages](Special_AllPages.xhtml#Virtual_Boy_controller)

The Virtual Boy controller has an NES-compatible protocol, and has been used in homebrew games. 

  * _Spook-O'-Tron_
  * _Candelabra - Estoscerro_



After strobing the controller, the following 16 bits can be read from the data line: 
    
    
     0 - Right D-pad Down
     1 - Right D-pad Left
     2 - Select
     3 - Start
     4 - Left D-pad Up
     5 - Left D-pad Down
     6 - Left D-pad Left
     7 - Left D-pad Right
    
    
    
     8 - Right D-pad Right
     9 - Right D-pad Up
    10 - L (rear left trigger)
    11 - R (rear right trigger)
    12 - B
    13 - A
    14 - Always 1
    15 - Battery voltage; 1 = low voltage
    

This is very analogous to the [SNES controller](SNES_controller.xhtml "SNES controller"), which reports its 4 face buttons where the VB reports its right d-pad. However, the last 4 bits (B, A, 1, battery) have no correspondence in the SNES controller report. Use this 1 to distinguish the Virtual Boy controller from that controller or a [mouse](Mouse.xhtml "Mouse"). 

## References

  * [VB Sacred Tech Scroll](https://web.archive.org/web/20190319100353/perfectkiosk.net/stsvb.html#hardwaregamepad): Virtual Boy Specifications
  * [PlanetVB](https://www.planetvb.com/modules/tech/?sec=docs): Documents
  * [Sly Dog Studios](https://slydogstudios.org/): Candelabra - Estoscerro demo available
  * [Forum post](https://forums.nesdev.org/viewtopic.php?f=22&t=15677): Spook-o'-tron - Virtual Boy Controller Fun



Categories: [Controllers](Category_Controllers.xhtml)
