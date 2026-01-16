# Zapper

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Zapper) | View [other pages](Special_AllPages.xhtml#Zapper)

[![](../wiki-images/Nintendo-Entertainment-System-NES-Zapper-Gray-R.jpg)](File_Nintendo_Entertainment_System_NES_Zapper_Gray_R_jpg.xhtml)

[](File_Nintendo_Entertainment_System_NES_Zapper_Gray_R_jpg.xhtml "Enlarge")

NES Zapper (gray)

The **Zapper** is a light gun, often associated with the game _Duck Hunt_. It reads light from a CRT SDTV and sends the brightness of the area where it is pointed on the controller port. 

The Zapper can be used in either [controller port](Input_devices.xhtml "Input devices"), though most games will only allow you to use it in controller port #2, leaving port #1 for a standard controller used for navigating through options, moving the view, changing weapons, etc. 

The Famicom Zapper is logically compatible, but can only be plugged into the [Famicom expansion port](Expansion_port.xhtml "Famicom expansion port pinout") and so only read from $4017 (i.e. controller port #2). The Vs. System Zapper is not compatible[1] (see below). 

## Contents

  * 1 Output ($4016/$4017 read)
  * 2 Light Sensor
  * 3 Sequence of operations
  * 4 Trivia
  * 5 External Links
  * 6 References



### Output ($4016/$4017 read)
    
    
    7  bit  0
    ---- ----
    xxxT WxxS
       | |  |
       | |  +- Serial data (Vs.)
       | +---- Light sensed at the current scanline (0: detected; 1: not detected) (NES/FC)
       +------ Trigger (0: released or fully pulled; 1: half-pulled) (NES/FC)
    

There are three hardware variants: 

NES
    Light sense and trigger are output on bit 3 and 4 of $4016 or $4017, depending on the port.
Famicom
    Light sense and trigger are output on bit 3 and 4 of $4017, as if the Zapper were plugged into port 2 of the NES. The pins for bits 3 and 4 in an AV Famicom's controller ports are normally not connected, but there is a published hardware modification to use an NES Zapper.
Vs. System
    This Zapper communicates with the same protocol as the [standard controller](Standard_controller.xhtml "Standard controller"), returning an 8-bit report after being strobed: 

    0, 0, 0, 0, 1, 0, Light sense (inverted), Trigger
    The "light sense" status corresponds to Left and the "trigger" to Right, and Up is always pressed.
    Unlike the NES/Famicom Zapper, the Vs. Zapper's "light sense" is 1 when detecting and 0 when not detecting.

Tests in the Zap Ruder test ROM show that the photodiode stays on for about 26 scanlines with pure white, 24 scanlines with light gray, or 19 lines with dark gray. For an emulator developer, one useful model of the light sensor's behavior is that luminance is collected as voltage into a capacitor, whose voltage drains out exponentially over the course of several scanlines, and the light bit is active while the voltage is above a threshold. 

The official Zapper's trigger returns 0 while the trigger is fully open, 1 while it is halfway in, and 0 again after it has been pulled all the way to where it goes clunk. The large capacitor (10µF) inside the Zapper when combined with the 10kΩ pullup inside the console means that it will take approximately 100ms to change to "released" after the trigger has been half-pulled. 

Some clone zappers, like the Tomee Zapp Gun implement a simpler switch that returns 1 while the trigger is not pulled, and 0 when it is pulled. This works with most existing zapper games which usually fire on a transition from 1 to 0. 

### Light Sensor

The light sensor in the NES Zapper has a filter tuned approimately to the CRT scanline frequency (~15 kHz). This helps filter out more slowly changing light signals (e.g. light bulbs), but unfortunately will strongly attenuate the light signal from many modern televisions (e.g. LCD based). Some clone zappers (e.g. Tomee Zapp Gun, or Simba's Jr) have a much weaker filter that responds more readily to these slower changing light sources. 

Light gun games also tend to expect no effective delay from the CRT, allowing the sensor to give a reading immediately as the picture is being drawn on the TV screen. This is also a problem for most modern televisions, which tend to have inherent delay (display lag). 

A combination of clone hardware with a weaker filter, and a software patch to compensate for the display lag delay can be effective for getting some zapper games to work with a modern television.[2]

### Sequence of operations

The common way to implement Zapper support in a game is as follows: 

  1. Each frame, check if the player has pulled the trigger. Keep running the game loop and remain in this step until the trigger is pulled.
  2. During vertical blanking, verify that the light gun is _not_ detecting light, to ensure that the player is actually pointing the gun at the screen. If bit 3 is false during vblank, a [standard controller](Standard_controller.xhtml "Standard controller") is probably plugged in. Do this near the end of your vertical blank code to let the light "drain out" if the gun happens to be pointed near the bottom of the screen. If you are using sprite 0 hit, a good time to read it is right after the sprite 0 hit flag turns off.
  3. Optional: Turn the entire screen white or display white boxes on all targets, and use timed code or a scanline IRQ handler to wait for the Zapper to start detecting light in order to see how far down the screen the Zapper is pointed. This can narrow the set of targets that must be checked in the next step.
  4. For each target the player may hit, display a black screen with a single white ($20) or light colored ($31-$3C) box at the target's location. Wait the entire frame to check if the light sense bit goes low. The sensor may turn on and off in 10 to 25 scanlines, so continue to check throughout a whole frame. If any of the targets is hit, register a 'hit' within the game; if not, move on to the next target or, if there are no additional targets, register a 'miss'.
  5. Restore the screen to its original state.



## Trivia

In _Wild Gunman_ (mode: GAME A: 1 OUTLAW), game's engine does not check what light gun is pointing at but just the time when trigger is pressed. As a result, this title is good choice nowadays for (partial) test of Zapper, where analog CRT TVs are quite rare to find. 

## External Links

  * [Forum post:](https://forums.nesdev.org/viewtopic.php?f=9&t=13021) Zapper test ROMs
  * [Forum post:](https://forums.nesdev.org/viewtopic.php?t=8108) Zap Ruder: The Zapper test ROM project
  * [Forum post:](https://forums.nesdev.org/viewtopic.php?f=3&t=11169) Zapruder calibration
  * [Forum post:](https://forums.nesdev.org/viewtopic.php?t=10138) Detecting screen (X,Y) location for the NES Zapper



## References

  1. ↑ [Forum post:](https://forums.nesdev.org/viewtopic.php?f=3&t=9917) VS zapper info?
  2. ↑ [NES zapper + LCD](https://forums.nesdev.org/viewtopic.php?p=225882#p225882): forum thread discussing the "NES LCD Mod" project.



Categories: [Controllers](Category_Controllers.xhtml)
