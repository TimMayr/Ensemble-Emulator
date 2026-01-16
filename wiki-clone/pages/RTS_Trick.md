# RTS Trick

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/RTS_Trick) | View [other pages](Special_AllPages.xhtml#RTS_Trick)

The RTS Trick is one implementation of [jump tables](Jump_table.xhtml "Jump table"), which are a neat way to avoid long, complicated branching code to subroutine calls. It is easier to implement than a pile of branches, reads well, and saves bytes. In fact, the syscall dispatcher for the Apple IIGS toolbox (JSL $E10000) uses this same approach. 

It is typcially 4 bytes smaller, but 1 cycle slower than using an indirect JMP instruction (see: [jump table](Jump_table.xhtml "Jump table")). Its primary advantage is being able to use the stack, rather than a fixed location in RAM to store the target address. 

## Contents

  * 1 About JSR and RTS
  * 2 Using RTS as a jump table
    * 2.1 Step 1: Return Address Table
    * 2.2 Step 2: Table-friendly Data Format
    * 2.3 Step 3: RTS Trick Launcher subroutine.
    * 2.4 Potential Pitfall
  * 3 Unofficial-MagicKit Macro
  * 4 Games that use the RTS Trick
  * 5 See also



## About JSR and RTS

Before talking about the RTS Trick, we need to make clear how JSR and RTS work. When you call a subroutine using JSR, two things happen: 

  * the address of the last byte of the JSR (that is, the next instruction minus 1) is pushed onto the stack
  * the program counter jumps to the subroutine indicated.



When the program encounters an RTS instruction, this happens: 

  * an address is popped off the stack.
  * the program counter jumps to this address + 1


    
    
    $C0E0: 20 00 80     JSR $8000
    $C0E3: A2 00        LDX #$00
    $C0E5: A0 00        LDY #$00
    
    $8000: A9 0F        LDA #$0F
    $8002: 8D 15 40     STA $4015
    $8005: 60           RTS
    

Let's run through this code. 

  * The Program Counter is at $C0E0.
  * We encounter a JSR instruction. The address of the next instruction - 1 is pushed onto the stack. The next instruction is $C0E3. $C0E3 - 1 is $C0E2. $C0, then $E2 are pushed onto the stack (note the order). Then we jump to $8000.
  * We load #$0F into the accumulator.
  * We store the accumulator in $4015.
  * We encounter an RTS instruction. Two bytes are popped off the stack. In this case, those two values will be E2 and C0. The program counter jumps to this address + 1. $C0E2 + 1 is $C0E3. The Program Counter jumps to $C0E3, which is the instruction after our original JSR.
  * We continue on with our program as normal from $C0E3.



The part we need to take note of for the RTS Trick is the fact that RTS expects the stack to have a destination address MINUS 1. For the rest of this article, let's call this a return address. The two byte return address should be positioned on the stack so that the low byte is popped first, then the high byte. That's all we need to know: Destination address minus 1. Low byte popped first. 

## Using RTS as a jump table

We said above that we want to avoid having a long chain of comparisons and branches, and that the RTS Trick will help us do that. So what is the RTS Trick? 

The RTS Trick consists of a lookup table of return addresses. You read a return address from this table, push it onto the stack, and then RTS. It's really, really simple to understand with an example. Let's use our textbox example above. What do we need? 

  * We need a table of return addresses
  * We need a data format that will allow us to index into this table easily.
  * We need a RTS Trick launcher subroutine (I'll tell you why below. Don't worry, it's super-simple)



### Step 1: Return Address Table

This is really easy to make. Assuming we already have a bunch of subroutines: 
    
    
    tb_end_dialogue:
        ;do stuff
        rts
       
    tb_clear_textbox:
        ;do stuff
        rts
       
    tb_pluralize_word:
        ;check a number and add an "s" if it's > 1
        rts
       
    tb_add_inventory:
        ;do stuff
        rts
       
    tb_set_flag:
        ;do stuff
        rts
      
    tb_insert_dramatic_pause:
        ;do stuff
        rts
       
    tb_play_dramatic_sound:
        ;do stuff
        rts
       
    tb_set_text_color:
        ;do stuff
        rts
    
        ... etc
    

This is how our return address table will look: 
    
    
    tb_opcode_rts_table:
         .word tb_end_dialogue-1
         .word tb_clear_textbox-1
         .word tb_pluralize_word-1
         .word tb_add_inventory-1
         .word tb_set_flag-1
         .word tb_insert_dramatic_pause-1
         .word tb_play_dramatic_sound-1
         .word tb_set_text_color-1
         .word ...etc
    

Finished. That's it. Recall that RTS will expect the address on the stack to be its destination - 1. So we tack a minus one to all of our addresses in the table. 

### Step 2: Table-friendly Data Format

Our text engine reads a byte from our data stream. It checks to see whether that value represents a letter or an opcode. If it is an opcode, we want to turn that value into an index into our return address table. Indexes start at 0 and go up. The data format we have now, counting down from $FF probably won't be quite as convenient as one that counts up. It doesn't really matter from where. It could start at $80, or $C0, or $E0, or it could start from $00 and reach $1F (such as $0A for a new line). For this example, let's keep it simple and choose $80. Here is our new data format: 
    
    
     $00-$7F: letters (a, b, c, d..etc)
     $80-$FF(potentially): opcodes
    
     Opcode list:
     
     $80 = end dialogue
     $81 = clear the textbox
     $82 = pluralize a word
     $83 = add an item to the character's inventory
     $84 = set a flag
     $85 = insert a pause
     $86 = play a sound
     $87 = set text color
     $88 = add to gold total
     $89 = subtract from gold total
     $8A ... etc
    

This Data Format for the opcodes is very table-friendly. It starts at $80 and goes up. When we read a byte from our data stream and determine it to be an opcode, all we have to do is chop off bit7 to turn our value into a table index: 
    
    
     $80 = %10000000
     $81 = %10000001
     $82 = %10000010
     $83 = %10000011
            |
            +----------- if we chop off this bit7, we will get $00, $01, $02, $03, etc.
                         That's easy.  Just AND with $7F (%01111111) 
    

Finished. Step 2 Complete. 

### Step 3: RTS Trick Launcher subroutine.

This is easy. We need a subroutine that will turn our data value into an index, read a return address from the table, push the values and RTS. 
    
    
    tb_opcode_launcher:
        asl             ;we have a table of addresses, which are two bytes each. double that index.
                        ;incidentally, this chops off bit 7 and stashes it in the carry.
        tax
        lda tb_opcode_rts_table+1, x    ;RTS will expect the low byte to be popped first,
                                        ;so we need to push the high byte first
        pha
        lda tb_opcode_rts_table, x      ;push the low byte
        pha
        rts             ;this rts will launch our opcode subroutine.
    

That's it! 

### Potential Pitfall

One point I would like to stress: your launcher must be a subroutine. The opcode subroutines that you are RTS-launching all end in an RTS of their own. Do you know where they will return? Consider the following two examples: 

The Right Way 
    
    
    tb_read_data:
        lda (data_pointer), y
        bmi opcode_stuff    ;if negative (bit7 set), we have an opcode
        ;do normal text stuff here
    
    opcode_stuff:
        jsr tb_opcode_launcher    ;key point!  This JSR will put a return address on the stack!
                                  ;When our opcode finishes execution, it will return to the right place!
        iny
        ;continue
    

The Wrong Way 
    
    
    tb_read_data:
        lda (data_pointer), y
        bmi opcode_stuff
        ;do normal text stuff
    
    opcode_stuff:
        and #$7F
        asl
        tax
        lda tb_opcode_rts_table+1, x
        pha
        lda tb_opcode_rts_table, x
        pha
        rts ;the opcode return address is on the stack, but our starting point isn't.
            ;When the opcode returns, our game will leave this subroutine entirely
        iny ;and will never get here.
    

Let's assume that in both cases, the last INY instruction happens to be located at address space $D333. 

In the first example, we JSR to our launcher. The JSR will put $D332 on the stack and jump to our launcher. Our launcher will read an opcode return address from our table and push it onto the stack. Then our launcher will RTS, which will pull the opcode return address off the stack, add one, and jump there. Our opcode will execute and return. Our opcode's RTS will pull our D332 address off the stack, add one and jump there. Good! We made it back to our INY instruction. 

In the second example, there is no JSR to our launcher. No values are pushed on the stack to mark our place. We read from the table and push an opcode return address onto the stack and RTS to it. Our opcode executes and returns. Our opcode's RTS will pull two bytes off the stack, but we have no idea what those two bytes are. Usually, this will be the return address of whatever called our launcher, but if you have pushed something else onto the stack, it could return anywhere. We are in big trouble. 

So make sure your launcher is wrapped up in a subroutine and called via JSR, or that it is [otherwise the last thing in a subroutine](https://en.wikipedia.org/wiki/Tail_call "wikipedia:Tail call"), and you will be safe. 

## Unofficial-MagicKit Macro

In the [Unofficial-MagicKit](https://forums.nesdev.org/viewtopic.php?p=97318) assembler, you may use a macro like the following: 
    
    
    	macro def_inst
    	macset 2,4,*
    	macset 3,4,?B
    	bank bank(instadl)
    	org instadl+(\1)
    	db low(\2-1)
    	org instadh+(\1)
    	db high(\2-1)
    	bank \3
    	org \2
    	endm
    

Note: This creates tables in a different format than mentioned above. It uses a separate table for the low-byte and high-byte, rather than putting them together. This avoids needing a shift operation (although you may also avoid a shift just by not defining any odd-numbered opcodes). 

Now you may have a table like (they must be in the same 8K bank, and they should be aligned to a 256-byte page): 
    
    
    instadl	ds 256
    instadh	ds 256
    

And then before the subroutines that would be called in this way you can do, for example: 
    
    
    	def_inst 42
    

## Games that use the RTS Trick

  * The Guardian Legend uses the RTS trick in its sound engine for opcodes.
  * [Concentration Room](User_Tepples_Concentration_Room.xhtml "User:Tepples/Concentration Room") uses the RTS trick for three jump tables: one for the menu, one for the state of the board, and one for the state of the AI.
  * Famicom Z-machine has two: one for Z-character decoding (which uses a state machine) and one for instruction decoding.
  * [Attribute Zone](User_Zzo38_Attribute_Zone.xhtml "User:Zzo38/Attribute Zone") uses it for keyboard decoding in the level editor.
  * [Haunted: Halloween '85](User_Tepples_Haunted__Halloween__85.xhtml "User:Tepples/Haunted: Halloween '85") uses it to call enemies' move handlers.
  * [240p test suite](https://github.com/pinobatch/240p-test-mini) has a jump table for main menu activities.
  * Games using [Pently](https://github.com/pinobatch/pently) audio engine version 5 or later use the RTS trick to process pattern opcodes that enable effects.



## See also

The [Fixed Bit Length Encoding](Fixed_Bit_Length_Encoding.xhtml "Fixed Bit Length Encoding") article has an example of the RTS Trick in action. 
