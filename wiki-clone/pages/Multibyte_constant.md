# Multibyte constant

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Multibyte_constant) | View [other pages](Special_AllPages.xhtml#Multibyte_constant)

This macro allows including long hexadecimal constants on one line, like an arbitrary extension of the .dbyt (2 byte big endian constant). 

## Contents

  * 1 asm6
  * 2 ca65
  * 3 Unofficial MagicKit
  * 4 External links



## asm6
    
    
    ; ASM6 uses HEX, Use a Macro to convert it to use MBYT!
    ; Macro by Hamtaro126, under no licence.
     macro mbyt string
     hex string
     endm
    

## ca65
    
    
    ; mbyt.s
    ; Multibyte constant macro for ca65
    ;
    ; Copyright 2013 Damian Yerrick
    ; Copying and distribution of this file, with or without
    ; modification, are permitted in any medium without royalty provided
    ; the copyright notice and this notice are preserved in any source
    ; code copies.  This file is offered as-is, without any warranty.
    ;
    .macro mbyt_hex2nibs highnib, lownib
    .local highdig, lowdig
      ; "dec0de" the hex nibbles
      .if highnib >= 'A' && highnib <= 'F'
        highdig = highnib - 'A' + 10
      .elseif highnib >= 'a' && highnib <= 'f'
        highdig = highnib - 'a' + 10
      .elseif highnib >= '0' && highnib <= '9'
        highdig = highnib - '0'
      .endif
      .if lownib >= 'A' && lownib <= 'F'
        lowdig = lownib - 'A' + 10
      .elseif lownib >= 'a' && lownib <= 'f'
        lowdig = lownib - 'a' + 10
      .elseif lownib >= '0' && lownib <= '9'
        lowdig = lownib - '0'
      .endif
      .byte highdig * $10 + lowdig
      ;.out .sprintf(".byte %02x", highdig * $10 + lowdig)
    .endmacro
    
    .macro mbyt inbytes
      ; thanks to thefox who recommended .set
      .local pos
      pos .set 0
      .out .sprintf("%s", inbytes)
      .repeat .strlen(inbytes)
        .if pos < .strlen(inbytes)
          nib .set .strat(inbytes, pos)
          ; these characters can be used as separators
          .if (nib = ' ' || nib = ',' || nib = '$' || nib = '_')
            pos .set pos + 1
          .else
            mbyt_hex2nibs nib, {.strat(inbytes, pos + 1)}
            pos .set pos + 2
          .endif
        .endif
      .endrepeat
    .endmacro
    
    ; use it like this:
    ; mbyt "09F91102 9D74E35B D84156C5 635688C0"
    

Movax12 has a [another implementation](http://pastebin.com/jiWdS68E). 

## Unofficial MagicKit
    
    
    ; Implementation of multibyte constants for Unofficial-MagicKit.
    ; By zzo38, Creative Commons Zero
    
    	macro mbyt
    	macset 2,4,\$1>1
    	macgoto mbyt_\2
    	endm
    
    	macro mbyt_0
    	endm
    
    	macro mbyt_1
    	macset 2,2,1
    	macset 2,5,2
    	db $\2
    	mbyt_0 \>1
    	macset 1,1,1
    	mbyt_0 \>1
    	macset 1,1,1
    	macgoto mbyt
    	endm
    
    ; Example:
    ;	mbyt 09F91102 9D74E35B D84156C5 635688C0
    

## External links

  * [Ned Batchelder: Hex words](http://nedbatchelder.com/text/hexwords.html) has test cases
  * [Alternate implementation by thefox](http://pastebin.com/0xkpKgCh)
  * [Multibyte constant](https://gbdev.gg8.se/wiki/articles/Multibyte_constant) on GbdevWiki


