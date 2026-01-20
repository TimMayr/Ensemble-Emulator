# Oeka Kids tablet

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Oeka_Kids_tablet) | View [other pages](Special_AllPages.xhtml#Oeka_Kids_tablet)

The Oeka Kids tablet is a Famicom accessory from Bandai that resembles a drawing tablet, and plugs into the expansion port. There are two known games that use this peripheral, using [iNES Mapper 096](INES_Mapper_096.xhtml "INES Mapper 096"), both of which contain a variety of activities, such as painting, drawing lessons, hiragana lessons, and a variety of minigames such as mazes and sliding puzzles. 

Very little information about this accessory is available, and only [basic reverse engineering](http://forums.nesdev.org/viewtopic.php?p=19454#19454) has been performed so far. As such, this information **may be incorrect** , but it seems to be acceptable for the two commercial games that use it. 

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxAS
           ||
           |+- Strobe (0 = Latch, 1 = Read mode)
           +-- Advance to next bit
    

Bits can be read only while S is 1. It advances to the next bit when S is 1 and A transitions from 0 to 1. 

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxx DSxx
         ||
         |+-- 0 if strobe is 1, 1 otherwise.
         +--- (Inverted) Serial data if strobe is 1, 0 if strobe is 0.
    

The serial data is returned _most significant bit first_ , and inverted (including the touch and click bits). 
    
    
    $4017.2
       ^
       |  <-- <-- <-- <--
       XXXXXXXXYYYYYYYYBA
       ||||||||||||||||||
       |||||||||||||||||+- Click
       ||||||||||||||||+-- Stylus is touching tablet
       ||||||||++++++++--- Stylus Y, scaled to 0-255
       ++++++++----------- Stylus X, scaled to 0-239
    

The fact that X and Y are scaled backwards is _not_ an error - the games which use this mapper rescale the coordinates by multiplying X by 256/240 and multiplying Y by 240/256. 

Note that the Stylus X and Y values are nonsense when the stylus is not touching the tablet. 

Categories: [Controllers](Category_Controllers.xhtml), [Pointing devices](Category_Pointing_devices.xhtml)
