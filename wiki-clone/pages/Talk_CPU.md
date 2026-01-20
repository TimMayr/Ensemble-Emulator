# Talk:CPU

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACPU) | View [other pages](Special_AllPages.xhtml#Talk_CPU)

## 6502.txt twos complement

Various places in the 6502.txt document that refer to negative numbers don't seem to realize that it's 6502 uses twos complement - not just a simple sign bit. This is semantically important, for example in the section about branch instructions it states that they have a range of 127 forward and backward - this is inaccurate, twos complement represents 128 values backwards (which is its advantage over a simple sign bit representation, which loses a bit by representing +0 and -0). 

## Registers article

Creating an article describing the registers as this seems to be a blindspot in the Wiki. [alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 02:09, 20 February 2015 (MST) 

## PAL frequency formula

15625 Hz * 283.75 + 25 Hz = 4433618.75 Hz 

source: <https://de.wikipedia.org/wiki/Phase_Alternating_Line#Wahl_der_PAL-Farbtr.C3.A4gerfrequenz>
