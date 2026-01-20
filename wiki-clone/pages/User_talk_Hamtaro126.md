# User talk:Hamtaro126

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AHamtaro126) | View [other pages](Special_AllPages.xhtml#User_talk_Hamtaro126)

## H126-ROM1 and H126-RAM1 mapper format (for next letter of ZZO38's Mapperset)

This mapper will be a DIY thing, I will not make carts, But it is possible to make them! 

Based off a combo of UxROM and MMC3 

PPU (H126-ROM1): 

  * $0000-$03FF: 4k CHR bank 1 (uses $40C0)
  * $0400-$07FF: 4k CHR bank 2 (uses $40C1)
  * $0800-$0BFF: 4k CHR bank 3 (uses $40C2)
  * $0C00-$0FFF: 4k CHR bank 4 (uses $40C3)
  * $1000-$13FF: 4k CHR bank 5 (uses $40C4)
  * $1400-$17FF: 4k CHR bank 6 (uses $40C5)
  * $1800-$1BFF: 4k CHR bank 7 (uses $40C6)
  * $1C00-$1FFF: 4k CHR bank 8 (uses $40C7)



PPU (H126-RAM1): 

  * $0000-$1FFF - 8k CHR RAM ($40C0-$40C7 are unused!)



RAM: 

  * $5000-$5FFF: 2k WRAM (optional)
  * $6000-$7FFF: 8k SaveRAM or WRAM



ROM: 

  * $8000-$9FFF: 8k ROM bank 1 (uses $40C8)
  * $A000-$BFFF: 8k ROM bank 2 (uses $40C9)
  * $C000-$DFFF: 8k ROM bank 3 (uses $40CA)
  * $E000-$FFFF: 8k ROM bank 4 (Fixed)



PPU Mirror: 

  * $40CB: Mirroring (0 = 1-Screen $2000, 1 = 1-Screen $2400, 2 = Horizontal, 3 = Vertical)



IRQ (optional): 

  * $40CC: Control Disable
  * $40CD: Control Enable
  * $40CE: Cycle IRQ Counter Value (MMC5 style)



This is a WIP, May Change!!! --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") (talk) 08:10, 29 June 2013 (MDT) 

## Discussion

I suggest to use $40C8, $40C9, $40CA for the PRG ROM bank select, and having $40CB unused. I think this would be more logical and probably simpler to build. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:10, 29 June 2013 (MDT) 

    EDIT1: Adjustments are Made, Tell me more, --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") (talk) 23:59, 29 June 2013 (MDT)

This is very similar to Taito's [X1-017](Taito_X1_017.xhtml "INES Mapper 082") or [TC0350](INES_Mapper_048.xhtml "INES Mapper 048"). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:12, 29 June 2013 (MDT) 

    NOTE1: I actually based the format off of complex mappers like Sunsoft and Taito's, But wanted the user to choose what options needed for their game, But this mapper is not too complex, It is designed for a variable sized package for cheapness, And the reason that MMC5-style instead of MMC3 IRQs are used because of ease of use! --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") (talk) 08:32, 30 June 2013 (MDT)

    

    I'd call this the logical evolution of the TC0350. You've increased it to 8x1 CHR banking instead of 2x2+4x1, moved the control registers, changed it to a scanline instead of cycle-counting IRQ, and increased the amount of memory. Note that 2KiB RAMs aren't really available anymore; if you're going to specify a form with more than 8KiB of flat RAM, you may as well specify _all_ of it to be and put 16320 bytes of RAM from $4040-$7FFF instead. (Or, keep an eye to implementation, and allocate 512*31 bytes from $4200-$7FFF from an iCE40 FPGA). Also, even thought people thought banking CHR RAM to be odd for Lagrange Point, on the balance it's useful. I wouldn't prohibit it a priori. (Also, is "4k" a typo?) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:34, 30 June 2013 (MDT)

You said you have no intention to build this cartridge, which might be OK, but you should at least have some idea of how to build such a cartridge (using discrete logic, Verilog, whatever). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 15:13, 4 July 2013 (MDT) 

I actually planned to learn all that, But the problem is I never actually wanted to until I learn to create better stuff, I have lots that are unfinished, Some that are study cases (mostly SMBDIS hacks), Some are actual works studying the PPU and ROM hacking, So I cannot do anything until I am done with my actual experience, actually LEARNING, and try to release actual stuff before doing this, I am sorry... 

Should I leave this info here, or should I erase it until then? --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") (talk) 15:39, 4 July 2013 (MDT) 

## Hamtaro126's Code/Macro Examples (WIP)

See the page on implenting a [Macro for Multibyte Constant](Multibyte_constant.xhtml "Multibyte constant") (specifically my ASM6 example) for my first macro submitted on this wiki, using the HEX command for ASM6 

Macros: 
    
    
    ;Easy Super Mario Bros. 3 TileMap Macro (16x16 Metatiles, 8x8 Split Tiles)
    ;ASM6 version
    ;By Hamtaro126, With thanks to Southbird for disassembling Super Mario Bros. 3
    ;Can also be used for Homebrew!!!
    ; 
    ;Example: SMB3Tile, $8000, $01, $23, $45, $67
    ;this splits these tiles into 256-byte SECTIONS at $8000, $8100, $8200, and $8300... have fun
    
    .macro SMB3Tile address, t1, t2, t3, t4
      .org address
      .db t1
      .org address+$0100
      .db t2
      .org address+$0200
      .db t3
      .org address+$0300
      .db t4
    .endm
    

Routines: 
    
    
    ;Straight from Super Mario World (SNES), but usable for NES now!
    ;BRA is changed to JMP for usability in any 6502 processor
    
    ;This code is useable since it is just generic code!
    
    HexToDec:
    	LDX #$00
    SetTens:
    	CMP #$0A
    	BCC ExitHToDRt
    	SBC #$0A
    	INX
    	JMP SetTens
    ExitHToDRt:
    	RTS
    

8k Cartridge RAM uploader for ASM6 (requires include): 
    
    
    src     = $00
    dst     = $02
    CartRAM	= $6000
    
    ;=============================================================================
    
    LoadCartRAM:
      lda #<UploadBank ; load the source address into a pointer in zero page
      sta src
      lda #>UploadBank
      sta src+1
      lda #<CartRAM    ; load the destination address into a pointer in zero page
      sta dst
      lda #>CartRAM
      sta dst+1
      ldx #$20       ; number of 256-byte pages to copy
      ldy #$00       ; starting index into the first page
    @loop:
      lda (src),y    ; loop copying 256 bytes.
      sta (dst),y
      iny
      bne @loop      ; repeat until we finish the page
      inc src+1      ; increment pointers by 256.
      inc dst+1
      dex
      bne @loop      ; repeat until we've copied enough pages
      rts
    
    RAMData:
    .include "rambank.asm"
    

CHR-RAM routine for ASM6 (requires include): 
    
    
    size    = $2000 ;8kb by default
    src     = $00
    PPUMASK = $2001
    PPUADDR = $2006
    PPUDATA = $2007
    
    LoadChrRam00:
      lda #<ChrRam00 ; load the source address into a pointer in zero page
      sta src
      lda #>ChrRam00
      sta src+1
      lda #$00
      sta PPUMASK    ; turn off rendering just in case
      sta PPUADDR    ; set the PPU address to $0000
      sta PPUADDR
      ldx #<size     ; number of 256-byte pages to copy
      ldy #>size     ; starting index into the first page
    @loop:
      lda (src),y    ; copy one byte
      sta PPUDATA    ; store into PPU data
      iny
      bne loop       ; repeat until we finish the page
      inc src+1      ; go to the next page
      dex
      bne @loop      ; repeat until we've copied enough pages
      rts
    
    ChrRam00:
      .incbin "ChrRam00.chr"
    

see [CHR ROM vs. CHR RAM](CHR_ROM_vs__CHR_RAM.xhtml "CHR ROM vs. CHR RAM") for more info. 
