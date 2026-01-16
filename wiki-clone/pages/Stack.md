# Stack

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Stack) | View [other pages](Special_AllPages.xhtml#Stack)

In computer programming, a **stack** is any of several data structures used to store values and retrieve them in the reverse order. CPUs use stacks to track which subroutines call which other subroutines, and programs use them to pass arguments to subroutines and save values for later use. 

All stacks support two operations: **push** to add a value, and **pull** (also called **pop**) to retrieve and remove a value. Stacks may also support **peek** , or retrieve a value _n_ positions from the top without removing it. 

Stacks may be implemented on top of an array or linked list. Some elements are used and others unused; an array index called the "stack pointer" sets the boundary between used and unused elements. The 6502 has hardware support for a stack implemented using a 256-byte array whose location is hardcoded at page $01 ($0100-$01FF), using the S register for a stack pointer. 

There are two ways to implement a stack on top of an array. A _descending_ stack (or one that _grows downward_) starts the stack pointer at the end of the array, decreases it on a push, and increases it on a pull. An _ascending_ stack (or one that _grows upward_) does the opposite. The 6502, Z80, 65816, MIPS, ARM/Thumb, and PowerPC all use a descending stack. 

Each of these allows two ways to interpret the stack pointer. In a _full_ stack, the stack pointer points to the topmost value. It is moved before pushing and after pulling. In an _empty_ stack, the stack pointer points to the element where the next value will be stored. It is moved after pushing and before pulling. ARM traditionally uses a full stack, but 6502 and 65816 use an empty stack. For example, on a 6502, if S = $FD, and the program pushes something, it'll be written to $01FD and then S becomes $FC to show that $01FC is available to store the next value. It is common practice on a 6502 to initialize the stack pointer to $FF [at reset time](Init_code.xhtml "Init code"). 

Imagine stacking a bunch of dinner plates on top of one another; you can't take a plate out from the middle of the stack, you have to take one off the very top each time. 

## Contents

  * 1 Pushing values
  * 2 Pulling values
  * 3 Overflow and underflow
  * 4 Multitasking
  * 5 Pop slide
  * 6 External links



## Pushing values

Let's look at some example code: 
    
    
    _init:
      ldx #$ff	; Set the stack pointer to $FF
      txs		; (e.g. $01FF)
    
    _pushstack:
      lda #$e0	; Push value $e0 on to the stack.
      pha		; $01FF now contains $e0, and S is now $FE.
    
      ldy #$bb	; Push value $bb on to the stack.
      tya
      pha		; $01FE now contains $bb, and S is now $FD.
    
      txa
      pha		; Push value $ff (from the _init routine) on to the stack.
    		; $01FD now contains $ff, and S is now $FC.
    

At this point in our program, we have pushed 3 values on to the stack: $e0, $bb, and $ff. Because the stack grows downward, these will appear as FF BB E0 in a memory viewer. Since $ff was the last thing we pushed onto the stack, it will be the first thing we pull off the stack. We can't pull the $bb value until $ff has been pulled off, and so on -- hence the term "stack". 

## Pulling values

Begin with the values pushed in the previous section, with S = $FC and $01FD-$01FF = FF BB E0. 
    
    
    _pullstack:
      pla		; Pull the value $ff off the stack, and put it into the X register.
      tax		; S now becomes $FD.
    
      pla		; Pull the next value ($bb) off the stack, and put it into the Y register.
      tay		; S now becomes $FE.
    
      pla		; Pull $e0 off the stack, and put it into the accumulator.
    		; S now becomes $FF -- which is where we started!
    

Pulling may be called "popping" by people who come from an 8080, Z80, or x86 background, where the instruction is called _pop_. 

## Overflow and underflow

A **stack overflow** occurs when a program attempts to push data to a full stack, or push more data than the stack can hold. A **stack underflow** occurs when a program attempts to pull data off an empty stack. Usually this implies a PHA vs. PLA mismatch of some sort. Another cause is misuse of a JMP to exit a subroutine without returning (and without [jumping to another subroutine that will return](https://en.wikipedia.org/wiki/tail_call "wikipedia:tail call")). Here, "full" and "empty" are defined in terms of whatever area a program uses for the stack. 

  * On the 6502, if a program is using the entire page ($0100-$01FF) for the stack, pulling when S = $FF causes an underflow, while an overflow occurs when a 257th byte is pushed to the stack (when S is also $FF, having wrapped around after reaching $00). This overflow is difficult to diagnose, as a problem won't appear until later after many pull operations and the original values at the bottom of the stack are needed again. In a debugger you may wish to breakpoint S = $00 to warn both that the stack is almost full, or that an underflow has just occurred.
  * Many NES programs use a stack far smaller than 256 bytes, leaving some of that page available for other uses. For example, a program might build a buffer of things to be copied to VRAM in $0100-$019F, leaving $01A0-$01FF for the stack. In this case, the stack pointer is initialized to $FF and overflows when more than 96 bytes are pushed. Here S <= $9E would indicate overflow, though the problem isn't triggered until access to the buffer overwrites $019F, corrupting the out-of-range stack.
  * Some programs just leave S uninitialized, which puts the base of the stack wherever it is when the NES is reset, and let it wrap within the stack page. This overflows when more than 256 bytes have been pushed, or underflows when more data is pulled than pushed. Not initializing the stack pointer is poor practice, however, as it creates an inconsistent program state on reset, and makes it difficult to use a debugger to inspect the stack.



## Multitasking

Most operating systems for 32-bit and larger machines support [multitasking](https://en.wikipedia.org/wiki/Computer_multitasking "wikipedia:Computer multitasking"), where each process or thread gets its own stack. When switching from one thread to another, the kernel saves the thread's registers (including its stack pointer) and loads that of the other thread. Implementing this on a 6502 is difficult because all tasks have to share the $0100-$01FF page. It becomes somewhat easier on a 65816, whose 16-bit stack pointer allows putting the stack anywhere in $000000-$00FFFF, though some systems reserve much of bank $00 for I/O and ROM for compatibility with 6502 software. 

## Pop slide

Most NES programs do not use the entire $0100-$01FF page as a stack for subroutine return addresses. Many use only a small portion, freeing up the rest for storing other data. 

Many NES programs write data to a buffer to be copied into video memory during vertical blanking. In an unrolled loop that accesses data linearly, the PLA instruction has two advantages. First, it's two bytes shorter than an absolute load, allowing the loop to be unrolled more in the same ROM space. Second, its pre-increment behavior frees the program from having to adjust the X register after a set of copies. So a program such as _Battletoads_ may write large amounts of data to the stack and use a "pop slide" to copy the data into video memory. It starts with code like this, with the data starting one byte after the top of stack: 
    
    
      pla
      sta $2007
      pla
      sta $2007
      pla
      sta $2007
      ; omitted
      pla
      sta $2007
      jmp finish
    

The program calculates where to jump based on the length of the data to be copied, much as in [Duff's device](https://en.wikipedia.org/wiki/Duff%27s_device "wikipedia:Duff's device"). 

When using a pop slide or anything else that uses multiple stacks, it's safest to leave a few bytes before the buffer unused in case something [interrupts](CPU_interrupts.xhtml "CPU interrupts") your process. This way, the return address and saved flags won't overwrite something important. 

The name "pop slide" was chosen, despite the 6502 "pull" naming convention, as a rhyme for "[NOP slide](https://en.wikipedia.org/wiki/NOP_slide "wikipedia:NOP slide")". 

## External links

  * [Wikipedia:Stack (abstract data type)](https://en.wikipedia.org/wiki/Stack_\(abstract_data_type\) "wikipedia:Stack \(abstract data type\)")


