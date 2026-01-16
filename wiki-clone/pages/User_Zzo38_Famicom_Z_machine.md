# User:Zzo38/Famicom Z-machine

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Famicom_Z-machine) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Famicom_Z_machine)

This file contains a copy of the working in progress for the Famicom Z-machine interpreter program. 

You are free to review it, question/comment, and even to modify it if you have improvements to make. It is placed here mainly in order to improve reviewing of the software, but you can use it for other purposes too. 

The assembler in use is Unofficial MagicKit (a modified version of NESASM). 

This program is being written by [User:Zzo38](User_Zzo38.xhtml "User:Zzo38"), and is using the Famicom keyboard. It is not yet complete (and likely contains errors). 

## Contents

  * 1 Main file
  * 2 C program
  * 3 Explanation



## Main file
    
    
    ; Famizork II
    ; Public domain
    
    debug	= 1  ; change this to 1 to enable breakpoints 0 to disable
    	     ; set a breakpoint on opcode $1A in the debugger
    
    	inesmap 380 ; Famizork II mapper
    	ineschr 1 ; 8K CHR ROM
    	inesmir 3 ; horizontal arrangement with battery
    
    ; Zero-page variables:
    ;   $02 = data stack pointer
    ;   $03 = call stack pointer
    ;   $04 = temporary
    ;   $05 = temporary
    ;   $06 = temporary
    ;   $07 = temporary
    ;   $09 = current temporary shift state
    ;   $0A = current permanent shift state
    ;   $0B = saved permanent shift state
    ;   $0D = number of locals
    ;   $0E = bit16 of program counter
    ;   $10 = bit7-bit0 of program counter
    ;   $11 = low byte of first operand
    ;   $12 = low byte of second operand
    ;   $13 = low byte of third operand
    ;   $14 = low byte of fourth operand
    ;   $15 = temporary
    ;   $16 = low byte of text address if inside fword
    ;   $17 = low byte of packed word
    ;   $18 = temporary
    ;   $19 = low byte of packed word if inside fword
    ;   $20 = bit15-bit8 of program counter
    ;   $21 = high byte of first operand
    ;   $22 = high byte of second operand
    ;   $23 = high byte of third operand
    ;   $24 = high byte of fourth operand
    ;   $25 = temporary
    ;   $26 = high byte of text address if inside fword
    ;   $27 = high byte of packed word
    ;   $28 = temporary
    ;   $29 = high byte of packed word if inside fword
    ;   $30 = output buffer pointer
    ;   $31 = low byte of nametable address of cursor
    ;   $32 = high byte of nametable address of cursor
    ;   $33 = Y scroll amount
    ;   $34 = lines to output before <MORE>
    ;   $35 = saved high byte of return address for text unpacking
    ;   $36 = bit16 of current text address
    ;   $37 = bit16 of current text address if inside fword
    ;   $38-$39 = return address for text unpacking
    ;   $3A = current background color
    ;   $3B = current foreground color
    ;   $3C = remember if battery RAM is present (255=yes 0=no)
    ;   $3D = ARCFOUR "i" register 
    ;   $3E = ARCFOUR "j" register
    ;   $40-$4F = low byte of locals
    ;   $50-$5F = high byte of locals
    ;   $E2-$FF = output buffer
    
    	code
    
    datasp	= $02
    callsp	= $03
    locall	= $40
    localh	= $50
    
    dstackl	= $200
    dstackh	= $300
    
    cstackl	= $400
    cstackm	= $480
    cstackh	= $500 ; bit4-bit1=number of locals, bit0=bit16 of PC
    cstackx	= $580 ; data stack pointer
    
    arcfour	= $600 ; use for random number generator
    
    	bank intbank+0,"Interpreter"
    	bank intbank+1,"Interpreter"
    	bank intbank+2,"Interpreter"
    	bank intbank+3,"Interpreter"
    
    	bank intbank
    	org $8000
    
    	macro breakpoint
    	if debug
    	db $1A ; unofficial NOP
    	endif
    	endm
    
    	macro breakpoint2
    	if debug
    	db $3A ; unofficial NOP
    	endif
    	endm
    
    	macro make_digit_table
    	macset 4,4,0
    	macgoto make_digit_table_0
    	endm
    
    	macro make_digit_table_0
    	db ((\4*\2)/\1)%10
    	macset 4,4,\4+1
    	macset 5,4,\4=\3
    	macgoto make_digit_table_\5
    	endm
    
    	macro make_digit_table_1
    	; Empty macro
    	endm
    
    globodd	= global&1
    
    	macro make_global_table
    	macset 2,4,16
    	macgoto make_global_table_0
    	endm
    
    	macro make_global_table_0
    	db \1(global+\2+\2-32)
    	macset 2,4,\2+1
    	macset 3,4,\2=256
    	macgoto make_global_table_\3
    	endm
    
    	macro make_global_table_1
    	; Empty macro
    	endm
    
    	macro make_object_table
    	macset 2,4,0
    	macgoto make_object_table_0
    	endm
    
    	macro make_object_table_0
    	db \1(object+(\2*9)+62-9)
    	macset 2,4,\2+1
    	macset 3,4,\2=256
    	macgoto make_object_table_\3
    	endm
    
    	macro make_object_table_1
    	; Empty macro
    	endm
    
    instadl	ds 256
    instadh	ds 256
    
    globadl	ds 16
    	make_global_table low
    globadh	ds 16
    	make_global_table high
    
    objadl	make_object_table low
    objadh	make_object_table high
    
    multabl	ds 256 ; x*x/4
    multabh	ds 512 ; x*x/1024
    
    digit0l	make_digit_table 1,1,256
    digit1l	make_digit_table 10,1,256
    digit2l	make_digit_table 100,1,256
    digit0h	make_digit_table 1,256,128
    digit1h	make_digit_table 10,256,128
    digit2h	make_digit_table 100,256,128
    digit3h	make_digit_table 1000,256,128
    
    bit1tab	db   0,  1,  3,  3,  7,  7,  7,  7, 15, 15, 15, 15, 15, 15, 15, 15
    	db  31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31
    	db  63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63
    	db  63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63
    	db 127,127,127,127,127,127,127,127,127,127,127,127,127,127,127,127
    	db 127,127,127,127,127,127,127,127,127,127,127,127,127,127,127,127
    	db 127,127,127,127,127,127,127,127,127,127,127,127,127,127,127,127
    	db 127,127,127,127,127,127,127,127,127,127,127,127,127,127,127,127
    	db 255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
    	db 255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
    	db 255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
    	db 255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
    	db 255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
    	db 255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
    	db 255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
    	db 255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
    
    zchad	ds 256
    
    ptsizt	db 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1
    	db 2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2
    	db 3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3
    	db 4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4
    	db 5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5
    	db 6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6
    	db 7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7
    	db 8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8
    
    flagad	if smalend
    	db 1,1,1,1,1,1,1,1
    	db 0,0,0,0,0,0,0,0
    	db 3,3,3,3,3,3,3,3
    	db 2,2,2,2,2,2,2,2
    	else
    	db 0,0,0,0,0,0,0,0
    	db 1,1,1,1,1,1,1,1
    	db 2,2,2,2,2,2,2,2
    	db 3,3,3,3,3,3,3,3
    	endif
    
    fwordsl	= *-32
    	ds 96
    fwordsh	= *-32
    	ds 96
    
    flagbit	db 128,64,32,16,8,4,2,1
    	db 128,64,32,16,8,4,2,1
    	db 128,64,32,16,8,4,2,1
    	db 128,64,32,16,8,4,2,1
    
    flagbic	db 127,191,223,239,247,251,253,254
    	db 127,191,223,239,247,251,253,254
    	db 127,191,223,239,247,251,253,254
    	db 127,191,223,239,247,251,253,254
    
    digit4h	make_digit_table 10000,256,128
    
    	; Z-character-decoding assigning macro
    	macro def_zchars
    	if \#=1
    	macset 2,4,\1
    	else
    	macset 2,4,\2
    	endif
    	macset 1,4,\1
    	macset 3,4,*
    	macset 4,4,?B
    	bank bank(zchad)
    	macgoto def_zchars_0
    	endm
    
    	macro def_zchars_0
    	macset 5,4,\1=\2
    	org zchad+\1
    	db low(\3-1)
    	if \3<$FE01
    	fail "Z-character routine out of range"
    	endif
    	if \3>$FF00
    	fail "Z-character routine out of range"
    	endif
    	macset 1,4,\1+1
    	macgoto def_zchars_\5
    	endm
    
    	macro def_zchars_1
    	bank \4
    	org \3
    	endm
    
    	; Instruction assigning macro
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
    
    	macro def_inst_2op
    	def_inst (\1)+$00
    	def_inst (\1)+$20
    	def_inst (\1)+$40
    	def_inst (\1)+$60
    	def_inst (\1)+$C0
    	endm
    
    	macro def_inst_2op_eq
    	def_inst (\1)+$00
    	def_inst (\1)+$20
    	def_inst (\1)+$40
    	def_inst (\1)+$60
    	endm
    
    	macro def_inst_1op
    	def_inst (\1)+$00
    	def_inst (\1)+$10
    	def_inst (\1)+$20
    	endm
    
    	macro def_inst_0op
    	def_inst (\1)+$00
    	endm
    
    	macro def_inst_ext
    	def_inst (\1)+$00
    	endm
    
    	; Fetch next byte of program
    	; Doesn't affect carry flag and overflow flag
    	macro fetch_pc
    	inc $1010
    	bne n\@
    	inc $1020
    	if large
    	bne n\@
    	inc <$0E
    n\@	ld\1 <$0E
    	\2 $5803,\1
    	else
    n\@	\2 $5803
    	endif
    	endm
    	; (Bytes of above: 17)
    	; (Cycles of above: 16 or 25 or 27)
    
    	; Initialization code
    reset	ldx #0
    	stx $2000
    	stx $2001
    	; Wait for frame
    	bit $2002
    vwait1	bit $2002
    	bpl vwait1
    	txa
    	stx <$0E ; bit16 of program counter
    	stx <$0D ; number of locals
    	stx <$33 ; Y scroll amount
    	stx <$3C ; battery flag
    	dex
    	stx <$03 ; call stack pointer
    	ldy #27
    	sty <$34 ; lines before <MORE>
    	ldy #$0F
    	sty <$3A ; background
    	ldy #$20
    	sty <$3B ; foreground
    	ldy #low(start-1)
    	sty <$10
    	ldy #$E2
    	sty <$30 ; output buffer pointer
    	ldy #$61
    	sty <$31 ; low byte of cursor nametable address
    	ldy #$27
    	sty <$32 ; high byte of cursor nametable address
    	; Wait for frame
    	bit $2002
    vwait2	bit $2002
    	bpl vwait2
    	; Clear the screen
    	tax
    	lda #32
    	sta $2006
    	ldx #9
    	stx $2006
    reset1	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	inx
    	bne reset1
    	; Initialize palette
    	lda #$FF
    	sta $2006
    	stx $2006
    	lda <$3A
    	sta $2007
    	sta $2007
    	ldy <$3B
    	sty $2007
    	sty $2007
    	sta $2007
    	sta $2007
    	sty $2007
    	sty $2007
    	sta $2007
    	sta $2007
    	sty $2007
    	sty $2007
    	sta $2007
    	sta $2007
    	sty $2007
    	sty $2007
    	; Check if F8 is pushed (erases save data)
    	ldx #5
    	stx $4016
    	dex
    	stx $4016
    	lda $4017
    	and #2
    	beq reset2
    	; Check battery
    	ldx #0
    	stx $1011
    	stx $1021
    	lda $5800
    	cmp #69
    	bne reset2
    	inc $1011
    	lda $5800
    	cmp #105
    	beq reset3
    	; No save file exists; try to create one
    reset2	stx $1011
    	lda #69
    	sta $5800
    	inc $1011
    	lda #105
    	sta $5800
    	inc $1011
    	stx $5800
    	lda #$FF
    	sta $1022
    	; Initialize ARCFOUR table
    reset2a	txa
    	sta arcfour,x
    	sta $1012
    	sta $5800
    	inx
    	bne reset2a
    	; Copy header from ROM into RAM
    	stx $1021
    reset2b	stx $1011
    	lda $5805
    	sta $5803
    	inx
    	bne reset2b
    	; Copy ROM starting from PURBOT into RAM
    	lda #high(purbot)
    	sta $1021
    	lda #low(purbot)
    	sta $1011
    reset2c	lda $5805
    	sta $5803
    	inc $1011
    	bne reset2c
    	inc $1021
    	if large=0
    	if maxaddr<$FF00
    	lda <$21
    	cmp #high(maxaddr)+1
    	endif
    	endif
    	bne reset2c
    	; Check if save file still exists
    	stx $1011
    	stx $1021
    	lda $5800
    	cmp #69
    	bne zrest
    	inc $1011
    	lda $5800
    	cmp #105
    	beq reset3
    	jmp zrest
    	; Battery is OK
    reset3	lda #255
    	sta <$3C
    	; Load and permute saved ARCFOUR table
    	sta $1021
    	ldy #0
    reset3a	sty $1011
    	lax $5800
    	sta arcfour,y
    	inx
    	stx $5800
    	iny
    	bne reset3a
    	; fall through
    
    	; *** RESTART
    	def_inst_0op 183
    zrest	ldx #0
    	stx <$0E ; bit16 of program counter
    	stx <$0D ; number of locals
    	stx $1021
    	dex
    	stx <$03 ; call stack pointer
    	; Load data from 64 to PURBOT from ROM into RAM
    	lda #64
    	sta $1011
    zrest1	lda $5805
    	sta $5803
    	inc $1011
    	bne zrest1
    	inc $1021
    	if purbot<$FF00
    	lda <$21
    	cmp #high(purbot)+1
    	endif
    	bne zrest1
    	; Initialize program counter
    	lda #low(start-1)
    	sta <$10
    	lda #high(start-1)
    	sta $1020
    	jmp zcrlf
    
    	; *** USL
    	def_inst_0op 188
    	; fall through
    
    	; *** SPLIT
    	def_inst_ext 234
    	; fall through
    
    	; *** SCREEN
    	def_inst_ext 235
    	; fall through
    
    	; *** NOOP
    	def_inst_0op 180
    	; fall through
    
    	; Decode the next instruction
    	; For EXT instructions, number of operands is in the X register
    nxtinst	fetch_pc y,ldx
    	lda instadh,x
    	pha
    	lda instadl,x
    	pha
    	txa
    	bmi not2op
    
    	; It is 2OP
    	ldx #0
    	asl a
    	sta <4
    	arr #$C0
    	fetch_pc y,lda
    	bcc is2op1
    	jsr varop0
    	fetch_pc y,lda
    	bvc is2op2
    	jmp is2op3
    is2op1	stx <$21
    	sta <$11
    	bit <4
    	fetch_pc y,lda
    	bvc is2op3
    is2op2	inx
    	jmp varop0
    is2op3	stx <$22
    	sta <$12
    	rts
    
    	; It isn't 2OP
    not2op	cmp #192
    	bcc notext
    
    	; It is EXT
    	fetch_pc y,lda
    	ldx #0
    isext0	sec
    	rol a
    	bcs isext1
    	bmi isext3
    
    	; Long immediate
    	sta <4
    	fetch_pc y,lda
    	if smalend
    	sta <$11,x
    	else
    	sta <$21,x
    	endif
    	fetch_pc y,lda
    	if smalend
    	sta <$21,x
    	else
    	sta <$11,x
    	endif
    	inx
    	lda <4
    	sec
    	rol a
    	jmp isext0
    
    	; Variable or no more operands
    isext1	bpl isext2
    
    	; No more operands
    	rts
    
    	; Variable
    isext2	sta <4
    	jsr varop
    	inx
    	lda <4
    	sec
    	rol a
    	jmp isext0
    
    	; Short immediate
    isext3	sta <4
    	lda #0
    	sta <$21,x
    	fetch_pc y,lda
    	sta <$11,x
    	inx
    	lda <4
    	sec
    	rol a
    	jmp isext0
    
    	; It isn't EXT; it is 1OP or 0OP
    notext	asl a
    	asl a
    	asl a
    	bcs notext1
    	bpl notext2
    
    	; 1OP - short immediate
    	fetch_pc y,lda
    	ldx #0
    	stx <$21
    	sta <$11
    	rts
    
    notext1	bmi notext3
    
    	; 1OP - variable
    	ldx #0
    	jmp varop
    
    	; 1OP - long immediate
    notext2	fetch_pc y,lda
    	if smalend
    	sta <$11,x
    	else
    	sta <$21,x
    	endif
    	fetch_pc y,lda
    	if smalend
    	sta <$21,x
    	else
    	sta <$11,x
    	endif
    	; fall through
    
    	; 0OP
    notext3	rts
    
    zcall0	jmp val8
    
    	; *** CALL
    	def_inst_ext 224
    	stx <4
    	lax <$11
    	ora <$21
    	beq zcall0 ; calling function zero
    	; Save to call stack
    	inc <callsp
    	ldy <callsp
    	lda <$10
    	stx <$10
    	sta cstackl,y
    	lda <$20
    	sta cstackm,y
    	lsr <$0E
    	lax <$0D
    	rol a
    	sta cstackh,y
    	lda <datasp
    	sta cstackx,y
    	; Save locals
    	txa
    	beq zcall2
    	clc
    	adc <datasp
    	tay
    zcall1	lda <locall,x
    	sta dstackl,y
    	lda <localh,x
    	sta dstackh,y
    	dey
    	dex
    	bne zcall1
    	lda <$0D
    	adc <datasp
    	sta <datasp
    	; Read function header (number of locals)
    zcall2	asl $1010
    	lda <$21
    	rol a
    	sta $1020
    	rol <$0E
    	ldy <$0E
    	lda $5803,y
    	sta <$0D
    	; Load initial values of locals
    	beq zcall4
    	; Load arguments
    	ldx <4
    	dex
    	beq zcall3
    	lda <$12
    	sta <$41
    	lda <$22
    	sta <$51
    	cpx #1
    	beq zcall2a
    	lda <$13
    	sta <$42
    	lda <$23
    	sta <$52
    	cpx #2
    	beq zcall2a
    	lda <$14
    	sta <$43
    	lda <$24
    	sta <$53
    zcall2a	txa
    	asl a ; now clears carry flag
    	adc <$10
    	sta <$10
    	lda #0
    	adc <$20
    	sta $1020
    	if large
    	bcc zcall3
    	inc <$0E
    	endif
    	; Load default values
    zcall3	fetch_pc y,lda
    	if smalend
    	sta <locall+1,x
    	else
    	sta <localh+1,x
    	endif
    	fetch_pc y,lda
    	if smalend
    	sta <localh+1,x
    	else
    	sta <locall+1,x
    	endif
    	inx
    	cpx <$0D
    	bne zcall3
    zcall4	jmp nxtinst
    
    	; *** RFALSE
    	def_inst_0op 177
    	lda #0
    	; fall through
    
    	; Return a 8-bit value (from A)
    ret8	pha
    	ldy <callsp
    	dec <callsp
    	lda cstackx,y
    	sta <datasp
    	lda cstackl,y
    	sta <$10
    	lda cstackm,y
    	sta $1020
    	lda cstackh,y
    	lsr a
    	sta <$0D
    	tax
    	rol a
    	anc #1
    	sta <$0E
    	; Restore locals
    	txa
    	beq ret8b
    	adc <datasp
    	tay
    ret8a	lda dstackl,y
    	sta <locall,x
    	lda dstackh,y
    	sta <localh,x
    	dey
    	dex
    	bne ret8a
    ret8b	pla
    	; fall through
    
    	; Value of instruction is 8-bits (from A)
    val8	fetch_pc y,ldx
    	bne val8a
    	; Push to stack
    	inc <datasp
    	ldy <datasp
    	sta dstackl,y
    	txa
    	sta dstackh,y
    	jmp nxtinst
    val8a	cpx #16
    	bcs val8b
    	; Local variable
    	sta <locall,x
    	lda #0
    	sta <localh,x
    	jmp nxtinst
    	; Global variable
    val8b	ldy globadl,x
    	sty $1014
    	ldy globadh,x
    	sty $1024
    	if smalend
    	sta $5801
    	else
    	ldy #0
    	sty $5801
    	endif
    	inc $1014
    	if globodd
    	bne val8c
    	inc $1024
    	endif
    val8c	if smalend
    	lda #0
    	endif
    	sta $5801
    	lda $1020
    	jmp nxtinst
    
    	; Read the variable using as an instruction operand
    	; X is operand number (0-3)
    varop	fetch_pc y,lda
    varop0	bne varop1
    	; Pop from stack
    	ldy <datasp
    	dec <datasp
    	lda dstackl,y
    	sta <$11,x
    	lda dstackh,y
    	sta <$21,x
    	rts
    varop1	cmp #16
    	bcs varop2
    	; Local variable
    	tay
    	lda locall,y
    	sta <$11,x
    	lda localh,y
    	sta <$21,x
    	rts
    	; Global variable
    varop2	tay
    	lda globadl,y
    	sta $1015
    	lda globadh,y
    	sta $1025
    	lda $5801
    	if smalend
    	sta <$11,x
    	else
    	sta <$21,x
    	endif
    	inc $1015
    	if globodd
    	bne varop3
    	inc $1025
    	endif
    varop3	lda $5801
    	if smalend
    	sta <$21,x
    	else
    	sta <$11,x
    	endif
    	lda $1020
    	rts
    
    	; *** RSTACK
    	def_inst_0op 184
    	ldx <datasp
    	lda dstackl,x
    	sta <$14
    	lda dstackh,x
    	jmp ret16
    
    	; *** RETURN
    	def_inst_1op 139
    	lda <$11
    	sta <$14
    	lda <$21
    ret16	sta <$24
    	ldy <callsp
    	dec <callsp
    	lda cstackx,y
    	sta <datasp
    	lda cstackl,y
    	sta <$10
    	lda cstackm,y
    	sta $1020
    	lda cstackh,y
    	lsr a
    	sta <$0D
    	tax
    	rol a
    	anc #1
    	sta <$0E
    	; Restore locals
    	txa
    	beq ret16b
    	adc <datasp
    	tay
    ret16a	lda dstackl,y
    	sta <locall,x
    	lda dstackh,y
    	sta <localh,x
    	dey
    	dex
    	bne ret16a
    ret16b	; fall through
    
    	; Value of instruction is 16-bits (from $x4)
    val16	lda <$14
    	fetch_pc y,ldx
    	bne val16a
    	; Push to stack
    	inc <datasp
    	ldy <datasp
    	sta dstackl,y
    	lda <$24
    	sta dstackh,y
    	jmp nxtinst
    val16a	cpx #16
    	bcs val16b
    	; Local variable
    	sta <locall,x
    	lda <$24
    	sta <localh,x
    	jmp nxtinst
    	; Global variable
    val16b	ldy globadl,x
    	sty $1015
    	ldy globadh,x
    	sty $1025
    	if smalend
    	sta $5801
    	else
    	ldy <$24
    	sty $5801
    	endif
    	inc $1015
    	if globodd
    	bne val16c
    	inc $1025
    	endif
    val16c	if smalend
    	lda <$24
    	endif
    	sta $5801
    	lda $1020
    	jmp nxtinst
    
    	; *** RTRUE
    	def_inst_0op 176
    	lda #1
    	jmp ret8
    
    	; *** EQUAL? (EXT)
    	def_inst_ext 193
    	lda <$11
    	ldy <$21
    	cmp <$12
    	bne zequal1
    	cpy <$22
    	beq tpredic
    zequal1	cpx #2
    	beq fpredic
    	cmp <$13
    	bne zequal2
    	cpy <$23
    	beq tpredic
    zequal2	cpx #3
    	beq fpredic
    	cmp <$14
    	bne fpredic
    	cmp <$24
    	beq tpredic
    	jmp fpredic
    
    	; *** GRTR?
    	def_inst_2op 3
    	lda <$12
    	cmp <$11
    	lda <$22
    	sbc <$21
    	bvc zgrtr1
    	and #128
    	jmp predic1
    zgrtr1	bmi tpredic
    	jmp fpredic
    
    	; *** LESS?
    	def_inst_2op 2
    	lda <$11
    	cmp <$12
    	lda <$21
    	sbc <$22
    	bvc zgrtr1
    	and #128
    	jmp predic1
    
    	; *** EQUAL? (2OP)
    	def_inst_2op_eq 1
    	lda <$11
    	eor <$21
    	bne fpredic
    	lda <$12
    	eor <$22
    	beq predic1
    	jmp fpredic
    
    	; *** ZERO?
    	def_inst_1op 128
    	lda <$11
    	ora <$21
    	beq tpredic
    	; falls through
    
    	; Predicate handling
    fpredic	lda #128
    	jmp predic1
    tpredic	lda #0
    predic1	fetch_pc x,eor
    	tax
    	arr #$C0
    	bcs predic8
    
    	; If it should branch
    	txa
    	bvs predic3
    
    	; Long offset
    	eor #$20
    	anc #$3F
    	adc #$E0
    	if large
    	bpl predic2
    	dec <$0E
    	endif
    predic2	clc
    	adc <$20
    	sta $1020
    	if large
    	bcc predick
    	inc <$0E
    	endif
    predick	fetch_pc y,lax
    	jmp predic4
    
    	; Short offset
    predic3	and #$3F
    	cmp #2
    	bcc predicq
    predic4	sbc #2
    	bcs predic5
    	if large
    	ldy <$20
    	dey
    	sty $1020
    	cpy #255
    	bne predic5
    	lsr <$0E
    	else
    	dec $1020
    	endif
    predic5	sec
    	adc <$10
    	sta <$10
    	bcc predic9
    	inc $1020
    	if large
    	bne predic9
    	inc <$0E
    	endif
    	jmp nxtinst
    
    	; If should not branch
    predic8	bvc predic9
    	inc <$10
    	bne predic9
    	inc $1020
    	if large
    	bne predic9
    	inc <$0E
    	endif
    predic9	jmp nxtinst
    
    predicq	jmp ret8
    
    	; *** IGRTR?
    	def_inst_2op 5
    	ldx <$11
    	jsr xvalue
    	inc <$14
    	bne zigrtr2
    	inc <$24
    zigrtr1	jsr xstore
    	lda <$14
    	cmp <$11
    	lda <$24
    	sbc <$21
    	bvc zigrtr2
    	and #128
    	jmp predic1
    zigrtr2	bmi zigrtr3
    	jmp fpredic
    zigrtr3	jmp tpredic
    
    	; *** DLESS?
    	def_inst_2op 4
    	ldx <$11
    	jsr xvalue
    	ldy <$14
    	dey
    	sty <$14
    	cpy #255
    	bne zdless1
    	dec <$24
    zdless1	jsr xstore
    	lda <$11
    	cmp <$14
    	lda <$21
    	sbc <$24
    	bvc zigrtr2
    	and #128
    	jmp predic1
    
    	; *** PTSIZE
    	def_inst_1op 132
    	lda $1021
    	ora #255
    	dcp $1011
    	bne zptsz1
    	dec $1021
    zptsz1	ldx $5801
    	lda ptsizt,x
    	jmp val8
    
    	; *** PUT
    	def_inst_ext 225
    	lda <$12
    	asl a
    	rol <$22
    	clc
    	adc <$11
    	sta $1011
    	lda <$22
    	adc <$21
    	sta $1021
    	if smalend
    	lda <$13
    	else
    	lda <$23
    	endif
    	sta $5801
    	inc $1011
    	bne zput1
    	inc $1021
    zput1	ds 0
    	if smalend
    	lda <$23
    	else
    	lda <$13
    	endif
    	sta $5801
    	bit $1020
    	jmp nxtinst
    
    	; *** PUTB
    	def_inst_ext 226
    	lda <$12
    	clc
    	adc <$11
    	sta $1011
    	lda <$22
    	adc <$21
    	sta $1021
    	lda <$13
    	sta $5801
    	bit $1020
    	jmp nxtinst
    
    	; *** GET
    	def_inst_2op 15
    	lda <$12
    	asl a
    	rol <$22
    	clc
    	adc <$11
    	sta $1011
    	lda <$22
    	adc <$21
    	sta $1021
    	lda $5801
    	if smalend
    	sta <$14
    	else
    	sta <$24
    	endif
    	inc $1011
    	bne zget1
    	inc $1021
    zget1	ds 0
    	lda $5801
    	if smalend
    	sta <$24
    	else
    	sta <$14
    	endif
    	bit $1020
    	jmp val16
    
    	; *** GETB
    	def_inst_2op 16
    	lda <$12
    	clc
    	adc <$11
    	sta $1011
    	lda <$22
    	adc <$21
    	sta $1021
    	lda $5801
    	bit $1020
    	jmp val8
    
    	; *** ADD
    	def_inst_2op 20
    	clc
    	lda <$11
    	adc <$12
    	sta <$14
    	lda <$21
    	adc <$22
    	sta <$24
    	jmp val16
    
    	; *** SUB
    	def_inst_2op 21
    	sec
    	lda <$11
    	sbc <$12
    	sta <$14
    	lda <$21
    	sbc <$22
    	sta <$24
    	jmp val16
    
    	; *** BAND
    	def_inst_2op 9
    	lda <$11
    	and <$12
    	sta <$14
    	lda <$21
    	and <$22
    	sta <$24
    	jmp val16
    
    	; *** BOR
    	def_inst_2op 8
    	lda <$11
    	ora <$12
    	sta <$14
    	lda <$21
    	ora <$22
    	sta <$24
    	jmp val16
    
    	; *** BCOM
    	def_inst_1op 143
    	lda <$11
    	eor #$FF
    	sta <$14
    	lda <$21
    	eor #$FF
    	sta <$24
    	jmp val16
    
    	; *** BTST
    	def_inst_2op 7
    	lda <$11
    	and <$12
    	eor <$12
    	sta <4
    	lda <$21
    	and <$22
    	eor <$22
    	ora <4
    	bne zbtst1
    	jmp predic1
    zbtst1	jmp fpredic
    
    	; *** MUL
    	def_inst_2op 22
    	lax <$11
    	clc
    	adc <$12
    	bcc zmul1
    	eor #255
    	adc #0
    zmul1	tay
    	txa
    	sec
    	sbc <$12
    	bcs zmul2
    	eor #255
    	adc #1
    	sec
    zmul2	tax
    	lda multabl,y
    	sbc multabl,x
    	sta <$14
    	php
    	lda <$11
    	clc
    	adc <$12
    	tay
    	bcc zmul3
    	lda multabh+256,y
    	jmp zmul4
    zmul3	lda multabh,y
    zmul4	plp
    	sbc multabh,x
    	sta <$24
    	; low*high
    	lax <$11
    	clc
    	adc <$22
    	bcc zmul5
    	eor #255
    	adc #0
    zmul5	tay
    	txa
    	sec
    	sbc <$22
    	bcs zmul6
    	eor #255
    	adc #1
    	sec
    zmul6	tax
    	lda multabl,y
    	sbc multabl,x
    	clc
    	adc <$24
    	sta <$24
    	; high*low
    	lax <$21
    	clc
    	adc <$12
    	bcc zmul7
    	eor #255
    	adc #0
    zmul7	tay
    	txa
    	sec
    	sbc <$12
    	bcs zmul8
    	eor #255
    	adc #1
    	sec
    zmul8	tax
    	lda multabl,y
    	sbc multabl,x
    	clc
    	adc <$24
    	sta <$24
    	jmp val16
    
    	; *** PUSH
    	def_inst_ext 232
    	inc <datasp
    	ldx <datasp
    	lda <$11
    	sta dstackl,x
    	lda <$21
    	sta dstackh,x
    	jmp nxtinst
    
    	; *** POP
    	def_inst_ext 233
    	ldx <datasp
    	dec <datasp
    	lda dstackl,x
    	sta <$12
    	lda dstackh,x
    	sta <$22
    	ldx <$11
    	jsr xstore
    	jmp nxtinst
    
    	; *** FSTACK
    	def_inst_0op 185
    	dec <datasp
    	jmp nxtinst
    
    	; *** SET
    	def_inst_2op 13
    	lda <$12
    	sta <$14
    	lda <$22
    	sta <$24
    	ldx <$11
    	jsr xstore
    	jmp nxtinst
    
    	; *** VALUE
    	def_inst_1op 142
    	ldx <$11
    	jsr xvalue
    	jmp val16
    
    	; *** INC
    	def_inst_1op 133
    	ldx <$11
    	jsr xvalue
    	inc <$14
    	bne zinc1
    	inc <$24
    zinc1	jsr xstore
    	jmp nxtinst
    
    	; *** DEC
    	def_inst_1op 134
    	ldx <$11
    	jsr xvalue
    	ldy <$14
    	dey
    	sty <$14
    	cpy #255
    	bne zinc1
    	dec <$24
    	jsr xstore
    	jmp nxtinst
    
    	; Store value from <$x4 into variable labeled X
    xstore	lda <$14
    	cpx #0
    	bne xstore1
    	; Top of stack
    	ldy <datasp
    	sta dstackl,y
    	lda <$24
    	sta dstackh,y
    	rts
    xstore1	cpx #16
    	bcs xstore2
    	; Local variable
    	sta <locall,x
    	lda <$24
    	sta <localh,x
    	rts
    	; Global variable
    xstore2	ldy globadl,x
    	sty $1014
    	ldy globadh,x
    	sty $1024
    	if smalend
    	sta $5801
    	else
    	ldy <$24
    	sty $5801
    	endif
    	inc $1014
    	if globodd
    	bne xstore3
    	inc $1024
    	endif
    xstore3	if smalend
    	lda <$24
    	endif
    	sta $5801
    	lda $1020
    	rts
    
    	; Read from variable labeled X into <$x4
    xvalue	txa
    	bne xvalue1
    	; Top of stack
    	ldy <datasp
    	lda dstackl,y
    	sta <$14
    	lda dstackh,y
    	sta <$24
    	rts
    xvalue1	cpx #16
    	bcs xvalue2
    	; Local variable
    	lda <locall,x
    	sta <$14
    	lda <localh,x
    	sta <$24
    	rts
    	; Global vaiable
    xvalue2	ldy globadl,x
    	sty $1015
    	ldy globadh,x
    	sty $1025
    	lda $5801
    	if smalend
    	sta <$14
    	else
    	sta <$24
    	endif
    	inc $1015
    	if globodd
    	bne xvalue3
    	inc $1025
    	endif
    xvalue3	lda $5801
    	if smalend
    	sta <$24
    	else
    	sta <$14
    	endif
    	bit $1020
    	rts
    
    	; *** IN?
    	def_inst_2op 6
    	ldx <$11
    	clc
    	lda objadl,x
    	adc #4
    	sta $5010
    	lda objadh,x
    	adc #0
    	sta $5020
    	lda $5801
    	bit $1020
    	eor <$21
    	bne zin1
    	jmp predic1
    zin1	jmp fpredic
    
    	; *** FSET?
    	def_inst_2op 10
    	ldx <$11
    	ldy <$12
    	clc
    	lda objadl,x
    	adc flagad,y
    	sta $5010
    	lda objadh,x
    	adc #0
    	sta $5020
    	lda $5801
    	and flagbit,y
    	bne zfsetp1
    	bit $1020
    	jmp predic1
    zfsetp1	jmp fpredic
    
    	; *** FSET
    	def_inst_2op 11
    	ldx <$11
    	ldy <$12
    	clc
    	lda objadl,x
    	adc flagad,y
    	sta $5010
    	lda objadh,x
    	adc #0
    	sta $5020
    	lda $5801
    	ora flagbit,y
    	sta $5801
    	bit $1020
    	jmp nxtinst
    
    	; *** FCLEAR
    	def_inst_2op 12
    	ldx <$11
    	ldy <$12
    	clc
    	lda objadl,x
    	adc flagad,y
    	sta $5010
    	lda objadh,x
    	adc #0
    	sta $5020
    	lda $5801
    	and flagbic,y
    	sta $5801
    	bit $1020
    	jmp nxtinst
    
    	; *** LOC
    	def_inst_1op 131
    	ldx <$11
    	clc
    	lda objadl,x
    	adc #4
    	sta $5010
    	lda objadh,x
    	adc #0
    	sta $5020
    	lda $5801
    	bit $1020
    	jmp val8
    
    	; *** FIRST?
    	def_inst_1op 130
    	ldx <$11
    	clc
    	lda objadl,x
    	adc #6
    	sta $5010
    	lda objadh,x
    	adc #0
    	sta $5020
    	lda $5801
    	bit $1020
    	jmp valp
    
    	; *** NEXT?
    	def_inst_1op 129
    	ldx <$11
    	clc
    	lda objadl,x
    	adc #5
    	sta $5010
    	lda objadh,x
    	adc #0
    	sta $5020
    	lda $5801
    	bit $1020
    	; fall through
    
    	; Value of instruction is 8-bits (from A)
    	; Predicate is then if value is nonzero
    valp	fetch_pc y,ldx
    	bne valpa
    	; Push to stack
    	inc <datasp
    	ldy <datasp
    	sta dstackl,y
    	sta <4
    	txa
    	sta dstackh,y
    	lda <4
    	jmp valpd1
    valpa	cpx #16
    	bcs valpb
    	; Local variable
    	sta <locall,x
    	ldy #0
    	sty <localh,x
    	jmp valpd
    	; Global variable
    valpb	ldy globadl,x
    	sty $1014
    	ldy globadh,x
    	sty $1024
    	if smalend
    	sta $5801
    	else
    	ldy #0
    	sty $5801
    	endif
    	inc $1014
    	if globodd
    	bne valpc
    	inc $1024
    	endif
    valpc	if smalend
    	ldy #0
    	sty $5801
    	else
    	sta $5801
    	endif
    	bit $1020
    valpd	tax
    valpd1	beq valpe
    	jmp fpredic
    valpe	jmp tpredic
    
    	; Macro to do one step of ARCFOUR
    	; Result is stored in accumulator
    	macro do_arcfour
    	inc <$3D
    	ldx <$3D
    	lda arcfour,x
    	pha
    	clc
    	adc <$3E
    	sta <$3E
    	tay
    	sta arcfour,y
    	pla
    	sta arcfour,x
    	clc
    	adc arcfour,y
    	tax
    	lda arcfour,x
    	endm
    
    	; *** RANDOM
    	def_inst_ext 231
    	ldx <$21
    	beq zrand1
    	lda bit1tab,x
    	sta <$23
    	lda #$FF
    	jmp zrand2
    zrand1	ldx <$11
    	lda bit1tab,x
    zrand2	sta <$13
    zrand3	do_arcfour
    	and <$23
    	sta <$24
    	cmp <$21
    	beq zrand4 ; exactly equal
    	bcs zrand1 ; try again; out of range
    	jmp zrand5 ; low byte doesn't need to check
    zrand4	do_arcfour
    	and <$13
    	cmp <$11
    	bcs zrand1 ; try again; out of range
    	adc #1
    	sta <$14
    	jmp zrand6
    zrand5	do_arcfour
    	sec
    	adc #0
    	sta <$14
    zrand6	lda #0
    	adc <$24
    	sta <$24
    	jmp val16
    
    	; *** JUMP
    	def_inst_1op 140
    	lda <$11
    	sec
    	sbc #2
    	tax
    	lda <$21
    	sbc #0
    	tay
    	bpl zjump1
    	dec <$0E
    zjump1	txa
    	clc
    	adc <$10
    	sta <$10
    	tya
    	adc <$20
    	sta $1020
    	bcc zjump2
    	inc <$0E
    zjump2	jmp nxtinst
    
    	; Macro to find a property, given object and property number
    	; Object in <$11, property in <$12, branch to \1 if found
    	; If \1 is with # at front then assume always will be found
    	; X contains property size only in high 3-bits if found
    	; X contains property number if not found
    	; Output is $1014 and $1024 with address of property id
    	macro propfind
    	; Find the property table
    	ldx <$11
    	clc
    	lda objadl,x
    	adc #7
    	sta $1015
    	lda objadh,x
    	adc #0
    	sta $1025
    	lda $5801
    	if smalend
    	sta <$14
    	else
    	sta <$24
    	endif
    	inc $1015
    	bne n\@a
    	inc $1025
    n\@a	lda $5801
    	if smalend
    	sta $1014
    	bit $1024
    	else
    	sta $1024
    	bit $1014
    	endif
    	; Skip the short description
    	lda $5801
    	sec
    	rol a
    	bcc n\@d
    	inc $1024
    	clc
    n\@d	adc <$14
    	sta $1014
    	bcc n\@b
    	inc $1024
    	; Find this property
    n\@b	lda $5081
    	if '\<1'!='#'
    	beq n\@c
    	endif
    	eor <$12
    	tax
    	and #$1F
    	if '\<1'='#'
    	beq n\@c
    	else
    	beq \1
    	endif
    	lda ptsizt,x
    	sec
    	adc <$14
    	sta $1014
    	bcc n\@b
    	inc $1024
    	jmp n\@b
    n\@c	ds 0
    	endm
    
    	; *** GETPT
    	def_inst_2op 18
    	propfind zgetpt1
    	lda $1020
    	and #0
    	jmp val8
    zgetpt1	lda $1020
    	inc <$14
    	bne zgetpt2
    	inc <$24
    zgetpt2	jmp val16	
    
    	; *** GETP
    	def_inst_2op 17
    	propfind zgetp2
    	; Use default value
    	asl <$11
    	rol <$21 ;clears carry
    	lda #low(object-2)
    	adc <$11
    	sta $1015
    	lda #high(object-2)
    	adc <$21
    	sta $1025
    	lda $5801
    	if smalend
    	sta <$14
    	else
    	sta <$24
    	endif
    	inc $1015
    	if object&1
    	bne zgetp1
    	inc $1025
    	endif
    zgetp1	lda $5801
    	if smalend
    	sta <$24
    	else
    	sta <$14
    	endif
    	bit $1020
    	jmp val16
    	; Use actual value
    zgetp2	inc $1014
    	bne zgetp3
    	inc $1024
    zgetp3	cpx #$20
    	bne zgetp5
    	; Long property
    	lda $5801
    	if smalend
    	sta <$14
    	else
    	sta <$24
    	endif
    	inc $1014
    	bne zgetp4
    	inc $1024
    zgetp4	lda $5801
    	if smalend
    	sta <$14
    	else
    	sta <$24
    	endif
    	jmp val16
    	; Short property
    zgetp5	lda $5801
    	bit $1020
    	jmp val8
    
    	; *** PUTP
    	def_inst_ext 227
    	propfind #
    	inc $1014
    	bne zputp2
    	inc $1024
    zputp2	cpx #$20
    	bne zputp4
    	; Long property
    	if smalend
    	lda <$13
    	else
    	lda <$23
    	endif
    	sta $5801
    	inc $1014
    	bne zputp3
    	inc $1024
    zputp3	if smalend
    	lda <$23
    	else
    	lda <$13
    	endif
    	sta $5801
    	lda $1020
    	jmp nxtinst
    	; Short property
    zputp4	lda <$13
    	sta $5801
    	lda $1020
    	jmp nxtinst
    
    	; *** NEXTP
    	def_inst_2op 19
    	ldx <$11
    	bne znextp4
    	; Find first property
    	clc
    	lda objadl,x
    	adc #7
    	sta $1015
    	lda objadh,x
    	adc #0
    	sta $1025
    	lda $5801
    	if smalend
    	sta <$14
    	else
    	sta <$24
    	endif
    	inc $1015
    	bne znextp1
    	inc $1025
    znextp1	lda $5801
    	if smalend
    	sta $1014
    	bit $1024
    	else
    	sta $1024
    	bit $1014
    	endif
    	; Skip the short description
    	lda $5801
    	sec
    	rol a
    	bcc znextp2
    	inc $1024
    	clc
    znextp2	adc <$14
    	sta $1014
    	bcc znextp3
    	inc $1024
    znextp3	lda $5801
    	and #$1F
    	bit $1020
    	jmp val8
    znextp4	propfind #
    	lda ptsizt,x
    	sec
    	adc <$14
    	sta $1014
    	bcc znextp5
    	inc $1024
    znextp5	lda $5801
    	bit $1020
    	and #$1F
    	jmp val8
    
    	; *** REMOVE
    	def_inst_1op 137
    	lda #0
    	sta <$12
    	; fall through
    
    	; *** MOVE
    	def_inst_2op 14
    	; Find the LOC of first object, see if need to remove
    	ldx <$11
    	clc
    	lda objadl,x
    	adc #4
    	sta $1013
    	lda objadh,x
    	adc #0
    	sta $1023
    	lda $5801
    	ldy <$12
    	sty $5801
    	tay
    	beq zmove2
    	; Look at the NEXT slot too
    	inc $1013
    	bne zmove1
    	inc $1023
    zmove1	ldy $5801
    	ldx #0
    	stx $5801
    	; Find it in the FIRST-NEXT chain of the parent object
    	tax
    	lda objadl,x
    	adc #6
    	sta $1014
    	lda objadh,x
    	adc #0
    	sta $1024
    	lax $5801 ; not adjust carry flag
    	eor <$11
    	bne zmove3
    	; It is the first child object
    	; Let First(Parent)=Next(Child)
    	sty $5801
    	jmp zmove2
    	; It is not the first child object
    zmove3	lda objadl,x
    	adc #5
    	sta $1014
    	lda objadh,x
    	adc #0
    	sta $1024
    	lax $5801
    	eor <$11
    	bne zmove3
    	; It is found
    	sty $5801
    	; Now insert the object into the new container (if nonzero)
    zmove2	ldx <$12
    	beq zmove4
    	lda objadl,x
    	adc #6
    	sta $1014
    	lda objadh,x
    	adc #0
    	sta $1024
    	ldy $5801
    	stx $5801
    	bit $1013
    	bit $1023
    	sty $5801
    zmove4	lda $1020
    	jmp nxtinst
    
    	; Print a space
    space	lda <$30
    	cmp #$E2
    	bne space1
    	lda <$31
    	and #$1F
    	bne space1
    	jsr bufout
    	lda <$31
    	and #$1F
    	bne space2
    space1	inc <$31
    space2	rts
    
    	; Output and clear the buffer
    bufout	lda <$31
    	anc #$1F
    	adc <$30
    	bcc bufout0
    	jsr addlin1
    bufout0	ldx #0
    	lda <$32
    	ldy <$31
    bufout1	bit $2002
    	bpl bufout1
    	stx $2001 ; render off
    	sta $2006
    	sty $2006
    	ldx #$E2
    	cpx <$30
    	beq bufout3
    bufout2	lda <0,x
    	sta $2007
    	inx
    	cpx <$30
    	bne bufout2
    bufout3	tya
    	anc #$1F
    	bne bufout4
    	; Blank the bottom row (just scrolled in)
    	lda <5
    	sta $2006
    	lda <4
    	sta $2006
    	lda #32
    	sta $2007 ;1
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007 ;10
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007 ;20
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007 ;30
    bufout4	lda #$F8
    	sta $2005
    	ldx <$33
    	stx $2005
    	anc #$08
    	sta $2001
    	sta $2000
    	lda <$30
    	sbc #$E1
    	clc
    	adc <$31
    	sta <$31
    	lda <$32
    	adc #0
    	sta <$32
    	lda #$E2
    	sta <$30
    bufout5	rts
    
    	; Skip to the next line
    addline	sec
    addlin1	lda <$33
    	adc #7
    	sta <$33
    	cmp #$F0
    	bcc addlin2
    	anc #0
    	sta <$33
    addlin2	lda <$31
    	and #$E0
    	adc #$20
    	sta <$31
    	lda <$32
    	adc #0
    	sta <$32
    	cmp #$27
    	bne addlin3
    	lda <$31
    	cmp #$C0
    	bne addlin3
    	lda #$24
    	sta <$32
    	lda #0
    	sta <$31
    	; Prepare address to blank out the line
    addlin3	lax <$31
    	clc
    	adc #$40
    	sta <4
    	lda <$32
    	adc #0
    	sta <5
    	cmp #$27
    	bcc addlin4
    	cpx #$80
    	bcc addlin4
    	lda #$24
    	sax <4
    	sta <5
    addlin4	dec <$34
    	bne addlin5
    	lda #27
    	sta <$34
    	jmp more
    addlin5	rts
    
    	; Display the <MORE> prompt
    more	ldx #0
    	lda <$32
    	ldy <$31
    more1	bit $2002
    	bpl more1
    	stx $2001 ; render off
    	sta $2006
    	sty $2006
    	lda #'<'
    	sta $2007
    	lda #'M'
    	sta $2007
    	lda #'O'
    	sta $2007
    	lda #'R'
    	sta $2007
    	lda #'E'
    	sta $2007
    	lda #'>'
    	sta $2007
    	; Blank the bottom row (just scrolled in)
    	lda <5
    	sta $2006
    	lda <4
    	sta $2006
    	lda #32
    	sta $2007 ;1
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007 ;10
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007 ;20
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007 ;30
    	; Re-enable rendering
    	lda #$F8
    	sta $2005
    	ldx <$33
    	stx $2005
    	anc #$08
    	sta $2001
    	sta $2000
    	; Wait for keyboard not pushed
    more2	ldx #5
    	stx $4016
    	dex
    	ldy #9
    more3	stx $4016
    	lda $4017
    	ora #$E1
    	eor #$FF
    	bne more2
    	lda #6
    	sta $4016
    	lda $4017
    	ora #$E1
    	eor #$FF
    	bne more2
    	dey
    	bne more3
    	; Wait for space-bar pushed
    	ldx #5
    	lda #4
    	ldy #6
    more4	stx $4016 ;reset
    	sta $4016 ;0/0
    	sty $4016 ;0/1
    	sta $4016 ;1/0
    	sty $4016 ;1/1
    	sta $4016 ;2/0
    	sty $4016 ;2/1
    	sta $4016 ;3/0
    	sty $4016 ;3/1
    	sta $4016 ;4/0
    	sty $4016 ;4/1
    	sta $4016 ;5/0
    	sty $4016 ;5/1
    	sta $4016 ;6/0
    	sty $4016 ;6/1
    	sta $4016 ;7/0
    	sty $4016 ;7/1
    	sta $4016 ;8/0
    	sty $4016 ;8/1
    	and $4017
    	bne more4
    	; Erase <MORE>
    	lda #0
    	sta $2001
    	lda <$32
    	sta $2006
    	lda <$31
    	sta $2006
    	lda #32
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	sta $2007
    	rts
    
    	; *** PRINTC
    	def_inst_ext 229
    	lda <$11
    	beq zprntc2
    	cmp #32
    	beq zprntc1
    	cmp #13
    	beq zcrlf
    	ldx <$30
    	beq zprntc2
    	sta <0,x
    	inc <$30
    zprntc1	jmp nxtinst
    zprntc2	jsr space
    	jmp nxtinst
    
    	; *** CRLF
    	def_inst_0op 187
    zcrlf	jsr bufout
    	lda <$31
    	ora #$1F
    	sta <$31
    zcrlf2	jmp nxtinst
    
    	; *** PRINTN
    	def_inst_ext 230
    	lda <$30
    	beq zcrlf2 ; ensure there is room in the buffer
    	ldy <$11
    	lax <$21
    	anc #$FF
    	bcc znum01
    	eor #$FF
    	sta <4
    	ldx <$30
    	inc <$30
    	lda #'-'
    	sta <0,x
    	tya
    	eor #$FF
    	tay
    	ldx <4
    znum01	lda digit0l,y
    	adc digit0h,x
    	pha
    	cmp #10
    	lda digit1l,y
    	adc digit1h,x
    	pha
    	cmp #10
    	lda digit2l,y
    	adc digit2h,x
    	pha
    	cmp #10
    	lda #0
    	adc digit3h,x
    	pha
    	cmp #10
    	lda #0
    	adc digit4h,x
    	ldx <$30
    	tay ; make the flag according to accumulator
    	beq znum02
    	; Five digits
    	sta <0,x
    	pla
    	sta 1,x
    	pla
    	sta 2,x
    	pla
    	sta 3,x
    	pla
    	sta 4,x
    	txa
    	axs #-5
    	stx <$30
    	jmp nxtinst
    znum02	pla
    	beq znum03
    	; Four digits
    	sta <0,x
    	pla
    	sta 1,x
    	pla
    	sta 2,x
    	pla
    	sta 3,x
    	txa
    	axs #-4
    	stx <$30
    	jmp nxtinst
    znum03	pla
    	beq znum04
    	; Three digits
    	sta <0,x
    	pla
    	sta 1,x
    	pla
    	sta 2,x
    	txa
    	axs #-3
    	stx <$30
    	jmp nxtinst
    znum04	pla
    	beq znum05
    	; Two digits
    	sta <0,x
    	inx
    	pla
    	sta <0,x
    	inx
    	stx <$30
    	jmp nxtinst
    znum05	pla
    	; One digit
    	sta <0,x
    	inc <$30
    	jmp nxtinst
    
    	; *** PRINTI
    	def_inst_0op 178
    	jsr textpc
    	jmp nxtinst
    
    	; *** PRINTR
    	def_inst_0op 179
    	jsr textpc
    	jsr bufout
    	lda <$31
    	ora #$1F
    	sta <$31
    	lda #1
    	jmp ret8
    
    	; *** PRINTB
    	def_inst_1op 135
    	jsr textba
    	jmp nxtinst
    
    	; *** PRINT
    	def_inst_1op 141
    	asl <$11
    	rol <$21
    	lda #0
    	rol a
    	sta <$36
    	jsr textwa
    	jmp nxtinst
    
    	; *** PRINTD
    	def_inst_1op 138
    	ldx <$11
    	clc
    	lda objadl,x
    	adc #7
    	sta $1012
    	lda objadh,x
    	adc #0
    	sta $1022
    	if smalend
    	lda $5801
    	else
    	ldy $5801
    	endif
    	inc $1012
    	bne zprntd1
    	inc $1022
    zprntd1	if smalend
    	adc #1
    	sta <$11
    	lda $5801
    	else
    	lda $5801
    	adc #1
    	sta <$11
    	tya
    	endif
    	adc #0
    	sta <$21
    	jsr textba
    	jmp nxtinst
    
    	; *** VERIFY
    	def_inst_0op 189
    	jmp tpredic ; there is no disk, so just assume it is OK
    
    	; *** QUIT
    	def_inst_0op 186
    	jsr bufout
    	lda <$31
    	ora #$1F
    	sta <$31
    	jsr bufout
    zquit	jmp zquit
    
    	; *** READ
    	jsr bufout
    	;TODO
    zread	jmp zread
    
    	bank intbank+3
    	; Z-character decoding
    	; high 3-bits = state, low 5-bits = value
    
    	org $F100-12
    	; Text starting from program counter
    textpc	lda #0
    	sta <$38
    	sta <$27
    	ldx #$A0
    	stx <$09
    	stx <$0A
    
    	org $F100
    	lda <$27
    	bmi textpc1
    	lda #$F2
    	sta <$39
    	lda #$FE
    	pha
    	fetch_pc y,lda
    	if smalend
    	sta <$17
    	else
    	sta <$27
    	endif
    	if smalend
    	fetch_pc y,lda
    	sta <$27
    	else
    	fetch_pc y,ldx
    	stx <$17
    	endif
    	lsr a
    	lsr a
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    textpc1	rts
    
    	org $F200
    	lda #$FE
    	pha
    	inc <$39
    	ldx <$17
    	stx <4
    	lda <$27
    	asl <4
    	rol a
    	asl <4
    	rol a
    	asl <4
    	rol a
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    
    	org $F300
    	lda #$F1
    	sta <$39
    	lda #$FE
    	pha
    	lda <$17
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    
    	org $F400-12
    	; Text from byte address
    textba	lda #0
    	sta <$38
    	sta <$27
    	ldx #$A0
    	stx <$09
    	stx <$0A
    
    	org $F400
    	lda <$27
    	bmi textba1
    	lda #$F5
    	sta <$39
    	lda #$FE
    	pha
    	lda $1011
    	lda $1021
    	lda $5803
    	if smalend
    	sta <$17
    	else
    	sta <$27
    	endif
    	inc $1011
    	bne textba2
    	inc $1021
    textba2	if smalend
    	lda $5803
    	sta <$27
    	else
    	ldx $5803
    	stx <$17
    	endif
    	inc $1011
    	bne textba3
    	inc $1021
    textba3	lsr a
    	lsr a
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    textba1	bit $1020
    	rts
    
    	org $F500
    	lda #$FE
    	pha
    	inc <$39
    	ldx <$17
    	stx <4
    	lda <$27
    	asl <4
    	rol a
    	asl <4
    	rol a
    	asl <4
    	rol a
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    
    	org $F600
    	lda #$F4
    	sta <$39
    	lda #$FE
    	pha
    	lda <$17
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    
    	org $F700-12
    	; Text from word address (aligned)
    textwa	lda #0
    	sta <$38
    	sta <$27
    	ldx #$A0
    	stx <$09
    	stx <$0A
    
    	org $F700
    	lda <$27
    	bmi textwa1
    	lda #$F8
    	sta <$39
    	lda #$FE
    	pha
    	lda $1011
    	lda $1021
    	ldy <$36
    	lda $5803,y
    	if smalend
    	sta <$17
    	else
    	sta <$27
    	endif
    	if smalend
    	inc $1011
    	lda $5803,y
    	sta <$27
    	else
    	ldx $5803,y
    	stx <$17
    	endif
    	inc $1011
    	bne textwa4
    	inc $1021
    	bne textwa4
    	inc <$36
    textwa4	lsr a
    	lsr a
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    textwa1	bit $1020
    	rts
    
    	org $F800
    	lda #$FE
    	pha
    	inc <$39
    	ldx <$17
    	stx <4
    	lda <$27
    	asl <4
    	rol a
    	asl <4
    	rol a
    	asl <4
    	rol a
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    
    	org $F900
    	lda #$F7
    	sta <$39
    	lda #$FE
    	pha
    	lda <$17
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    
    	org $FA00-20
    	; Text from frequent word
    textfw	lda #0
    	sta <$38
    	sta <$29
    	lda <$0A
    	sta <$0B
    	ldx #$A0
    	stx <$09
    	stx <$0A
    	lda <$39
    	sta <$35
    
    	org $FA00
    	lda <$29
    	bmi textfw1
    	lda #$FB
    	sta <$39
    	lda #$FE
    	pha
    	ldy <$37
    	lda $5803,y
    	if smalend
    	sta <$19
    	else
    	sta <$29
    	endif
    	inc $1016
    	if smalend
    	lda $5803,y
    	sta <$29
    	else
    	ldx $5803,y
    	stx <$19
    	endif
    	inc $1016
    	bne textfw2
    	inc $1026
    	bne textfw2
    	inc <$37
    textfw2	lsr a
    	lsr a
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    textfw1	bit $1020
    	lda <$35
    	sta <$39
    	lda <$0B
    	sta <$0A
    	sta <$09
    	jmp [$38]
    
    	org $FB00
    	lda #$FE
    	pha
    	inc <$39
    	ldx <$19
    	stx <4
    	lda <$29
    	asl <4
    	rol a
    	asl <4
    	rol a
    	asl <4
    	rol a
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    
    	org $FC00
    	lda #$FA
    	sta <$39
    	lda #$FE
    	pha
    	lda <$19
    	anc #31
    	ora <$09
    	tax
    	lda zchad,x
    	pha
    	rts
    
    	; States can be:
    	;   0   = Second step of ASCII escape
    	;   1-3 = Fwords
    	;   4   = First step of ASCII escape
    	;   5-7 = Shift states 0,1,2
    
    	; These subroutines are entered with X set to the state.
    	; Also has carry flag cleared.
    	org $FE01
    
    	; ** Emit a space
    	def_zchars $A0
    	def_zchars $C0
    	def_zchars $E0
    zch32	jsr space
    	jmp [$38]
    
    	; ** Second escape
    	def_zchars $00,$1F
    	txa
    	ora <5
    	beq zch1
    	cmp #32
    	beq zch32
    	cmp #13
    	beq zch13
    	ldx <$30
    	beq zch1
    	sta <0,x
    	inc <$30
    	lda <$0A
    	sta <$09
    	jmp [$38]
    
    	; ** First escape
    	def_zchars $80,$9F
    	txa
    	asl a
    	asl a
    	asl a
    	asl a
    	asl a
    	sta <5
    	anc #0
    	sta <$09
    	jmp [$38]
    
    	; ** Frequent words
    	def_zchars $20,$7F
    	lda fwordsl,x
    	sta $1015
    	lda fwordsh,x
    	sta $1025
    	lda $5801
    	if smalend
    	asl a
    	sta <$16
    	else
    	sta <$26
    	lda #0
    	rol a
    	sta <$37
    	endif
    	inc $1015
    	bne zfw1
    	inc $1025
    zfw1	lda $5801
    	if smalend
    	rol a
    	sta <$26
    	else
    	asl a
    	sta <$16
    	rol <$26
    	endif
    	lda #0
    	adc #0
    	sta <$37
    	jmp textfw
    
    	; ** Begin escape
    	def_zchars $E6
    	lda #$80
    	sta <$09
    	jmp [$38]
    
    	; ** Direct character code
    	def_zchars $A6,$BF
    	def_zchars $C6,$DF
    	def_zchars $E8,$FF
    	ldy <$30
    	beq zch1
    	stx <$E0,y
    	inc <$30
    zch1	lda <$0A
    	sta <$09
    	jmp [$38]
    
    	; ** Emit a line break
    	def_zchars $E7
    zch13	jsr bufout
    	lda <$31
    	ora #$1F
    	sta <$31
    	lda <$0A
    	sta <$09
    	jmp [$38]
    
    	; ** Begin frequent words state 0-31
    	def_zchars $A1
    	def_zchars $C1
    	def_zchars $E1
    	lda #$20
    	sta <$09
    	jmp [$38]
    
    	; ** Begin frequent words state 32-63
    	def_zchars $A2
    	def_zchars $C2
    	def_zchars $E2
    	lda #$40
    	sta <$09
    	jmp [$38]
    
    	; ** Begin frequent words state 64-95
    	def_zchars $A3
    	def_zchars $C3
    	def_zchars $E3
    	lda #$60
    	sta <$09
    	jmp [$38]
    
    	; ** Temporary shift 1
    	def_zchars $A4
    	lda #$C0
    	sta <$09
    	jmp [$38]
    
    	; ** Temporary shift 2
    	def_zchars $A5
    	lda #$E0
    	sta <$09
    	jmp [$38]
    
    	; ** Permanent shift 1 or 2
    	def_zchars $C4
    	def_zchars $E5
    	and #$F0
    	sta <$0A
    	jmp [$38]
    
    	; ** Permanent shift 0
    	def_zchars $C5
    	def_zchars $E4
    	lda #$A0
    	sta <$09
    	sta <$0A
    	jmp [$38]
    
    	; Reset vector
    	bank intbank+3
    	org $FFFA
    	dw 0,reset,0
    
    	; Pattern tables
    	bank intbank+4
    	org $0000
    	incbin "pc.chr"
    
    	; Cursor icon
    	org $07F0
    	defchr $00000000, \
    	       $03030300, \
    	       $00303030, \
    	       $03030300, \
    	       $00303030, \
    	       $03030300, \
    	       $00303030, \
    	       $00000000
    
    	; Postprocessor
    	emu
    
    	org $0000
    	lda 0
    	sta $2012
    	inc <1
    	rts
    
    	org $0040
    	db "0123456789012345"
    	db "6789012345678901"
    
    	org $0080
    	db "                                "   ; $80-$9F
    	db "      abcdefghijklmnopqrstuvwxyz"   ; $A0-$BF
    	db "      ABCDEFGHIJKLMNOPQRSTUVWXYZ"   ; $C0-$DF
    	db "      **0123456789.,!?_#'\"/\\-:()" ; $E0-$FF
    
    	org $8000
    	cld
    
    	; Make duplicates of ASCII characters as Z-characters
    	lda #1
    	sta $200D
    	lda #0
    	sta $200E
    	lda #8
    	sta $200F
    	ldx #$80
    pp1	lda #4
    	sta <2
    	lda <0,x
    	asl a
    	rol <2
    	asl a
    	rol <2
    	asl a
    	rol <2
    	asl a
    	rol <2
    	sta <1
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	inx
    	bne pp1
    
    	; Make duplicate of digits for use with PRINTN
    	ldx #0
    	stx $200E
    	stx $200F
    pp2	lda #4
    	sta <2
    	lda <$40,x
    	asl a
    	rol <2
    	asl a
    	rol <2
    	asl a
    	rol <2
    	asl a
    	rol <2
    	sta <1
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	jsr 0
    	inx
    	cpx #32
    	bne pp2
    
    	; Finished
    	hlt
    
    	org $FFFC
    	dw $8000
    
    	code
    	bank intbank+4
    

## C program

This program is generating a stub file and story ROM for its use. 
    
    
    /*
      This file is part of Famizork II and is in the public domain.
    */
    
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
    
    static FILE*fp;
    static int c;
    static int d;
    static int gamesize;
    static char endian;
    static unsigned char mem[0x20000];
    static char buf[256];
    
    #define OUTHEADER(x,y) fprintf(fp,"%s\t= %u\n",x,(mem[y*2+endian]<<8)|mem[y*2+1-endian])
    
    int main(int argc,char**argv) {
      if(argc<2) return 1;
      fp=fopen(argv[1],"rb");
      fseek(fp,0,SEEK_END);
      gamesize=ftell(fp);
      if(gamesize>0x20000 || gamesize<0) return 1;
      fseek(fp,0,SEEK_SET);
      fread(mem,1,gamesize,fp);
      fclose(fp);
      if(*mem!=3) return 1;
      sprintf(buf,"%s.asm",argv[1]);
      fp=fopen(buf,"w");
      endian=mem[1]&1;
      mem[1]&=3;
      mem[1]|=16;
      c=(gamesize>0x10000?16:gamesize>0x8000?8:gamesize>0x4000?4:2);
      fprintf(fp,"\tnes2prgram 0,131072\n");
      fprintf(fp,"\tinesprg %d\n",(c>>1)+2);
      fprintf(fp,"intbank\t= %d\n",c);
      fprintf(fp,"smalend\t= %d\n",endian);
      fprintf(fp,"large\t= %d\n",gamesize>=0x10000);
      if(gamesize<0x10000) fprintf(fp,"maxaddr\t= %u\n",gamesize-1);
      OUTHEADER("start",3);
      OUTHEADER("vocab",4);
      OUTHEADER("object",5);
      OUTHEADER("global",6);
      OUTHEADER("purbot",7);
      OUTHEADER("fwords",12);
      fprintf(fp,"\tcode\n\tbank 0\n\tincbin \"%s.rom\"\n\tinclude \"famizork2.asm\"\n",argv[1]);
      fprintf(fp,"\n\tbank %d\n\torg fwordsl\n",c);
      d=(mem[24+endian]<<8)|mem[25-endian];
      for(c=0;c<192;c+=2) fprintf(fp,"\tdb %d\n",(d+c)&255);
      for(c=0;c<192;c+=2) fprintf(fp,"\tdb %d\n",((d+c)>>8)&255);
      fprintf(fp,"\torg multabl\n");
      for(c=0;c<255;c++) fprintf(fp,"\tdb %d\n",((c*c)>>2)&255);
      for(c=0;c<512;c++) fprintf(fp,"\tdb %d\n",((c*c)>>10)&255);
      fprintf(fp,"\tbank intbank+4\n");
      fclose(fp);
      sprintf(buf,"%s.rom",argv[1]);
      fp=fopen(buf,"wb");
      if(gamesize>0x10000) {
        fwrite(mem+0x10000,1,0x10000,fp);
        fwrite(mem,1,0x10000,fp);
      } else {
        fwrite(mem,1,gamesize,fp);
      }
      fclose(fp);
      return 0;
    }
    

## Explanation

The explanation of the mapper is [User:Zzo38/Mapper_I](User_Zzo38_Mapper_I.xhtml "User:Zzo38/Mapper I"). 

The pattern table is arranged in this way (although $7F is the cursor picture, not shown here): 
    
    
    0123456789012345
    6789012345678901
     !"#$%&'()*+,-./
    0123456789:;<=>?
    @ABCDEFGHIJKLMNO
    PQRSTUVWXYZ[\]^_
    `abcdefghijklmno
    pqrstuvwxyz{|}~
                    
                    
          abcdefghij
    klmnopqrstuvwxyz
          ABCDEFGHIJ
    KLMNOPQRSTUVWXYZ
          **01234567
    89.,!?_#'"/\-:()
    

As you can see there are many duplicates, in particular each digit occurs five times, except 0 and 1 which occur six times each. Many other characters also occur twice. These will improve the speed of the program, since it does not have to convert Z-characters and numbers into ASCII before displaying them. 

Many things are precomputed at compile-time in order to improve speed (also improves size of the interpreter): 

  * The mode byte is set to indicate that the status bar is unavailable
  * Address of objects, global variables, and frequent words table
  * Starting address of execution of program
  * Endianness and various calculations related to it
  * The size of the story file, which can be used to determine needed ROM sizes and optimizing of the interpreter
  * Self-inserting-breaks
  * Stuff to optimize the vocabulary (not yet)
  * Multiplication tables



A custom mapper is used, which bankswitches only one byte at a time. This makes much of the logic for addressing the story file much simpler than it otherwise would be. It also overlaps bankswitching registers with mirrors of the RAM internal to the console, and since they respond to multiple addresses, this means you can save the bankswitched value at the same time as bankswitching, that you can store multiple bank numbers at once (even though the mapper can only remember one at once), and that reading from the RAM mirrors will also bankswitch (allows you to restore a saved bankswitch in four cycles; with other mappers it is usually seven). 

There is an instruction decoding table, which is one table for all instructions and contains duplicates for the different forms of the instruction (such as EXT forms of 2OP instructions, the different variable/immediate combinations for 2OP, and the different operand types for 1OP). However, a different opcode is used when EQUAL? is encoded as EXT than as 2OP, in order to improve the speed in the 2OP case. The code can just use "jmp nxtinst" to begin decoding the next instruction; it doesn't use return from subroutine. 

Instruction decoding tables, as well as the Z-character decoding tables, both use the RTS trick, although in the case of Z-character decoding, the table contains only the low byte of the address since the code is small enough in this case. 

Also it is using several stable unofficial opcodes. 
