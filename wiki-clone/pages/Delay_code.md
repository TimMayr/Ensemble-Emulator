# Delay code

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Delay_code) | View [other pages](Special_AllPages.xhtml#Delay_code)

Code that causes a parametrised number of cycles of delay. 

Note that all branch instructions are written assuming that no page wrap occurs. If you want to ensure this condition at compile time, use the bccnw/beqnw/etc. macros that are listed at [Fixed cycle delay](Fixed_cycle_delay.xhtml "Fixed cycle delay"). 

## Contents

  * 1 Inline code
    * 1.1 2—3 cycles of delay: delay=r+2; 0 ≤ r ≤ 1, r⊢Z, Δr = 0)
    * 1.2 4—5 cycles of delay: delay=r+4; 0 ≤ r ≤ 1, Δr = 0)
    * 1.3 4—5 cycles of delay: delay=X+4; 0 ≤ X ≤ 1)
    * 1.4 5—7 cycles of delay: delay=A+5; 0 ≤ A ≤ 2, A⊢Z)
    * 1.5 5—7 cycles of delay: delay=r+5; 0 ≤ r ≤ 2, Δr = 0)
    * 1.6 5—7 cycles of delay: delay=X+5; 0 ≤ X ≤ 2)
    * 1.7 6—9 cycles of delay: delay=A+6; 0 ≤ A ≤ 3, A⊢Z)
    * 1.8 7—10 cycles of delay: delay=A+7; 0 ≤ A ≤ 3)
    * 1.9 8—11 cycles of delay: delay=X+8; 0 ≤ X ≤ 3)
    * 1.10 9—14 cycles of delay: delay=A−242; 251 ≤ A ≤ 255; C = 0)
    * 1.11 10—14 cycles of delay: delay=X+10; 0 ≤ X ≤ 4)
    * 1.12 9—14 cycles of delay: delay=A+9; 0 ≤ A ≤ 5)
    * 1.13 9—16 cycles of delay: delay=A+9; 0 ≤ A ≤ 7)
    * 1.14 11—19 cycles of delay: delay=A+11; 0 ≤ A ≤ 8)
    * 1.15 12—23 cycles of delay: delay=A+12; 0 ≤ A ≤ 11)
    * 1.16 15—270 cycles of delay: delay=A+15; 0 ≤ A ≤ 255)
    * 1.17 16—271 cycles of delay: delay=A+16; 0 ≤ A ≤ 255)
    * 1.18 5—65285 cycles of delay: delay = 256×X + 5
    * 1.19 18—218103813 cycles of delay: delay = 13×(65536×Y + 256×A + X) + 18
  * 2 Callable functions
    * 2.1 A + 25 cycles of delay, clobbers A, Z&N, C, V
    * 2.2 A + 27 cycles of delay, clobbers A, Z&N, C, V
    * 2.3 256×A + X + 33 cycles of delay, clobbers A, Z&N, C, V
    * 2.4 256×A + 16 cycles of delay, clobbers A, Z&N, C, V
    * 2.5 256×X + 16 cycles of delay, clobbers X, Z&N
    * 2.6 256×X + A + 30 cycles of delay, clobbers A, X, Z&N, C, V
    * 2.7 851968×Y + 3328×A + 13×X + 30 cycles of delay, clobbers A, X, Y, Z&N, C, V
  * 3 See also



## Inline code

### 2—3 cycles of delay: delay=r+2; 0 ≤ r ≤ 1, r⊢Z, Δr = 0)
    
    
            bne @1
    @1:

### 4—5 cycles of delay: delay=r+4; 0 ≤ r ≤ 1, Δr = 0)
    
    
            ora #0 ; use ora=A, cpx=X, cpy=Y
            bne @1
    @1:

### 4—5 cycles of delay: delay=X+4; 0 ≤ X ≤ 1)
    
    
            dex
            bpl @1
    @1:

### 5—7 cycles of delay: delay=A+5; 0 ≤ A ≤ 2, A⊢Z)
    
    
            beq @2
            lsr
    @2:     bne @3
    @3:

### 5—7 cycles of delay: delay=r+5; 0 ≤ r ≤ 2, Δr = 0)
    
    
            cmp #1 ; use cmp=A, cpx=X, cpy=Y
            bcc @3
            bne @3
    @3:

### 5—7 cycles of delay: delay=X+5; 0 ≤ X ≤ 2)
    
    
            dex
            bmi @3
            bne @3
    @3:

### 6—9 cycles of delay: delay=A+6; 0 ≤ A ≤ 3, A⊢Z)
    
    
            beq @2
            lsr
    @2:     beq @4
            bcs @4
    @4:

### 7—10 cycles of delay: delay=A+7; 0 ≤ A ≤ 3)
    
    
            lsr
            beq @3
            bpl @3
    @3:     bcs @4
    @4:

### 8—11 cycles of delay: delay=X+8; 0 ≤ X ≤ 3)
    
    
            dex
            bmi @4
            dex
            bmi @5
    @4:     bne @5
    @5:

### 9—14 cycles of delay: delay=A−242; 251 ≤ A ≤ 255; C = 0)
    
    
            adc #3  ;  2 2 2 2 2  FE FF 00 01 02
            bcc @4  ;  3 3 2 2 2  FE FF 00 01 02 ;bmi works too
            lsr     ;  - - 2 2 2  -- -- 00 00 01
            beq @5  ;  - - 3 3 2  -- -- 00 00 01
    @4:     lsr     ;  2 2 - - 2  7F 7F -- -- 00
    @5:     bcs @6  ;  2 3 2 3 2  7F 7F 00 00 00
    @6:

### 10—14 cycles of delay: delay=X+10; 0 ≤ X ≤ 4)
    
    
            cpx #3
            bcc @3
            bne @3
    @3:     dex
            bmi @6
            bne @6
    @6:

### 9—14 cycles of delay: delay=A+9; 0 ≤ A ≤ 5)
    
    
            lsr
            bcs @2
    @2:     beq @5
            lsr
            bcs @6 ;beq works too
    @5:     bne @6
    @6:

### 9—16 cycles of delay: delay=A+9; 0 ≤ A ≤ 7)
    
    
            lsr
            bcs @2
    @2:     beq @6
            lsr
            beq @7
            bcc @7
    @6:     bne @7
    @7:

### 11—19 cycles of delay: delay=A+11; 0 ≤ A ≤ 8)
    
    
    ;      Cycles | A | 0  0  0  0  0  0  0  0  0  | 0  1  2  3  4  5  6  7  8
            lsr       ; 2  2  2  2  2  2  2  2  2  | 0  0  1  1  2  2  3  3  4
            bcs @3    ; 2  3  2  3  2  3  2  3  2  | 0 c0  1 c1  2 c2  3 c3  4
            adc #255  ; 2  -  2  -  2  -  2  -  2  |-1  - c0  - c1  - c2  - c3
    @3:     beq @7    ; 2  3  3  2  2  2  2  2  2  |-1 c0 c0 c1 c1 c2 c2 c3 c3
            bcc @9    ; 3  -  -  2  2  2  2  2  2  |-1  -  - c1 c1 c2 c2 c3 c3 ;bmi works too
            lsr       ; -  -  -  2  2  2  2  2  2  | -  -  - c0 c0  1  1 c1 c1
            beq @9    ; -  -  -  3  3  2  2  2  2  | -  -  - c0 c0  1  1 c1 c1
    @7:     bcc @9    ; -  2  2  -  -  3  3  2  2  | - c0 c0  -  -  1  1 c1 c1
            bne @9    ; -  2  2  -  -  -  -  3  3  | - c0 c0  -  -  -  - c1 c1
    @9:       ;Total:  11 12 13 14 15 16 17 18 19
    

### 12—23 cycles of delay: delay=A+12; 0 ≤ A ≤ 11)
    
    
    ;      Cycles | A | 0  0  0  0  0  0  0  0  0  0  0  0  | 0  1  2  3  4  5  6  7  8  9 10 11
            lsr       ; 2  2  2  2  2  2  2  2  2  2  2  2  | 0  0  1  1  2  2  3  3  4  4  5  5
            bcs @2    ; 2  3  2  3  2  3  2  3  2  3  2  3  | 0  0  1  1  2  2  3  3  4  4  5  5
    @2:     lsr       ; 2  2  2  2  2  2  2  2  2  2  2  2  | 0  0  0  0  1  1  1  1  2  2  2  2
            bcc @5    ; 3  3  2  2  3  3  2  2  3  3  2  2  | 0  0  0  0  1  1  1  1  2  2  2  2
            bcs @5    ; -  -  3  3  -  -  3  3  -  -  3  3  | -  -  0  0  -  -  1  1  -  -  2  2 ;bpl works too
    @5:     beq @10   ; 3  3  3  3  2  2  2  2  2  2  2  2  | 0  0  0  0  1  1  1  1  2  2  2  2
            lsr       ; -  -  -  -  2  2  2  2  2  2  2  2  | -  -  -  -  0  0  0  0  1  1  1  1
            bcs @10   ; -  -  -  -  3  3  3  3  2  2  2  2  | -  -  -  -  0  0  0  0  1  1  1  1 ;beq works too
            delay_n 5 ; -  -  -  -  -  -  -  -  5  5  5  5  | -  -  -  -  -  -  -  -  1  1  1  1
    @10:      ;Total:  12 13 14 15 16 17 18 19 20 21 22 23
    

For delay_n 5, anything that causes 5 cycles of delay works. Examples: `inc $00`, `nop` \+ `cmp $C5`

### 15—270 cycles of delay: delay=A+15; 0 ≤ A ≤ 255)

This code peels slices of 5 cycles with a SBC-BCS loop, and then executes the delay code for A=251—255. The same code will appear later as a function version (which adds 12 cycles overhead due to JSR+RTS cost). 
    
    
            sec     
    @L:     sbc #5  
            bcs @L  ;  6 6 6 6 6  FB FC FD FE FF
            adc #3  ;  2 2 2 2 2  FE FF 00 01 02
            bcc @4  ;  3 3 2 2 2  FE FF 00 01 02
            lsr     ;  - - 2 2 2  -- -- 00 00 01
            beq @5  ;  - - 3 3 2  -- -- 00 00 01
    @4:     lsr     ;  2 2 - - 2  7F 7F -- -- 00
    @5:     bcs @6  ;  2 3 2 3 2  7F 7F 00 00 00
    @6:

### 16—271 cycles of delay: delay=A+16; 0 ≤ A ≤ 255)

This code peels slices of 9 cycles with a CMP-BCC-SBC-BCS loop, and then executes the delay code for A=0—8. 
    
    
    @L:     cmp #9          ;2
            bcc @0          ;2 (+1)
            sbc #9          ;2
            bcs @L          ;3
    ;      Cycles | A | 5  5  5  5  5  5  5  5  5  | 0  1  2  3  4  5  6  7  8
    @0:     lsr       ; 2  2  2  2  2  2  2  2  2  | 0  0  1  1  2  2  3  3  4
            bcs @3    ; 2  3  2  3  2  3  2  3  2  | 0 c0  1 c1  2 c2  3 c3  4
            adc #255  ; 2  -  2  -  2  -  2  -  2  |-1  - c0  - c1  - c2  - c3
    @3:     beq @7    ; 2  3  3  2  2  2  2  2  2  |-1 c0 c0 c1 c1 c2 c2 c3 c3
            bcc @9    ; 3  -  -  2  2  2  2  2  2  |-1  -  - c1 c1 c2 c2 c3 c3
            lsr       ; -  -  -  2  2  2  2  2  2  | -  -  - c0 c0  1  1 c1 c1
            beq @9    ; -  -  -  3  3  2  2  2  2  | -  -  - c0 c0  1  1 c1 c1
    @7:     bcc @9    ; -  2  2  -  -  3  3  2  2  | - c0 c0  -  -  1  1 c1 c1
            bne @9    ; -  2  2  -  -  -  -  3  3  | - c0 c0  -  -  -  - c1 c1
    @9:       ;Total:  16 17 18 19 20 21 22 23 24

### 5—65285 cycles of delay: delay = 256×X + 5

Clobbers A: 
    
    
    @0:     txa       ;2
            beq @10   ;3
            nop       ;2
            tya       ;2
             ldy #48  ;2
    @l:      dey      ;2×48
             bne @l   ;3×48
            tay       ;2−1
            dex       ;2
            jmp @0    ;3
    @10:

Doesn’t clobber A (2 bytes longer): 
    
    
    @0:     cpx #0    ;2
            beq @10   ;3
            pha       ;3
            tya       ;2
             ldy #47  ;2
    @l:      dey      ;2×47
             bne @l   ;3×47
            tay       ;2−1
            pla       ;4
            jmp @0    ;3
    @10:

### 18—218103813 cycles of delay: delay = 13×(65536×Y + 256×A + X) + 18
    
    
            iny
    @l1:    nop
            nop
    @l2:    cpx #1
            dex
            sbc #0
            bcs @l1
            dey
            bne @l2
            rts

## Callable functions

### A + 25 cycles of delay, clobbers A, Z&N, C, V

This code peels slices of 7 cycles with a CMP-BCS-SBC loop, and then executes the delay code for 9—16 cycles with A = 0—6. The reason its overhead is smaller than in the version that peels 5 cycles is because the case for A<7 executes only two instructions instead of three. This comes at the cost that the entry point is not the first instruction. Therefore the code can only exist as a callable function and not inline code. 
    
    
    ;;;;;;;;;;;;;;;;;;;;;;;;
    ; Delays A clocks + overhead
    ; Clobbers A. Preserves X,Y.
    ; Time: A+25 clocks (including JSR)
    ;;;;;;;;;;;;;;;;;;;;;;;;
                      ;       Cycles              Accumulator         Carry flag
                      ; 0  1  2  3  4  5  6          (hex)           0 1 2 3 4 5 6
                      ;
                      ; 6  6  6  6  6  6  6   00 01 02 03 04 05 06
    :      sbc #7     ; carry set by CMP
    delay_a_25_clocks:
           cmp #7     ; 2  2  2  2  2  2  2   00 01 02 03 04 05 06   0 0 0 0 0 0 0
           bcs :-     ; 2  2  2  2  2  2  2   00 01 02 03 04 05 06   0 0 0 0 0 0 0
           lsr        ; 2  2  2  2  2  2  2   00 00 01 01 02 02 03   0 1 0 1 0 1 0
           bcs *+2    ; 2  3  2  3  2  3  2   00 00 01 01 02 02 03   0 1 0 1 0 1 0
           beq :+     ; 3  3  2  2  2  2  2   00 00 01 01 02 02 03   0 1 0 1 0 1 0
           lsr        ;       2  2  2  2  2         00 00 01 01 01       1 1 0 0 1
           beq @rts   ;       3  3  2  2  2         00 00 01 01 01       1 1 0 0 1
           bcc @rts   ;             3  3  2               01 01 01           0 0 1
    :      bne @rts   ; 2  2              3   00 00             01   0 1         0
    @rts:  rts        ; 6  6  6  6  6  6  6   00 00 00 00 01 01 01   0 1 1 1 0 0 1
    ; Total cycles:    25 26 27 28 29 30 31

### A + 27 cycles of delay, clobbers A, Z&N, C, V

This function has longer overhead than delay_a_25_clocks, but it can be appended into other functions, as the execution begins from the first instruction. 
    
    
    ;;;;;;;;;;;;;;;;;;;;;;;;
    ; Delays A clocks + overhead
    ; Clobbers A. Preserves X,Y.
    ; Time: A+27 clocks (including JSR)
    ;;;;;;;;;;;;;;;;;;;;;;;;
    delay_a_27_clocks:
            sec     
    @L:     sbc #5  
            bcs @L  ;  6 6 6 6 6  FB FC FD FE FF
            adc #3  ;  2 2 2 2 2  FE FF 00 01 02
            bcc @4  ;  3 3 2 2 2  FE FF 00 01 02
            lsr     ;  - - 2 2 2  -- -- 00 00 01
            beq @5  ;  - - 3 3 2  -- -- 00 00 01
    @4:     lsr     ;  2 2 - - 2  7F 7F -- -- 00
    @5:     bcs @6  ;  2 3 2 3 2  7F 7F 00 00 00
    @6:     rts     ;

It is created by wrapping the code for 15—270 cycles of delay into a function. The JSR+RTS instructions adds 12 cycles of overhead. 

### 256×A + X + 33 cycles of delay, clobbers A, Z&N, C, V
    
    
    ;;;;;;;;;;;;;;;;;;;;;;;;
    ; Delays A:X clocks+overhead
    ; Time: 256*A+X+33 clocks (including JSR)
    ; Clobbers A. Preserves X,Y. Has relocations.
    ;;;;;;;;;;;;;;;;;;;;;;;;
    :	; 5 cycles done, do 256-5 more.
    	sbc #1			; 2 cycles - Carry was set from cmp
    	pha                     ; 3
    	 lda #(256-27 - 16)     ; 2
    	 jsr delay_a_27_clocks  ; 240
    	pla                     ; 4
    delay_256a_x_33_clocks:
    	cmp #1			; +2
    	bcs :-			; +3 
    	; 0-255 cycles remain, overhead = 4
    	txa 			; -1+2; 6; +27 = 33
    	;passthru
    <<Place the function delay_a_27_clocks immediately following here>>

Can be trivially changed to swap X, Y. 

If you can clobber Y, change the part that begins with "pha" and ends with "pla" into this, for 1 byte shorter code: 
    
    
    	ldy #49  ; 2
    @l:     dey      ; 49*2
            bne @l   ; 49*3
            ldy $A4  ; 3-1

### 256×A + 16 cycles of delay, clobbers A, Z&N, C, V
    
    
    ;;;;;;;;;;;;;;;;;;;;;;;;
    ; Delays A*256 clocks + overhead
    ; Clobbers A. Preserves X,Y.
    ; Time: A*256+16 clocks (including JSR)
    ;;;;;;;;;;;;;;;;;;;;;;;;
    delay_256a_16_clocks:
    	cmp #0
    	bne delay_256a_11_clocks_
    	rts
    delay_256a_11_clocks_:
    	;5 cycles done. Must consume 256 cycles; 251 cycles remain.
            pha                      ;3 - 3
            tya                      ;2 - 5
             ldy #46                 ;2 - 7
    @l:      dey                     ;2*46 - 99
             bne @l                  ;3*46 - 237
             nop                     ;2 - 238
            tay                      ;2 - 240
            pla                      ;4 - 244
    	sec                      ;2 - 246
    	sbc #1                   ;2 - 248
    	jmp delay_256a_16_clocks ;3 - 251

If you can clobber Y, change the part that begins with pha and ends in pla, into this, for shorter code: 
    
    
    	ldy #48  ; 2
    @l:     dey      ; 49*2
            bne @l   ; 49*3
            ldy $A4  ; 3-1

### 256×X + 16 cycles of delay, clobbers X, Z&N
    
    
    ;;;;;;;;;;;;;;;;;;;;;;;;
    ; Delays X*256 clocks + overhead
    ; Clobbers X,Y. Preserves A. Relocatable.
    ; Time: X*256+16 clocks (including JSR)
    ;;;;;;;;;;;;;;;;;;;;;;;;
    delay_256x_16_clocks:
    	cpx #0
    	bne delay_256x_11_clocks_
    	rts
    delay_256x_11_clocks_:
    	;5 cycles done. Must consume 256 cycles; 251 cycles remain.
            pha                      ;3
            tya                      ;2
             ldy #46                 ;2
    @l:      dey                     ;2*46
             bne @l                  ;3*46
             nop                     ;2-1
             nop                     ;2
            tay                      ;2
            pla                      ;4
    	dex                      ;2
    	jmp delay_256x_16_clocks ;3

Can be trivially changed to swap X, Y. 

If you can clobber Y, change the part that begins with pha and ends in pla, into this, for shorter code: 
    
    
            ldy #50                  ;2-1
    @l:	dey                      ;2*50
    	bne @l                   ;3*50

### 256×X + A + 30 cycles of delay, clobbers A, X, Z&N, C, V
    
    
    ;;;;;;;;;;;;;;;;;;;;;;;;
    ; Delays X*256+A clocks + overhead
    ; Clobbers A,X. Preserves Y.
    ; Depends on delay_a_25_clocks within short branch distance
    ; Time: X*256+A+30 clocks (including JSR)
    ;;;;;;;;;;;;;;;;;;;;;;;;
    delay_256x_a_30_clocks:
            cpx #0                  ;2
            beq delay_a_25_clocks   ;3
            ;4 cycles done. Must consume 256 cycles; 252 cycles remain.
            pha                             ;3
             lda #(256-4-(3+2+4+2+3))-25    ;2
             jsr delay_a_25_clocks          ;238
            pla                             ;4
            dex                             ;2
            jmp delay_256x_a_30_clocks      ;3

Can be trivially changed to swap X, Y. 

Alternative version that does not depend on other delay functions, but has otherwise the same implications: 
    
    
    :      sbc #7    ; carry set by CMP
    delay_256x_a_30_clocks_b:
           cmp #7    ; 2  2  2  2  2  2  2   00 01 02 03 04 05 06   0 0 0 0 0 0 0
           bcs :-    ; 2  2  2  2  2  2  2   00 01 02 03 04 05 06   0 0 0 0 0 0 0
           lsr       ; 2  2  2  2  2  2  2   00 00 01 01 02 02 03   0 1 0 1 0 1 0
           bcs @2    ; 2  3  2  3  2  3  2   00 00 01 01 02 02 03   0 1 0 1 0 1 0
    @2:    beq @6    ; 3  3  2  2  2  2  2   00 00 01 01 02 02 03   0 1 0 1 0 1 0
           lsr       ;       2  2  2  2  2         00 00 01 01 01       1 1 0 0 1
           beq @do_x ;       3  3  2  2  2         00 00 01 01 01       1 1 0 0 1
           bcc @do_x ;             3  3  2               01 01 01           0 0 1
    @6:    bne @do_x ; 2  2              3   00 00             01   0 1         0
    @do_x: txa       ;2
           beq @rts  ;3
           ;4 cycles done. Must consume 256 cycles; 252 cycles remain.
           nop       ;2
           tya       ;2
            ldy #48  ;2
    @l:     dey      ;2*48
            bne @l   ;3*48
           tay       ;2-1
           dex       ;2
           jmp @do_x ;3
    @rts:  rts

This function is constructed by concatenating delay_a_25_clocks and the inline delay code for 5—65285 cycles. 

### 851968×Y + 3328×A + 13×X + 30 cycles of delay, clobbers A, X, Y, Z&N, C, V
    
    
    ;;;;;;;;;;;;;;;;;;;;;;;;
    ; Delays 30+13*(65536*Y+256*A+X) cycles including JSR.
    ; Clobbers A,X,Y.
    delay_851968y_3328a_13x_30_clocks:
            iny
    @l1:    nop
            nop
    @l2:    cpx #1
            dex
            sbc #0
            bcs @l1
            dey
            bne @l2
            rts

This is constructed by wrapping the 18—218103813 cycles inline delay code in a function. 

## See also

  * [Cycle counting](Cycle_counting.xhtml "Cycle counting")
  * [Fixed cycle delay](Fixed_cycle_delay.xhtml "Fixed cycle delay")


