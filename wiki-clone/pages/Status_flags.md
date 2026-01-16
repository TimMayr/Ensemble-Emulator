# Status flags

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Status_flags) | View [other pages](Special_AllPages.xhtml#Status_flags)

The **flags** register, also called **processor status** or just **P** , is one of the six architectural registers on the 6502 family CPU. It is composed of six one-bit registers. Instructions modify one or more bits and leave others unchanged. 

## Contents

  * 1 Flags
    * 1.1 C: Carry
    * 1.2 Z: Zero
    * 1.3 I: Interrupt Disable
    * 1.4 D: Decimal
    * 1.5 V: Overflow
    * 1.6 N: Negative
    * 1.7 The B flag
  * 2 External links
    * 2.1 References



## Flags

Instructions that save or restore the flags map them to bits in the architectural 'P' register as follows: 
    
    
    7  bit  0
    ---- ----
    NV1B DIZC
    |||| ||||
    |||| |||+- Carry
    |||| ||+-- Zero
    |||| |+--- Interrupt Disable
    |||| +---- Decimal
    |||+------ (No CPU effect; see: the B flag)
    ||+------- (No CPU effect; always pushed as 1)
    |+-------- Overflow
    +--------- Negative
    

  * The PHP (Push Processor Status) and PLP (Pull Processor Status) instructions can be used to retrieve or set this register directly via the stack.
  * [Interrupts](CPU_interrupts.xhtml "Interrupts") (NMI and IRQ/BRK) implicitly push the status register to the stack.
  * Interrupts returning with RTI will implicitly pull the saved status register from the stack.
  * The two bits with no CPU effect are ignored when pulling flags from the stack; there are no corresponding registers for them in the CPU.
  * When P is displayed as a single 8-bit register by debuggers, there is no convention for what values to use for bits 5 and 4 and their values should not be considered meaningful.



### C: Carry

* * *

  * After ADC, this is the carry result of the addition.
  * After SBC or CMP, both of which do subtraction, this flag will be set if no borrow was the result, or alternatively a "greater than or equal" result.
  * After a shift instruction (ASL, LSR, ROL, ROR), this contains the bit that was shifted out.
  * Increment and decrement instructions do not affect the carry flag.
  * Can be set or cleared directly with SEC or CLC.



### Z: Zero

* * *

  * After most instructions that have a value result, this flag will either be set or cleared based on whether or not that value is equal to zero.



### I: Interrupt Disable

* * *

  * When set, IRQ [interrupts](CPU_interrupts.xhtml "Interrupts") are inhibited. NMI, BRK, and reset are not affected.
  * Can be set or cleared directly with SEI or CLI.
  * Automatically set by the CPU after pushing flags to the stack when any interrupt is triggered (NMI, IRQ/BRK, or reset). Restored to its previous state from the stack when leaving an interrupt handler with RTI.
  * If an IRQ is pending when this flag is cleared (i.e. the [/IRQ](IRQ.xhtml "IRQ") line is low), an interrupt will be triggered immediately. However, the effect of toggling this flag is delayed 1 instruction when caused by SEI, CLI, or PLP.



### D: Decimal

* * *

  * On the NES, decimal mode is disabled and so this flag has no effect. However, it still exists and can be observed and modified, as normal.
  * On the original 6502, this flag causes some arithmetic instructions to use [binary-coded decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal "wikipedia:Binary-coded decimal") representation to make base 10 calculations easier.
  * Can be set or cleared directly with SED or CLD.



### V: Overflow

* * *

  * ADC and SBC will set this flag if the signed result would be invalid[1], necessary for making signed comparisons[2].
  * BIT will load bit 6 of the addressed value directly into the V flag.
  * Can be cleared directly with CLV. There is no corresponding set instruction, and the NES CPU does not expose the 6502's Set Overflow (SO) pin.



### N: Negative

* * *

  * After most instructions that have a value result, this flag will contain bit 7 of that result.
  * BIT will load bit 7 of the addressed value directly into the N flag.



### The B flag

* * *

While there are only six flags in the processor status register within the CPU, the value pushed to the stack contains additional state in bit 4 called the B flag that can be useful to software. The value of B depends on what caused the flags to be pushed. Note that this flag does not represent a register that can hold a value, but rather a transient signal in the CPU controlling whether it was processing an interrupt when the flags were pushed. B is 0 when pushed by interrupts ([NMI](NMI.xhtml "NMI") and [IRQ](IRQ.xhtml "IRQ")) and 1 when pushed by instructions (BRK and PHP). 

Cause | B flag   
---|---  
[NMI](NMI.xhtml "NMI") | 0   
[IRQ](IRQ.xhtml "IRQ") | 0   
BRK | 1   
PHP | 1   
  
Because IRQ and BRK use the same IRQ vector, testing the state of the B flag pushed by the interrupt to the stack is the only way for an IRQ handler to distinguish between them. The B flag also allows software to identify whether [interrupt hijacking](CPU_interrupts.xhtml#Interrupt_hijacking "CPU interrupts") has occurred, where an NMI overrides the BRK instruction, but the B flag is still set by the BRK. However, testing this bit from the stack is fairly slow, which is one reason why BRK wasn't used as a syscall mechanism. Instead, it was more often used to trigger a patching mechanism that hung off the IRQ vector: a single byte in programmable ROM would be forced to 0, and the IRQ handler would pick something to do instead based on the program counter. 

Some debugging tools, such as [Visual6502](Visual6502wiki_JssimUserHelp.xhtml "Visual6502wiki/JssimUserHelp"), display the B flag as bit 4 of P as a matter of convenience. The user can see it turn off at the start of an interrupt and back on after the CPU reads the vector. 

## External links

  * [koitsu's explanation](http://forums.nesdev.org/viewtopic.php?p=64224#p64224)



### References

  1. ↑ [Article: The Overflow (V) Flag Explained](http://www.6502.org/tutorials/vflag.html)
  2. ↑ [Article: Beyond 8-bit Unsigned Comparisons](http://www.6502.org/tutorials/compare_beyond.html#5), Signed Comparisons


