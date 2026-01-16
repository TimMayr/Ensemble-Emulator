# Talk:Controller reading code

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AController_reading_code) | View [other pages](Special_AllPages.xhtml#Talk_Controller_reading_code)

## Reorganization needed with [Standard controller](Standard_controller.xhtml "Standard controller")

See: [Talk:Standard_controller#Reorganization_needed_with_Controller_Reading](Talk_Standard_controller.xhtml#Reorganization_needed_with_Controller_Reading "Talk:Standard controller")

## Removed last-frame compromise

This code appeared in the [DPCM Safety using Repeated Reads](Controller_reading.xhtml#DPCM_Safety_using_Repeated_Reads "Controller Reading") section. I have removed it, because I think it hurts comprehension for new readers who may not understand what they are trading away to save a few cycles they may not need. There's probably a place for this code somewhere, but I don't think it belongs in the middle of the article, and certainly not in lieu of a simpler and more standard example. 

I left a description of the technique, but not the code. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:14, 16 April 2019 (MDT) 

Code: 
    
    
    last_frame_buttons1 = $00
    last_frame_buttons2 = $01
    first_read_buttons1 = $02
    first_read_buttons2 = $03
    
    readjoy_safe:
        lda buttons2
        sta last_frame_buttons2
        lda buttons1
        sta last_frame_buttons1
    
        ; Read the controllers once and stash the result
        jsr readjoy
        lda buttons2
        sta first_read_buttons2
        lda buttons1
        sta first_read_buttons1
    
        ; Read the controllers again and compare
        jsr readjoy
        ldx #1
    cleanup_loop:
        ; Ignore read values if a bit deletion occurred
        lda buttons1,x
        cmp first_read_buttons1,x
        beq not_glitched
            lda last_frame_buttons,x
            sta buttons1,x
        not_glitched:
    
        dex
        bpl cleanup_loop
    
        rts
    

    In november 2023 it was demonstrated that this is still subject to DPCM errors because the readjoy loop does not complete fast enough (allowing 2 matching corruptions of the same controller). So, aside from the compromise of 1 frame of lag, it wasn't completely DPCM error free either. Note that tepples has since replaced this code with the OAMDMA sync solution in his NROM template ([gitub: pinobatch/nrom-template changes](https://github.com/pinobatch/nrom-template/commit/5656fc285aaa92f34a9b56246362c145aeaef930)). So, perhaps this code may just be retired here for eternity. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:27, 11 January 2024 (UTC)
