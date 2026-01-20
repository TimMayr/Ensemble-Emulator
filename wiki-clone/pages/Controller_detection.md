# Controller detection

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Controller_detection) | View [other pages](Special_AllPages.xhtml#Controller_detection)

In some NES programs, it's worthwhile to support multiple controllers. Because different controllers use different protocols, a program supporting multiple controllers should follow a process of elimination to determine what protocol to use. This can be done after reset; real-time controller detection isn't very useful because connecting a controller often causes a voltage sag that freezes the CPU. However, this also means that the player needs to use the same controller for selecting an activity and for actually using it. And when a single cartridge contains activities designed to use different controllers, the menu has to support all of them. For example, the [Action 53](Action_53.xhtml "Action 53") menu supports a standard controller or Super NES Mouse in port 1, a standard controller in the Famicom expansion port, and the Zapper in port 2. 

The following procedure is intended to distinguish among a [standard controller](Standard_controller.xhtml "Standard controller"), a [Four Score](Four_player_adapters.xhtml "Four Score"), a Super NES [Mouse](Mouse.xhtml "Mouse"), an [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller"), a [Zapper](Zapper.xhtml "Zapper"), and a [Power Pad](Power_Pad.xhtml "Power Pad"). 

## Contents

  * 1 Detection
    * 1.1 Four Score
    * 1.2 Zapper
    * 1.3 Arkanoid controller
    * 1.4 Power Pad
    * 1.5 Mouse
    * 1.6 Standard controller
  * 2 Confirmation



## Detection

Much of the detection involves strobing the controller (write 1 then 0 to $4016) and reading nine times. Circuits that ignore the clock will return the same value nine times in a row; circuits that use the clock will return values that vary. So keep track of which bits were 0 throughout and which bits were 1 throughout. In some cases, detection will be inconclusive if the player is holding a button. Repeat the detection each frame until the player releases the button. 

### Four Score

A Four Score or Satellite has four controller ports and produces a 24-bit report. $4016 D0 has eight buttons for player 1, eight buttons for player 3, followed by a signature $10. $4017 D0 has eight buttons for player 2, eight buttons for player 4, and signature $20. The momentary movements of two mice may produce the signature, appearing as if players 3 and 4 held Right, but this is exceedingly unlikely. If player 3 is not pressing Right, and player 4 is not pressing Right, and the signature is present on both ports, **the controllers in both ports are a Four Score.**

The Famicom counterpart to the Four Score is the Hori 4 Players Adapter. It behaves just like a Four Score, with the ports wired so as to swap the signatures: $20 on $4016 D1 and $10 on $4017 D1. 

### Zapper

A Zapper has a photosensor (1=dark) on D3 and a trigger (1=half pulled) on D4. After a few lines into vblank, the photosensor should return dark. Strobe the controller ports and read $4017 nine times. If D3 is constant 1 and D4 is constant 0, **the controller is a Zapper.** A standard controller or mouse will never have D3=1, a correctly calibrated Arkanoid controller will never have D4 constant, and a Power Pad will have D3=1 starting at the ninth read. If both D3 and D4 are constant 1, the controller is either a Zapper with its trigger half pulled or a Power Pad with all buttons held down. 

### Arkanoid controller

An Arkanoid controller for NES has a fire button (1=pressed) on D3 and a control knob (serial) on D4. If D3 is constant 0 and D4 is not constant, **the controller is an Arkanoid controller.** If D3 is constant 1 and D4 is not constant, the controller could be an Arkanoid controller with the fire button held down or a Power Pad with buttons 1-2, 5-7, and 9-11 held down. Wait for the player to let go of the fire button. 

### Power Pad

The Power Pad has 8-bit serial on D3 and 4-bit serial on D4, in both cases followed by constant 1 values. We need not require the player to step off the Power Pad, just to not intentionally press all 12 buttons. If D3 is not constant, D4 is not constant, the bit corresponding to button 4 is 0, and the last 4 of the first 8 bits from D4 are 1, **the controller is a Power Pad.**

### Mouse

A [Super NES Mouse](Mouse.xhtml "Mouse") can be connected with an adapter to port 1 or 2 of an NES or AV Famicom or to the Famicom expansion port. It returns a 32-bit report on D0 or D1 whose 13th through 16th bits are the signature 0001. If this signature is present, **the controller is a SNES mouse.**

### Standard controller

If D4 and D3 are constant 0, and the controller is not a mouse or Four Score, **it's probably a standard controller.** The hardwired controllers on a Famicom and the 7-pin ports on an NES or AV Famicom are connected to D0. A Famicom expansion port adapter adds two additional controllers on D1. In Famicom games with one or two players, the user expects to be able to use the expansion controllers as players 1 and 2 in case the hardwired controllers are worn. 

The bits after the report aren't quite as predictable as they are with the other serial devices. The Super NES controller has four more buttons and four zero bits after the existing buttons. Some clone controllers return a stream of 0 after the report instead of a stream of 1. 

## Confirmation

Just because a controller is connected doesn't mean a player is holding it. Wait for a player to claim ownership of a controller before expecting further input from it. 

Standard controller, Four Score
    Wait for an A (10000000) or Start (00010000) press.
Mouse
    Show a sprite for the mouse pointer and controls for "Start" and the current mouse speed. Wait for the player to move the pointer over a control and press the left mouse button. Clicking the mouse speed changes it to the next setting; clicking Start confirms the mouse.
Zapper
    Once a player pulls the trigger, brighten the screen and count scanlines until the photodiode goes bright. If it doesn't, the player shot offscreen.
Arkanoid controller
    Wait for the fire button to be pressed.
Power Pad
    Wait for button 4 (top right) to be 0 then 1.
