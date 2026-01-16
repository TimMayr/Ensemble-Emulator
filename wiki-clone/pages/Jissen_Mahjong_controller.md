# Jissen Mahjong controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Jissen_Mahjong_controller) | View [other pages](Special_AllPages.xhtml#Jissen_Mahjong_controller)

This is a 21-button controller used by the following Capcom games: 

  * _Ide Yousuke Meijin no Jissen Mahjong_
  * _Ide Yousuke Meijin no Jissen Mahjong 2_



## Contents

  * 1 Button layout
  * 2 Hardware interface
    * 2.1 Input ($4016 write)
    * 2.2 Output ($4017 read)
  * 3 References



## Button layout

A | B | C | D | E | F | G | H | I | J | K | L | M | Ⓝ   
---|---|---|---|---|---|---|---|---|---|---|---|---|---  
| SEL | ST | カン | ポン | チー | リーチ | ロン   
  
The [カン (kan)](https://en.wikipedia.org/wiki/Japanese_Mahjong#Kan "wikipedia:Japanese Mahjong"), [ポン (pon)](https://en.wikipedia.org/wiki/Japanese_Mahjong#Pon "wikipedia:Japanese Mahjong"), [チー (chii)](https://en.wikipedia.org/wiki/Japanese_Mahjong#Ch.C4.AB "wikipedia:Japanese Mahjong"), [リーチ (riichi)](https://en.wikipedia.org/wiki/Japanese_Mahjong#R.C4.ABchi "wikipedia:Japanese Mahjong") and [ロン (ron)](https://en.wikipedia.org/wiki/Japanese_Mahjong#Winning "wikipedia:Japanese Mahjong") buttons are Mahjong-related. 

## Hardware interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xRRS
          |||
          ||+- Strobe
          |+-- Row selection (bit 0)
          +--- Row selection (bit 1)
    

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxx xxDx
           |
           +- Serial data
    

Reading the buttons is very similar to the [standard controller](Standard_controller.xhtml "Standard controller"), but one of three rows is selected for loading values _into_ the shift register depending on the contents of the two Row selection pins while the Strobe bit is high. 

After toggling the strobe bit from 1 to 0, the controller will return 8 bits worth of data (1 bit per read) based on the "row selection" bits. 
    
    
    Row 3 returns: <empty>, ロン, リーチ, チー, ポン, カン, Start, Select
    Row 2 returns: H, G, F, E, D, C, B, A
    Row 1 returns: <empty>, <empty>, N, M, L, K, J, I
    Row 0 returns the bitwise OR of rows 1 and 2.
    

Like the standard controller, buttons return 1 when extant and held down, 0 otherwise. 

The buttons are arranged as a 3x8 keyboard matrix similar to the Famicom keyboard. A variety of diodes, resistors, and an NPN BJT pull one of the rows low; the eight columns are then loaded into the same 4021 shift register as used on the standard controller. [1] The Serial Input to the 4021 is tied to ground; all subsequent reads beyond the first eight should return 0V (logic 1 due to the 74368 inside the Famicom). 

Like the [Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard"), there are no diodes to prevent [ghosting](https://en.wikipedia.org/wiki/Rollover_\(key\)#Key_jamming_and_ghosting "wikipedia:Rollover \(key\)"), and pressing three buttons simultaneously could cause the incorrect detection of a 4th button press. 

There are (at least) two hardware revisions of the PCB inside, [[1]](http://archive.is/r170z) but no behavioral differences between the two are known nor any believed to exist. 

## References

  1. ↑ [(?whose?) reverse engineered schematic](http://archive.is/k6jkR)



  * [EveryNES§Jissen Mahjong controller specs](http://problemkaputt.de/everynes.htm#controllerskeypads)
  * [Koi-Koi's pictures on the forum](https://forums.nesdev.org/viewtopic.php?p=245593#p245593)



Categories: [Controllers](Category_Controllers.xhtml)
