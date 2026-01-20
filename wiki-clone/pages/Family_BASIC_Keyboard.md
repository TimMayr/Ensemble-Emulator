# Family BASIC Keyboard

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Family_BASIC_Keyboard) | View [other pages](Special_AllPages.xhtml#Family_BASIC_Keyboard)

The Family BASIC Keyboard (HVC-007) was a peripheral released with the Family BASIC package in 1984. With the [data recorder](Family_BASIC_Data_Recorder.xhtml "Family BASIC Data Recorder") that could be attached to it, it allowed the [Famicom](Family_Computer.xhtml "Famicom") to have the abilities of the average home computer of around that time (including using cassette tape to load/store data in supported games). It is a generic 72 button keyboard (using common matrix logic) which is connected to the Famicom's [ expansion port](Expansion_port.xhtml#Famicom "Expansion port"). 

## Contents

  * 1 Keyboard map
  * 2 Hardware interface
    * 2.1 Input ($4016 write)
    * 2.2 Output ($4017 read)
  * 3 Usage
  * 4 Matrix
  * 5 Hardware
  * 6 Miscellaneous
  * 7 Keyboard detection in other games
  * 8 See Also
  * 9 References



## Keyboard map

[![](../wiki-images/Famicom_keyboard.jpg)](File_Famicom_keyboard_jpg.xhtml)

[](File_Famicom_keyboard_jpg.xhtml "Enlarge")

Top-down photo of keyboard layout

[![Family keyboard.svg](../wiki-images/Family_keyboard.svg)](File_Family_keyboard_svg.xhtml)

In text format: 
    
    
    F1  F2  F3  F4  F5  F6  F7  F8
     1 2 3 4 5 6 7 8 9 0 - ^ ¥ STOP    CH* INS DEL
    ESC Q W E R T Y U I O P @ [ RETURN     UP
    CTR A S D F G H J K L ; : ] KANA   LEFT RIGHT
    SHIFT Z X C V B N M , . / _ SHIFT     DOWN
           GRPH   SPACE
    
    * CH = CLR HOME
    

If you have proper font support, with full-width spaces (i.e. "=" is above "-"): 
    
    
    　　　Ｆ１　　　　　　Ｆ２　　　　　　Ｆ３　　　　　　Ｆ４　　　　　　Ｆ５　　　　　　Ｆ６　　　　　　Ｆ７　　　　　　Ｆ８
    
    　　　　！　　　＂　　　＃　　　＄　　　％　　　＆　　　＇　　　（　　　）　　　　　　　＝
    　　　１　ァ　２　ィ　３　ゥ　４　ェ　５　ォ　６　　　７　　　８　　　９　　　０　　　－　　　＾　　　￥　　　 STOP
    　　　　ア　　　イ　　　ウ　　　エ　　　オ　　　ナ　　　ニ　　　ヌ　　　ネ　　　ノ　　　ラ　　　リ　　　ル　　　　　　　　ＣＬＲ
    　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　ＩＮＳ　ＤＥＬ
    　ＥＳＣ　Ｑ　　　Ｗ　　　Ｅ　　　Ｒ　　　Ｔ　　　Ｙ　パ　Ｕ　ピ　Ｉ　プ　Ｏ　ペ　Ｐ　ポ　＠　　　［　「　ＲＥＴＵＲＮ　　 HOME
    　　　　　　カ　　　キ　　　ク　　　ケ　　　コ　　　ハ　　　ヒ　　　フ　　　ヘ　　　ホ　　　レ　　　ロ
    
    　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　＋　　　＊　　　　　　　　　　　　　　　　　▲
    　　ＣＴＲ　Ａ　　　Ｓ　　　Ｄ　　　Ｆ　　　Ｇ　　　Ｈ　　　Ｊ　　　Ｋ　　　Ｌ　　　；　　　：　　　］　」　 カナ
    　　　　　　　サ　　　シ　　　ス　　　セ　　　ソ　　　マ　　　ミ　　　ム　　　メ　　　モ　　　ー　　　。
    
    　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　〈　　　〉　　　？　　　␣　　　　　　　　　　　　　◀　　　　　　　▶
    ＳＨＩＦＴ　　Ｚ　　　Ｘ　　　Ｃ　　　Ｖ　　　Ｂ　　　Ｎ　　　Ｍ　　　，　　　．　　　／　　　　　　　　ＳＨＩＦＴ
    　　　　　　　　タ　　　チ　　　ツ　　　テ　　　ト　　　ヤ　　　ユ　　　ヨ　　　ワ　　　ヲ　　　ン
    　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　▼
    
    　　　　　　　　　　　 GRPH ［．．．．．．．．．．．．ＳＰＡＣＥ．．．．．．．．．．．．］
    
    

Mind that the DEL key function is actually that of _Backspace_ we all know and love. 

## Hardware interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xKCR
          |||
          ||+-- Reset the keyboard to the first row.
          |+--- Select column, row is incremented if this bit goes from high to low.
          +---- Enable keyboard matrix (if 0, all voltages inside the keyboard will be 5V, reading back as logical 0 always)
    

Incrementing the row from the (keyless) 10th row will cause it to wrap back to the first row. 

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxK KKKx
       | |||
       +-+++--- Receive key status of currently selected row/column.
    

Any key that is held down, will read back as 0. 

($4016 reads from the data recorder.) 

Similar to the [Family Trainer Mat](Power_Pad.xhtml "Power Pad"), there are parasitic capacitances that the program must wait for to get a valid result. Family BASIC and the [ FDS BIOS](FDS_BIOS.xhtml#Other_BIOS_calls "FDS BIOS") wait at least 12 cycles (16 if load instructions are considered) between resetting the keyboard and reselecting column 0, and approximately 50 cycles after selecting each column before assuming the output is valid. 

## Usage

Family BASIC and the FDS BIOS read the keyboard state with the following procedure: 

  1. Write $05 to $4016 (reset to row 0, column 0), followed by 6 NOPs (12 cycles)
  2. Write $04 to $4016 (select column 0, next row if not just reset), followed by a delay of ~50 cycles
  3. Read column 0 data from $4017
  4. Write $06 to $4016 (select column 1), followed by a delay of ~50 cycles
  5. Read column 1 data from $4017
  6. Repeat steps 2-5 eight more times



Differences between Family BASIC and the FDS BIOS: 

  * The FDS BIOS terminates the routine early if all keys are pressed on column 0 of any row (it determines that the keyboard is disconnected). Family BASIC always reads all rows/columns.
  * The FDS BIOS writes to $4016 with bit 2 clear at the end of the routine (thus disabling the keyboard matrix), but Family BASIC does not.



There are currently no known commercial [FDS](Family_Computer_Disk_System.xhtml "FDS") games which use the BIOS routine for keyboard reading. (TODO: Check for games which potentially read the keyboard, including cart-to-disk ports of games that support the data recorder.) 

## Matrix

| Column 0  | Column 1   
---|---|---  
$4017 bit  | 4 | 3 | 2 | 1 | 4 | 3 | 2 | 1   
Row 0  | ] | [ | RETURN | F8 | STOP | ¥ | RSHIFT | KANA   
Row 1  | ; | : | @ | F7 | ^ | - | / | _   
Row 2  | K | L | O | F6 | 0 | P | , | .   
Row 3  | J | U | I | F5 | 8 | 9 | N | M   
Row 4  | H | G | Y | F4 | 6 | 7 | V | B   
Row 5  | D | R | T | F3 | 4 | 5 | C | F   
Row 6  | A | S | W | F2 | 3 | E | Z | X   
Row 7  | CTR | Q | ESC | F1 | 2 | 1 | GRPH | LSHIFT   
Row 8  | LEFT | RIGHT | UP | CLR HOME | INS | DEL | SPACE | DOWN   
  
## Hardware

The Family BASIC Keyboard is implemented using a CD4017 decade counter (to scan the rows of the keyboard matrix), a CD4019 quad AND-OR gate, and one sixth of a CD4069 hex inverter. The latter two are combined to make a quad 1-of-2 selector, equivalent to a CD4519 or a 74'157. (Another three inverters are used to interface to the Family BASIC Data Recorder) 

## Miscellaneous

  * Unlike the PC keyboard, but similar to the Commodore 64 keyboard, the sixteen keys corresponding to ASCII $2C-$3B all specify the ASCII code point should be XORed with $10 when the SHIFT key is pressed. This can be used to simplify the keyboard decoding logic in your program.
  * There is no backslash key, however, historical reasons have given to using the yen key and symbol for the same meaning.
  * The kana are arranged in (grid) alphabetical order, not in the way that modern Japanese computers are.



## Keyboard detection in other games

_Lode Runner_ allows saving level data to tape by pressing Select during Edit Mode, but will only provide that option if it detects the Family BASIC Keyboard. The detection procedure (CPU $E9B8) selects the tenth row and expects $4017 AND $1E to return $1E, then writes $00 to $4016 to disable the keyboard and expects $4017 AND $1E to return $00. 

## See Also

  * Standalone Family BASIC Keyboard tests: 
    * [Key status report only](http://rdev.php.xdomain.jp/makimura/archive/homebrew/keyboard-input-test)
    * [Key status report + display](http://rdev.php.xdomain.jp/makimura/archive/homebrew/keyboard-input-test2)
    * [FDS keyboard test](https://www.romhacking.net/homebrew/124/) (single key display, uses BIOS routine)



## References

  * Reverse-engineered schematics by [Enri](http://cmpslv3.stars.ne.jp/): 
    * <http://cmpslv3.stars.ne.jp/Famic/Fambas.htm>
    * Also available [here](https://web.archive.org/web/20141124200942/http://atariusa.com/Famicom_Schematics/FC%20Family%20Basic%20Keyboard%20and%20Sound%20Circuit%20Schematic.png) and [here](https://web.archive.org/web/20141124200941/http://atariusa.com/Famicom_Schematics/FC%20Keyboard%20Key%20Schematic.png)
  * Photographs of the keyboard by [Evan-Amos](https://commons.wikimedia.org/wiki/File:Nintendo-Famicom-Family-Basic-Keyboard-wCart.jpg)
  * [Stylized keycaps diagram](http://www.pfu.fujitsu.com/hhkeyboard/kb_collection/images/hvc-007.gif) from [PFU's Happy Hacking Keyboard keyboard history library](http://www.pfu.fujitsu.com/hhkeyboard/kb_collection/)



Categories: [Controllers](Category_Controllers.xhtml)
