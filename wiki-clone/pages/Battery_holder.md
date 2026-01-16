# Battery holder

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Battery_holder) | View [other pages](Special_AllPages.xhtml#Battery_holder)

![](../wiki-images/Ambox_content.png) |  The information presented in this article is out of date and provides a riskier, more intrusive repair method. Better alternative methods exist, but the article has yet to be updated. As such, **Proceed with repairs at your own risk.**  
---|---  
  
Adding a **battery holder** to an NES Game Pak PCB with a dead battery lets a player use that Game Pak to save again. There are many tutorials online that show one how to replace batteries by using electrical tape to wrap the new battery onto the cartridge and hold it in place. This method does not seem very stable. Both methods listed herein differentiate themselves by replacing the battery with a battery holder, which allows for easy replacement of batteries as they wear out and ensures that the battery is always held in rigidly by the design of the holder itself. The first method details how to do this by hot-gluing the battery holder to the bottom side of the NES cartridge. The second method shows how to do this by hot-gluing the battery holder beside the NES cartridge. 

## Contents

  * 1 Disclaimer
  * 2 Materials
    * 2.1 Notes
  * 3 Method 1
    * 3.1 Opening the case
    * 3.2 Removing the battery
    * 3.3 Adding the battery holder
    * 3.4 Wiring the holder
    * 3.5 Finishing up
    * 3.6 Pros and cons
  * 4 Method 2
    * 4.1 Adding the battery holder
    * 4.2 Wiring the battery holder
    * 4.3 Finishing up
    * 4.4 Pros and cons
  * 5 Acknowledgements



## Disclaimer

This tutorial is not a guarantee that anyone utilizing the same method will have the same results. Even the youngest Nintendo cartridges are now 15 years old and applying heat to an electronic component shortens its lifespan. I can only guarantee that these methods have worked for me and I have had only positive results from employing them. Follow the instructions in this tutorial AT YOUR OWN RISK. 

## Materials

The first 11 items are shown in Figure 1 with numbers to show their location. Some of the other materials will be shown in their own sections. 

  1. 3.8mm Security Bit
  2. CR2032 Battery
  3. CR2032 Battery Holder
  4. Soldering Iron
  5. Diagonal Cutter
  6. Wire Stripper
  7. Hot glue gun (low temp)
  8. Solder
  9. 22 or 24 AWG (0.2-0.3 mm²) Stranded Wire
  10. Solder Wick
  11. Solder Sucker



Other materials: 

  1. Rubbing Alcohol
  2. Q-Tips (Between 2-6)
  3. Dremel or other rotary tool (Method 1 Only) 
     1. 2-56 x ¼” screws (Optional – used only if cart screws are lost)
  4. Needle-Nose pliers (Optional)



[![](../wiki-images/BatteryTutorial01.jpg)](File_BatteryTutorial01_jpg.xhtml)

[](File_BatteryTutorial01_jpg.xhtml "Enlarge")

Figure 1 – Tools and Required Components for Changing an NES Battery.

  


### Notes

The 3.8mm security bit is not available at stores and can only be purchased online. You will almost certainly want some sort of driver that the bit will attach to, so that you can make turning it easier. 

Make sure that the CR2032 battery holder is an insertion mount type rather than a surface mount type. A surface mount type would be much more difficult to apply with this method. I use Radio Shack model 270-009. 

You will want a soldering iron with a thin tip so that you only heat the components that you want to heat. The iron that I employee is 40 watts which may be too much for a small scale electronic application such as this one. If you are unused to soldering or have little confidence in your technique then I recommend using a 30 or 25 watt iron so that you do not cause unnecessary damage from heat. 

Choosing the right diagonal cutters is difficult. I often employ two of various sizes. This is because you want a small cutter to be able to work on the small confines of the board, but I have only been able to cut through the “prongs” holding the battery on to the circuit board by using a much larger cutter. 

For the wire strippers, simply choose a type that has a setting for the size of wire that you will be using. 

Make sure that your hot glue gun is the low temp variety. High temp may be stronger over a greater range of temperatures but it also takes much longer to set. The NES cartridge is unlikely to get hot enough to melt the low temp glue under normal operating conditions so it should be sufficient for your needs. However, if you have the time and wish to hold your battery holder in place long enough for high temp variety to set then that is also a valid option. 

I use 60/40 0.32 diameter rosin-core solder but any small gauge solder should work. 

## Method 1

This is my preferred method for replacing the batteries on NES carts with battery holders. 

### Opening the case

Take your NES security bit and apply it to the NES cartridge screws as shown in Figure 2. Turn counter-clockwise and turn slowly. The screws easily strip out the threads on the cartridge but don’t worry if you do that, we’ll cover a solution later on. 

[![](../wiki-images/BatteryTutorial02.jpg)](File_BatteryTutorial02_jpg.xhtml)

[](File_BatteryTutorial02_jpg.xhtml "Enlarge")

Figure 2 – NES Security Screw Locations

  


### Removing the battery

After you have removed the security screws, you should be able to locate the battery as shown in Figure 3. Please note that the boards within NES cartridges do vary so your board and the location of the battery may change but the battery itself will look the same as shown in the picture every time. From here on we will always refer to this side of the board as the “top”. 

You will note that the battery is held in place by two metal prongs – one below the battery and one above. Take your diagonal cutters and cut both prongs. Try to be careful by removing the battery in such a way that it doesn’t touch any part of the board. When you are done, your board should look like it does in Figure 4. 

At this point I would desolder the remains of the prongs. Use your solder wick and solder sucker in combination to remove enough solder to pull these free. Discussing soldering and desoldering technique is beyond the scope of this tutorial but there are plenty of references on the internet. The only real thing to keep in mind is that you should not apply heat to the board for extended periods of time as this can only shorten the life of the cart. If you are worried about your technique then I suggest practicing on old circuit boards or buying a demo board from Radio Shack or some other hardware store. 

[![](../wiki-images/BatteryTutorial03.jpg)](File_BatteryTutorial03_jpg.xhtml)

[](File_BatteryTutorial03_jpg.xhtml "Enlarge")

Figure 3 – NES Board with battery

  


[![](../wiki-images/BatteryTutorial04.jpg)](File_BatteryTutorial04_jpg.xhtml)

[](File_BatteryTutorial04_jpg.xhtml "Enlarge")

Figure 4 – NES board without battery

  


### Adding the battery holder

Take the CR2032 battery holder and find the end with the two pointed metal terminals that look like legs. Cut about half of the leg off with your diagonal cutters. Then “tin” the leg. Tinning refers to melting a small amount of solder on the leg which will make it easier to solder a wire to it easier. Now take your battery holder press it up against the edge of the board so that the main circular part of the holder is aligned with the flat edge of the board. I find it best to do this as close as possible to where the battery was positioned on the board in the first place (such as on the left hand side of the board shown in Figure 4). Also make sure that the battery holder’s “arm” which holds the battery down is also facing toward you and that the side of the holder where the arm is mounted is closer to the “+” terminal on the board. 

Now flip the entire set over so that you are looking at the “bottom” of the board and the battery holder. The previous step of aligning everything while looking at the top of the board can be skipped once you have done this enough times. While holding everything steady, now take your hot glue gun and glue the battery holder to the board. Hold the battery holder in place for some time to allow the glue to dry. Once it is dry you may want to add more glue in various locations to make a stronger hold. Make sure that you provide some support while doing this since the existing glue can be heated up by new glue and become soft again. The finished product should look like Figure 5. 

[![](../wiki-images/BatteryTutorial05.jpg)](File_BatteryTutorial05_jpg.xhtml)

[](File_BatteryTutorial05_jpg.xhtml "Enlarge")

Figure 5 – Battery holder glued to NES board (Method 1)

  


### Wiring the holder

You are now ready to wire from the “+” and “-“ terminals of the board shown in Figure 4 to the battery holder. Cut two pieces of wire longer than what you think that you need. Strip back the wire about ¼” and tin the exposed portion. Place each wire in the “+” and “-“ terminals placing them in from the bottom side of the board so that you can only see a small portion of the tinned wire on the top side of the board. Now flip the board so that the top is facing up and solder the wire in place on the board. 

If you have positioned the battery holder as shown in Figure 5 then each wire will connect to the side of the battery terminal closest to it. Now measure how much wire you need from end to the other , giving yourself about an extra ¼”. Now cut the wire at that point, strip back ¼”, and tin the exposed wire. Place the tinned portion of the wire against the leg of the battery holder. You should be able to briefly touch this pair with the solder iron to cause them to melt together. We do this step this way for speed so that the glue won’t get to hot and go soft. 

If you look at the top of the board now, it should look like Figure 6 shown below. 

[![](../wiki-images/BatteryTutorial06.jpg)](File_BatteryTutorial06_jpg.xhtml)

[](File_BatteryTutorial06_jpg.xhtml "Enlarge")

Figure 6 – Completed board with battery holder. (Method 1)

  


### Finishing up

Now place a battery in the CR2032 battery holder so that the “+” side is facing toward the arm. Place the board back in your NES cartridge shell as it was when you removed it. Put the two halves of the shell together and use the security bit to put the screws back in again. This time turn clockwise and remember to do it slowly or you will strip the threads on the cartridge. If this does happen or if you lose one of the screws then buy enough #2-56 x ¼” screws at your local hardware store for your needs and screw those in their place. These are large enough that they will create their own threads on the cartridge. 

### Pros and cons

The main advantage to this method is that there is no internal pressure in the cartridge. Nothing on the board sticks out enough for you to have to force the two sides of the NES shell together. 

The one con that I have heard is that the long-term strength of the glue is untested. While this is true, this method ensures that even if the glue does get soft and the battery holder does fall, nothing bad will happen. Basically, the battery holder will move down a millimeter or two and touch the NES shell which will then stop it from moving. The shell is made of an insulator and will not conduct electricity. At the same time, the wires are semi-rigid and will keep the battery from sliding around so there will be no damage. 

## Method 2

This is not my preferred method but it is tried and true and developed by a professional NES reproduction cart maker. After you have done steps 4.1 and 4.2, do the following: 

### Adding the battery holder

Turn the board so that the bottom is facing toward you. Place the battery holder on top of the board so that the legs are just hanging off of the longer edge opposite the copper terminals. Place it as near as possible on this edge to the position that the battery originally resided in. Turn the battery holder so that the side where the arm that holds the battery down is nearer the “+” terminal on the board as shown in Figure 4. 

Now take your hot glue gun and glue it in place. You should only have to keep the battery holder from moving around while you wait for the glue to set. The finished product should look like Figure 7 shown below. 

[![](../wiki-images/BatteryTutorial08.jpg)](File_BatteryTutorial08_jpg.xhtml)

[](File_BatteryTutorial08_jpg.xhtml "Enlarge")

Figure 8 – Battery holder glued to NES board (Method 2)

  


### Wiring the battery holder

You are now ready to wire from the “+” and “-“ terminals of the board shown in Figure 4 to the battery holder. Cut two pieces of wire longer than what you think that you need. Strip back the wire about ¼” and tin the exposed portion. Place each wire in the “+” and “-“ terminals placing them in from the top side of the board so that you can only see a small portion of the tinned wire on the bottom side of the board. Now flip the board so that the facing is facing up and solder the wire in place on the board. 

If you have positioned the battery holder as shown in Figure 8 then each wire will connect to the side of the battery terminal closest to it. Now measure how much wire you need from end to the other , giving yourself about an extra ¼”. Now cut the wire at that point, strip back ¼”, and tin the exposed wire. Place the tinned portion of the wire against the leg of the battery holder. You should be able to briefly touch this pair with the solder iron to cause them to melt together. We do this step this way for speed so that the glue won’t get to hot and go soft. 

If you look at the top of the board now, it should look like Figure 9 shown below. 

[![](../wiki-images/BatteryTutorial09.jpg)](File_BatteryTutorial09_jpg.xhtml)

[](File_BatteryTutorial09_jpg.xhtml "Enlarge")

Figure 9 – Completed Board with Battery Holder (Method 2)

  


### Finishing up

Now place a battery in the CR2032 battery holder so that the “+” side is facing toward the arm. Your battery holder has now made the NES board taller so you will have to modify your cart now. As you can see in Figure 10, there are two ridges on the inside of the cart shell. One of these is right above where the battery will reside and will hit against the battery. Take your dremel and grind down this ridge. You can see the after picture in Figure 11. 

[![](../wiki-images/BatteryTutorial10.jpg)](File_BatteryTutorial10_jpg.xhtml)

[](File_BatteryTutorial10_jpg.xhtml "Enlarge")

Figure 10 – Inside of the NES Shell

  


[![](../wiki-images/BatteryTutorial11.jpg)](File_BatteryTutorial11_jpg.xhtml)

[](File_BatteryTutorial11_jpg.xhtml "Enlarge")

Figure 11 – Inside of the NES Shell with the ridge ground down.

  


Now you will want to put the two halves of your NES shell together. Note that even with the ridge ground down, the cart will hit against the battery and you may have trouble putting the two halves together. Do not be surprised if you see something like Figure 12 if you are not applying any pressure to the two halves to keep them together. 

[![](../wiki-images/BatteryTutorial12.jpg)](File_BatteryTutorial12_jpg.xhtml)

[](File_BatteryTutorial12_jpg.xhtml "Enlarge")

Figure 12 – Shell using Method 2 with no pressure.

  


Take your NES security bit with one hand and force the two halves of the shell together with your others and screw in the security screws. Turn clockwise and turn slowly to prevent stripping the threads in the NES cartridge shell. Remember to use #2-56 x ¼” screws if stripping does occur. It is normal for the shell to make a creaking noise whenever you touch it due to the forces on the inside trying to push the two halves apart. 

### Pros and cons

I no longer use this method because I am concerned about the long-term effect that it will have on the shell and board mechanically. Because the shell is plastic, it bows slightly since the battery holder is constantly pressing against it. I fear that cracks will form and eventually the shell might break. It definitely has a bad effect on the threads in the NES carts. Their tendency to strip becomes much higher since the shell is constantly fighting against the screws to snap open and it is difficult to remove the screws slowly enough for this not to become a problem. 

The main pro is that this method definitely has far greater mechanical strength with respect to the battery holder. The battery holder is certain to remain in place rigidly on the board with this method. 

## Acknowledgements

Thanks to Nesreproductions.com for developing Method 2 which inspired me to create Method 1. 
