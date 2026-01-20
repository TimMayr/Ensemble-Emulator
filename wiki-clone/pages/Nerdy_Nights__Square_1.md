# Nerdy Nights: Square 1

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Nerdy_Nights%3A_Square_1) | View [other pages](Special_AllPages.xhtml#Nerdy_Nights__Square_1)

## Square 1 Channel

Let's make a beep. This week we'll learn how to produce a sound on the Square 1 channel. The Square channels are everybody's favorites because you can control the volume and tone and perform sweeps on them. You can produce a lot of interesting effects using the Squares. 

Square 1 is controlled via ports $4000-$4003. The first port, $4000, controls the duty cycle (ie, tone) and volume for the channel. It looks like this: 
    
    
    7654 3210  SQ1_ENV ($4000)
    |||| ||||
    |||| ++++- Volume
    |||+------ Saw Envelope Disable (0: use internal counter for volume; 1: use Volume for volume)
    ||+------- Length Counter Disable (0: use Length Counter; 1: disable Length Counter)
    ++-------- Duty Cycle
    

For our purposes, we will focus on Volume and Duty Cycle. We will set Saw Envelope Disable and Length Counter Disable to 1 and then forget about them. If we leave Saw Envelopes on, the volume of the channel will be controlled by an internal counter. If we turn them off, WE have control of the volume. If WE have control, we can code our own envelopes (much more versatile). Same thing with the Length Counter. If we disable it, we have more control over note lengths. If that didn't make sense, don't worry. It will become clearer later. For now we're just going to disable and forget about them. 

Volume controls the channel's volume. It's 4 bits long so it can have a value from 0-F. A volume of 0 silences the channel. 1 is very quiet and F is loud. 

Duty Cycle controls the tone of the Square channel. It's 2 bits long, so there are four possible values: 

  * 00 = a weak, grainy tone. Think of the engine sounds in RC Pro-Am. (12.5% Duty)
  * 01 = a solid mid-strength tone. (25% Duty)
  * 10 = a strong, full tone, like a clarinet or a lead guitar (50% Duty)
  * 11 = sounds a lot like 01 (25% Duty negated)



The best way to know the difference in sound is to listen yourself. I recommend downloading FamiTracker and playing with the different Duty settings in the Instrument Editor. 

For those interested, Duty Cycle actually refers to the percentage of time that the wave is in "up" position vs. "down" position. Here are some pictures: 

[diagram missing] 

Don't sweat it if graphs and waves aren't your thing. Use your ears instead. 

Here's a code snippet that sets the Duty and Volume for the Square 1 channel: 
    
    
       lda #%10111111; Duty 10 (50%), volume F (max!)
       sta $4000
    

$4001 controls sweeps for Square 1. We'll skip them for now. 

### Setting the note

$4002 and $4003 control the period of the wave, or in other words what note you hear (A, C#, G, etc). Periods are 11-bits long. $4002 holds the low 8-bits and $4003 holds the high 3-bits of the period. We'll get into more detail in a future tutorial, but for now just know that changing the values written to these ports will change the note that is played. 
    
    
    7654 3210  SQ1_LO ($4002)
    |||| ||||
    ++++-++++- Low 8-bits of period
    
    7654 3210  SQ1_HI ($4003)
    |||| ||||
    |||| |+++- High 3-bits of period
    ++++-+---- Length Counter
    

The Length Counter, if enabled, controls how long the note is played. We disabled it up in the $4000 section, so we can forget about it for now. 

Here is some code that will produce an eternal beep on the Square 1 channel: 
    
    
       lda #%00000001
       sta $4015 ;enable square 1
    
       lda #%10111111 ;Duty 10, Volume F
       sta $4000
    
       lda #$C9    ;0C9 is a C# in NTSC mode
       sta $4002
       lda #$00
       sta $4003
       
    

### Putting it all together

Download and unzip the [square1.zip](http://tummaigames.com/square1.zip) sample files. All the code above is in the square1.asm file. Make sure square1.asm and square1.bat are all in the same folder as NESASM3, then double click square1.bat. That will run NESASM3 and should produce the square1.nes file. Run that NES file in FCEUXD SP to listen to your beep! Edit square1.asm to change the Volume (0 to F), or to change the Duty Cycle for the square wave. Try changing the period to produce different notes. 

Categories: [APU](Category_APU.xhtml)
