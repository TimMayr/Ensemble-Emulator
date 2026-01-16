# Exciting Boxing Punching Bag

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Exciting_Boxing_Punching_Bag) | View [other pages](Special_AllPages.xhtml#Exciting_Boxing_Punching_Bag)

This punching bag/doll is used in a single game: 

  * _Exciting Boxing_



It contains 8 sensors, each returned as a single bit when reading $4017: 

  * Left/Right Hook
  * Left/Right Jab
  * Left/Right Move
  * Straight
  * Body



## Contents

  * 1 Hardware interface
    * 1.1 Input ($4016 write)
    * 1.2 Output ($4017 read)
  * 2 See Also



## Hardware interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxAx
           |
           +- Select output sensors (see below)
          
    

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxE DCBx
       | |||
       | ||+-- Left Hook (A = 0), Left Jab (A = 1)
       | |+--- Move Right (A = 0), Body (A = 1)
       | +---- Move Left (A = 0), Right Jab (A = 1)
       +------ Right Hook (A = 0), Straight (A = 1)
    

## See Also

  * [image and English description](https://famicomworld.com/system/controllers/exciting-boxing/)



Categories: [Controllers](Category_Controllers.xhtml)
