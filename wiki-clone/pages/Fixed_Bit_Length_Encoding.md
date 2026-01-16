# Fixed Bit Length Encoding

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Fixed_Bit_Length_Encoding) | View [other pages](Special_AllPages.xhtml#Fixed_Bit_Length_Encoding)

A byte is 8 bits and can have a value from 0-255. But let's pretend your data only uses the values 0-7. In this case, you don't need a whole 8 bits to represent each data item. You only need 3 bits. Fixed Bit Length Encoding will compress your data by packing it into the minimum number of bits needed to represent all possible values for your data. Using our 3-bit example, let's say we have this sequence of data: 
    
    
    7 1 2 4 7 7 7 1 1 1 2 3 4.
    

In binary, that's: 
    
    
    00000111, 00000001, 00000010, 00000100, 00000111, 00000111, 00000111,
    00000001, 00000001, 00000001, 00000010, 00000011, 00000100
    

Those left 5 bits are always zero, so we can chop them off. Let's compress this data using Fixed Bit Length Encoding (3-bit). 00000111 will become 111, 00000001 will become 001, etc. to give us this: 
    
    
    11100101, 01001111, 11111001, 00100101, 0011100x
    

That's some nice savings! 

## Contents

  * 1 4-bit Encoding
  * 2 5-bit Encoding Example
    * 2.1 Code Example
  * 3 See also



## 4-bit Encoding

The most common type of Fixed Bit Length Encoding is 4-bit or "nibble" encoding. 4-bit encoding is really easy to work with. Each byte will have exactly two data items: the left nibble is one data item, the right nibble is another. Here is one way to pull the two data items out of a byte that is 4-bit encoded: 
    
    
        ;left nibble
        lda data, y                ;read a byte from our data stream
        lsr
        lsr
        lsr
        lsr                        ; right shift four times to get the left nibble
        jsr do_stuff                
    
        ;right nibble
        lda data, y                ;read a byte from our data stream
        and #$0F                   ; ANDing with #$0F zeros out the left side, 
        jsr do_stuff
    

## 5-bit Encoding Example

4-bit encoding works out nice because two data items fit cleanly into one byte. But other Fixed Bit Length Encodings will have data items bleed into more than one byte. Let's look at an example. 

Dragon Warrior 2 uses Fixed Bit Length Encoding (5-bit) to compress its text (modified and coupled with a dictionary, but nevermind). Using 5 bits allows the data to have values in the range 0-31 ($00-$1F). Let's say we have this sequence of data: 
    
    
    $02, $17, $16, $1F.
    

In binary, that's: 
    
    
    00000010, 00010111, 00010110, 00011111
    

Let's compress this using 5-bit encoding (ie, chop off the left 3 bits and smoosh them together): 
    
    
    00010101 11101101 1111xxxx
    

Notice that some of our data items bleed across two bytes: 
    
    
    [00010][101   11][10110][1   1111]xxxx
    

This makes decompressing the data more complicated. To decompress we will need to: 

  * read 2 bytes from the data stream at a time instead of one
  * keep track of the bit position within the first byte
  * have a different bit-extraction routine for each possible bit position



  


### Code Example

Let's look at a code example that will decompress data encoded with Fixed Bit Length Encoding (5-bit): 
    
    
    ; if we have the following values we want to compress:
    ;    14, 08, 07, 00, 1C, 06, 1E, 1F
    ;    01, 0F, 0C, 1F, 1F, 1F, 00, 13
    ;
    ; this is how they look in binary: 
    ;    00010100, 00001000, 00000111, 00000000, 00011100, 00000110, 00011110, 00011111
    ;    00000001, 00001111, 00001100, 00011111, 00011111, 00011111, 00000000, 00010011
    ;
    ;below are these values compressed in Fixed Bit Length Encoding (5bit):
      
    compressed_data:
        .byte %10100010, %00001110, %00001110, %00011011, %11011111
        .byte %00001011, %11011001, %11111111, %11111100, %00010011
    
     
    ;;;;;;;;;;;;;;;;;;;;;;;
    ;
    ;  Reads two bytes from the data stream.
    ;  Called everytime bit_position crosses from byte1 to byte2
    ;       When that happens, the old byte2 becomes the new byte1
    ;       and a new byte2 is read from the data stream.
    ;
    get_next_2_bytes:
        lda compressed_data, y
        sta byte1
        iny
        lda compressed_data, y
        sta byte2
        rts
    
    ;;;;;;;;;;;;;;;;;;;;;;;;;
    ;
    ;  This routine is called after every 5-bit value is extracted from the data.
    ;  It finds the starting point for the next 5-bit value
    ;
    update_bit_position:
        lda bit_position
        clc
        adc #$05            ;next bit position is 5 bits later
        and #$07            ;keep the value between 0 and 7
        sta bit_position
        cmp #$05            ;in the case of updating 0->5, 1->6, or 2->7,
                            ;we are still in the first byte.
                            ;We don't need to change anything
        bcs @skip
        jsr get_next_2_bytes ;bit position updates of 3->0, 4->1, 5->2, 6->3, or 7->4
                             ;indicate that we have jumped from byte1 to byte2, 
                             ;so we need to make byte2 the new byte1 
                             ;and grab the next byte from the data stream (the new byte2)
    @skip:
        rts
    
    ;;;;;;;;;;;;;;;;;;;;;;;
    ;
    ;  depending on our bit position, we will enter this subroutine from a different spot. 
    ;  we will use the [[RTS Trick]] to enter this subroutine (avoids a ridiculous 8-way branch)
    ;   
    ;  temp1 is a copy of byte1,
    ;  temp2 is a copy of byte2
    ;  register A holds the value of byte1
    ;
    bitposition0:        ;[xxxxx]xxx xxxxxxxx   shift right 3 times to get our 5 bits
        lsr
    bitposition1:        ;x[xxxxx]xx xxxxxxxx   shift right 2 times, AND with #$1F
        lsr
    bitposition2:        ;xx[xxxxx]x xxxxxxxx   shift right once, AND with #$1F
        lsr
    bitposition3:        ;xxx[xxxxx] xxxxxxxx   AND with #$1F
        jmp done
    bitposition7:        ;xxxxxxx[x xxxx]xxxx   roll left 4 times, AND with #$1F
        asl temp2
        rol temp1
    bitposition6:        ;xxxxxx[xx xxx]xxxxx   roll left 3 times, AND with #$1F
        asl temp2
        rol temp1
    bitposition5:        ;xxxxx[xxx xx]xxxxxx   roll left 2 times, AND with #$1F
        asl temp2
        rol temp1
    bitposition4:        ;xxxx[xxxx x]xxxxxxx    roll left once, AND with #$1F
        asl temp2
        rol temp1
        lda temp1
    done:
        and #$1F
        rts
    
    bit_pos_table:
        .word bitposition0-1    ;subtract 1 from the address because we are doing the [[RTS Trick]]    
        .word bitposition1-1    ;(ie push the address-1 onto the stack and rts to it)
        .word bitposition2-1
        .word bitposition3-1
        .word bitposition4-1
        .word bitposition5-1
        .word bitposition6-1
        .word bitposition7-1
    
    ;;;;;;;;;;;;;;;;;;;;;;;;;;;;
    ;
    ;  this subroutine take an address from bit_pos_table, pushes it on the stack, and rts to it
    ;  this trick saves us from having a long 8-way conditional branch
    ;  (see [[RTS Trick]] for more details)
    ;
    extract_5bits:
        lda bit_position
        asl
        tax
        lda bit_pos_table+1, x
        pha
        lda bit_pos_table, x
        pha
    
        lda byte2        ;make temporary copies of byte1 and byte2
        sta temp2
        lda byte1
        sta temp1        ;byte1 is in A when we start extracting
       
        rts                  ;return to the address we just pushed
    
    main:                   ;use this as your reset code to try it out
        lda #$00
        sta bit_position
        sta ram_index       ;init variables
       
        tay
        jsr get_next_2_bytes ;grab the first 2 bytes from the data stream
    
    @loop:   
        jsr extract_5bits    ;pull out the next 5-bit value
        ldx ram_index
        sta $200, x          ;output the result to RAM
        inx
        stx ram_index
    
        jsr update_bit_position
     
        cpx #$10             ; loop 16 times (because we are extracting 16 bytes.)
        bne @loop
    forever:
        jmp forever
    

## See also

  * [Text compression](Text_compression.xhtml "Text compression")


