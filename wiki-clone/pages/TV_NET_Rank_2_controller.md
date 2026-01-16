# TV-NET Rank 2 controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/TV-NET_Rank_2_controller) | View [other pages](Special_AllPages.xhtml#TV_NET_Rank_2_controller)

The TV-NET Rank 2 controller is an expansion port device intended for use with the TV-NET Rank 2 MC-4800 modem. It is compatible with software expecting the [TV-NET controller](TV_NET_controller.xhtml "TV-NET controller") and provides 8 additional buttons, though some button labels do not match those of any TV-NET controller variant. It is suspected the extra buttons are used for software built into the modem. The P/T switch on the back signals to software whether to dial using pulse or Touch-Tone. This controller can be used alongside the TV-NET MCP-24 printer, which also connects through the Famicom expansion port, by using a multi-port adapter believed to have been bundled with the printer. 

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
    xxxx xxSx
           |
           +- Controller status bit
    

## Protocol

Button state is returned in a 32-bit report across 32 reads. The first 24 bits match those of the [TV-NET controller](TV_NET_controller.xhtml "TV-NET controller"). 
    
    
     0 - P/T switch (1 if T)
     1 - • / 終了 (End)
     2 - 後退 (Backspace) / F3
     3 - (Always 1)
     4 - F1
     5 - 番組 (Program) / F2
     6 - 印字 (Typing) / F4
     7 - 取消 (Cancel) / F5
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
    18 - # / 実行 (Run)
    19 - Right
    20 - 8
    21 - 9
    22 - 0
    23 - ,
    24 - 入力 (Input)
    25 - Up
    26 - Down
    27 - 文字 (Character)
    28 - 機能 (Function)
    29 - 切替 (Exchange) / F6
    30 - 再送 (Resend) / F7
    31 - 停再 (Stop again) / F8
    
    32+ - (Always 1)
    

Categories: [Controllers](Category_Controllers.xhtml)
