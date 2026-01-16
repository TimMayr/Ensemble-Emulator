# NMI thread

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NMI_thread) | View [other pages](Special_AllPages.xhtml#NMI_thread)

[![](../wiki-images/Sprite_0_in_top_status_bar.jpg)](File_Sprite_0_in_top_status_bar_jpg.xhtml)

Sprite 0 hit is used to split the screen.

There are three main ways that a game loop on the NES can be organized: 

Main only
    Game logic and output in the main loop, and only incrementing a flag in the [NMI](NMI.xhtml "NMI") handler to let the main loop know that vertical blanking has begun. The drawback is that it's very easy for music to slow down or for raster splits to fail if the game is using too much CPU time. Several Squaresoft games use something like this.
NMI only
    Game logic and output with the NMI handler in an infinite JMP loop. The NMI handler performs the following steps in order: PPU output, APU output, read controls, game logic. For example, _Super Mario Bros._ uses this.
NMI and main
    Game logic in the main loop, PPU and APU output in the NMI handler.

Waiting for vertical blank by waiting for the NMI handler to increment a flag is [the simplest thing that could possibly work](http://c2.com/xp/DoTheSimplestThingThatCouldPossiblyWork.html). And it does work for games without a status bar or for games whose upper limit on CPU use is easy to predict. It may also work for games whose mapper has a scanline counter that triggers an IRQ. 

But once your game world becomes more complex, the simple method may cause problems. For example, consider a video game that has a status bar and several critters running around. It may occasionally take longer than one screen to process AI, physics, and display updates once enough critters with complex movement patterns are spawned, such as multiple Hammer Brothers in _Super Mario Bros._ or all the ~~ducks~~ turtles in the middle of World 3-7 of _Super Mario Bros. 3_. This will cause your game to slow down when an NMI occurs while your game is doing something else. And if your scanline counter is based on sprite 0 hit and not an IRQ, it will cause visual glitches as the status bar flickers. 

So sometimes, it's useful to do more than the bare minimum in your NMI handler. To make a top status bar rock-solid in the face of excessive game logic, you can move VRAM uploads and sprite 0 handling into the NMI handler. The main program [prepares VRAM updates in main RAM](The_frame_and_NMIs.xhtml "The frame and NMIs"), and once the VRAM update request is ready, it turns on a flag VRAM_update_ready to let the NMI handler know. This is similar to [multithreaded programming](https://en.wikipedia.org/wiki/Thread_\(computer_science\) "wikipedia:Thread \(computer science\)"), but because the NMI handler itself is never interrupted, the locking can be much simpler than it is in multithreaded programming on a PC. 

The NMI thread has these steps: 

  1. Let the main program know that NMI has occurred, as in the simple method.
  2. Push all registers.
  3. If VRAM_update_ready is false, go to step 5.
  4. Copy data from the VRAM update request areas in RAM into VRAM and OAM.
  5. Set VRAM_update_ready to false.
  6. Set the scroll position using [PPUCTRL and PPUSCROLL](PPU_registers.xhtml "PPU registers").
  7. (Optional) Run the music code.
  8. (Optional) Wait for sprite 0 hit and change the VRAM address.
  9. Pull all registers and return.



Make sure to do anything related to VRAM and OAM _before_ other things like music. Vertical blanking time is valuable, and you should make the most of it. 

Even though the NES CPU has only one NMI vector, ordinarily within ROM, it's possible to switch among multiple NMI handlers using a small amount of self-modifying code. 
    
    
    RTI_opcode = $40
    JMP_opcode = $4C
    
    .segment "BSS"
    nmi_trampoline: .res 3
    
    .segment "CODE"
    ; other code
    
      ldx #<some_nmi_handler
      ldy #>some_nmi_handler
      jsr change_nmi_handler
    
    ; other code
    
    ;;
    ; Sets the NMI handler to the ISR whose code begins at YYXX.
    ; If an NMI occurs during this process, it will be ignored (RTI).
    change_nmi_handler:
      lda #RTI_opcode
      sta nmi_trampoline
      stx nmi_trampoline+1
      sty nmi_trampoline+2
      lda #JMP_opcode
      sta nmi_trampoline
      rts
    
    .segment "VECTORS"
      .addr nmi_trampoline, reset_handler, irq_trampoline
    

## See also

  * [Consistent frame synchronization](Consistent_frame_synchronization.xhtml "Consistent frame synchronization")



## External links

  * ["interrupts is threads"](https://onevariable.com/blog/interrupts-is-threads/)


