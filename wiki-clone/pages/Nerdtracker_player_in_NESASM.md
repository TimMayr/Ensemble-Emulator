# Nerdtracker player in NESASM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Nerdtracker_player_in_NESASM) | View [other pages](Special_AllPages.xhtml#Nerdtracker_player_in_NESASM)
    
    
       .inesprg 2
       .ineschr 0
       .inesmir 0
       .inesmap 0
    
       .bank 1
       .org $FFFA
       .dw nmi
       .dw reset
       .dw irq
    
    init = $8083
    play = $8080
    load = $8000
    
       .bank 0
    
       .org $8000
       .incbin "song.nsf"
    
    reset:
    
        lda #0 ; song 0
        ldx #0 ; NTSC speed
        jsr init
    
       lda #%10000000 
       sta $2000 ; <- enable play after init
    
    loop:
       jmp loop
    
    
    nmi:
        jsr play
    irq:
        rti 
    

Categories: [APU Demos](Category_APU_Demos.xhtml), [Programming Demos](Category_Programming_Demos.xhtml)
