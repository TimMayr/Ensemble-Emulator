# The frame and NMIs

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/The_frame_and_NMIs) | View [other pages](Special_AllPages.xhtml#The_frame_and_NMIs)

The [PPU](PPU.xhtml "PPU") generates a video signal for one frame of animation, then it rests for a brief period called vertical blanking. The CPU can load graphics data into the PPU only during this rest period. From [NMI](NMI.xhtml "NMI") to the pre-render scanline, the NTSC NES PPU stays off the bus for 20 scanlines or 2273 cycles. Taking into account overhead to get in and out of the NMI handler, you can probably use roughly 2250 cycles. To get the most out of limited vblank time, don't compute your changes in vblank time. Instead, prepare a buffer in main RAM (for example, use unused parts of the stack at $0100-$019F) before vblank, and then copy from that buffer into VRAM during vblank. On NTSC, count on being able to copy 160 bytes to nametables or the palette using a moderately unrolled loop, plus one 256-byte display list to OAM. 

Original Source: [The frame and NMIs](http://web.archive.org/web/20090714010738/http://nesdevhandbook.googlepages.com/theframe.html) by Disch 

## Contents

  * 1 VBlank, Rendering Time, and NMIs
  * 2 Separating Drawing from Logic
  * 3 Buffer Formats
  * 4 Buffering Other Things
  * 5 When to Turn Off PPU, NMIs
  * 6 Take Full Advantage of NMI
  * 7 Being Interrupt-Aware



## VBlank, Rendering Time, and NMIs

Many tutorials don't stress how important understanding the frame layout is - in fact, most don't even cover it. At best, they'll casually say something like "you can only draw during VBlank", leaving the reader wondering what in the world VBlank is. This lack of understanding is a major cause of bugs for new NESDev'ers. 

Contrary to how it may seem when you run these programs, the code is not executed instantaneously. All code takes a little bit of time to run. While the CPU is spending this time reading your program and executing code, the PPU is simultaneously doing its own work, like things related to outputting video to the TV. When your program communicates with the PPU via one of the registers, you will be poking it at different stages in this process. During some stages, the PPU will be too busy to deal with you, and you will screw things up by trying. This is why it's important to know what stage the PPU is in. 

The PPU operates on a series of _frames_. The PPU does all this work to output a frame to the TV, then it repeats the exact same process for the next frame, and the next, and the next. The frame can further be split into two generalized sections: _VBlank_ and _Rendering time_. 

Rendering time makes up the bulk of the frame. This is the time during which the PPU is fetching tiles, evaluating sprites, and outputting pixels to the screen. The PPU is _very_ busy during this time, so busy that if you try to access it with drawing code, you will screw it up and have visible and possibly disastrous glitches in your game. VBlank, on the other hand, is when the PPU is idle. This is when you can do all of your drawing code safely. 

A good way to visualize this by thinking of a clock face, and imagining one frame as one hour. VBlank would be a small portion of that hour, say the first 5 minutes (when the minute hand is between 12 and 1). _No matter what you do in your program_ the minute hand is always spinning around the clock, moving in and out of that 5-minute butterzone... and the PPU is always moving in and out of VBlank. In order to update the PPU in your game, you must make sure your drawing code falls within that small period of time. Failure to do this will cause all sorts of display glitches. 

To make matters even more complicated, as a programmer you have no way to actually see this clock in your code, so there is no way to tell whether or not the PPU is currently in VBlank, or how close to VBlank it is, or how much VBlank time is left. There _is_ , however, a way your program can be notified when VBlank first starts (as soon as the minute hand hits the 12 on the clock). This notification comes in the form of an NMI, or "non-maskable interrupt". 

Every time that clock hand reaches the 12 and the PPU starts VBlank again, the PPU will attempt to notify you that this has happened. It does this by sending an NMI to your program. These NMIs can be disabled so that you don't have to use them all the time (this is controlled via [$2000 (PPUCTRL)](http://wiki.nesdev.org/w/index.php/PPU_registers#Controller_.28.242000.29_.3E_write)). However, in a game you must use them because they are the _only_ [reliable] way to catch VBlank (and thus allow for realtime drawing without turning the PPU off). Plus they are the easiest, most natural, and most convenient logic framerate regulator. 

Now, the drawing code I'm talking about so far is stuff that is to be drawn while rendering is enabled. In your program, you _can_ disable rendering (aka, "turn off the PPU") via [$2001 (PPUMASK)](http://wiki.nesdev.org/w/index.php/PPU_registers#Mask_.28.242001.29_.3E_write). If rendering is disabled, it is perfectly sane to perform drawing code at any time (even during rendering time). This, however, does not stop that clock hand from moving. Even when 'disabled'. The PPU is still moving in and out of VBlank, and NMIs will still be generated (if enabled). 

Note, however, the clear difference between NMI and VBlank. NMI is a notification, whereas VBlank is a time period. A lot of newbies wonder why their drawing code is spilling outside of VBlank when they finish it all before their `rti`. They think that because NMI happens at the start of VBlank, then `rti` must be the end of VBlank. This, of course, is totally wrong. As soon as VBlank happens, it's like a race for you to get all of your drawing code done before time is up. Time may run out long before you hit your `rti`. 

## Separating Drawing from Logic

The key to ensuring that all of your drawing code happens in VBlank is to separate your drawing code from your logic code. A lot of newbies have a "do it now" mentality when it comes to drawing. Say, for example, they want to draw a text message to the user when the A button is pressed. They might try something like this: 
    
    
    lda a_button
    bne draw_message
    

This might seem like a good way to do it, but it is bad, bad, _bad_. You should almost never write code like this. This is mixing logic code (examining user input and deciding what to do) with drawing code (actually outputting something to the screen). 

The reason this is bad is because drawing code requires you be in VBlank. Therefore the above code must be run in VBlank because it draws. However it also does non-drawing stuff (logic), which can be done any time. Putting logic code in VBlank means you're wasting precious VBlank time on code that doesn't need to use it. This means you burn up your VBlank time that much faster, which reduces the amount of drawing you can get done in a frame, and increases the risk of having drawing code spill outside of VBlank. 

A solution to this is to _flag_ that the drawing needs to be done, and then actually do it next VBlank. This could be done like so: 
    
    
      ;; when processing game logic
      lda a_button
      beq :+          ; if a was pressed...
        inc drawflag  ; set the draw flag
    :
    
      ;; --------------------
      ;;  then, next vblank:
    
      lda drawflag
      beq :+              ; if draw flag is set
        jsr draw_message  ; actually draw the message
    :
    

However, this in and of itself isn't much of an improvement. `draw_message` might require some additional logic of its own. And this method isn't very versatile at all. What if there are 10 different ways the screen could update? Do we keep 10 flags and check them all every VBlank? Not very efficient. In practice, you'll want to do something a little more generalized than this, something that will work with virtually everything. 

The better solution to this is to _buffer_ your drawing. That is, you copy what you want drawn somewhere to memory, then copy it to the PPU next VBlank. This extra copy might seem wasteful, but in practice it isn't really. It's not as efficient as direct drawing, but the idea is you're sacrificing Rendering time (which you have lots of), to gain more VBlank time (which you have very little of). Doing a little extra work in your logic code to ease up on your drawing code goes a long way. 

Buffering is a very common and practical way to accomplish drawing. In fact, you've probably already done it with sprites in your programs. When you update sprite data, you don't write to [OAM ($2004)](http://wiki.nesdev.org/w/index.php/PPU_registers#OAM_data_.28.242004.29_.3C.3E_read.2Fwrite) directly (or at least I should hope not), you write to **shadow OAM** (i.e. a 256-byte region in standard system RAM, most commonly $0200-$02FF) and then later copy that to OAM with Sprite DMA ($4014). That is the same concept that we're doing here. Instead of drawing to the PPU immediately, we're getting it ready to be drawn, and then saying "don't draw it now, draw it next VBlank". 

## Buffer Formats

In order to employ this buffering technique, you need to give yourself room for the buffer in RAM. Just like you need to designate a full page of RAM to shadow OAM, you should probably designate a significant portion of RAM to your drawing buffer. It doesn't have to be a full page, but you don't want to run out of space. 

You also need to decide on a data format in which to store the information that tells your drawing code what to draw, and how to draw it. The best way to do this is to have the drawing code know as little as possible about the rest of your program. Make this format as generic and flexible as possible. Most techniques use a system where you have a chain of "_strings_ " and each "string" tells the drawing code what to draw, where, and how. This is actually much simpler than it sounds. For example, here's a very simple implementation of such a format, and an example of how it would be used: 
    
    
      byte    0 = length of data (0 = no more data)
      byte    1 = high byte of target PPU address
      byte    2 = low byte of target PPU address
      bytes 3-X = the data to draw (number of bytes determined by the length)
    

If the drawing buffer contains the following data: 
    
    
     05 20 16 CA AB 00 EF 05 01 2C 01 00 00 
      | \___/ \____________/  | \___/  |  |
      |   |         |         |   |    |  |
      |   |         |         |   |    |  length (0, so no more)
      |   |         |         |   |    byte to copy
      |   |         |         |   PPU Address $2C01
      |   |         |         length=1
      |   |         bytes to copy
      |   PPU Address $2016
      length=5
    

Your drawing code, upon coming across this data, would then do the following: 

  1. Copy 5 bytes (`CA AB 00 EF 05`) to PPU address $2016
  2. Copy 1 byte (`00`) to PPU address $2C01
  3. Come across a length of 0, and thus stop drawing.



To make this work, you fill _this buffer_ during your logic code instead of drawing straight to the PPU. Then in VBlank, you read this buffer and copy the data to the PPU. Because the format lays it out in simple terms, there isn't any heavy calculation that needs to be done to perform this drawing, and thus the drawing can be done very quickly, helping to make sure that it doesn't spill out of VBlank. A format like this is also very flexible. In fact if you make the format flexible enough, you should be able to use it for _everything in the game_. 

The above format, while simple and somewhat flexible, does have one major flaw. That being it doesn't allow you to specify whether or not you want to draw vertical rows of tiles (inc-by-32). To add that functionality, you could tweak the format by adding a _flags_ byte: 
    
    
      byte    0 = length of data (0 = no more data)
      byte    1 = high byte of target PPU address
      byte    2 = low byte of target PPU address
      byte    3 = drawing flags:
                    bit 0 = set if inc-by-32, clear if inc-by-1
      bytes 4-X = the data to draw (number of bytes determined by the length)
    

Same idea, but now we can draw rows or columns of tiles! This makes the format generic enough so that you can use it to do any drawing task. 

But wait. We can add even more to this. What if the data we want to draw is already sitting in ROM somewhere? It's a little wasteful to copy it to RAM, then copy it to the PPU when we can just copy it straight from ROM. Maybe you want to tweak the format a bit to allow for that: 
    
    
      byte    3 = drawing flags:
                    bit 0 = set if inc-by-32, clear if inc-by-1
                    bit 1 = set if copy-from-ROM, clear if copy-from-RAM
    
         if copy-from-RAM:
      bytes 4-X = the data to draw (number of bytes determined by the length)
    
         if copy-from-ROM:
      bytes 4,5 = CPU address from which to read the data
    

But _wait_ , there's more! What if you want to draw a bunch of zeros? Like to clear a row of tiles or something? Why copy a bunch of zeros to RAM when you can put in a basic RLE scheme: 
    
    
      byte    3 = drawing flags:
                    bit 0 = set if inc-by-32, clear if inc-by-1
                    bit 1 = set if copy-from-ROM, clear if copy-from-RAM
                    bit 2 = set if RLE, clear if not RLE
    
         if copy-from-RAM, not RLE:
      bytes 4-X = the data to draw (number of bytes determined by the length)
    
         if copy-from-ROM, not RLE:
      bytes 4,5 = CPU address from which to read the data
    
         if RLE:
      byte    4 = single byte to repeat 'length' times
    

The possibilities are endless! Beware, however, that the more of these special conditions you add, the bigger and slower your drawing routine gets, which means less stuff can be drawn. It's up to you how far you want to go before drawing the line. Don't get too carried away with super complicated features you don't really need. Remember, the whole point of this is to simplify and quicken the process of drawing, not to make the most feature-rich routine imaginable. 

One buffer format appearing in several games is [Stripe Image](Tile_compression.xhtml#NES_Stripe_Image_RLE "Tile compression"). It supports both the increment and RLE modes but doesn't support copying from ROM. The [Popslide library](https://forums.nesdev.org/viewtopic.php?f=22&t=15440) can be used to rapidly copy a Stripe Image buffer from a buffer on the stack page to VRAM. 

## Buffering Other Things

Drawing code is not the only thing that can be buffered. Changes in PPU state, such as scroll changes, turning the PPU on/off, etc, can (and should) be buffered as well, provided the effect truly isn't desired immediately (timed raster effects, for example, might be desired immediately, but that's a later topic). 

The first instinct for the newbie is to turn the PPU on and off directly. They'll finish all the drawing they needed to do to get the first screen to display, and they go to switch the PPU on: 
    
    
    lda #%00011000
    sta $2001
    

They then are shocked to see that when they do this, the screen "jumps" because the PPU was in the middle of Rendering time when they turned it on. The solution for this is simple: Buffer the changes to $2000 (PPUCTRL) and $2001 (PPUMASK). I say you should keep variables called `soft2000` and `soft2001` (really, call them whatever you like). You should then copy these values to the real $2000/$2001 next VBlank. With this setup, when you want to turn the PPU on, you can do the following: 
    
    
    lda #%00011000
    sta soft2001
    

And take comfort knowing that the PPU will be [safely] turned on next VBlank. This works for turning the PPU off as well, or any other state change. If you need the change to happen "_now_ ", like for instance if you want to turn the PPU off so you can draw a full new screen to the nametable you'd need to shut the PPU off before you could do any drawing. You can do the following: 
    
    
    lda #%00011000
    sta soft2001
    jsr WaitFrame
    

...where `WaitFrame` is a routine which waits for an NMI to occur before returning. This will ensure the state is changed before the logic code proceeds, but also makes sure the change is safe (in VBlank). 

## When to Turn Off PPU, NMIs

Drawing can be separated into two general types of drawing. You have _bulk drawing_ which will happen during a major transition in a game, and you'll have _updating_ which will happen constantly as the user is playing the game. 

For an example of each of these types, let's say you're making an RPG where the player walks around a scrolling map. When the player takes a step, new tiles will have to be drawn to show the area of the map they're walking towards. This is an example of _updating_. Now when the player gets in a battle, or enters a town or dungeon, you'll have to draw an entirely different image on the screen. This is an example of _bulk drawing_. 

Updates should be done in the method we've talked about. Buffer them, and make sure they happen in VBlank. The reason we need VBlank is because you can't switch off the PPU in order to do the drawing otherwise the visual display will black out. Could you imagine the screen flashing black every time you took a step? 

Conversely, bulk drawing should _not_ be done with the methods described so far. Buffering bulk drawing serves no benefit, since all the drawing couldn't fit in VBlank time anyway. What's more, turning the screen off and having it black for a few frames might even be _desirable_ during such transitions. Therefore, you should [safely] switch the PPU off, then do your bulk drawing. It doesn't even hurt to put the bulk drawing right in with the logic code! Go ahead and mix it right in. The only reason that was a bad idea before is because you want to keep unnecessary logic code out of precious VBlank. But now since the PPU is off and we can draw whatever we want, that's a nonissue. 

The one exception to the "everything is safe if PPU is off" rule is palette updates. Palette updates should be in VBlank (buffered), even if the PPU is off. This is due to a particular quirk of the NES PPU where pointing its I/O address at the palette while rendering is disabled will cause it to display that particular color on the screen - it won't _kill_ anything, but you'll have a weird "rainbow stripe" flash across your screen. This isn't terribly relevant if your game is still booting up, though. 

Now you might be thinking, "Well if I'm doing a bulk drawing routine, I want to disable NMIs too because I don't want an NMI to interrupt me while I'm drawing." That is a reasonable thought, but ultimately it's a bad way to go. As a general rule of thumb, _leave NMIs on all the time_. The only time they should be off is during your startup code where you're initializing everything. After that, once you turn them on, leave them on unless you have a very compelling reason to turn them off. There are many reasons for this: 

  * If you code your NMI handler properly, it will know to do nothing when it has nothing to do. There's no real harm in letting it run even if you don't need it to (other than losing a very small handful of cycles).
  * There are things you might want to happen every frame, even during these transitions. For example, you might want the music to keep playing rather than require it pause/stop during the transition. This can be done by updating the music engine in your NMI handler, but is virtually impossible to do if you disable NMIs.
  * Once you turn NMIs off, it is very easy to forget to turn them back on. What's worse, if you forget to read $2002 and you turn them on in the middle of VBlank, it will immediately trigger an NMI and cause your handler to run past the end of VBlank, starting all sorts of havoc.



## Take Full Advantage of NMI

Another newbie issue is that they are often unsure how to use NMI. Some of them either have NMI change a single variable and exit immediately ("everything in main"), or they enable NMIs, then run the main code into an infinite loop, and have their whole game run from the NMI handler ("everything in NMI"). Both of these are ill advised for a full, complex game. If you've done things this way, don't feel bad - even some _commercial games_ do it like that. So it's not that those methods don't work, it's just that they're not as advantageous. 

What you should realize is that your NMI handler is extremely valuable in that (provided you leave NMIs on) it is the only area of code in your program that is _guaranteed_ to run every single frame. Not only every frame, but every _VBlank_. Utilizing this gives you a much tighter grip around how you want the game to operate. 

The shortcomings of these "everything in XXX" approaches are that they handle game slowdown poorly, and prevent you from doing some basic things that you should be able to do. For example, taking the "everything in NMI" approach causes all sorts of problems: 

  * You probably have to disable NMIs when one trips, then re-enable them when logic is done. Otherwise an NMI could trip when you're still doing NMI stuff, which could be disastrous.
  * You can't keep the music updated during slowdown, or during bulk drawing transitions, because NMIs will be disabled.
  * Slowdown will cause you to "miss" frames. If you're doing a screen split or other raster effect, you won't be able to set it up properly, and the status bar or other effect will be visually glitchy in the next frame.



The "everything in main" approach is none better, and it suffers from the same problems, just introduced differently. 

So how do you use NMI to take full advantage of it? All you have to do is keep it enabled all the time, and have it do things that you will always want done every frame. You can have a series of flags to make some tasks conditional for some frames (so you don't have to draw anything if there's nothing to draw, etc). Here's an example of what a better NMI handler and some supporting routines might look like: 
    
    
       ;---------------------------------------
    
       ;  note I use 'varname = x' for simplicity, but there are many good reasons not
       ; to use this method in a real game.  Assume all these variables have a
       ; unique space in memory
    
       ; designate a oam and drawing buffer
       oam          = $0200    ; shadow oam
       drawingbuf   = $0300    ; buffer for PPU drawing
    
       ; other variables
       soft2000     = x    ; buffering $2000 writes
       soft2001     = x    ; buffering $2001 writes
    
       needdma      = x    ; nonzero if NMI should perform sprite DMA
       needdraw     = x    ; nonzero if NMI needs to do drawing from the buffer
       needppureg   = x    ; nonzero if NMI should update $2000/$2001/$2005
       sleeping     = x    ; nonzero if main thread is waiting for VBlank
    
       xscroll      = x
       yscroll      = x
    
    
       ;--------------------------------------
       ; WaitFrame - waits for VBlank, returns after NMI handler is done
    
       WaitFrame:
         inc sleeping
         @loop:
           lda sleeping
           bne @loop
         rts
    
       ;--------------------------------------
       ; DoFrame - same idea as WaitFrame, but also does some other stuff
       ;   that the game logic will want done every frame.  Things that
       ;   shouldn't be put in NMI
    
       DoFrame:
         lda #1
         sta needdraw
         sta needoam
         sta needppureg
         jsr WaitFrame
         jmp UpdateJoypadData
    
       ;--------------------------------------
       ; NMI - the NMI handler
    
       NMI:
         pha         ; back up registers (important)
         txa
         pha
         tya
         pha
    
         lda needdma
         beq :+
           lda #0      ; do sprite DMA
           sta $2003   ; conditional via the 'needdma' flag
           lda #>oam
           sta $4014
    
      :  lda needdraw       ; do other PPU drawing (NT/Palette/whathaveyou)
         beq :+             ;  conditional via the 'needdraw' flag
           bit $2002        ; clear VBl flag, reset $2005/$2006 toggle
           jsr DoDrawing    ; draw the stuff from the drawing buffer
           dec needdraw
    
      :  lda needppureg
         beq :+
           lda soft2001   ; copy buffered $2000/$2001 (conditional via needppureg)
           sta $2001
           lda soft2000
           sta $2000
    
           bit $2002
           lda xscroll    ; set X/Y scroll (conditional via needppureg)
           sta $2005
           lda yscroll
           sta $2005
    
      :  jsr MusicEngine
    
         lda #0         ; clear the sleeping flag so that WaitFrame will exit
         sta sleeping   ;   note that you should not 'dec' here, as sleeping might
                        ;   already be zero (will be the case during slowdown)
    
         pla            ; restore regs and exit
         tay
         pla
         tax
         pla
         rti
    
       ;-----------------------------------------
    

With the above code structure, all your program has to do is keep NMI enabled, and all your drawing, scroll setting, and music updating will be done automagically. All you have to do from there is put the stuff you want drawn in your drawing buffer, and `jsr` to 'DoFrame' to keep the game going. It couldn't be easier! This will allow you to keep your logic code easy to follow and straightforward, as well as providing all the other benefits talked about in this doc. 

A few things to note about the order and the way in which this example routine does things: 

  * Timing-critical stuff (drawing code) is done first. Then it moves on to setting the scroll, then to stuff that can be done any time (music updating).
  * Nearly everything is conditional (you can make everything conditional if you want). Making the drawing and DMA conditional will prevent the NMI from drawing data that is "not yet finished" (for example, if NMI occurs while you're filling one of the buffers).



## Being Interrupt-Aware

The above mentioned approach might seem bulletproof. NMI just does its own thing, the logic code is freed from timing concerns, and all you have to do to make it work is update a few variables when you want an onscreen change. Really, though, this approach opens itself up to a different set of problems that the "everything in XXX" methods don't normally have to worry about. Because your handler for NMIs (and your handler for IRQs, for that matter) are doing a substantial workload, and because they are left enabled most/all of the time, they can cause conflicts with your main code and can really cause you serious problems if they happen when your logic code isn't expecting them. In order to prevent these _conflicts_ from occurring, your code needs to be _interrupt-aware_. 

Conflicts, in this context, are when the NMI/IRQ changes something (either a variable, reg, or other system state) that the main code is using, resulting in the main code doing something _totally_ unexpected. A very simple way to visualize this is changing a 6502 reg: 
    
    
    ;;  if this is your NMI handler
    NMI:
      lda #0
      rti
    
    ;;  and if you do this somewhere else in your program
    
    RefillPlayerHP:
      lda playermaxhp
      sta playerhp
      rts
    

Do you see the problem? It's subtle, but `RefillPlayerHP` might actually end up _killing_ the player! The reason why is because NMIs can happen any time, and if an NMI happens between your `lda` and `sta` commands, you get totally screwed. Here's a flow of what might happen: 
    
    
      lda playermaxhp
                       ; -------->NMI------>
                                          lda #0
                                          rti
                       ; <--------RTI<------
      sta playerhp
      rts
    

The problem is much more obvious when you draw it out like this. You can see that if an NMI happens at that point, you'll be setting the player's hp to zero, instead of refilling it. 

Believe it or not, this is _much_ more likely to happen than you might think. And if you're using IRQs, it's almost a guarantee that cases like this will come up. What's worse, these problems are _very_ hard to diagnose from the symptoms, because they're not easily reproduced. This is why it's so important for your code to be interrupt-aware from day 1. 

An easy and very common solution to this particular problem is to backup and restore the 6502's registers by pushing them to the stack first thing in your NMI and IRQ handlers and then restoring them just before your `rti`. This is why many examples (including my example NMI handler above) start with the `pha txa pha tya pha` business. This ensures that A, X, and Y will be unchanged from their original value when NMI exits. 

It's not just cpu registers, though. Variable conflicts can also occur. Say you have a generic `ptr` variable that you use somewhere in your logic code to read map data that you're loading. And say the NMI handler is also using it in its drawing routine. The conflict here happens if an NMI occurs during your map loading routine. NMI will overwrite the pointer with what it needs, and when control returns to the main path, you'll start reading map data from whatever NMI was pointing to, rather than what you really wanted. 

The solution here is also very simple and easy. Just _do not_ have NMI or IRQ share variables or RAM space with your main code (or with each other -- remember that while an IRQ during NMI is impossible, an NMI during IRQ is very possible!). Of course, you'll still need some variables that both your interrupts and your main code use in order to communicate between them, such as `needdraw` in the above example. 

Beware, however, that these variables need to be quickly accessible. You are most vulnerable to conflicts when something critical takes multiple instructions. For example, looking at the above you might think "He's got 3 'needsomething' flags up there, and they're all separate variables. That's wasteful. I'm going to combine all of those into a single variable where each bit is a flag." Sounds smart, right? Use 1 byte of RAM instead of 3, and all the flags are consolidated into a single variable. The downside to this, however, is it makes you vulnerable to conflicts because changing a flag now requires a _series_ of instructions, rather than a single `sta` command: 
    
    
      lda needflags     ; grab the need flags
      ora #NEEDFLG_DMA  ; flip on the flag marking sprite DMA
      sta needflags     ; write back
    

The conflict here happens when NMI trips between the `lda` and `sta` commands, and when the NMI routine changes `needflags`. This _will_ happen sometimes. For instance, the previous NMI handler example performs `dec needflags` after it finishes drawing. It's a good thing that it does this, because otherwise it will waste time drawing the same thing over and over every frame. 

Another interesting and often overlooked problem is a subtle _system state_ change. Say you're doing a bulk drawing routine and you set the PPU address to $2400: 
    
    
      lda #>$2400
      sta $2006
      lda #<$2400
      sta $2006
    
    ;; elsewhere, in your NMI handler:
    NMI:
      pha
      bit $2002    ; clear vblank flag
    
      lda xscroll  ; set the scroll
      sta $2005
      lda yscroll
      sta $2005
    
      pla
      rti
    

Harmless, right? But there is a disastrous conflict there. Do you see it? It happens when an NMI occurs between the $2006 writes. It's bad in two ways: 

  1. `bit $2002` will reset the PPU toggle so that the next write to $2006 is the _first_ write. This means that your main code won't actually be setting the PPU address to $2400, because the second write actually becomes another first write because of the toggle.
  2. Setting the scroll changes the temp PPU address (aka `[loopy_t](PPU_scrolling.xhtml#PPU_registers "PPU scrolling")`). This messes with the address the main code is trying to set with its $2006 writes.



Yet another example of this happening is on mappers which require two or more writes in order to bankswitch. If the NMI or IRQ also needs to bankswitch, you have yourself yet another conflict. 

Just remember the key to being interrupt-aware is to spot vulnerabilities. You're most vulnerable when something seemingly basic takes several instructions to do. Just be extra careful when writing code like that in your program, and make state changing code in your NMI/IRQ handlers conditional so that your main code can disable it for sections where it might introduce conflict vulnerabilities. 
