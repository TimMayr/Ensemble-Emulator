# Talk:RTS Trick

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ARTS_Trick) | View [other pages](Special_AllPages.xhtml#Talk_RTS_Trick)

## Contents

  * 1 Indirect vs. RTS
  * 2 Self-modifying
  * 3 Fast RTS Trick
  * 4 When to use the RTS Trick



## Indirect vs. RTS

Is there an advantage over using JMP ($0200), where $0200 has been loaded from the same kind of jump-table? that's what I wonder, but I'm not gonna count up the cpu cycles needed for either method right now. --[Memblers](https://www.nesdev.org/w/index.php?title=User:Memblers&action=edit&redlink=1 "User:Memblers \(page does not exist\)") 20:21, 25 Jun 2009 (PDT) 

    Not sure. Seems like a pick'em to me. Here are some things that come to mind: 

  * RTS Trick doesn't require any RAM.
  * I personally think the RTS Trick is more readable. If I see a table of pointers in my (or somebody else's) code and they all have a "-1" after them, I immediately know their purpose and how they are used.
  * PHA, PHA, RTS requires less bytes than STA, STA, JMP (3 vs. 9).


    \--[MetalSlime](User_MetalSlime.xhtml "User:MetalSlime") 00:08, 26 Jun 2009 (PDT) 

    JMP ($0200) isn't re-entrant. The RTS/RTI approach uses the stack therefore can be done by mainline and interrupt code freely. [Blargg](User_Blargg.xhtml "User:Blargg") ([talk](User_talk_Blargg.xhtml "User talk:Blargg")) 02:12, 23 January 2014 (MST)

## Self-modifying

If you use self modifying code and assure that the table has to start at a page border (and store pointers to the routines, without the -1) then you can use a smaller and faster code: 
    
    
    tb_opcode_launcher_smc:
    	; bytes, cycles
    	asl ; 1, 2
    	sta smc+2 ; 3, 4
    smc:
    	jmp (tb_opcode_rts_table) ; 3, 5
    	; total 7 bytes and 11 cycles
    
    
    
    tb_opcode_launcher:
    	; bytes, cycles
    	asl ; 1, 2
    	tax ; 1, 2
    	lda tb_opcode_rts_table+1, x ; 3, 4
    	pha ; 1, 3
    	lda tb_opcode_rts_table, x ; 3, 4
    	pha ; 1, 3
    	rts ; 1, 6
    	; total 11 bytes and 24 cycles
    

\--~~212.8.208.194~~ ([talk](https://www.nesdev.org/w/index.php?title=User_talk:212.8.208.194&action=edit&redlink=1 "User talk:212.8.208.194 \(page does not exist\)")) 

    Assuming that by `sta smc+2` you meant `sta smc+1` because 6502 is little-endian. But if you're doing any sort of nontrivial work in the NMI or IRQ handler, you would need separate 7-byte self-modifying trampolines in RAM for the main, NMI, and possibly IRQ handlers. And with the NES's 2048 byte RAM, 21 bytes might be a lot, though it's still not as bad as it would be on the Atari 2600. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 11:18, 21 May 2013 (MDT) 

    If you have extra PRG RAM in the cartridge, you could use that, too. Also, self-modifying code seems to be especially suitable for disk-based programs (although in many ways other than just this!). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 16:45, 8 December 2013 (MST)

## Fast RTS Trick

You can make a bit faster RTS trick also if all of the jump targets are in one 256-byte page. Alternatively, you can align each jump target to a separate page and then not even use a lookup table. In addition, even when using the full addresses, I prefer to store the high bytes in one table and the low bytes in another table, instead of doing it together. This results in a smaller and faster code. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 16:45, 8 December 2013 (MST) 

## When to use the RTS Trick

This section is confusing to me, because the "RTS trick" is functionally equivalent to jump tables or other implementations of function pointers. It might as well be called "when to use function pointers"? It jumps right in talking about finite state machines and textboxes??? I don't think any of this is really relevant to the article. I've moved that text here for the time being. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:39, 30 April 2017 (MDT) 

To clarify, I think a tutorial on how to use jump tables, or a bytecode scripting format, or FSM, etc. would be good, but if it belongs anywhere it would be [jump table](Jump_table.xhtml "Jump table") maybe, but I'm not even sure it's approrpiate there. This really just confuses the point of things-- RTS trick is just an optimization trick for function pointers, it is not the concept of function pointers itself. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:43, 30 April 2017 (MDT) 

  

    
    
    --- removed text below this line to end of section ---

The RTS Trick is especially useful for anything that can be clearly expressed as a [finite-state machine](https://en.wikipedia.org/wiki/finite-state_machine "wikipedia:finite-state machine"). 

Sometimes a module or data format will have a large set of subroutines that will be invoked based on the value of a variable or a data read. One example would be a textbox module for an RPG game. In addition to the mundane displaying of letters you will likely need subroutines to perform various textbox-related tasks. Some examples might be: 

  * clear the textbox
  * freeze the textbox (to wait for a button press, for example)
  * insert a pause
  * pluralize words (that is, add or omit an "s" based on a number in a case like: 1 *piece of gold/43 pieces of gold)
  * play a sound
  * close the textbox (end the dialogue)
  * update a variable (set flags, add items to inventory, etc)
  * pop up a yes-no box and prompt the player for a response
  * etc..



Another example would be a music engine. Your song data will have values that represent note values and note lengths, but your data format will also have opcodes that invoke subroutines to perform different tasks. For example: 

  * set a _segno_ , or loop point (possibly more than one, so you can nest loops)
  * _dal segno_ , or loop
  * alter the pitch of a playing note
  * change tempo
  * silence a channel or otherwise set volume
  * set volume envelope
  * set sweep envelope
  * set square duty
  * apply vibrato
  * etc.



The more opcode-invoked subroutines you have at your disposal, the more unwieldy branching code will become: 
    
    
        lda (data_pointer), y   ;read a byte from the text data stream
        bmi textbox_opcodes  ;let's say a value >= #$80 indicates an opcode
                             ;in our imaginary text data format
        ;...do ordinary text stuff
        
    textbox_opcodes:
        cmp #$FF
        bne not_end
        jsr tb_end_dialogue
        jmp end_of_opcodes
    not_end:
        cmp #$FE
        bne not_clear
        jsr tb_clear_dialogue
        jmp end_of_opcodes
    not_clear:
        cmp #$FD
        bne not_pluralize
        jsr tb_pluralize_word
        jmp end_of_opcodes
    not_pluralize:
        cmp #$FC
        bne not_add_inventory
        jsr tb_add_inventory
        jmp end_of_opcodes
    not_add_inventory:
         
        ...
         
        etc...
    

In the above code example, we read a byte from our data stream and then determine whether that byte represents a character of text or a [control character](https://en.wikipedia.org/wiki/control_character "wikipedia:control character"). If it's an opcode, we have to check which opcode. If it FF? no. well is it FE? no. well is it FD? crap. how about FC? The more opcodes you have, the more checks you have to do, and the more painful it will be to rearrange if you decide later to swap the values around. This is where jump tables can help, and the RTS Trick is one efficient way to make a jump table on 6502. 

    I fully agree, none of this has anything to do on the "RTS trick" page. How state machines work and how this applies to NES games is out of topic to this wiki, even moreso on this page - in other words I don't think this wiki is intended as teaching programming.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 03:33, 1 May 2017 (MDT)
