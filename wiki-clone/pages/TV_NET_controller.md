# TV-NET controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/TV-NET_controller) | View [other pages](Special_AllPages.xhtml#TV_NET_controller)

The TV-NET controller is an expansion port device intended for use with TV-NET MC-1200 modem software. It comes in at least 4 variations that differ only in how the buttons are labeled. It has 21 buttons and 1 switch, all of which function independently. The P/T switch on the back signals to software whether to dial using pulse or Touch-Tone. 

## Contents

  * 1 Input ($4016 write)
  * 2 Output ($4016 read)
  * 3 Protocol
    * 3.1 TV-NET
    * 3.2 Piste
    * 3.3 Nikko no Home Trade One
    * 3.4 Daiwa no My Trade, Universal no My Trade
  * 4 Sources



## Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxS
            |
            +- Controller shift register strobe
    

This matches the normal strobe behavior used by the [standard controller](Standard_controller.xhtml#Input_.28.244016_write.29 "Standard controller"). 

## Output ($4016 read)
    
    
    7  bit  0
    ---- ----
    xxxx xxEx
           |
           +- Controller status bit
    

## Protocol

Button state is returned in a 24-bit report across 24 reads. The different variations are all functionally identical, but use different naming for several buttons. 

### TV-NET
    
    
     0 - P/T switch (1 if T)
     1 - 終了 (End)
     2 - F3
     3 - (Always 1)
     4 - F1
     5 - F2
     6 - F4
     7 - F5
     8 - 1
     9 - 4
    10 - 7
    11 - (Always 1)
    12 - 2
    13 - 3
    14 - 5
    15 - 6
    16 - *
    17 - Left
    18 - 実行 (Run)
    19 - Right
    20 - 8
    21 - 9
    22 - 0
    23 - .
    
    24+ - (Always 1)
    

### Piste
    
    
     0 - P/T switch (1 if T)
     1 - 終了 (End)
     2 - Memory
     3 - (Always 1)
     4 - Menu
     5 - 投票 (Vote)
     6 - Submenu
     7 - Clear
     8 - 1
     9 - 4
    10 - 7
    11 - (Always 1)
    12 - 2
    13 - 3
    14 - 5
    15 - 6
    16 - Up
    17 - Left
    18 - 実行 (Run)
    19 - Right
    20 - 8
    21 - 9
    22 - 0
    23 - Down
    
    24+ - (Always 1)
    

### Nikko no Home Trade One
    
    
     0 - P/T switch (1 if T)
     1 - 終了 (End)
     2 - 口座入力 (Account entry)
     3 - (Always 1)
     4 - Menu
     5 - Submenu
     6 - #
     7 - 項目消去 (Item deletion)
     8 - 1
     9 - 4
    10 - 7
    11 - (Always 1)
    12 - 2
    13 - 3
    14 - 5
    15 - 6
    16 - *
    17 - Left
    18 - 実行 (Run)
    19 - Right
    20 - 8
    21 - 9
    22 - 0
    23 - .
    
    24+ - (Always 1)
    

### Daiwa no My Trade, Universal no My Trade
    
    
     0 - P/T switch (1 if T)
     1 - 終了 (End)
     2 - ᐊ / 前ページ (Previous page)
     3 - (Always 1)
     4 - Menu
     5 - Submenu
     6 - ᐅ / 次ページ (Next page)
     7 - C
     8 - 1
     9 - 4
    10 - 7
    11 - (Always 1)
    12 - 2
    13 - 3
    14 - 5
    15 - 6
    16 - *
    17 - #
    18 - ◎ / 実行 (Run)
    19 - Right
    20 - 8
    21 - 9
    22 - 0
    23 - .
    
    24+ - (Always 1)
    

## Sources

  * [Re: TV-NET MC-1200](https://forums.nesdev.org/viewtopic.php?p=248486#p248486)



Categories: [Controllers](Category_Controllers.xhtml)
