# Pointer table

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Pointer_table) | View [other pages](Special_AllPages.xhtml#Pointer_table)

## Contents

  * 1 Preface
  * 2 The problem
  * 3 Organization of the data file
  * 4 Using the pointer table
  * 5 Code pointer tables



## Preface

A pointer table is any area of memory that is filled with the addresses of other specific areas of memory. When we store the address of an area of memory this is referred to as a "pointer", because it "points to" that area of memory. This article demonstrates how a pointer table can be used to access variable-length data. 

## The problem

When we have multiple pieces of variable-length data, such as complex sprite meta-tile definitions, we need to be able to access those pieces of data using some index number. This allows us to uniquely identify each meta-tile compactly. This also allows us to go to the previous or next meta-tile without having to know anything about the underlying data. 

## Organization of the data file

Typically the pointer table and the data it points to is defined together within an assembly file. This allows us to easily change the data and re-arrange the pointer table as needed. 

_Tip: If you prefer to keep this data in a external file format such as XML and process it with a custom tool, have the tool generate the assembly file described below rather than a binary format. This is often much easier to debug and address._

In our sprite meta-tile example we might have several sprite frame definitions at the bottom of the file. These definitions will use the assembler's BYTE directive (typically .db or .byte) to declare the data. Each frame's data will be preceded by a unique label that we will reference from within the pointer table. 

The pointer table usually comes at the top of the file. The pointer table will use the assembler's BYTE directive again to store the high and low bytes of our frame labels in two different tables. 

Finally the length of the data must be stored so we know how much data to copy. We use the assembler's ability to perform math on labels for this. Not all assemblers support this however. 
    
    
    ; Pointer table for our frames
    frame_pointers_lo:
        .byte <_frame01, <_frame02, <_frame03, <_frame04
    frame_pointers_hi:
        .byte >_frame01, >_frame02, >_frame03, >_frame04
    
    ; Data lengths
    frame_data_lengths:
        .byte _frame02 - _frame01, _frame03 - _frame02
        .byte _frame04 - _frame03, _frame_end - _frame04
    
    ; Frame Data
    _frame01:
        .byte "Some random data"
    _frame02:
        .byte "related to the frames"
    _frame03:
        .byte "that is of variable"
    _frame04:
        .byte "length", $00
    _frame_end:
    

## Using the pointer table

To use the pointer table to access our data we will need to use the frame index as an offset into the pointer table. We then load this pointer into a zero-page variable so we can use it to load data. 

Example: 
    
    
    ; Routine to load the sprite frame definition for a given frame.
    ; Y = The frame to load data for
    .proc load_frame_data
        ; Local variables
        .segment "zp"
            frame_pointer: .word 0 ; Our zero-page pointer for the frame data.
        .segment "ram"
            frame_length: .byte 0 ; The number of bytes in our frame data
    
        ; Routine Start
        .segment "prg"
    
        ; Frame pointer low byte
        lda frame_pointers_lo,y
        sta frame_pointer
    
        ; Frame pointer high byte
        lda frame_pointers_hi,y
        sta frame_pointer
    
        ; Iterate over each byte
        lda frame_data_lengths,y
        sta frame_length
        ldy #0
    data_loop:
        lda (frame_pointer),y
        ; Do something with the frame byte in A, like transform it into OAM data
        iny
        cpy frame_length
        bne data_loop
    
        rts
    .endproc
    

## Code pointer tables

Code is another form of data, and subroutines can be accessed through pointer tables as well. See [Jump table](Jump_table.xhtml "Jump table"). 
