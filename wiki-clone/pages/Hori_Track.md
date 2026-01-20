# Hori Track

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Hori_Track) | View [other pages](Special_AllPages.xhtml#Hori_Track)

[![](../wiki-images/Hori-track.png)](File_Hori_track_png.xhtml)

[](File_Hori_track_png.xhtml "Enlarge")

Top view of trackball

Hori produced a trackball compatible with _Moero Pro Soccer_ , _Moero Pro Soccer_ , _Putt Putt Golf_ , and _US Championship V'Ball_. It was released in Japan, and what appears to be a prototype U.S. version was [exhibited behind glass in Nintendo World](https://forums.nesdev.org/viewtopic.php?f=2&t=15184&p=183787#p183787), but the U.S. version never reached stores. 

Report byte 1 is the embedded [standard controller](Standard_controller.xhtml "Standard controller"). 

Byte 2, MSB first: 
    
    
    7654 3210
    |||| ++++- Axis 2, signed 4 bit, XOR with $F
    ++++------ Axis 1, signed 4 bit, XOR with $F
    

Byte 3, MSB first: 
    
    
    7654 3210
    |||| ++++- Unknown (read and unused by games)
    ||++------ ID bits (1 or 2 depending on version)
    |+-------- Speed switch (0: Hi, 1: Lo)
    +--------- Rotation mode switch (0: R, 1: L)
    

In rotation mode L, Up on the Control Pad points up, axis 1 points down, and axis 2 points right. In rotation mode R, Up on the Control Pad points right, axis 1 points left, and axis 2 points down. 

Unlike the rotation mode switch, the speed switch will alter the axis values before they are presented to the console. 

The ID bits are in the same place as those of the [Four Score](Four_player_adapters.xhtml "Four Score") hub, which is also based on a Hori design. It is speculated that axis 2 can be used to distinguish them. 

Categories: [Controllers](Category_Controllers.xhtml), [Pointing devices](Category_Pointing_devices.xhtml)
