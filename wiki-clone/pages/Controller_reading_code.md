# Controller reading code

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Controller_reading_code) | View [other pages](Special_AllPages.xhtml#Controller_reading_code)

This page contains example code for reading the [NES controller](Standard_controller.xhtml "NES controller"). 

See also: [Controller reading](Controller_reading.xhtml "Controller reading")

## Contents

  * 1 Controller Reading Code
    * 1.1 Basic Example
    * 1.2 Standard Read for 2 Controllers and Famicom
    * 1.3 Alternative 2 Controllers Read
    * 1.4 DPCM Safety
      * 1.4.1 DPCM Safety using Repeated Reads
      * 1.4.2 DPCM Safety using OAM DMA
  * 2 Other Useful Operations
    * 2.1 Button Flags
    * 2.2 Calculating Presses and Releases
    * 2.3 Directional Safety
  * 3 External Examples
  * 4 References



## Controller Reading Code

### Basic Example

This is a tutorial example of the bare minimum needed to read the controller. It will explain the basic principles in detail, but once understood, you may wish to continue to the Standard Read example that follows, as a more complete and ready-to-use code example. 

This code describes an efficient method of reading the [standard controller](Standard_controller.xhtml "Standard controller") using [ca65](http://cc65.github.io/cc65/) syntax. 

The result byte _buttons_ should be placed in zero page to save a cycle each time through the loop. 
    
    
    ; we reserve one byte for storing the data that is read from controller
    .zeropage
    buttons .res 1
    

When reading from _JOYPAD*_ what is read might be different from $01/$00 for various reasons. (See [Controller reading](Controller_reading.xhtml "Controller reading").) In this code the only concern is bit 0 read from _JOYPAD*._. 
    
    
    JOYPAD1 = $4016
    JOYPAD2 = $4017
    

This is the end result that will be stored in _buttons_. **1** if the button was pressed, **0** otherwise. 

bit  |  7  |  6  |  5  |  4  |  3  |  2  |  1  |  0   
---|---|---|---|---|---|---|---|---  
button  | A  | B  | Select  | Start  | Up  | Down  | Left  | Right   
  
This subroutine takes 132 cycles to execute but ignores the Famicom expansion controller. Many controller reading subroutines use the X or Y register to count 8 times through the loop. But this one uses a more clever [ring counter](https://en.wikipedia.org/wiki/Ring_counter "wikipedia:Ring counter") technique: $01 is loaded into the result first, and once eight bits are shifted in, the 1 bit will be shifted out, terminating the loop. 
    
    
    ; At the same time that we strobe bit 0, we initialize the ring counter
    ; so we're hitting two birds with one stone here
    readjoy:
        lda #$01
        ; While the strobe bit is set, buttons will be continuously reloaded.
        ; This means that reading from JOYPAD1 will only return the state of the
        ; first button: button A.
        sta JOYPAD1
        sta buttons
        lsr a        ; now A is 0
        ; By storing 0 into JOYPAD1, the strobe bit is cleared and the reloading stops.
        ; This allows all 8 buttons (newly reloaded) to be read from JOYPAD1.
        sta JOYPAD1
    loop:
        lda JOYPAD1
        lsr a        ; bit 0 -> Carry
        rol buttons  ; Carry -> bit 0; bit 7 -> Carry
        bcc loop
        rts
    

Continue to the next example for a more complete read routine that handles both controllers and the standard Famicom expansion controllers. 

### Standard Read for 2 Controllers and Famicom

Adding support for controllers on the Famicom's DA15 expansion port and for player 2's controller is straightforward. Something similar to the following routine is used in most Famicom games. Even though the expansion port is unused on the NES, the unconnected bit will read as 0, so this solution works safely with both Famicom and NES hardware. 
    
    
    .zeropage
    buttons: .res 2     ; space for 2 reads
    
    .code
    readjoyx2:
        ldx #$00
        jsr readjoyx    ; X=0: read controller 1
        inx
        ; fall through to readjoyx below, X=1: read controller 2
    
    readjoyx:           ; X register = 0 for controller 1, 1 for controller 2
        lda #$01
        sta JOYPAD1
        sta buttons, x
        lsr a
        sta JOYPAD1
    loop:
        lda JOYPAD1, x
        and #%00000011  ; ignore bits other than controller
        cmp #$01        ; Set carry if and only if nonzero
        rol buttons, x  ; Carry -> bit 0; but 7 -> Carry
        bcc loop
        rts
    

If playing DPCM samples, there is an additional reread step to prevent errors (see below). 

Note that the `and` to ignore bits is not optional, as the upper bits of `JOYPAD1` read are not guaranteed to be 1 or 0[1]. 

### Alternative 2 Controllers Read

Alternatively, we could combine both controller reads into 1 loop with a single strobe, though this routine is not safe to use with DPCM samples playing (see below). 
    
    
    .zeropage
    buttons: .res 2
    
    .code
    readjoy2:
        lda #$01
        sta JOYPAD1
        sta buttons+1 ; player 2's buttons double as a ring counter
        lsr a
        sta JOYPAD1
    loop:
        lda JOYPAD1
        and #%00000011
        cmp #$01
        rol buttons+0
        lda JOYPAD2
        and #%00000011
        cmp #$01
        rol buttons+1
        bcc loop
        rts
    

### DPCM Safety

* * *

The NTSC CPU has a bug where the [DMA](DMA.xhtml "DMA") that fetches DPCM sample data for the [APU's DMC channel](APU_DMC.xhtml "APU DMC") can cause extra reads of the joypads. This is fixed on PAL. For standard controllers, this bug results in at least one bit being deleted from the controller's serial report, causing the following buttons to be shifted over and usually resulting in the right direction appearing to be pressed. Software cannot directly detect that this bug has happened, so controller reading code in games using sampled audio has to be resilient against it. There are currently two approaches for dealing with this problem, each with significant pros and cons. 

Note that both approaches share some limitations: 

  * They may become unsafe if interrupted by NMI or IRQ.
  * Poor support across most emulators can make it hard to know if the code works correctly on hardware.
  * Using third party controllers may make testing harder: official standard controllers shift in 1's on extra reads that cause obvious errant right presses, while third party controllers may shift in 0's.
  * It is possible for joypad state to corrupt even if the controller reading code works as intended. Any read from $4000-401F provides an opportunity for DMC DMA to trigger a joypad read, and this can cause controllers with state that is slow or impossible to recover (e.g. [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller")) to lose that state before the controllers are intentionally read. Some sound engines cause such reads by doing indexed stores to sound registers. For controllers with normal strobe behavior where the full current state is immediately loaded on strobe, this is not a problem.



Neither approach is a one-size-fits-all solution; there are cases where one or the other solution does not work. 

#### DPCM Safety using Repeated Reads

The most common technique, as seen in _Super Mario Bros. 3_[2] and other games, will read each controller twice, but in the event of a mismatch it will keep re-reading the controller until two results in a row are the same. 
    
    
    readjoy2_safe:
        ldx #$00
        jsr readjoyx_safe  ; X=0: safe read controller 1
        inx
        ; fall through to readjoyx_safe, X=1: safe read controller 2
    
    readjoyx_safe:
        jsr readjoyx
    reread:
        lda buttons, x
        pha
        jsr readjoyx
        pla
        cmp buttons, x
        bne reread
        rts
    

Note that the time between the start of one read and the end of the next read must be less than the length of the fastest DMC fetch period minus 4 (428 cycles). This is because multiple bit deletions could cause both passes to result in the same incorrect data. For this reason, it is normal to read controllers one at a time with this method, rather than attempting both at once. (Note: `readjoy2` above takes too long to be suitable.) [_Gimmick!_ has such a bug](Game_bugs.xhtml "Game bugs") resulting from trying to read both at once. 

Most often a controller will be read 2 times, and 3 or 4 in the case of a DPCM corruption, or the player pressing a button during the read. With the assistance of tools, a malicious controller input could change the buttons on every read, holding it in this reread loop indefinitely[3][4], but this is generally not an important edge case to account for. 

This approach to DPCM safety has these benefits: 

  * The code is easy to understand and modify.
  * It is position-independent: branching across page boundaries does not inherently break it, and it can be done at any time in the frame.
  * It works in all emulators, even if they don't implement the bug correctly or at all.



It also has these drawbacks: 

  * Repeated reads cannot be used for controllers that lose state on reads (e.g. [SNES mouse](Super_NES_Mouse.xhtml "SNES mouse")), take a long time before data is available to read again (e.g. [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller")), or have too much data to read twice within the cycle budget (e.g. [Famicom Network Controller](Famicom_Network_Controller.xhtml "Famicom Network Controller")).
  * It can be hard to know if the code is safe: 
    * The budget limitations are not obvious because of obscure edge cases with the bug. All joypad reads for 2 complete passes must finish within one DMC period minus 4 (usually 428 cycles). Reading both controllers in a single pass takes too long; they must be handled individually.
    * Hardware differences can make real hardware testing difficult: the bug behaves one way on the AV Famicom and NES, and another way on other Famicom models. The Famicom behavior may mitigate the bug when using repeated reads.
    * Even most accurate emulators don't emulate all the ways repeated reads can fail. Mesen v2 does and is recommended.
  * This approach does not prevent the bug, it merely works around it, so other negative side effects of the bug still happen: 
    * The sample byte read by the APU can be corrupted when the bug occurs. However, it is difficult to hear this corruption.
    * The APU's frame IRQ may be lost when the bug occurs. However, this feature is very rarely used.
  * It makes controller reads take at least twice as long and does not guarantee a maximum runtime. Normally, it will take up to 4 passes.



#### DPCM Safety using OAM DMA

Because halts for DPCM fetches have very specific timing, it is possible to get glitch-free controller reads by timing all **$4016** and **$4017** reads to fall on one cycle parity. This is because the CPU alternates between two kinds of cycles: 'get' and 'put'. DPCM fetches normally occur only on put cycles, and because the first cycle after OAM DMA is normally guaranteed to be a get cycle, timed code can use OAM DMA to ensure the joypad reads land on get cycles. [5] This is a relatively new technique and is not supported by several older emulators[6], but generally works in currently-supported emulators. In the following example code, the controller1 and controller2 labels must be in zeropage for the timing to work. 

This solution might be ideal, using fewer cycles than doing repeated reads, and taking a consistent amount of time. It can also be adapted for DPCM-conflict free reading of controllers that require a longer report, like [four player adapters](Four_player_adapters.xhtml "Four player adapters") or the [SNES mouse](Super_NES_Mouse.xhtml "SNES mouse") (see notes below). 

However, unlike the repeated reads solution, it can't be executed at any time. Instead it must be integrated into your NMI routine to coincide with your sprite OAM DMA once per frame. 
    
    
        lda #OAM
        sta $4014          ; ------ OAM DMA ------
        ldx #1             ; get put          <- strobe code must take an odd number of cycles total
        stx buttons+0      ; get put get      <- buttons must be in the zeropage
        stx $4016          ; put get put get
        dex                ; put get
        stx $4016          ; put get put get
    loop:
        lda $4017          ; put get put GET  <- loop code must take an even number of cycles total
        and #3             ; put get
        cmp #1             ; put get
        rol buttons+1, x   ; put get put get put get (X = 0; waste 1 cycle for alignment)
        lda $4016          ; put get put GET
        and #3             ; put get
        cmp #1             ; put get
        rol buttons+0      ; put get put get put
        bcc loop           ; get put [get]    <- this branch must not be allowed to cross a page
    

Note that this example routine only reads two 8-bit controllers and does not take enough time to span more than one DPCM fetch. Routines longer than this must contend with two additional constraints: 

  * When DMC DMA is delayed by an odd number of cycles, it takes 3 cycles instead of 4, changing the cycle parity. If extending this function to read more bits, care must be taken so that _all_ CPU write cycles are aligned. Instructions with a single write cycle must align the write to avoid conflict with the DPCM fetch, and double-write instructions like ROL need to align both writes so that the DPCM fetch falls on the first write.[7] If an interrupt can occur during the routine, it must be aligned so the fetch can only fall on the second of the three automatic stack writes.
  * When DMC DMA occurs near the end of OAM DMA, it steals an odd number of cycles, inverting the cycle parity. Every DMC period after that, a misaligned DPCM fetch will occur. Care must be taken to ensure this does not land on a joypad read. For longer functions, these misaligned cases can be resynced by landing the DPCM fetch on a single write cycle.



See [DMA](DMA.xhtml "DMA") for detailed information on DMA timing. However, game developers writing synced code may find it easier to consider just the specific case of DMC DMA used for reloading the sample buffer. These timings do not apply to the initial DMC DMA load caused by writing to $4015, and they are not sufficient for emulating DMA timings; emulator authors should refer to [DMA](DMA.xhtml "DMA") for comprehensive information. A DMC DMA reload takes these cycle counts in these cases: 

  * 4 cycles if it falls on a CPU read cycle
  * 3 cycles if it falls on an odd number of CPU write cycles
  * 4 cycles if it falls on an even number of CPU write cycles
  * An even number of cycles if it occurs during OAM DMA
  * An odd number of cycles if it occurs at the end of OAM DMA (changing cycle parity for the following code)



This approach to DPCM safety has these benefits: 

  * It can support any controller because it only does one pass and has no cycle budget.
  * It is nearly as fast as standard reads.
  * It avoids the bug and thus does not corrupt the sample byte or interfere with the APU's frame IRQ.



It also has these drawbacks: 

  * It uses timed code and relies on complicated DMA behavior, so it is harder to understand and modify.
  * Moving the code can break it by causing branches to cross a page boundary, changing the timing.
  * It must directly follow OAM DMA. Unusual features that skip OAM DMA on gameplay frames are not compatible with this approach. (On lag frames, normally both OAM DMA and controller reads are skipped. This is not a problem for synced reads.)
  * It does not work in emulators that emulate the bug but have incorrect DMA timing (e.g. Nestopia). Many modern emulators either don't emulate the bug or have correct-enough DMA timing.
  * Synced code that lasts longer than one DMC period minus 4 (usually 428 cycles) is difficult to write correctly. This is because write cycles must also be synced and specific desync cases occur if DMC DMA lands at the end of OAM DMA. However, this is the only approach that supports code of this length.



## Other Useful Operations

### Button Flags

It is helpful to define the buttons as a series of bit flags: 
    
    
    BUTTON_A      = 1 << 7
    BUTTON_B      = 1 << 6
    BUTTON_SELECT = 1 << 5
    BUTTON_START  = 1 << 4
    BUTTON_UP     = 1 << 3
    BUTTON_DOWN   = 1 << 2
    BUTTON_LEFT   = 1 << 1
    BUTTON_RIGHT  = 1 << 0
    

And then buttons can be checked as follows: 
    
    
        lda buttons
        and #BUTTON_A | BUTTON_B
        beq notPressingAorB
        ; Handle presses.
    notPressingAorB:
    

### Calculating Presses and Releases

To calculate newly pressed and newly released buttons by comparing against the last frame's buttons: 
    
    
        ; newly pressed buttons: not held last frame, and held now
        lda last_frame_buttons, x
        eor #%11111111
        and buttons, x
        sta pressed_buttons, x
    
        ; newly released buttons: not held now, and held last frame
        lda buttons, x
        eor #%11111111
        and last_frame_buttons, x
        sta released_buttons, x
    

### Directional Safety

Opposing directions (Up+Down and Left+Right) are possible on non-standard controllers, or even on a worn out standard one. Both directions can be tested for at the same time in an efficient bitwise operation: 
    
    
        lda buttons, x
        and #%00001010    ; Compare Up and Left...
        lsr a
        and buttons, x    ; to Down and Right
        beq not_opposing
            ; Use previous frame's directions
            lda buttons, x
            eor last_frame_buttons, x
            and #%11110000
            eor last_frame_buttons, x
            sta buttons, x
        not_opposing:
    

Diagonal directions can also be detected at the same time as opposing ones, by testing if more than 1 directional bit is set. This could be used as part of a 4-way joystick style control scheme: 
    
    
        lda buttons, x
        and #%00001111    ; If A & (A - 1) is nonzero, A has more than one bit set
        beq not_diagonal
        sec
        sbc #1
        and buttons, x
        beq not_diagonal
            ; Use previous frame's directions
            lda buttons, x
            eor last_frame_buttons, x
            and #%11110000
            eor last_frame_buttons, x
            sta buttons, x
        not_diagonal:
    

As an example, the code above rejects the detected cases by using the direction bits from the previous frame, but depending on the situation another response might be more appropriate (e.g. cancel just the opposing bits, or clear all directions). 

## External Examples

  * [Forum post:](http://forums.nesdev.org/viewtopic.php?t=4124) Blargg's DMC-fortified controller read routine
  * [Forum post:](http://forums.nesdev.org/viewtopic.php?p=171971) Rahsennor's OAM-synchronized controller read
  * [Forum post:](http://forums.nesdev.org/viewtopic.php?f=2&t=14197) Drag's bitwise DMC-safe controller reading
  * [pads.s:](https://github.com/pinobatch/nrom-template/blob/master/src/pads.s) pinobatch's NROM-template controller read routine



## References

  1. ↑ [Controller reading: Unconnected data lines and open bus](Controller_reading.xhtml#Unconnected_data_lines_and_open_bus "Controller reading")
  2. ↑ [Super Mario Bros. 3 controller reread method](https://forums.nesdev.org/viewtopic.php?p=151720#p151720)
  3. ↑ [Ars Technica: How to beat Super Mario Bros. 3 in less than a second](https://arstechnica.com/gaming/2016/07/how-to-beat-super-mario-bros-3-in-less-than-a-second/)
  4. ↑ <https://tasvideos.org/6466S> Tool-Assisted Speedrun of Super Mario Bros. 3 which first demonstrated abusing controller reread routines.
  5. ↑ [Forum post:](http://forums.nesdev.org/viewtopic.php?p=171971) Rahsennor's OAM-synchronized controller read
  6. ↑ [Forum post:](http://forums.nesdev.org/viewtopic.php?f=2&t=14319&start=15#p172099) as of May 2016, Nintendulator and Nestopia do not accurately emulate OAM-synchronized controller reading.
  7. ↑ [Forum post:](https://forums.nesdev.org/viewtopic.php?p=231604#p231604) demonstration of how ROL instruction affects alignment for OAM DMA synchronized controller reading.


