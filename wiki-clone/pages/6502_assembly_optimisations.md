# 6502 assembly optimisations

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/6502_assembly_optimisations) | View [other pages](Special_AllPages.xhtml#6502_assembly_optimisations)

This page is about optimisations that are possible in assembly language, or various things one programmer has to keep in mind to make their code as optimal as possible. 

There is two major kind of optimisations: Optimisation for speed (code executes in fewer cycles) and optimisation for size (the code takes fewer bytes). 

There is also some other kinds of optimisations, such as constant-executing-time optimisation (code execute in a constant number of cycle no matter what it has to do), or RAM usage optimisation (use as few variables as possible). Because those optimisations have more to do with the algorithm than with its implementation in assembly, only speed and size optimisations will be discussed in this article. 

## Contents

  * 1 Optimise both speed and size of the code
    * 1.1 Avoid a jsr + rts chain
    * 1.2 Split word tables in high and low components
    * 1.3 Use Jump tables with RTS instruction instead of JMP indirect instruction
    * 1.4 Inline subroutine called only one time through the whole program
    * 1.5 Easily test 2 upper bits of a variable
    * 1.6 Avoiding the need for CLC/SEC with ADC/SBC
    * 1.7 Test bits in decreasing order
    * 1.8 Test bits in increasing order
    * 1.9 Test bits without destroying the accumulator
    * 1.10 Test for equality preserving carry
    * 1.11 Test whether all specified bits are set, preserving carry
    * 1.12 Use opposite rotate instead of a great number of shifts
    * 1.13 Avoid compare instructions in loops
    * 1.14 Avoid compare instructions for certain constants
  * 2 Optimise speed at the expense of size
    * 2.1 Use identity look-up table instead of temp variable
    * 2.2 Use look-up table to shift left 4 times
  * 3 Optimise code size at the expense of cycles
    * 3.1 Use the stack instead of a temp variable
    * 3.2 Use an "intelligent" argument system
    * 3.3 Using relative branch instruction instead of absolute
    * 3.4 BIT trick
  * 4 See also
  * 5 Notes



## Optimise both speed and size of the code

### Avoid a jsr + rts chain

A [tail call](https://en.wikipedia.org/wiki/tail_call "wikipedia:tail call") occurs when a subroutine finishes by calling another subroutine. This can be optimised into a JMP instruction: 
    
    
    MySubroutine
      lda Foo
      sta Bar
      jsr SomeRandomRoutine
      rts
    

becomes : 
    
    
    MySubroutine
      lda Foo
      sta Bar
      jmp SomeRandomRoutine
    

Savings : 9 cycles, 1 byte 

### Split word tables in high and low components

This optimisation is not human friendly, makes the source code much bigger, but still makes the compiled[1] size smaller and faster: 
    
    
    Example
      lda FooBar
      asl A
      tax
      lda PointerTable,X
      sta Temp
      lda PointerTable+1,X
      sta Temp+1
      ....
    
    PointerTable
      .dw Pointer1, Pointer2, ....
    

Becomes : 
    
    
    Example
      ldx FooBar
      lda PointerTableL,X
      sta Temp
      lda PointerTableH,X
      sta Temp+1
      ....
    
    PointerTableL
      .byt <Pointer1, <Pointer2, ....
    
    PointerTableH
      .byt >Pointer1, >Pointer2, ....
    

Some assemblers may have a way to implement a macro to automatically make the table coded like this (Unofficial MagicKit Assembler is one such program). 

Savings : 2 bytes, 4 cycles 

### Use Jump tables with RTS instruction instead of JMP indirect instruction

The so-called [RTS Trick](RTS_Trick.xhtml "RTS Trick") is a method of implementing jump tables by pushing a subroutine's entry point to the stack. 
    
    
    ; This:
    
      ldx JumpEntry
      lda PointerTableH,X
      sta Temp+1
      lda PointerTableL,X
      sta Temp
      jmp [Temp]
    
    ; becomes this:
    
      ldx JumpEntry
      lda PointerTableH,X
      pha
      lda PointerTableL,X
      pha
      rts
    

Note that PointerTable entries must point to one byte _before_ the intended target when the RTS trick is used, because RTS will add 1 to the offset. 

If `Temp` is outside zero page, this saves 6 bytes and 1 cycle. If `Temp` is within zero page, this saves 4 bytes and costs 1 cycle. 

In either case, it frees up the RAM occupied by `Temp` for other uses so long as the use of the RTS Trick does not occur at peak stack depth. In addition, it's [reentrant](https://en.wikipedia.org/wiki/Reentrancy_\(computing\) "wikipedia:Reentrancy \(computing\)"), which means the NMI and IRQ handlers do not need dedicated 2-byte RAM allocations for their own `Temp` variables. 

Combining this with the tail call optimization squeezes 1 more byte and 9 more cycles: 
    
    
    ; This:
    
      jsr SomeOtherFunction  ; MUST NOT modify JumpEntry
      ldx JumpEntry
      lda PointerTableH,X
      pha
      lda PointerTableL,X
      pha
      rts
    
    ; Becomes this:
    
      ldx JumpEntry
      lda PointerTableH,X
      pha
      lda PointerTableL,X
      pha
      jmp SomeOtherFunction
    

Here, the CPU runs `SomeOtherFunction`, then returns to the function from the jump table, then returns to what called this dispatcher. One example is a `SomeOtherFunction` that mixes player input into the random number generator's entropy pool before calling the routine for a particular game state. 

### Inline subroutine called only one time through the whole program

There is no reason to call a subroutine if it is only called a single time. It would be more optimal to just insert the code where the subroutine is called. However this makes the code less structured and harder to understand. [Inline expansion](https://en.wikipedia.org/wiki/Inline_expansion "wikipedia:Inline expansion") of a subroutine into another subroutine can be done with a macro. One drawback is that if the subroutine is called in a loop, it may require some JMPing to work around the 128-byte limit on branch length. 

How macros are used depends on the assembler so no code examples will be placed here to avoid further confusion. In C, the `static inline` keyword acts as a hint to expand a function as a macro. 

Savings : 4 bytes, 12 cycles. 

### Easily test 2 upper bits of a variable
    
    
        lda FooBar
        asl A         ;C = b7, N = b6
    

Alternative: 
    
    
        bit Foobar    ;N = b7, V = b6, regardless of the value of A.
    

This can be e.g. used to poll the sprite-0-hit flag in $2002. 

### Avoiding the need for CLC/SEC with ADC/SBC

When using ADC #imm, somewhere where it is known carry is already cleared, there's no need to use a CLC instruction. However, that carry is known to be _set_ (for example, the code is located in a branch that is only ever entered with a BCS instruction), it's still possible to avoid using CLC by just doing ADC #(value-1). The `PLOT` subroutine in the Apple II Monitor uses this. 

Similarly for SBC #imm: When it is known that carry is _clear_ , SEC instruction can be avoided by just doing SBC #(value-1) or ADC #<-value. 

### Test bits in decreasing order
    
    
       lda foobar 
       bmi bit7_set 
       cmp #$40  ; we know that bit 7 wasn't set 
       bcs bit6_set 
       cmp #$20 
       bcs bit5_set 
                 ; and so on
    

Or the value of A doesn't need to be preserved : 
    
    
       lda foobar 
       bmi bit7_set 
       asl
       bmi bit6_set 
       asl
       bmi bit5_set 
                 ; and so on
    

This saves one byte per comparison. 

### Test bits in increasing order
    
    
       lda foobar 
       lsr
       bcs bit0_set
       lsr
       bcs bit1_set
       lsr
       bcs bit2_set
                 ; and so on
    

Note: This does not preserve the value of A. 

### Test bits without destroying the accumulator

The AND instruction can be used to test bits, but this destroy the value in the accumulator. The BIT can do this but it has no immediate adressing mode. A way to do it is to look for an opcode that has the bits that needs to be tested, and using bit $xxxx on this opcode. 
    
    
    Example
       lda foobar
       and #$30
       beq bits_clear
       lda foobar
       ....
    
    bits_clear
       lda foobar
       .....
    

becomes : 
    
    
    Example
       lda foobar
       bit _bmi_instruction ;equivalent to and #$30 but preserves A
       beq bits_clear
       ....
    
    bits_clear
       .....
    
    anywhere_in_the_code
        ....
    _bmi_instruction    ;The BMI opcode = $30
        bmi somewhere
    

Savings : 2 cycles, 3 bytes 

### Test for equality preserving carry

To test whether A equals some other value, EOR can be used instead of CMP to avoid overwriting the carry flag. However, unlike CMP, EOR destroys A.[2]

Before: 
    
    
       php        ; store carry
       cmp myVal
       beq equal
       plp        ; restore carry; A != myVal
       ...
    equal:
       plp        ; restore carry; A == myVal
       ...
    

After (unlike the previous snippet, destroys A): 
    
    
       eor myVal
       beq equal
       ...        ; A was not equal to myVal
    equal:
       ...        ; A was equal to myVal
    

If necessary, the original value of A can be restored with another EOR, which is still faster than the first snippet: 
    
    
       eor myVal
       beq equal
       eor myVal  ; A was not equal to myVal; restore A
       ...
    equal:
       eor myVal  ; restore A
       ...
    

Savings: 7 cycles and 3 bytes if A destroyed, 3 cycles and -1 byte if A preserved. 

### Test whether all specified bits are set, preserving carry

To test whether all specified bits are set, EOR & AND can be used instead of AND & CMP to avoid overwriting the carry flag.[3]

Before (testing bits 7, 6, 1 and 0 of A): 
    
    
      php             ; preserve carry
      and #%11000011
      cmp #%11000011
      beq all_set
      plp             ; not all set
      ...
    all_set:
      plp
      ...
    

After: 
    
    
      eor #$ff
      and #%11000011
      beq all_set
      ...             ; not all set
    all_set:
      ...
    

Savings: 7 cycles, 3 bytes. 

### Use opposite rotate instead of a great number of shifts

To retrieve the 3 highest bits of a value in the low positions, it is tempting to do 5 LSRs in a row. However, if it is not needed for the 5 top bits to be cleared, this is more efficient: 
    
    
      lda value   ; got: 76543210 c
      rol         ; got: 6543210c 7
      rol         ; got: 543210c7 6 
      rol         ; got: 43210c76 5
      rol         ; got: 3210c765 4
      ; Only care about these ^^^
    

It works the same for replacing 5 ASLs with 4 RORs. 

3 RORs can replace 6 ASLs : 
    
    
      lda value   ; got: 76453210 c
      ror         ; got: c7654321 0
      ror         ; got: 0c765432 1
      ror         ; got: 10c76543 2
      and #$C0    ; got: 10------
    

### Avoid compare instructions in loops

In a loop, comparing the loop counter using CMP/CPX/CPY #constant can often be avoided by choosing the direction of the loop and the final value carefully, so the branch instruction can operate using only the Z and N flags from INX/DEX/etc. 

Increasing values of X up to 127, starting from 255 or 0-127: 
    
    
    loop:
      ...
      inx
      bpl loop
    

Increasing values of X up to 255: 
    
    
    loop:
      ...
      inx
      bne loop
    

Decreasing values of X down to 1: 
    
    
    loop:
      ...
      dex
      bne loop
    

Decreasing values of X down to 0, starting from 0-128: 
    
    
    loop:
      ...
      dex
      bpl loop
    

Decreasing values of X down to 128, starting from 129-255 or 0: 
    
    
    loop:
      ...
      dex
      bmi loop
    

Arrays can be offset by a constant to get a suitable final value for the loop counter: 
    
    
    ; copy indexes 10-13 from my_array to $2007
    ldx #252              ; 256 - number of bytes
    loop:
      lda my_array-242,x  ; 242 = first X - first index in my_array
      sta $2007
      inx
      bne loop
    

Savings: 2 cycles, 2 bytes (less if applying an offset to an array prevents using zero page addressing). 

### Avoid compare instructions for certain constants

Comparing values to 0 is unnecessary after an instruction which modifies A/X/Y or a memory address. Example: 
    
    
    ; this:
      lda Val
      clc
      adc #$02
      cmp #$00
      beq Zero
      bmi Negative
    
    ; becomes:
      lda Val
      clc
      adc #$02 ; this sets the zero/negative flags
      beq Zero
      bmi Negative
    

Savings: 2 bytes, 2 cycles. 

  
Similarly, CMP/CPX/CPY #constant can be avoided for -1 ($ff), 1, and 2 at the cost of clobbering the register. 

Comparing to -1 ($ff): 
    
    
    ; this:
      lda Val
      cmp #$ff
      beq Equals
      bne NotEquals
    
    ; becomes:
      ldx Val ; Y will also work
      inx
      beq Equals
      bne NotEquals
    

Comparing to 1: 
    
    
    ; this:
      lda Val
      cmp #$01
      beq Equals
      bne NotEquals
    
    ; becomes:
      ldx Val ; Y will also work
      dex
      beq Equals
      bne NotEquals
    

Comparing to 2 (for range checks): 
    
    
    ; this:
      lda Val
      cmp #$02
      bcc LessThan
      bcs GreaterEqualTo
    
    ; becomes:
      lda Val
      lsr a
      beq LessThan
      bne GreaterEqualTo
    

Savings: 1 byte. Using Read-Modify-Write instructions on the variable itself can save an extra byte at the cost of clobbering its value. 

## Optimise speed at the expense of size

Those optimisations will make code faster to execute, but use more ROM. Therefore, it is useful in NMI routines and other things that need to run fast. 

### Use identity look-up table instead of temp variable

_Main article:[Identity table](Identity_table.xhtml "Identity table")_
    
    
    Example
        ldx Foo
        lda Bar
        stx Temp
        clc
        adc Temp    ;A = Foo + Bar
    

becomes : 
    
    
    Example
        ldx Foo
        lda Bar
        clc
        adc Identity,X    ;A = Foo + Bar
    
    Identity
        .byt $00, $01, $02, $03, .....
    

If the program is very large (such as in large games), it is possible that this way eventually saves ROM; also, it might save one byte of RAM in some circumstances. 

Savings : 2 cycles 

### Use look-up table to shift left 4 times

Provided that the high nibble is already cleared, the value can be shifted left by 4 by making a look-up table. 
    
    
    Example:
      lda rownum
      asl A
      asl A
      asl A
      asl A
      rts
    

becomes 
    
    
    Example:
      ldx rownum
      lda times_sixteen,x
      rts
    
    times_sixteen:
      .byt $00, $10, $20, $30, $40, $50, $60, $70
      .byt $80, $90, $A0, $B0, $C0, $D0, $E0, $F0
    

In very large programs, this might save some ROM space. However, it will use up the X register, so it might not be ideal. 

Savings: 4 cycles 

## Optimise code size at the expense of cycles

Those optimisations will produce code that is smaller but takes more cycles to execute. Therefore, it can be used in the parts of the program that do not have to be fast. 

### Use the stack instead of a temp variable
    
    
    Example
       lda Foo
       sta Temp
       lda Bar
       ....
       ....
       lda Temp   ;Restores Foo
       .....
    

becomes: 
    
    
    Example
       lda Foo
       pha
       lda Bar
       ....
       ....
       pla   ;Restores Foo
       .....
    

Savings : 2 bytes. 

### Use an "intelligent" argument system

Each time a routine needs multiple bytes of arguments (>3) it's hard to code it without wasting a lot of bytes. 
    
    
    Example
       lda Argument1
       sta Temp
       lda Argument2
       ldx Argument3
       ldy Argument4
       jsr RoutineWhichNeeds4Args
       .....
    

Becomes something like: 
    
    
    Example
       jsr PassArguments
       .dw RoutineWhichNeeds4Args
       .db Argument1, Argument2, Argument3, Argument4
       .db $00
       ....
    
    PassArguments
       pla 
       tay 
       pla 
       pha                    ; put the high byte back 
       sta pointer+1 
       ldx #$00 
       beq SKIP 
    LOOP 
       sta parameters,x 
       inx 
    SKIP 
       iny                    ; pointing one short first pass here fixes that 
       lda (pointer),y 
       bne LOOP      
       iny 
       lda (pointer),y 
       beq LOOP
    
       dey                    ; fix the return address guess we can't return to a 
                             ;  break        
       tya 
       pha 
       jmp (parameters)
    

Syscalls in Apple ProDOS[4] and FDS BIOS work this way. 

Savings : Complicated to estimate - only saves bytes if the trick is used fairly often across the program, in order to compensate for the size of the PassArguments routine. 

### Using relative branch instruction instead of absolute

If the state of one of the processor flags is already known at this point and the branch target is not too far away, then a condition branch instruction can be used. For example, 
    
    
    lda #1
    jmp target
    

becomes 
    
    
    lda #1
    bne target  ; zero flag is always clear
    

or 
    
    
    lda #1
    bpl target  ; negative flag is always clear
    

Savings : 1 byte. 

### BIT trick

The BIT instruction ($24 and $2C) can be used to "mask out" the following 1- or 2-byte instruction without affecting non-flags CPU registers. Care should be taken, however, to ensure a masked 2-byte instruction does not match the address of a register with read side effects, as this would trigger them. 

This trick can be used to create multiple entry points to a subroutine that takes one argument in a register. For example, 
    
    
    sub_11:
        lda #11
    sub:
        sta $2007
        sta $2007
        rts
    ...
    lda #5
    jsr sub
    lda #7
    jsr sub
    jsr sub_11
    lda #13
    jsr sub
    

becomes 
    
    
    sub_5:
        lda #5
        .byte $2c  ; BIT absolute opcode
    sub_7:
        lda #7     ; turns into the operand of BIT if sub_5 was called
        .byte $2c  ; BIT absolute opcode
    sub_11:
        lda #11    ; turns into the operand of BIT if sub_5 or sub_7 was called
    sub:
        sta $2007
        sta $2007
        rts
    ...
    jsr sub_5
    jsr sub_7
    jsr sub_11
    lda #13
    jsr sub
    

Code size is reduced if `sub_5` and `sub_7` are called more than once. 

[5]

## See also

  * [Synthetic instructions](Synthetic_instructions.xhtml "Synthetic instructions")



## Notes

  1. ↑ Pedants may complain that "compile" is incorrect terminology for "translate a program written in assembly language into object code". But it is the most familiar term meaning "translate a program, no matter the language, into object code", and the same issues apply to code generators within a compiler that targets the 6502 as to programs written in 6502 assembly language.
  2. ↑ [EOR #$FF - 6502 Ponderables and Befuddlements](https://archive.org/details/eor6502), puzzle $00
  3. ↑ foobles on NESDev Discord 2023-08-30 (UTC)
  4. ↑ _ProDOS 8 Technical Reference Manual_
  5. ↑ Used in _Super Mario Bros._ (e.g. `MoveSpritesOffscreen` in doppelganger's disassembly)


