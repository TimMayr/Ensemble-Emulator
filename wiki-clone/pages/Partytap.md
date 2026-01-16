# Partytap

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Partytap) | View [other pages](Special_AllPages.xhtml#Partytap)

The Partytap is used in the following games: 

  * _Casino Derby_
  * _Gimmi a Break - Shijou Saikyou no Quiz OuKetteisen_
  * _Gimmi a Break - Shijou Saikyou no Quiz OuKetteisen 2_
  * _Project Q_



It consists of 6 separate 1-button controllers (labelled buttons 1 to 6 below). 

## Contents

  * 1 Hardware interface
    * 1.1 Input ($4016 write)
    * 1.2 Output ($4017 read)
  * 2 References



## Hardware interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxS
            |
            +- Controller shift register strobe
    

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxC BAxx
       | ||
       | |+- Serial data (Button 1, Button 4)
       | +-- Serial data (Button 2, Button 5)
       +---- Serial data (Button 3, Button 6)
    

The first read returns the state of buttons 1 to 3, the 2nd read gives buttons 4 to 6. The third read apparently returns a detection value ($14). 

The device supposedly requires a relatively large number of cycles between writes/reads (games wait up to 80-500 cycles in-between some operations). 

## References

  * [nocash's NES documentation](http://problemkaputt.de/everynes.htm#controllerspushbuttons)
  * [Forums - Party Tap](https://forums.nesdev.org/viewtopic.php?t=903) (in Japanese)



Categories: [Controllers](Category_Controllers.xhtml)
