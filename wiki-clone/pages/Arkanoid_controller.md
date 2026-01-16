# Arkanoid controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Arkanoid_controller) | View [other pages](Special_AllPages.xhtml#Arkanoid_controller)

The Arkanoid controller (commonly called the Vaus controller) was included with _Arkanoid_ and _Arkanoid II_ , and is only used for these and _Taito Chase HQ_. Multiple versions of this controller were produced. The Famicom and NES versions use different joypad bits and must be handled by software separately. The Arkanoid II controller has a [Famicom expansion port](Expansion_port.xhtml#Famicom "Expansion port") intended for multiplayer using another Famicom Arkanoid controller of either type. No version is compatible with [four player adapters](Four_player_adapters.xhtml "Four player adapters"). 

## Contents

  * 1 Layout
  * 2 Interface
    * 2.1 Input ($4016 write)
    * 2.2 Output ($4016/$4017 read)
    * 2.3 Control knob data
    * 2.4 Conversion
  * 3 Arkanoid II expansion port
  * 4 DPCM safety
  * 5 Hardware
  * 6 Test ROMs



## Layout

The controller has a single fire button and a control knob connected to a potentiometer. Some Arkanoid I controllers have a centering pot accessible under a plastic cover to adjust the control knob's value range; others do not have this pot at all. Arkanoid II controllers have this centering pot and also a scaling pot under this cover that adjusts the size of the control knob's value range, and have a Famicom expansion port next to the cord for connecting a second Arkanoid controller for multiplayer. 
    
    
    Arkanoid I controller:
                 ,--------------------------------------.   |
                 |                  %%                  |   |
                 |   ,---.          %%                % |   |
                 |  /     \         %%         ,-.   %% |   |
      Control--->| |       |        %%        (   )  %% |<- Fire button
        knob     |  \     /         %%         `-'   %% |   |    
                 |   `---'          %%                % |==/
     Centering ->|     ()           %%                  |
     pot cover   `--------------------------------------'
    
    Arkanoid II controller:
                 ,--------------------------------------.   |
                 |                  %%                  |   |
                 |   ,---.          %%                  |==/
                 |  /     \         %%       ,-.        |
      Control--->| |       |        %%      (   )       |<- Fire button
        knob     |  \     /         %%       `-'        |
                 |   `---'          %%                  |<- Famicom EXP port
    Calibration->|    (    )        %%                  |
     pots cover  `---------------------------------------
    

## Interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxC
            |
            +- Start ADC
    

When C goes from 0 to 1, a conversion is started. The results of this conversion are collected in a counter that is reset while C is 1, so C should be quickly returned to 0. Once the conversion is complete, the [shift register](4021.xhtml "4021") is automatically loaded with the results of the counter. Additional strobes during a conversion do not restart the conversion. Keeping C at 1 or strobing again before the conversion is complete will negatively shift the result's range (shift it left) based on when during the conversion C was last set to 0. 

The analog to digital conversion time depends on the control knob's position; smaller results will convert faster. Experimentally, the conversion takes up to 7 ms, so at least this amount of time should pass between strobes. Strobing again too soon will return bad data. 

_Arkanoid II_ and _Taito Chase H.Q._ perform a strobe of OUT1, but the controller cannot see this because the pin is not connected, so it has no effect. The reason for this strobe is not known. 

### Output ($4016/$4017 read)

Famicom: 
    
    
    $4016:
    7  bit  0
    ---- ----
    xxxx xxBx
           |
           +-- Fire button (1: pressed)
    
    $4017:
    7  bit  0
    ---- ----
    xxxx xxDx
           |
           +-- Serial control knob data (9-bit, inverted, MSb first)
    

NES: 
    
    
    7  bit  0
    ---- ----
    xxxD Bxxx
       | |
       | +---- Fire button (1: pressed)
       +------ Serial control knob data (9-bit, inverted, MSb first)
    

Fire button status is returned directly from the button and is not affected by the strobe. 

Control knob data is returned in 9 bits, most-significant-bit first. Each bit's data is returned inverted, but is discussed in this article according to its raw, pre-inverted state. Turning the knob left reduces the value, and turning it right increases the value. The data is stored in an 8-bit [shift register](4021.xhtml "4021"), which latches when a requested conversion completes. The 9th bit is the shift register's serial in, so it is only latched after the shift register has been read at least once. It is recommended that data be read before strobing, but because conversions take so long, the 8 latched bits can be safely read after strobing the controller and before the new conversion completes (and games rely on this). However, because the 9th bit is not initially latched, it is lost if a conversion is started before reading. 

Outside of a conversion, reading beyond the 9th bit simply repeats the 9th bit, because that bit is the shift register's serial in. During a conversion, the serial in is the current 9th bit according to the in-progress conversion, so each read latches transient conversion state that is returned 8 reads later. Software can use this to determine the rate at which the conversion is counting, which can be useful for calibration. 

### Control knob data

The control knob range depends on the controller and, if present, on the centering and scaling pot settings. Different games have different expectations. The physical knob has a 180 degree range and the canonical value range is believed to be approximately $140 steps (or $A0 steps for an 8-bit read), though the Arkanoid II controller's scaling pot can change the number of steps. The knob boundaries are set by plastic, so wear or additional force may result in values beyond the norm, and fast clockwise turns have been observed returning values briefly exceeding the maximum. 

To avoid requiring adjustment of the centering pot, which is not even present on all versions, new games can attempt to find the edges programmatically. Software can attempt to determine the number of steps by measuring the rate of the inner timer (see #Conversions below), and can set a range by watching for the leftmost position (smallest value) returned by the controller, because the leftmost edge is more reliable. Another approach is to track the minimum and maximum positions seen and use the middle $A0 steps. Note that values may wrap around, complicating this detection. 

Arkanoid I controller 8-bit measurements: 

  * Centering pot minimum: $0D-AD
  * Centering pot maximum: $5C-FC
  * Variations in the analog-to-digital conversion components and pot physical range can vary the results slightly.



Software expectations: 

  * _Arkanoid_ 8-bit range: $62-$02
  * _Arkanoid II_ 8-bit range: $4D-$E2 (normal paddle), $4D-$F2 (tiny paddle) 
    * With $A0 steps, the tiny paddle cannot reach right-side warps.



### Conversion

Conversions use two timers and a 12-bit binary counter. The inner timer runs repeatedly and increments the counter each time it expires, and its duration is determined by the scaling pot. The outer timer controls the length of the conversion, and its duration is determined by the control knob and centering pot. The value contained in the counter is latched by a [shift register](4021.xhtml "4021") upon completion of the conversion. The conversion completes when the outer timer expires, and the inner timer only runs during the conversion. 

From a hardware perspective, each timer works by charging a capacitor from empty up to a voltage threshold, where the pots control the speed at which the capacitor charges. If the timers are modeled using an integer count, this means the pots control the rate at which it increments, rather than the limit it is counting toward. This distinction matters if the pot is rotated during a conversion; the rate changes as the knob is turned, affecting the total accumulation so far. Players regularly turn the control knob while conversions are in progress, so this happens in practice on real hardware, but it may not be relevant for emulation. If the knob is treated as being at a fixed position throughout the conversion, then using a fixed timer length per position is sufficient. 

When the outer timer is not running, it holds the inner timer in reset and watches OUT0 for a rising edge. When this edge occurs, both timers begin running. A conversion always runs until completion; toggling OUT0 will not interrupt or restart an in-progress conversion. While OUT0 is 1, the counter is also continuously cleared. This causes the counter to ignore increments while set, and clears the counter if set mid-conversion. 

When the conversion ends, the inner timer is put back in reset and bits 8-1 of the counter are latched into the shift register. Bit 0 is the shift register's serial in, so this value is shifted into the shift register on every CPU read. Bit 0 maintains its value while a conversion is not running. During a conversion, it changes each time the counter increments, and reading brings its current value into the shift register. If the CPU repeatedly reads during a conversion, these transient values expose the rate at which the inner timer is incrementing the counter. 

Conversions have the following quirks: 

  * If OUT0 is 1 when a conversion would complete, the conversion instead continues until OUT0 is cleared. In hardware, this is because the timer capacitor's voltage must exceed that of a signal produced from OUT0, which it can't do when OUT0 is set.
  * There is a small race condition where a conversion may be able to start while the timer capacitors are discharging at the end of the previous conversion, causing them to start the next conversion with a non-zero value. It is estimated that the length of this race is on the order of 10 cycles for the outer timer and 1 cycle for the inner timer.



## Arkanoid II expansion port

_Arkanoid II'_ s controller has its own [Famicom expansion port](Expansion_port.xhtml#Famicom "Expansion port"). The connected device is able to see OUT0 and Joypad 2 /OE, and its Joypad 1 and 2 D1 outputs are redirected to Joypad 2 D3 and D4. No other signals are passed through. Because of this, any additional devices daisy-chained beyond two Arkanoid II controllers will be unable to send input to the console, but will still receive OUT0 and Joypad 2 /OE. 
    
    
     Arkanoid II EXP |  | Famicom EXP
    -----------------+--+-----------------
             GND   1 |--| 1   GND
     Joypad 2 D1   7 |->| 4   Joypad 2 D4
    Joypad 2 /OE   9 |<-| 9   Joypad 2 /OE
            OUT0  12 |<-| 12  OUT0
     Joypad 1 D1  13 |->| 5   Joypad 2 D3
              5V  15 |--| 15  5V
    
    (All other Arkanoid II EXP pins not connected)
    

This port is intended to be used with another Arkanoid controller, but is not limited to this. When used with an Arkanoid controller, $4017 works as follows: 
    
    
    $4017 read:
    7  bit  0
    ---- ----
    xxxD BxDx
       | | |
       | | +-- Controller 1 serial control knob data
       | +---- Controller 2 fire button
       +------ Controller 2 serial control knob data
    

It has been observed that adjustments on one controller's control knob can cause slight changes in the result from the other controller's knob. 

Although _Arkanoid II_ uses a [Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard") with [Family BASIC Data Recorder](Family_BASIC_Data_Recorder.xhtml "Family BASIC Data Recorder") for saving and loading custom levels, the device cannot work correctly over the Arkanoid II controller's expansion port because OUT2, a required input, is not connected. Thus, the keyboard must be plugged directly into the console to save and load. 

## DPCM safety

The Arkanoid controller is particularly susceptible to the bugs caused by DMC DMA when using DPCM playback (see [DMA register conflicts](DMA.xhtml#Register_conflicts "DMA") and [DPCM safety](Controller_reading_code.xhtml#DPCM_Safety "Controller reading code") for details). It is not compatible with the common [repeated reads](Controller_reading_code.xhtml#DPCM_Safety_using_Repeated_Reads "Controller reading code") solution because conversions take as long as nearly half a frame to complete, so [reads synced with OAM DMA](Controller_reading_code.xhtml#DPCM_Safety_using_OAM_DMA "Controller reading code") are required. However, even when using synced reads, the controller state may still become corrupted elsewhere. This is because any CPU read in the range $4000-401F can cause DMC DMA to trigger a joypad read, deleting bits from the Arkanoid controller's latched state before the controller reading code can safely access it. These reads are commonly present in sound engines as a side effect of absolute indexed stores (e.g. STA $4000,X). This problem can be worked around in two ways: 

  * Ensure that only the joypad reading code ever reads from the range $4000-401F. This may require modifying the sound engine or using a different sound engine.
  * Ensure that reads to $4000-401F do not occur between the time a conversion completes and when the latched data is read. This can be done by deliberately ordering the code so that these unsafe reads occur after the controller reading code, but before the strobe that starts another conversion. This requires that this unsafe code does not take so long that the conversion may not complete in time for the next controller read, something that is not a problem for most sound engines that are likely the only thing causing these unsafe reads. For most games, the ordering would be: vblank tasks, OAM DMA, synced controller reads, sound engine, controller strobe, game logic.



Software can also attempt to detect and ignore corrupted controller reads by checking for spurious accelerations, where the [second difference](https://en.wikipedia.org/wiki/finite_difference "wikipedia:finite difference") _x_ _t_ \- 2 _x_ _t_ \- 1 \+ _x_ _t_ \- 2 exceeds about eight units. 

## Hardware

The controller uses a 556 timer (containing 2 timers), a 4040B 12-bit binary counter, and an 8-bit shift register. One timer repeatedly clocks the binary counter and the other measures the control knob position. A centering pot adjusts the control knob timer and a scaling pot adjusts the counter timer. When OUT0 goes high, the counter clears and the control knob timer starts and releases the counter timer from reset, allowing it to begin incrementing the counter. When the control knob timer expires, it puts the counter timer back into reset and latches bits 8-1 of the counter into the shift register. Bit 0 of the counter connects to the shift register's serial input. 

## Test ROMs

  * [Vaus Test](https://forums.nesdev.org/viewtopic.php?t=10662) shows adaptation to trimpot setting, repeated read rate, and first and second differences
  * [9-bit results](https://forums.nesdev.org/viewtopic.php?t=23801) shows and visualizes the 9-bit result
  * [ADC conversion snooping](https://forums.nesdev.org/viewtopic.php?p=294425#p294425) shows the number of counter toggles detected in a fixed period of time



Categories: [Controllers](Category_Controllers.xhtml)
