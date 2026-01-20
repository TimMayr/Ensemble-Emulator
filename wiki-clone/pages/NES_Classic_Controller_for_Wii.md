# NES Classic Controller for Wii

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_Classic_Controller_for_Wii) | View [other pages](Special_AllPages.xhtml#NES_Classic_Controller_for_Wii)

This tutorial will instruct you in how to build yourself a Mayflash NES/SNES-to-Wiimote controller adapter with a full fledged standard 7-pin NES connector instead of the provided DE-9 pin connector. 

## Contents

  * 1 Background
  * 2 Materials
  * 3 Procedure
  * 4 Video
  * 5 References



## Background

In late 2009, a Chinese peripheral manufacturer called Mayflash released an interesting controller adapter for the Wii. With this adapter, one could use NES or Super NES controllers on the Wii by connecting them to the adapter and then the adapter to a Wii Remote, essentially fooling the Wii into thinking these older controllers are in fact the Wii Classic Controller. 

Using NES or Super NES controllers on a Wii is really a matter of preference and nostalgia. Holding the Wii Remote on its side emulates the style of control that the NES provided. Holding a real NES controller is a lot more satisfying. If you didn't grow up with an NES, getting used to the brick design can be difficult. For you guys, I recommend the NES2 Dogbone. These are more difficult to find, but definitely more comfortable. The Wii Classic Controller is basically a dumbed down copy of the SNES pad with two thumbsticks attached. Most games that support the Classic Controller do not need either thumbstick and they just sort of get in the way. Most retro gamers agree that the original SNES controller is one of the best ever designed, so why wouldn't you want to use it on the Wii? 

The idea is to allow you to plug in your oldschool controllers both on the Wii and their original hardware. But unfortunately, A: Unfortunately, Mayflash designed their adapter to use a [DE-9](https://en.wikipedia.org/wiki/D-subminiature "wikipedia:D-subminiature") pin connector for NES controller functionality. In Asia, most clones of the NES hardware (commonly called Famiclones) use this type of controller connector. In America, Europe and even Japan, the Nintendo hardware used a NES 7-pin connector instead. 

In the 1980s, many videogame systems including the Atari 2600, Commodore 64 and even Sega Genesis used a standard nine pin controller port called a DE-9, which had a simple parallel interface for the buttons. This meant that many controllers were interchangeable with each other, allowing gamers to use Genesis controllers to play Atari games. 

However, the Nintendo Entertainment System (NES) used a proprietary 7-pin controller plug with an SPI-like serial interface. The two plugs are shaped very differently. This mod project involves replacing the DE-9 pin connector with a standard NES 7-pin. 

The Retroports that Retrozone sell are marketed as NES & SNES to Wii adapters, but they are in reality only NES & SNES to Gamecube adapters. This means that they work on all Gamecube games that support digital control, but on the Wii they only work with games that alllow the use of Gamecube controllers. Increasingly, more games are being released on the Wii that support the Wii Remote on its side and Classic Controller only, such as Tetris, Dr. Mario, Mega Man 9, and Mega Man 10. Also, since the Mayflash adapter plugs into the Wii Remote and not the Wii itself, it sort of makes the NES/SNES wired controllers wireless, preventing trip hazards. 

The Mayflash SNES controller has the correct size and pinout and does not need replacing. 

## Materials

You'll need a Wii console, a Wii Remote, and Mayflash NES/SNES to Wii adapter - various online stores, ebay NES and SNES controller - check your local used game stores. eBay has them, but be careful of knock-offs. Always look for Nintendo's logo. The design patents for the controllers have expired, but knock-offs will never have Nintendo written on them. 

salvaged female NES 7pin controller plug - Out of an old front-loading NES or a Four Score accessory. The ideal place to find one is if you have or know someone who has a dead/unwanted NES. Alternatively, check ebay for the part itself that someone has already removed or buy an NES for parts. Getting it out of the system is real easy - Simply remove the screws holding the case together, remove the RF shield and simply unplug one of the controller ports. Finally unscrew both screws on the black plastic piece that keeps the 7pin connectors in place. 

  
small star screwdriver - your garage/junk drawer/hardware store. soldering iron - hardware store. desoldering pump/braid - hardware store. scissors - your kitchen/department store glue - may not be needed, depending on your skill. a sharp work knife - your kitchen/craft store. small pair of pliers - garage/hardware store. 

## Procedure

  * Disclaimer: Following this tutorial should allow you to achieve desirable results, but that being said I take no responsibility for following my advice here. By continuing to read this you take full responsibility of damaging your hardware, burning or cutting yourself. Don't be a jackass and you should be fine.



1\. Take your Mayflash NES/SNES to Wii adapter apart by removing the four screws on the back. Set aside the top part of the shell along with the four small screws and the turbo fire button in a safe place, preferably in a bowl so you don't lose anything. 

2\. Unhook the cord and remove the PCB from the bottom tray. Place bottom tray in bowl with the other side and the screws. 

3\. Carefully examine the PCB. You'll notice that the Famiclone DE-9 plug is connected to the PCB using all nine pins, even though Famicom/NES controllers only use five pins. On the real hardware, the two extra pins to make of the 7pin connector were only used by specialty controllers such as the Zapper or PowerGlove. You will notice a pattern that looks like this: 
    
    
        _________
    1 \ o o o o o / 5
       \ o o o o /
       6 `"""""' 9
    

Note that this diagram shows the correct pins that face away from the PCB. In other words, the pins that are normally visible and plug into DE-9 controllers. 

4\. Use a soldering iron and soldering pump/braid to heat and remove the solder that holds the DE-9 pin plug. This is a slow process, be patient: Don't try to forcefully remove the pins as that will most likely result in damaging the entire PCB. 

5\. Once you have the old DE-9 pin adapter removed, either discard or keep for a future project. Either way, you're done with the stock DE-9 pin connector for now. 

6\. Prepare your salvaged NES 7pin adapter. If you're using one taken from an old NES, you can easily finish this project just by soldering the correctly colored wires. Wires may be different for third party NES female plugs, I'm not sure. 

I number the [controller pins](Controller_port_pinout.xhtml "Controller port pinout") this way, with, the wire color and the job each does: 
    
    
          .-
    GND - |1\
    CLK - |32\ - +5V
    OUT - |54| - D3
     D0 - |76| - D4
          '--'
    

  1. Brown Ground
  2. White 5 Volts
  3. Red Clock
  4. Purple Not Used on standard controllers.
  5. Orange Latch/Strobe
  6. Blue Not Used on standard controllers.
  7. Yellow Data



You will need only the brown, white, red, orange and yellow wires. Clip the blue and purple (used for expansion controllers like the [Zapper](Zapper.xhtml "Zapper") and [Power Pad](Power_Pad.xhtml "Power Pad")) right out of the way so you don't get confused by them. 

7\. Back where the DE-9 pin adapter was, remove any excess solder and ensure you can place wires in each small hole. Here is the DE-9 standard layout for most Famiclone DE-9 connectors, including the Mayflash: 

  1. N/A
  2. DATA
  3. LATCH/STROBE
  4. CLOCK
  5. N/A
  6. +5V
  7. N/A
  8. GND
  9. N/A



Reading the pins of a DE-9 is simple. The top row is 1-5 and the bottom is 6-9. 

DE-9 pins you need to solder to are in represented by "o". "x" shows pins that you can leave disconnected. 
    
    
        __________
    1 \ x o o o x / 5
       \ o x o x /
        6 `"""' 9
    

Just to double check specifically, solder the NES wires to these places: 

  * 2 Yellow
  * 3 Orange
  * 4 Red
  * 6 White
  * 8 Brown



8\. Simply solder away. Take your time. Remember to solder on the underside only. 

9\. Before reassembly, try it out. Your oldschool NES controllers should now work perfectly. 

10\. Depending on where you want to place the NES 7pin connector, use your knife to cut away plastic in the way and possibly glue it in place. On my adapter I simply cut away enough plastic for the new NES connector to fit snugly in place and tightened up the screws to hold it in place. No plug, no mess. 

11\. Boot up your favorite NES, TurboGrafx, Select Genesis WiiWare or Wii games including both Megaman 9 and 10 and enjoy them using an authentic NES controller - that will still also work with the real NES! 

## Video

<http://www.youtube.com/watch?v=WYWVrFq3Cu4>

## References

  * [SatoshiMatrix's post on BBS](http://forums.nesdev.org/viewtopic.php?p=60109#p60109)
  * [Newer version of tutorial; please import pictures](http://satoshimatrix.wordpress.com/2010/04/25/160/)


