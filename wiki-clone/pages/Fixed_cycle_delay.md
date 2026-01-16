# Fixed cycle delay

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Fixed_cycle_delay) | View [other pages](Special_AllPages.xhtml#Fixed_cycle_delay)

Shortest possible CPU code that creates N cycles of delay, depending on constraints. 

## Contents

  * 1 Code
    * 1.1 Explanations on the requirements
    * 1.2 Instructions, addressing modes, byte counts, cycle counts and notes
    * 1.3 2 cycles
    * 1.4 3 cycles
    * 1.5 4 cycles
    * 1.6 5 cycles
    * 1.7 6 cycles
    * 1.8 7 cycles
    * 1.9 8 cycles
    * 1.10 9 cycles
    * 1.11 10 cycles
    * 1.12 11 cycles
    * 1.13 12 cycles
    * 1.14 13 cycles
    * 1.15 14 cycles
    * 1.16 15 cycles
    * 1.17 16 cycles
    * 1.18 17 cycles
    * 1.19 18 cycles
    * 1.20 19 cycles
    * 1.21 20 cycles
    * 1.22 21 cycles
    * 1.23 22 cycles
    * 1.24 23 cycles
    * 1.25 24 cycles
    * 1.26 25 cycles
    * 1.27 26 cycles
    * 1.28 27 cycles
    * 1.29 28 cycles
    * 1.30 29 cycles
    * 1.31 30 cycles
    * 1.32 31 cycles
    * 1.33 32 cycles
    * 1.34 33 cycles
    * 1.35 34 cycles
    * 1.36 35 cycles
    * 1.37 36 cycles
    * 1.38 37 cycles
    * 1.39 38 cycles
    * 1.40 39 cycles
    * 1.41 40 cycles
    * 1.42 41 cycles
    * 1.43 42 cycles
    * 1.44 43 cycles
    * 1.45 44 cycles
    * 1.46 45 cycles
    * 1.47 46 cycles
    * 1.48 47 cycles
    * 1.49 48 cycles
    * 1.50 49 cycles
    * 1.51 50 cycles
  * 2 Sanity checks
  * 3 See also



## Code

All code samples are written for CA65. 

Assumptions: 

  * No page wrap occurs during any branch instruction. If a page wrap occurs, it adds +1 cycle for each loop, completely thwarting the accurate delay.
  * No interrupt / NMI occurs during the delay code.



It is permissible for DMA to steal cycles during the loops. If you are expecting that to happen, you have to manually adjust the delay cycle count (and it is in fact possible to do so) in order to get the correct delay. 

### Explanations on the requirements

  * @rts12 means you know a memory address that contains byte $60 (`RTS`).



cycle instruction that fits your constraints (such as `LDA $00`), followed by `RTS`. 

### Instructions, addressing modes, byte counts, cycle counts and notes

Addressing mode  | Instruction type  | Bytes  | Cycle count  | Example instruction  | Notes   
---|---|---|---|---|---  
Implied  | Inter-register  | 1 | 2 | `TAX` | `NOP` has no side effects. Flag-manipulations like `CLC`, and `SEC``CLV` are used when their effects are desired.   
Implied  | Stack push  | 1 | 3 | `PHA` | `PHP` is only paired with `PLP`.   
Implied  | Stack pop  | 1 | 4 | `PLA` |   
Implied  | Return  | 1 | 6 | `RTS` | Used indirectly when paired with `JSR`. Similarly for `RTI`.   
Immediate  |  | 2 | 2 | `CMP #$C5` | Includes instructions like `LDA`, `LDX` and `LDY`. Other ALU instructions are used in more complex situations.   
Relative  | Branch  | 2 | 2—4 | `BCC *+2` | Branch takes 3 cycles when taken, 2 otherwise. A page crossing adds +1 cycle when branch is taken, but because of difficulties setting that up, we don't use it.   
Zeropage  | Read, write  | 2 | 3 | `LDA $A5`  
Zeropage  | RMW  | 2 | 5 | `INC @zptemp` | Writing to zeropage is only permitted when @zptemp is available. Technically we could save @zptemp into register and restore at end, but it is bytewise inferior to other techniques.   
Zeropage indexed  | Read, write  | 2 | 4 | `LDA $EA,X` | Inferior to 2 × `NOP`, but useful for hiding additional code to be executed in a loop.   
Zeropage indexed  | RMW  | 2 | 6 | `INC @zptemp,X` | Only doable when X is known to be 0, or when entire zeropage can be clobbered.   
Indexed indirect  | Read, write  | 2 | 6 | `STA (@ptrtemp,X)` | Only doable when X is known to be 0.   
Indexed indirect  | RMW  | 2 | 8 | `SLO (@ptrtemp,X)` | The most cost-effective instruction. Only doable when X is known to be 0, lest we write to a random address. All instructions in this category are unofficial.   
Indirect indexed  | Read  | 2 | 5—6 | `LDA (@ptrtemp),Y` | Never used by this code.   
Indirect indexed  | Write  | 2 | 6 | `STA (@ptrtemp),Y` | Only doable when Y is known to be 0.   
Indirect indexed  | RMW  | 2 | 8 | `SLO (@ptrtemp),Y` | All instructions in this category are unofficial.   
Absolute  | Jump  | 3 | 3 | `JMP *+3` |   
Absolute  | Read, write  | 3 | 4 | `LDA $2808` | Inferior to 2 × `NOP`, but can be used carefully to hide additional code to be executed in a loop.   
Absolute  | RMW  | 3 | 6 | `INC $4018` | Inferior to 3 × `NOP`.   
Absolute indexed  | Read  | 3 | 4—5 | `LDA $0200,X` | Inferior to shorter alternatives.   
Absolute indexed  | Write  | 3 | 5 | `STA $0200,X` | Inferior to shorter alternatives.   
Absolute indexed  | RMW  | 3 | 7 | `INC $4018,X` | Only doable when writing into the given address is harmless considering the possible values of X.   
Absolute indirect  | Jump  | 3 | 5 | `JMP (@ptrtemp)` | Inferior to shorter alternatives.   
  
{{#css: 
    
    
     .testtable td{padding:2px} .testtable td pre{padding:2px;margin:2px}
    

}} 

  


### 2 cycles

1 bytes   
---  
      
    
    EA       NOP

| No requirements   
  
  * All instructions cost at least 2 cycles. There is no way to do 1 cycle of delay (though _−1 cycles_ may sometimes appear in branch cost calculations).



  


### 3 cycles

2 bytes   
---  
      
    
    C5 C5    CMP $C5

| Clobbers Z&N, and C   
      
    
    24 24    BIT $24

| Clobbers Z&N, and V   
      
    
    A5 A5    LDA $A5

| Clobbers A, and Z&N   
      
    
    A6 A6    LDX $A6

| Clobbers X, and Z&N   
      
    
    A4 A4    LDY $A4

| Clobbers Y, and Z&N   
3 bytes   
      
    
    4C xx xx JMP *+3

| No requirements   
  
  * Not relocatable means that the target address is hardcoded into the code. In ROM hacking, it sometimes makes sense to move code blobs around, and a hardcoded address makes it difficult to relocate the code. This restriction does not apply to branches, which use relative addressing. It is also assumed to not apply to `JSR` instructions, as chances are the JSR target is outside the code being relocated.



  


### 4 cycles

2 bytes   
---  
      
    
    EA   ... NOP      × 2

| No requirements   
  
  * zp-indexed modes such as `LDA $00,X` also do 4 cycles, but having side effects, these two-byte instructions are inferior to a simple 2 × `NOP`.
  * There is also an unofficial opcode `NOP $00,X` (34 00), but there is no reason to use this instruction when the official equivalent has the same performance.



  


### 5 cycles

3 bytes   
---  
      
    
    18       CLC
    90 00    BCC *+2

| Clobbers C   
      
    
    B8       CLV
    50 00    BVC *+2

| Clobbers V   
      
    
    EA       NOP
    A5 A5    LDA $A5

| Clobbers A, and Z&N   
      
    
    EA       NOP
    A6 A6    LDX $A6

| Clobbers X, and Z&N   
      
    
    EA       NOP
    A4 A4    LDY $A4

| Clobbers Y, and Z&N   
4 bytes   
      
    
    EA       NOP
    4C xx xx JMP *+3

| No requirements   
  
  * abs-indexed modes such as `LDA $1234,X` cause 4 or 5 cycles of delay, depending whether a page wrap occurred. Because you need extra setup code to make sure that a wrap does occur, you do not see this mode in these samples, outside situations where circumstances permit.



  


### 6 cycles

3 bytes   
---  
      
    
    EA   ... NOP      × 3

| No requirements   
  
  * zp-indexed RMW instructions such as `INC @zptemp,X` do 6 cycles. Unless we know the value of X, it might write into any address between $00-$FF. This option is only useful if the entire range of $00-$FF is free for clobbering with random data, or if X has a known value.
  * ix instructions like `LDA ($00,X)` do 6 cycles, but the value of X decides where a pointer is read from, and said pointer can point anywhere. We only do that when the value of X is known.
  * iy instructions like `LDA ($00),Y` also do 5-6 cycles, but in addition to the note above, we cannot predict whether a wrap occurs or not. So we don't use this mode.
  * Absolute RMW instructions like `INC $4018` do 6 cycles, but weighing 3 bytes with side-effects it would be inferior to 3 × `NOP`.



  


### 7 cycles

2 bytes   
---  
      
    
    08       PHP
    28       PLP

| No requirements   
  
  * `PHP-PLP` is very efficient for 7 cycles of delay, but it does modify stack contents. S register remains unchanged though.
  * `PLA-PHA` does not overwrite any bytes in stack. It just writes back the same byte. But it does clobber A and Z+N. It is not interrupt-unsafe either: If an interrupt happens, the stack byte does get temporarily clobbered, but the value is still in A when the interrupt exits, and gets written back in stack.
  * abs-indexed RMW instructions such as `INC abs,X` do 7 cycles. We only do this when either we know the value of X (for instance, `INC $4018,X` is safe when X is 0—7, or when the entire 256-byte page can be safely overwritten with random data.



  


### 8 cycles

4 bytes   
---  
      
    
    EA   ... NOP      × 4

| No requirements   
  
  * Unofficial ix and iy RMW instructions such as `SLO ($00,X)` or `SLO ($00),Y` would do 8 cycles for 2 bytes of code. We only do that if we know X or Y to be zero, and we have a known pointer to safely rewritable data.



  


### 9 cycles

3 bytes   
---  
      
    
    EA       NOP
    08       PHP
    28       PLP

| No requirements   
  
  * Jumping into the middle of another instruction and thereby reusing code is a very efficient way of reducing code size. Note that all code samples using branches on this page require that no page wrap occurs.



  


### 10 cycles

4 bytes   
---  
      
    
    08       PHP
    C5 C5    CMP $C5
    28       PLP

| No requirements   
  
  * The `ROL-ROR` sequence preserves the original value of the memory address. Carry is also preserved.



  


### 11 cycles

4 bytes   
---  
      
    
    EA   ... NOP      × 2
    08       PHP
    28       PLP

| No requirements   
  
  


### 12 cycles

3 bytes   
---  
      
    
    20 xx xx JSR @rts12

| Requires @rts12   
4 bytes   
      
    
    36 36    ROL $36,X
    76 36    ROR $36,X

| Clobbers Z&N   
5 bytes   
      
    
    08       PHP
    18       CLC
    90 00    BCC *+2
    28       PLP

| No requirements   
  
  * `JSR-RTS` causes 12 cycles of delay. But it does write a function return address in the stack, which may be unwanted in some applications. S is not modified.
  * Again, `ROL-ROR` does not have side effects (as long as an interrupt does not happen in the middle), except for Z+N.



  


### 13 cycles

5 bytes   
---  
      
    
    EA   ... NOP      × 3
    08       PHP
    28       PLP

| No requirements   
  
  


### 14 cycles

4 bytes   
---  
      
    
    08       PHP       \ × 2
    28       PLP       /

| No requirements   
  
  


### 15 cycles

5 bytes   
---  
      
    
    08       PHP
    BA       TSX
    28       PLP
    9A       TXS
    28       PLP

| Clobbers X   
      
    
    C5 C5    CMP $C5
    20 xx xx JSR @rts12

| Clobbers Z&N, and C; and requires @rts12   
      
    
    24 24    BIT $24
    20 xx xx JSR @rts12

| Clobbers Z&N, and V; and requires @rts12   
      
    
    A5 A5    LDA $A5
    20 xx xx JSR @rts12

| Clobbers A, and Z&N; and requires @rts12   
      
    
    A4 A4    LDY $A4
    20 xx xx JSR @rts12

| Clobbers Y, and Z&N; and requires @rts12   
6 bytes   
      
    
    08       PHP
    28       PLP
    EA   ... NOP      × 4

| No requirements   
  
  


### 16 cycles

5 bytes   
---  
      
    
    EA       NOP
    08       PHP       \ × 2
    28       PLP       /

| No requirements   
  
  


### 17 cycles

6 bytes   
---  
      
    
    08       PHP
    48       PHA
    A5 A5    LDA $A5
    68       PLA
    28       PLP

| No requirements   
  
  


### 18 cycles

6 bytes   
---  
      
    
    EA   ... NOP      × 2
    08       PHP       \ × 2
    28       PLP       /

| No requirements   
  
  


### 19 cycles

5 bytes   
---  
      
    
    08       PHP
    28       PLP
    20 xx xx JSR @rts12

| Requires @rts12   
6 bytes   
      
    
    08       PHP
    36 36    ROL $36,X
    76 36    ROR $36,X
    28       PLP

| No requirements   
  
  


### 20 cycles

5 bytes   
---  
      
    
    A9 2A    LDA #$2A ;hides 'ROL A'
    38       SEC
    10 FC    BPL *-2

| Clobbers A, Z&N, and C   
7 bytes   
      
    
    EA   ... NOP      × 3
    08       PHP       \ × 2
    28       PLP       /

| No requirements   
  
  


### 21 cycles

5 bytes   
---  
      
    
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    90 FD    BCC *-1

| Clobbers A, Z&N, and C   
      
    
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
6 bytes   
      
    
    08       PHP       \ × 3
    28       PLP       /

| No requirements   
  
  


### 22 cycles

6 bytes   
---  
      
    
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    38       SEC
    10 FC    BPL *-2

| Clobbers A, Z&N, and C   
      
    
    A2 02    LDX #2
    EA       NOP
    CA       DEX
    10 FC    BPL *-2

| Clobbers X, and Z&N   
      
    
    A0 03    LDY #3
    EA       NOP
    88       DEY
    D0 FC    BNE *-2

| Clobbers Y, and Z&N   
7 bytes   
      
    
    08       PHP
    BA       TSX
    08       PHP
    28   ... PLP      × 2
    9A       TXS
    28       PLP

| Clobbers X   
      
    
    08       PHP
    C5 C5    CMP $C5
    28       PLP
    20 xx xx JSR @rts12

| Requires @rts12   
8 bytes   
      
    
    08       PHP       \ × 2
    28       PLP       /
    EA   ... NOP      × 4

| No requirements   
  
  


### 23 cycles

6 bytes   
---  
      
    
    18   ... CLC      × 2
    A9 2A    LDA #$2A ;hides 'ROL A'
    90 FD    BCC *-1

| Clobbers A, Z&N, and C   
      
    
    EA       NOP
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    EA       NOP
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
7 bytes   
      
    
    EA       NOP
    08       PHP       \ × 3
    28       PLP       /

| No requirements   
  
  


### 24 cycles

4 bytes   
---  
      
    
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1

| Clobbers A, Z&N, and C   
6 bytes   
      
    
    20 xx xx JSR @rts12× 2

| Requires @rts12   
7 bytes   
      
    
    A6 A6    LDX $A6
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    A4 A4    LDY $A4
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
8 bytes   
      
    
    08       PHP
    C5 C5    CMP $C5
    28       PLP       \ × 2
    08       PHP       /
    28       PLP

| No requirements   
  
  


### 25 cycles

7 bytes   
---  
      
    
    98       TYA
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1
    A8       TAY

| Clobbers A, and Z&N   
      
    
    EA   ... NOP      × 2
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    EA   ... NOP      × 2
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
8 bytes   
      
    
    EA   ... NOP      × 2
    08       PHP       \ × 3
    28       PLP       /

| No requirements   
  
  


### 26 cycles

5 bytes   
---  
      
    
    18       CLC
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1

| Clobbers A, Z&N, and C   
      
    
    A2 04    LDX #4
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
      
    
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
7 bytes   
      
    
    EA       NOP
    20 xx xx JSR @rts12× 2

| Requires @rts12   
8 bytes   
      
    
    08       PHP
    48       PHA
    36 36    ROL $36,X
    76 36    ROR $36,X
    68       PLA
    28       PLP

| No requirements   
  
  


### 27 cycles

6 bytes   
---  
      
    
    A5 A5    LDA $A5
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1

| Clobbers A, Z&N, and C   
7 bytes   
      
    
    48       PHA
    A9 2A    LDA #$2A ;hides 'ROL A'
    38       SEC
    10 FC    BPL *-2
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A9 2A    LDA #$2A ;hides 'ROL A'
    38       SEC
    10 FC    BPL *-2
    28       PLP

| Clobbers A   
      
    
    24 2C    BIT <$2C ;hides 'BIT $FDA2'
    A2 FD    LDX #253
    E8       INX
    D0 FA    BNE *-4

| Clobbers X, Z&N, and V   
      
    
    24 2C    BIT <$2C ;hides 'BIT $FDA0'
    A0 FD    LDY #253
    C8       INY
    D0 FA    BNE *-4

| Clobbers Y, Z&N, and V   
      
    
    A4 AC    LDY <$AC ;hides 'LDY $82A2'
    A2 82    LDX #130
    CA       DEX
    30 FA    BMI *-4

| Clobbers X, Y, and Z&N   
8 bytes   
      
    
    EA   ... NOP      × 3
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    EA   ... NOP      × 3
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
      
    
    24 24    BIT $24
    20 xx xx JSR @rts12× 2

| Clobbers Z&N, and V; and requires @rts12   
      
    
    20 xx xx JSR @rts12
    08       PHP
    BA       TSX
    28       PLP
    9A       TXS
    28       PLP

| Clobbers X; and requires @rts12   
9 bytes   
      
    
    EA   ... NOP      × 3
    08       PHP       \ × 3
    28       PLP       /

| No requirements   
  
  


### 28 cycles

6 bytes   
---  
      
    
    38   ... SEC      × 2
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1

| Clobbers A, Z&N, and C   
      
    
    EA       NOP
    A2 04    LDX #4
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
      
    
    EA       NOP
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
7 bytes   
      
    
    48       PHA
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    90 FD    BCC *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    90 FD    BCC *-1
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
8 bytes   
      
    
    08       PHP       \ × 4
    28       PLP       /

| No requirements   
  
  


### 29 cycles

6 bytes   
---  
      
    
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    EA       NOP
    90 FC    BCC *-2

| Clobbers A, Z&N, and C   
      
    
    A2 04    LDX #4
    EA       NOP
    CA       DEX
    D0 FC    BNE *-2

| Clobbers X, and Z&N   
      
    
    A0 04    LDY #4
    EA       NOP
    88       DEY
    D0 FC    BNE *-2

| Clobbers Y, and Z&N   
8 bytes   
      
    
    48       PHA
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    38       SEC
    10 FC    BPL *-2
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    38       SEC
    10 FC    BPL *-2
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 02    LDX #2
    EA       NOP
    CA       DEX
    10 FC    BPL *-2
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 03    LDY #3
    EA       NOP
    88       DEY
    D0 FC    BNE *-2
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    08       PHP
    28       PLP
    08       PHP
    C5 C5    CMP $C5
    28       PLP
    20 xx xx JSR @rts12

| Requires @rts12   
10 bytes   
      
    
    08       PHP
    C5 C5    CMP $C5
    28       PLP
    08       PHP
    36 36    ROL $36,X
    76 36    ROR $36,X
    28       PLP

| No requirements   
  
  


### 30 cycles

7 bytes   
---  
      
    
    98       TYA
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    A8       TAY

| Clobbers A, and Z&N   
      
    
    EA   ... NOP      × 2
    A2 04    LDX #4
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
      
    
    EA   ... NOP      × 2
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
8 bytes   
      
    
    48       PHA
    18   ... CLC      × 2
    A9 2A    LDA #$2A ;hides 'ROL A'
    90 FD    BCC *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    18   ... CLC      × 2
    A9 2A    LDA #$2A ;hides 'ROL A'
    90 FD    BCC *-1
    28       PLP

| Clobbers A   
      
    
    EA       NOP
    08       PHP
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    EA       NOP
    08       PHP
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    08       PHP
    48       PHA
    18       CLC
    A9 6A    LDA #$6A ;hides 'ROR A'
    90 FD    BCC *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 31 cycles

5 bytes   
---  
      
    
    18       CLC
    A9 0A    LDA #$0A ;hides 'ASL A'
    90 FD    BCC *-1

| Clobbers A, Z&N, and C   
      
    
    A2 05    LDX #5
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
      
    
    A0 06    LDY #6
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
6 bytes   
      
    
    48       PHA
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    28       PLP

| Clobbers A   
8 bytes   
      
    
    08       PHP
    28       PLP
    20 xx xx JSR @rts12× 2

| Requires @rts12   
9 bytes   
      
    
    08       PHP
    A6 A6    LDX $A6
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A4 A4    LDY $A4
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
10 bytes   
      
    
    08       PHP
    36 36    ROL $36,X \ × 2
    76 36    ROR $36,X /
    28       PLP

| No requirements   
  
  


### 32 cycles

6 bytes   
---  
      
    
    A2 05    LDX #5 ;hides 'ORA zp'
    CA       DEX ;first loop only
    CA       DEX
    D0 FB    BNE *-3

| Clobbers A, X, and Z&N   
      
    
    A0 05    LDY #5 ;hides 'ORA zp'
    88       DEY ;first loop only
    88       DEY
    D0 FB    BNE *-3

| Clobbers A, Y, and Z&N   
7 bytes   
      
    
    A9 2A    LDA #$2A ;hides 'ROL A'
    EA   ... NOP      × 3
    10 FA    BPL *-4

| Clobbers A, Z&N, and C   
8 bytes   
      
    
    EA       NOP
    98       TYA
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    A8       TAY

| Clobbers A, and Z&N   
      
    
    A6 A6    LDX $A6
    A2 04    LDX #4
    EA       NOP
    CA       DEX
    D0 FC    BNE *-2

| Clobbers X, and Z&N   
      
    
    A4 A4    LDY $A4
    A0 04    LDY #4
    EA       NOP
    88       DEY
    D0 FC    BNE *-2

| Clobbers Y, and Z&N   
9 bytes   
      
    
    48       PHA
    98       TYA
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    68       PLA

| Clobbers Z&N   
      
    
    08       PHP
    98       TYA
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    28       PLP

| Clobbers A   
      
    
    EA   ... NOP      × 2
    08       PHP
    A2 04    LDX #4
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    EA   ... NOP      × 2
    08       PHP
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
10 bytes   
      
    
    08       PHP
    48       PHA
    18   ... CLC      × 2
    A9 6A    LDA #$6A ;hides 'ROR A'
    90 FD    BCC *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 33 cycles

6 bytes   
---  
      
    
    18   ... CLC      × 2
    A9 0A    LDA #$0A ;hides 'ASL A'
    90 FD    BCC *-1

| Clobbers A, Z&N, and C   
      
    
    EA       NOP
    A2 05    LDX #5
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
      
    
    EA       NOP
    A0 06    LDY #6
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
7 bytes   
      
    
    48       PHA
    18       CLC
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    18       CLC
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 04    LDX #4
    CA       DEX
    10 FD    BPL *-1
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    EA       NOP
    08       PHP
    28       PLP
    20 xx xx JSR @rts12× 2

| Requires @rts12   
10 bytes   
      
    
    08       PHP       \ × 2
    28       PLP       /
    08       PHP
    36 36    ROL $36,X
    76 36    ROR $36,X
    28       PLP

| No requirements   
  
  


### 34 cycles

5 bytes   
---  
      
    
    A9 0A    LDA #$0A ;hides 'ASL A'
    18       CLC
    10 FC    BPL *-2

| Clobbers A, Z&N, and C   
      
    
    A0 88    LDY #136 ;hides 'DEY'
    88       DEY
    30 FC    BMI *-2

| Clobbers Y, and Z&N   
7 bytes   
      
    
    A6 A6    LDX $A6
    A2 05    LDX #5
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
8 bytes   
      
    
    C5 C5    CMP $C5
    48       PHA
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A5 A5    LDA $A5
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    28       PLP

| Clobbers A   
9 bytes   
      
    
    08       PHP
    48       PHA
    A9 2A    LDA #$2A ;hides 'ROL A'
    38       SEC
    10 FC    BPL *-2
    68       PLA
    28       PLP

| No requirements   
  
  


### 35 cycles

6 bytes   
---  
      
    
    A9 2A    LDA #$2A ;hides 'ROL A'
    08       PHP
    28       PLP
    10 FB    BPL *-3

| Clobbers A, Z&N, and C   
      
    
    A2 F8    LDX #248 ;hides 'SED'
    E8   ... INX      × 2
    D0 FB    BNE *-3

| Clobbers X, Z&N, and D   
      
    
    A0 88    LDY #136 ;hides 'DEY'
    88   ... DEY      × 2
    30 FB    BMI *-3

| Clobbers Y, and Z&N   
7 bytes   
      
    
    98       TYA
    A0 06    LDY #6
    88       DEY
    D0 FD    BNE *-1
    A8       TAY

| Clobbers A, and Z&N   
      
    
    EA   ... NOP      × 2
    A2 05    LDX #5
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
8 bytes   
      
    
    48       PHA
    38   ... SEC      × 2
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    38   ... SEC      × 2
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    28       PLP

| Clobbers A   
      
    
    EA       NOP
    08       PHP
    A2 04    LDX #4
    CA       DEX
    10 FD    BPL *-1
    28       PLP

| Clobbers X   
      
    
    EA       NOP
    08       PHP
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    08       PHP
    48       PHA
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    90 FD    BCC *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 36 cycles

5 bytes   
---  
      
    
    A9 E9    LDA #$E9 ;hides 'SBC #$2A'
    2A       ROL A ;first loop only
    B0 FC    BCS *-2

| Clobbers A, Z&N, C, and V   
      
    
    A2 07    LDX #7
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1

| Clobbers Y, and Z&N   
6 bytes   
      
    
    38       SEC
    A9 0A    LDA #$0A ;hides 'ASL A'
    38       SEC
    10 FC    BPL *-2

| Clobbers A, Z&N, and C   
8 bytes   
      
    
    48       PHA
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    EA       NOP
    90 FC    BCC *-2
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    EA       NOP
    90 FC    BCC *-2
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 04    LDX #4
    EA       NOP
    CA       DEX
    D0 FC    BNE *-2
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 04    LDY #4
    EA       NOP
    88       DEY
    D0 FC    BNE *-2
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    20 xx xx JSR @rts12× 3

| Requires @rts12   
10 bytes   
      
    
    08       PHP
    48       PHA
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    38       SEC
    10 FC    BPL *-2
    68       PLA
    28       PLP

| No requirements   
  
  


### 37 cycles

7 bytes   
---  
      
    
    A5 A5    LDA $A5
    A9 0A    LDA #$0A ;hides 'ASL A'
    18       CLC
    10 FC    BPL *-2

| Clobbers A, Z&N, and C   
      
    
    A2 04    LDX #4
    EA   ... NOP      × 2
    CA       DEX
    D0 FB    BNE *-3

| Clobbers X, and Z&N   
      
    
    A0 04    LDY #4
    EA   ... NOP      × 2
    88       DEY
    D0 FB    BNE *-3

| Clobbers Y, and Z&N   
8 bytes   
      
    
    EA       NOP
    98       TYA
    A0 06    LDY #6
    88       DEY
    D0 FD    BNE *-1
    A8       TAY

| Clobbers A, and Z&N   
9 bytes   
      
    
    48       PHA
    98       TYA
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    68       PLA

| Clobbers Z&N   
      
    
    08       PHP
    98       TYA
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    28       PLP

| Clobbers A   
      
    
    EA   ... NOP      × 2
    08       PHP
    A2 04    LDX #4
    CA       DEX
    10 FD    BPL *-1
    28       PLP

| Clobbers X   
      
    
    EA   ... NOP      × 2
    08       PHP
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
10 bytes   
      
    
    08       PHP
    48       PHA
    18   ... CLC      × 2
    A9 2A    LDA #$2A ;hides 'ROL A'
    90 FD    BCC *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 38 cycles

6 bytes   
---  
      
    
    38       SEC
    A9 69    LDA #$69 ;hides 'ADC #$EA'
    EA       NOP ;first loop only
    B0 FC    BCS *-2

| Clobbers A, Z&N, C, and V   
      
    
    EA       NOP
    A2 07    LDX #7
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    EA       NOP
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1

| Clobbers Y, and Z&N   
7 bytes   
      
    
    48       PHA
    18       CLC
    A9 0A    LDA #$0A ;hides 'ASL A'
    90 FD    BCC *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    18       CLC
    A9 0A    LDA #$0A ;hides 'ASL A'
    90 FD    BCC *-1
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 05    LDX #5
    CA       DEX
    10 FD    BPL *-1
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 06    LDY #6
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
8 bytes   
      
    
    08       PHP
    48       PHA
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 39 cycles

4 bytes   
---  
      
    
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1

| Clobbers A, Z&N, and C   
7 bytes   
      
    
    A6 A6    LDX $A6
    A2 07    LDX #7
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    A4 A4    LDY $A4
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1

| Clobbers Y, and Z&N   
8 bytes   
      
    
    98       TYA
    A0 88    LDY #136 ;hides 'DEY'
    88   ... DEY      × 2
    30 FB    BMI *-3
    A8       TAY

| Clobbers A, and Z&N   
      
    
    08       PHP
    A2 05    LDX #5 ;hides 'ORA zp'
    CA       DEX ;first loop only
    CA       DEX
    D0 FB    BNE *-3
    28       PLP

| Clobbers A, and X   
      
    
    08       PHP
    A0 05    LDY #5 ;hides 'ORA zp'
    88       DEY ;first loop only
    88       DEY
    D0 FB    BNE *-3
    28       PLP

| Clobbers A, and Y   
9 bytes   
      
    
    48       PHA
    A9 2A    LDA #$2A ;hides 'ROL A'
    EA   ... NOP      × 3
    10 FA    BPL *-4
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A9 2A    LDA #$2A ;hides 'ROL A'
    EA   ... NOP      × 3
    10 FA    BPL *-4
    28       PLP

| Clobbers A   
10 bytes   
      
    
    EA       NOP
    48       PHA
    98       TYA
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    68       PLA

| Clobbers Z&N   
      
    
    08       PHP
    A6 A6    LDX $A6
    A2 04    LDX #4
    EA       NOP
    CA       DEX
    D0 FC    BNE *-2
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A4 A4    LDY $A4
    A0 04    LDY #4
    EA       NOP
    88       DEY
    D0 FC    BNE *-2
    28       PLP

| Clobbers Y   
11 bytes   
      
    
    08       PHP
    48       PHA
    98       TYA
    A0 04    LDY #4
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    68       PLA
    28       PLP

| No requirements   
  
  


### 40 cycles

6 bytes   
---  
      
    
    A2 05    LDX #5 ;hides 'ORA zp'
    EA       NOP
    CA       DEX
    D0 FB    BNE *-3

| Clobbers A, X, and Z&N   
      
    
    A0 05    LDY #5 ;hides 'ORA zp'
    EA       NOP
    88       DEY
    D0 FB    BNE *-3

| Clobbers A, Y, and Z&N   
7 bytes   
      
    
    98       TYA
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1
    A8       TAY

| Clobbers A, and Z&N   
      
    
    EA   ... NOP      × 2
    A2 07    LDX #7
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    EA   ... NOP      × 2
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1

| Clobbers Y, and Z&N   
8 bytes   
      
    
    48       PHA
    18   ... CLC      × 2
    A9 0A    LDA #$0A ;hides 'ASL A'
    90 FD    BCC *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    18   ... CLC      × 2
    A9 0A    LDA #$0A ;hides 'ASL A'
    90 FD    BCC *-1
    28       PLP

| Clobbers A   
      
    
    EA       NOP
    08       PHP
    A2 05    LDX #5
    CA       DEX
    10 FD    BPL *-1
    28       PLP

| Clobbers X   
      
    
    EA       NOP
    08       PHP
    A0 06    LDY #6
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    08       PHP
    48       PHA
    18       CLC
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 41 cycles

5 bytes   
---  
      
    
    38       SEC
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1

| Clobbers A, Z&N, and C   
      
    
    A2 08    LDX #8
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    A0 08    LDY #8
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
7 bytes   
      
    
    48       PHA
    A9 0A    LDA #$0A ;hides 'ASL A'
    18       CLC
    10 FC    BPL *-2
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A9 0A    LDA #$0A ;hides 'ASL A'
    18       CLC
    10 FC    BPL *-2
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A0 88    LDY #136 ;hides 'DEY'
    88       DEY
    30 FC    BMI *-2
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    08       PHP
    A6 A6    LDX $A6
    A2 05    LDX #5
    CA       DEX
    10 FD    BPL *-1
    28       PLP

| Clobbers X   
10 bytes   
      
    
    08       PHP
    48       PHA
    A5 A5    LDA $A5
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 42 cycles

6 bytes   
---  
      
    
    A5 A5    LDA $A5
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1

| Clobbers A, Z&N, and C   
7 bytes   
      
    
    EA       NOP
    A2 05    LDX #5 ;hides 'ORA zp'
    EA       NOP
    CA       DEX
    D0 FB    BNE *-3

| Clobbers A, X, and Z&N   
      
    
    EA       NOP
    A0 05    LDY #5 ;hides 'ORA zp'
    EA       NOP
    88       DEY
    D0 FB    BNE *-3

| Clobbers A, Y, and Z&N   
8 bytes   
      
    
    48       PHA
    A9 2A    LDA #$2A ;hides 'ROL A'
    08       PHP
    28       PLP
    10 FB    BPL *-3
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A9 2A    LDA #$2A ;hides 'ROL A'
    08       PHP
    28       PLP
    10 FB    BPL *-3
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 F8    LDX #248 ;hides 'SED'
    E8   ... INX      × 2
    D0 FB    BNE *-3
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 88    LDY #136 ;hides 'DEY'
    88   ... DEY      × 2
    30 FB    BMI *-3
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    48       PHA
    98       TYA
    A0 06    LDY #6
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    68       PLA

| Clobbers Z&N   
10 bytes   
      
    
    08       PHP
    48       PHA
    38   ... SEC      × 2
    A9 0A    LDA #$0A ;hides 'ASL A'
    10 FD    BPL *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 43 cycles

6 bytes   
---  
      
    
    38   ... SEC      × 2
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1

| Clobbers A, Z&N, and C   
      
    
    A2 05    LDX #5
    EA       NOP
    CA       DEX
    10 FC    BPL *-2

| Clobbers X, and Z&N   
      
    
    A0 06    LDY #6
    EA       NOP
    88       DEY
    D0 FC    BNE *-2

| Clobbers Y, and Z&N   
7 bytes   
      
    
    48       PHA
    A9 E9    LDA #$E9 ;hides 'SBC #$2A'
    2A       ROL A ;first loop only
    B0 FC    BCS *-2
    68       PLA

| Clobbers Z&N, C, and V   
      
    
    08       PHP
    A9 E9    LDA #$E9 ;hides 'SBC #$2A'
    2A       ROL A ;first loop only
    B0 FC    BCS *-2
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 07    LDX #7
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1
    28       PLP

| Clobbers Y   
8 bytes   
      
    
    48       PHA
    38       SEC
    A9 0A    LDA #$0A ;hides 'ASL A'
    38       SEC
    10 FC    BPL *-2
    68       PLA

| Clobbers Z&N, and C   
10 bytes   
      
    
    08       PHP
    48       PHA
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    EA       NOP
    90 FC    BCC *-2
    68       PLA
    28       PLP

| No requirements   
  
  


### 44 cycles

6 bytes   
---  
      
    
    A9 0A    LDA #$0A ;hides 'ASL A'
    EA   ... NOP      × 2
    10 FB    BPL *-3

| Clobbers A, Z&N, and C   
      
    
    A0 88    LDY #136 ;hides 'DEY'
    EA       NOP
    88       DEY
    30 FB    BMI *-3

| Clobbers Y, and Z&N   
7 bytes   
      
    
    A6 A6    LDX $A6
    A2 08    LDX #8
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
9 bytes   
      
    
    C5 C5    CMP $C5
    48       PHA
    A9 0A    LDA #$0A ;hides 'ASL A'
    18       CLC
    10 FC    BPL *-2
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A5 A5    LDA $A5
    A9 0A    LDA #$0A ;hides 'ASL A'
    18       CLC
    10 FC    BPL *-2
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 04    LDX #4
    EA   ... NOP      × 2
    CA       DEX
    D0 FB    BNE *-3
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 04    LDY #4
    EA   ... NOP      × 2
    88       DEY
    D0 FB    BNE *-3
    28       PLP

| Clobbers Y   
10 bytes   
      
    
    EA       NOP
    48       PHA
    98       TYA
    A0 06    LDY #6
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    68       PLA

| Clobbers Z&N   
11 bytes   
      
    
    08       PHP
    48       PHA
    98       TYA
    A0 05    LDY #5
    88       DEY
    D0 FD    BNE *-1
    A8       TAY
    68       PLA
    28       PLP

| No requirements   
  
  


### 45 cycles

7 bytes   
---  
      
    
    98       TYA
    A0 08    LDY #8
    88       DEY
    D0 FD    BNE *-1
    A8       TAY

| Clobbers A, and Z&N   
      
    
    EA   ... NOP      × 2
    A2 08    LDX #8
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    EA   ... NOP      × 2
    A0 08    LDY #8
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
8 bytes   
      
    
    48       PHA
    38       SEC
    A9 69    LDA #$69 ;hides 'ADC #$EA'
    EA       NOP ;first loop only
    B0 FC    BCS *-2
    68       PLA

| Clobbers Z&N, C, and V   
      
    
    08       PHP
    38       SEC
    A9 69    LDA #$69 ;hides 'ADC #$EA'
    EA       NOP ;first loop only
    B0 FC    BCS *-2
    28       PLP

| Clobbers A   
      
    
    EA       NOP
    08       PHP
    A2 07    LDX #7
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    EA       NOP
    08       PHP
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    08       PHP
    48       PHA
    18       CLC
    A9 0A    LDA #$0A ;hides 'ASL A'
    90 FD    BCC *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 46 cycles

5 bytes   
---  
      
    
    A2 08    LDX #8
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
      
    
    A0 09    LDY #9
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
6 bytes   
      
    
    48       PHA
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1
    28       PLP

| Clobbers A   
9 bytes   
      
    
    08       PHP
    A6 A6    LDX $A6
    A2 07    LDX #7
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A4 A4    LDY $A4
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1
    28       PLP

| Clobbers Y   
10 bytes   
      
    
    48       PHA
    98       TYA
    A0 88    LDY #136 ;hides 'DEY'
    88   ... DEY      × 2
    30 FB    BMI *-3
    A8       TAY
    68       PLA

| Clobbers Z&N   
11 bytes   
      
    
    08       PHP
    48       PHA
    A9 2A    LDA #$2A ;hides 'ROL A'
    EA   ... NOP      × 3
    10 FA    BPL *-4
    68       PLA
    28       PLP

| No requirements   
  
  


### 47 cycles

8 bytes   
---  
      
    
    98       TYA
    A0 06    LDY #6
    EA       NOP
    88       DEY
    D0 FC    BNE *-2
    A8       TAY

| Clobbers A, and Z&N   
      
    
    EA   ... NOP      × 3
    A2 08    LDX #8
    CA       DEX
    D0 FD    BNE *-1

| Clobbers X, and Z&N   
      
    
    08       PHP
    A2 05    LDX #5 ;hides 'ORA zp'
    EA       NOP
    CA       DEX
    D0 FB    BNE *-3
    28       PLP

| Clobbers A, and X   
      
    
    EA   ... NOP      × 3
    A0 08    LDY #8
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
      
    
    08       PHP
    A0 05    LDY #5 ;hides 'ORA zp'
    EA       NOP
    88       DEY
    D0 FB    BNE *-3
    28       PLP

| Clobbers A, and Y   
9 bytes   
      
    
    48       PHA
    98       TYA
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1
    A8       TAY
    68       PLA

| Clobbers Z&N   
      
    
    08       PHP
    98       TYA
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1
    A8       TAY
    28       PLP

| Clobbers A   
      
    
    EA   ... NOP      × 2
    08       PHP
    A2 07    LDX #7
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    EA   ... NOP      × 2
    08       PHP
    A0 06    LDY #6
    88       DEY
    10 FD    BPL *-1
    28       PLP

| Clobbers Y   
10 bytes   
      
    
    08       PHP
    48       PHA
    18   ... CLC      × 2
    A9 0A    LDA #$0A ;hides 'ASL A'
    90 FD    BCC *-1
    68       PLA
    28       PLP

| No requirements   
  
  


### 48 cycles

6 bytes   
---  
      
    
    EA       NOP
    A2 08    LDX #8
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
      
    
    EA       NOP
    A0 09    LDY #9
    88       DEY
    D0 FD    BNE *-1

| Clobbers Y, and Z&N   
7 bytes   
      
    
    48       PHA
    38       SEC
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    38       SEC
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 08    LDX #8
    CA       DEX
    D0 FD    BNE *-1
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 08    LDY #8
    88       DEY
    D0 FD    BNE *-1
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    08       PHP
    48       PHA
    A9 0A    LDA #$0A ;hides 'ASL A'
    18       CLC
    10 FC    BPL *-2
    68       PLA
    28       PLP

| No requirements   
  
  


### 49 cycles

4 bytes   
---  
      
    
    A0 88    LDY #136 ;hides 'DEY'
    30 FD    BMI *-1

| Clobbers Y, and Z&N   
7 bytes   
      
    
    18       CLC
    A9 2A    LDA #$2A ;hides 'ROL A'
    08       PHP
    28       PLP
    90 FB    BCC *-3

| Clobbers A, Z&N, and C   
      
    
    A6 A6    LDX $A6
    A2 08    LDX #8
    CA       DEX
    10 FD    BPL *-1

| Clobbers X, and Z&N   
8 bytes   
      
    
    C5 C5    CMP $C5
    48       PHA
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    A5 A5    LDA $A5
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1
    28       PLP

| Clobbers A   
10 bytes   
      
    
    08       PHP
    48       PHA
    A9 2A    LDA #$2A ;hides 'ROL A'
    08       PHP
    28       PLP
    10 FB    BPL *-3
    68       PLA
    28       PLP

| No requirements   
  
  


### 50 cycles

6 bytes   
---  
      
    
    A9 E9    LDA #$E9 ;hides 'SBC #$2A'
    2A       ROL A ;first loop only
    EA       NOP
    B0 FB    BCS *-3

| Clobbers A, Z&N, C, and V   
      
    
    A2 07    LDX #7
    EA       NOP
    CA       DEX
    D0 FC    BNE *-2

| Clobbers X, and Z&N   
      
    
    A0 06    LDY #6
    EA       NOP
    88       DEY
    10 FC    BPL *-2

| Clobbers Y, and Z&N   
7 bytes   
      
    
    98       TYA
    A0 09    LDY #9
    88       DEY
    D0 FD    BNE *-1
    A8       TAY

| Clobbers A, and Z&N   
8 bytes   
      
    
    48       PHA
    38   ... SEC      × 2
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1
    68       PLA

| Clobbers Z&N, and C   
      
    
    08       PHP
    38   ... SEC      × 2
    A9 4A    LDA #$4A ;hides 'LSR A'
    D0 FD    BNE *-1
    28       PLP

| Clobbers A   
      
    
    08       PHP
    A2 05    LDX #5
    EA       NOP
    CA       DEX
    10 FC    BPL *-2
    28       PLP

| Clobbers X   
      
    
    08       PHP
    A0 06    LDY #6
    EA       NOP
    88       DEY
    D0 FC    BNE *-2
    28       PLP

| Clobbers Y   
9 bytes   
      
    
    08       PHP
    48       PHA
    A9 E9    LDA #$E9 ;hides 'SBC #$2A'
    2A       ROL A ;first loop only
    B0 FC    BCS *-2
    68       PLA
    28       PLP

| No requirements   
  
  


## Sanity checks

It is possible to verify on compile time that no page wrap occurs, by replacing all branches with these macros: 
    
    
    .macro branch_check opc, dest
        opc dest
        .assert >* = >(dest), warning, "branch_check: failed, crosses page"
    .endmacro
    .macro bccnw dest
            branch_check bcc, dest
    .endmacro
    .macro bcsnw dest
            branch_check bcs, dest
    .endmacro
    .macro beqnw dest
            branch_check beq, dest
    .endmacro
    .macro bnenw dest
            branch_check bne, dest
    .endmacro
    .macro bminw dest
            branch_check bmi, dest
    .endmacro
    .macro bplnw dest
            branch_check bpl, dest
    .endmacro
    .macro bvcnw dest
            branch_check bvc, dest
    .endmacro
    .macro bvsnw dest
            branch_check bvs, dest
    .endmacro

## See also

  * [Cycle counting](Cycle_counting.xhtml "Cycle counting")
  * [Delay code](Delay_code.xhtml "Delay code") for functions that produce runtime-determined amount of delay
  * Bisqwit’s “vending machine” for producing a ca65-compatible delay_n macro for arbitrary number of cycles, with more fine-grained configurable constraints: <http://bisqwit.iki.fi/utils/nesdelay.php> The samples on this page are excerpts from files generated by this online tool.


