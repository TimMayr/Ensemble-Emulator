# Super NES Mouse

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Super_NES_Mouse) | View [other pages](Special_AllPages.xhtml#Super_NES_Mouse)

[![](../wiki-images/SNES-Mouse-and-Pad.jpg)](File_SNES_Mouse_and_Pad_jpg.xhtml)

[](File_SNES_Mouse_and_Pad_jpg.xhtml "Enlarge")

SNES mouse

The **Super NES Mouse** (SNS-016) is a peripheral for the Super NES that was originally bundled with _Mario Paint_. It can be used with an NES through an adapter, made from an NES controller extension cord and a Super NES controller extension cord, that [connects the respective power, ground, clock, latch, and data pins](Controller_port_pinout.xhtml "Controller port pinout"). The **Hyper Click Retro Style Mouse** by Hyperkin is an optical mouse mostly compatible with software for the Super NES Mouse, with some behavior quirks. 

As with the [standard controller](Standard_controller.xhtml "Standard controller"), the mouse is read by turning the latch ($4016.d0) on and off, and then reading bit 0 or bit 1 of $4016 or $4017 several times, but its report is 32 bits long as opposed to 8 bits. 

On an NES or AV Famicom, the mouse may be connected to bit 0 through the front controller ports. On the original Famicom, it would normally have to be connected to bit 1 instead through the expansion port. 

## Contents

  * 1 Report
  * 2 Motion
  * 3 Sensitivity
  * 4 Other notes
  * 5 DPCM-safe code
  * 6 Example games
  * 7 References



## Report

The report is divided functionally into four bytes. The most significant bit is delivered first: 
    
    
    76543210  First byte
    ++++++++- Always zero: 00000000
    
    76543210  Second byte
    ||||++++- Signature: 0001
    ||++----- Current sensitivity (0: low; 1: medium; 2: high)
    |+------- Left button (1: pressed)
    +-------- Right button (1: pressed)
    
    76543210  Third byte
    |+++++++- Vertical displacement since last read
    +-------- Direction (1: up; 0: down)
    
    76543210  Fourth byte
    |+++++++- Horizontal displacement since last read
    +-------- Direction (1: left; 0: right)
    

After the fourth byte, subsequent bits will read as all 1, though the Hyperkin clone mouse instead reads a single 1 then all 0s. [1]

The Hyper Click mouse will not give a stable report if it is read too fast. Between each read and the next, there should be at least 14 CPU cycles. Between the 2nd and 3rd byte (16th and 17th bit) of the report should be at least 28 CPU cycles. Reading faster than this will result in corrupted values.[2]. 

## Motion

Motion of the mouse is given as a displacement since the last mouse read, delivered in the third and fourth bytes of the report. 

The displacements are in [sign-and-magnitude](https://en.wikipedia.org/wiki/Signed_number_representations#Sign-and-magnitude_method "wikipedia:Signed number representations"), not [two's complement](https://en.wikipedia.org/wiki/Signed_number_representations#Two.27s_complement "wikipedia:Signed number representations"). For example, $05 represents five mickeys (movement units) in one direction and $85 represents five mickeys in the other. To convert these to two's complement, use [negation](Synthetic_instructions.xhtml#Negate_A "Synthetic instructions"): 
    
    
      ; Convert to two's complement
      lda third_byte
      bpl :+
      eor #$7F
      sec
      adc #$00
    :
      sta y_velocity
    
      lda fourth_byte
      bpl :+
      eor #$7F
      sec
      adc #$00
    :
      sta x_velocity
    

When the magnitude of motion is 0, the reported sign will repeat the last used sign value for that coordinate. 

## Sensitivity

The mouse can be set to low, medium, or high sensitivity. 

On the original SNES mouse this can be changed by sending a clock while the latch ($4016.d0) is turned on: 
    
    
      ldy #1
      sty $4016
      lda $4016
      dey
      sty $4016
    

Some revisions of the mouse's microcontroller power up in an unknown state and may return useless values before the sensitivity is changed for the first time.[3]

The Hyper Click mouse will not cycle its sensitivity this way. Instead it has a manual button on the underside that must be pressed by the user to cycle sensitivity. It will always report 0 for sensitivity, regardless of its manual setting. For this reason, it is not advised to use the software sensitivity cycling to automatically detect the presence of a mouse.[4]

On the original SNES mouse, sensitivity setting 0 responds linearly to motion, at a rate of 50 counts per inch[5]. Values range from 0 to 63, but values higher than 25 are increasingly difficult to produce. [6]

Sensitivity settings 1 and 2 appear to remap the equivalent setting 0 values 0-7 to a table, and clamping at the highest value. (Rarely, however, other values may be seen in settings 1 and 2.) 

Sensitivity  | Value   
---|---  
0  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | ...   
1  | 0 | 1 | 2 | 3 | 8 | 10 | 12 | 21 | 21 | 21 | ...   
2  | 0 | 1 | 4 | 9 | 12 | 20 | 24 | 28 | 28 | 28 | ...   
  
The Hyper Click's two manually selected sensitivities both scale linearly with motion speed. Low sensitivity produces 0-31, and high sensitivity produces 0-63. The magnitude of the result is not dependent on the rate of polling, so it appears to report the current speed rather than the distance travelled since the last poll. The maximum value (31/63) at either sensitivity appears to correspond roughly to a speed of 8 inches per second. (This mouse should be used on a surface with a visible texture.)[7]

## Other notes

Using more than two mice on an AV Famicom is not recommended for two reasons: 

  * A mouse draws 50 mA, which is much more current than the standard controller draws. Drawing too much current is likely to cause the voltage regulator to overheat.
  * Changing player 1's sensitivity also affects player 3's, and changing player 2's sensitivity also affects player 4's.



Some documents about interfacing with the mouse recommend reading the first 16 bits at one speed, delaying a while, and reading the other 16 bits at another speed, following logic analyzer traces from a Super NES console. However, these different speeds are merely an artifact of the main loop of _Mario Paint_ , and the authentic mouse will give a correct report when read at any reasonable speed. For example, a program could read 8 bits, wait a couple thousand cycles, and then read the other 24. The Hyper Click needs a delay after the first 16 bits, though not nearly as much as these documents recommend. 

## DPCM-safe code

[Reading controllers while DPCM is playing](Controller_reading_code.xhtml#DPCM_Safety "Controller reading code") can result in bits being lost on non-PAL consoles. This is often solved by [reading the controller repeatedly](Controller_reading_code.xhtml#DPCM_Safety_using_Repeated_Reads "Controller reading code"), but this solution is not compatible with the mouse because the mouse returns a delta since its last read, so reading it changes its state. The mouse also cannot be read fast enough to work with repeated reads. Therefore, DPCM-safe mouse reads require that the [reads be synced with OAM DMA](Controller_reading_code.xhtml#DPCM_Safety_using_OAM_DMA "Controller reading code") so that they never occur on the same cycle as DMC DMA. See [DMA](DMA.xhtml "DMA") and especially [register conflicts](DMA.xhtml#Register_conflicts "DMA") for more information. 

The following code can safely read both the mouse (official or Hyperkin) and either an NES or SNES controller. It requires 5 or 6 zero page variables and its branches must not cross a page boundary. This code must not be interrupted by an NMI or IRQ. 
    
    
    CONFIG_JOYPAD_SIZE = 2        ; Report size in bytes. 1 for NES controller or 2 for SNES controller.
    CONFIG_MOUSE_SENSITIVITY = 1  ; 1: Allow mouse sensitivity to be clocked. 0: Disable.
    
    ; This code assumes fixed ports for the mouse and joypad. You can swap them here. If you want to be able
    ; to support both in the same game, it's recommended to make two copies of this function, one for each
    ; configuration, and then call the correct copy for the configuration you detected. These must not be
    ; the same register.
    CONFIG_JOYPAD_REGISTER = $4016
    CONFIG_MOUSE_REGISTER = $4017
    
    ; These zero page variables MUST be in zero page; the code relies on zero page timing.
    .segment "ZEROPAGE"
    
    mouse: .res 4 + CONFIG_JOYPAD_SIZE
      kMouseZero = 0  ; The mouse's 0 signature is written here. Can be replaced with joypad 1 state after.
      kMouseButtons = 1
      kMouseY = 2
      kMouseX = 3
    joypad1_down := mouse+4  ; This must immediately follow the mouse variables.
    
    ; This mask tells the code which register bit to use for reading the mouse. The idea is that the game
    ; will detect which bit the mouse is using at power-on and then set that bit to 1 in here, with all the
    ; other bits 0. When using an NES controller port, this will normally have just bit 0 set (#$01). When
    ; using a Famicom expansion port, this will normally have just bit 1 set (#$02).
    mouse_mask: .res 1
    
    
    .segment "BSS"
    
    .if CONFIG_MOUSE_SENSITIVITY <> 0
    ; If non-zero, tells the mouse reading code to clock the mouse sensitive once. The variable is then
    ; cleared. Sensitivity clocking works with the official mouse, but not the Hyperkin mouse.
    advance_sensitivity: .res 1
    .endif  ; CONFIG_MOUSE_SENSITIVITY
    
    
    .segment "CODE"
    
    ; Performs OAM DMA and reads a mouse and one controller.
    .proc OamDmaAndJoypads
      ; Strobe the joypads.
      ldx #$00
      ldy #$01
      sty mouse
      sty JOYPAD1
    
     .if CONFIG_MOUSE_SENSITIVITY <> 0
      ; Clock official mouse sensitivity.
      lda advance_sensitivity
      beq :+
      lda CONFIG_MOUSE_REGISTER
      stx advance_sensitivity
     :
     .endif  ; CONFIG_MOUSE_SENSITIVITY
    
      stx JOYPAD1
    
      ; Do OAM DMA.
      lda #>oam_buffer
      sta OAM_DMA
    
      ; Desync cycles: 432, 576, 672, 848, 432*2-4 (860)
    
      ; DMC DMA:                  ; PUT GET PUT GET        ; Starts: 0
    
    mouse_loop:
      lda mouse_mask              ; get put get*     *576  ; Starts: 4, 158, 312, 466, [620]
      and CONFIG_MOUSE_REGISTER   ; put get put GET
      cmp #$01                    ; put get
      rol mouse, x                ; put get put get* PUT GET  *432
      bcc mouse_loop              ; put get (put)          ; Must not cross page boundary.
    
      inx                         ; put get
      cpx #$04                    ; put get
      sty mouse, x                ; put get put GET
      bne mouse_loop              ; put get (put)          ; Must not cross page boundary.
    
    standard_loop1:
      lda CONFIG_JOYPAD_REGISTER  ; put get put GET        ; Starts: 619
      and #$03                    ; put get*         *672
      cmp #$01                    ; put get
      rol joypad1_down            ; put get put get put    ; This can desync, but we finish before it matters.
      bcc standard_loop1          ; get put (get)          ; Must not cross page boundary.
    
      ; Next cycle: 746
    
      ; For the SNES controller, we read another byte.
     .if CONFIG_JOYPAD_SIZE > 1
      sty joypad1_down+1          ; get put get
      nop                         ; put get
    standard_loop2:
      lda CONFIG_JOYPAD_REGISTER  ; put get* put GET *848  ; Starts: 751, [879]
      and #$03                    ; put get
      cmp #$01                    ; put get
      rol joypad1_down+1          ; put get put get put    ; This can desync, but we finish before it matters.
      bcc standard_loop2          ; get* put (get)   *860  ; Must not cross page boundary.
    
      ; Next cycle: 878
     .endif  ; CONFIG_JOYPAD_SIZE
    
      ; -- Reads are finished and we no longer need to be synced. Now we can calculate button presses here. --
      
      rts
    .endproc
    

## Example games

Games 

  * _[Thwaite](User_Tepples_Thwaite.xhtml "User:Tepples/Thwaite")_
  * _[Sliding Blaster](Action_53.xhtml "Action 53")_
  * _[NESert Golfing](https://rainwarrior.itch.io/nesert-golfing)_



Applications 

  * _[Theremin](Action_53.xhtml "Action 53")_
  * [Pently Sound Effects Editor](Audio_drivers.xhtml#Pently "Audio drivers")
  * The menu of any _[Action 53](Action_53.xhtml "Action 53")_ volume including a game or application with mouse support



Tests 

  * [Meece](https://github.com/pinobatch/little-things-nes/tree/master/meece), the first
  * [allpads](Emulator_tests.xhtml#Input_Tests "Emulator Tests")
  * [240p Test Suite](https://github.com/pinobatch/240p-test-mini)



## References

    

  * [Mouse](https://snes.nesdev.org/wiki/Mouse) at SNESdev Wiki



  1. ↑ [forum post](https://forums.nesdev.org/viewtopic.php?p=231607#p231607): Hyperkin SNES mouse investigation
  2. ↑ [forum post](https://forums.nesdev.org/viewtopic.php?p=236484#p236484): Hyperkin mouse reads have a speed limit
  3. ↑ Martin Korth. "[Fullsnes: SNES Controllers Mouse Two Button Mouse](https://problemkaputt.de/fullsnes.htm#snescontrollersmousetwobuttonmouse)".
  4. ↑ [forum post](https://forums.nesdev.org/viewtopic.php?p=231600#p231600): Hyperkin SNES Mouse cannot software-cycle sensitivity
  5. ↑ [FullSNES](http://problemkaputt.de/fullsnes.htm#snescontrollersmousetwobuttonmouse) \- Nocash SNES Mouse documentation
  6. ↑ [forum post](https://forums.nesdev.org/viewtopic.php?p=232667#p232667): SNES Mouse sensitivity measurements
  7. ↑ [forum post](https://forums.nesdev.org/viewtopic.php?p=232668#p232668): Hyperkin Mouse sensitivity measurements



Categories: [Controllers](Category_Controllers.xhtml), [Pointing devices](Category_Pointing_devices.xhtml), [Super NES](Category_Super_NES.xhtml)
