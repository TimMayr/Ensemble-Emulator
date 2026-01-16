# Famicom 3D System

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Famicom_3D_System) | View [other pages](Special_AllPages.xhtml#Famicom_3D_System)

[![](../wiki-images/Famicom-3D-System.jpg)](File_Famicom_3D_System_jpg.xhtml)

[](File_Famicom_3D_System_jpg.xhtml "Enlarge")

Famicom 3D System

The Famicom 3D System is a Famicom expansion port device that uses active LCD shutter glasses to provide 3D visuals. The headset (HVC-031) plugs into a control box (HVC-032) using a 3.5mm connector. The control box has two such ports, allowing up to two headsets, and passes through a Famicom expansion port so other controllers can be used. Compatible games manually control which eye sees which frame, alternating to give each eye a unique perspective that creates a 30 Hz stereoscopic image. While these glasses do not require a CRT TV to correctly function, modern TVs without very low latency and some kind of impulse display, such as black frame insertion, may not be compatible. 

The 3D System is supported by the following Famicom games: 

  * _Attack Animal Gakuen_ (1987)
  * _Cosmic Epsilon_ (1989)
  * _Falsion_ (1987)
  * _Famicom Grand Prix II: 3D Hot Rally_ (1988)
  * _Fuuun Shourin Ken: Ankoku no Maou_ (1988)
  * _Highway Star_ (1987)
  * _JJ: Tobidase Daisakusen Part II_ (1987)
  * _Tobidase Daisakusen_ (1987)



In these games, the 3D mode can be toggled with the select button except in _Fuuun Shourin Ken: Ankoku no Maou_ , where the mode is enabled by holding A while pressing start on the title screen. 

Note that the 3D System was only released in Japan. Versions of these games released internationally (_3-D WorldRunner_ and _Rad Racer_) instead achieve 3D with an anaglyph mode using passive red and cyan glasses. 

### Output ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxFx
           |
           +--- 0: Next field of video is for left eye (left eye lens is transparent; right eye lens is opaque)
                1: Next field of video is for right eye
    

Because the Famicom only generated images at 60Hz, each resultant image will flicker at 30Hz. 
