# Konami Hyper Shot

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Konami_Hyper_Shot) | View [other pages](Special_AllPages.xhtml#Konami_Hyper_Shot)

The Konami Hyper Shot is used (and required for gameplay) in the following games: 

  * _Hyper Olympic (J) Konami (1985)_
  * _Hyper Sports (J) Konami (1985)_



## Contents

  * 1 Hardware interface
    * 1.1 Input ($4016 write)
    * 1.2 Output ($4017 read)
  * 2 See Also



## Hardware interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xEFx
          ||
          |+- 0=Enable Player 1 Buttons
          +-- 0=Enable Player 2 Buttons
    

  


### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxD CBAx
       | |||
       | ||+-- Player 1 Run
       | |+--- Player 1 Jump
       | +---- Player 2 Run
       +------ Player 2 Jump
    

The Jump/Run bits for a player will always be 0 if the corresponding enable bit in $4016 is set. 

## See Also

  * [video of outside, teardown, and game play](https://youtu.be/6PSbI03aGKc)
  * [SG-1000 version](https://segaretro.org/HyperShot)



Categories: [Controllers](Category_Controllers.xhtml)
