# User:Bregalad/Scrolling sandbox

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ABregalad/Scrolling_sandbox) | View [other pages](Special_AllPages.xhtml#User_Bregalad_Scrolling_sandbox)

This is my proposal for a new [scrolling](PPU_scrolling.xhtml "Scrolling") page. Some information which is currently on the [mirroring](Mirroring.xhtml "Mirroring") page should be moved here. On the other hand, complex scrolling techniques are moved to a new [scroll split](https://www.nesdev.org/w/index.php?title=Scroll_split&action=edit&redlink=1 "Scroll split \(page does not exist\)") page. Feedback and constructive critisism is welcome. 

Sections _Unidirectional scrolling_ and _Mirroring chart_ were just moved arround, other sections were significantly altered by myself (Bregalad). 

**!! CURRENTLY THIS PAGE IS A WORK IN PROGRESS !!**

**Scrolling** is the movement of the displayed portion of the map. Games scroll to show an area larger than the 256x240 pixel screen. For example, areas in _Super Mario Bros._ may be up to 24 screens wide. The NES's first major improvement over its immediate predecessors (ColecoVision and Sega Mark 1) was pixel-level scrolling of playfields. 

## Contents

  * 1 Unidirectional scrolling
    * 1.1 Frequent pitfalls
    * 1.2 PPU registers
  * 2 Multi-directional scrolling techniques
    * 2.1 Horiontal scrolling with horizontal mirroring
    * 2.2 Vertical scrolling with vertical mirroring
    * 2.3 Status bars and single-screen mirroring
  * 3 Mirroring chart



## Unidirectional scrolling

Ordinarily, a program writes to two [PPU registers](PPU_registers.xhtml "PPU registers") to set the scroll position in its NMI handler: 

  1. Find the X and Y coordinates of the upper left corner of the visible area (the part seen by the "camera")
  2. Write the X coordinate to [PPUSCROLL](PPU_registers.xhtml "PPUSCROLL") ($2005)
  3. Write the Y coordinate to PPUSCROLL
  4. Write the starting page (high order bit of X and Y) to bits 0 and 1 of [PPUCTRL](PPU_registers.xhtml "PPUCTRL") ($2000)



The scroll position written to PPUSCROLL is applied at the end of vertical blanking, just before rendering begins, therefore these writes need to occur before the end of vblank. Also, because writes to [PPUADDR](PPU_registers.xhtml "PPUADDR") ($2006) can overwrite the scroll position, the two writes to PPUSCROLL must be done after any updates to VRAM using PPUADDR. 

By itself, this allows moving the camera within a usually two-screen area (see [Mirroring](Mirroring.xhtml "Mirroring")), with horizontal and vertical wraparound if the camera goes out of bounds. To scroll over a larger area than the two screens that are already in VRAM, new tile and attribute data decompressed from leve data has to be filled in the nametables, as seen in the animation below. The area that needs rewritten at any given time is sometimes called the "seam" of the scroll. 

[![SMB1 scrolling seam.gif](../wiki-images/SMB1_scrolling_seam.gif)](File_SMB1_scrolling_seam_gif.xhtml)

### Frequent pitfalls

Taking too long
    If a NMI handler routine takes too long and PPUSCROLL ($2005) is not set before the end of vblank, the scroll will not be correctly applied the following frame. Most games do not write more than 64 bytes to VRAM per NMI, more than this may require on NTSC consoles advanced loop unrolling techniques to fit the narrow window of time offered by the VBlank period.
Set the scroll last
    PPUSCROLL ($2005) must always be set after using PPUADDR ($2006). They have a shared internal register and using PPUADDR ($2006) will overwrite the scroll position.

### PPU registers

When split scrolling is required, some more advanced usage of $2005 and $2006 registers is needed. The complete information is is on a separate page : [split scrolling](https://www.nesdev.org/w/index.php?title=Split_scrolling&action=edit&redlink=1 "Split scrolling \(page does not exist\)"). 

## Multi-directional scrolling techniques

Scrolling in a direction in which the screen is not mirrored is usually easy, since there is a whole screen of buffer space to update data. On the other hand, dealing with scrolling updates is harder when the axis in the direction it is scrolled to is [mirroring](https://www.nesdev.org/w/index.php?title=Mirrored&action=edit&redlink=1 "Mirrored \(page does not exist\)"), because graphics leaving one side of the screen directly enters the other side. 

The rarely-available L- or X-shaped [mirroring](Mirroring.xhtml "Mirroring") could facilitate changes in scrolling direction on screen-boundaries without having to flip between Horizontal and Vertical mirroring. 

### Horiontal scrolling with horizontal [mirroring](Mirroring.xhtml "Mirroring")

Doing any horizontal scrolling using horizontal mirroring is hard to do smoothly because the data on the right of the screen is immediately show on the left due to mirroring. Clever use of hardware left-side screen clipping will hide all name table glitches, but because the attribute tables have a resolution of 2x2 tiles, there will always be attribute glitches on the left and/or the right side of the screen. Some televisions overscan up to 8 pixels on both left and right border, but most doesn't. PAL NESes will automatically clip the left and right 2 pixels. Several solutions exists: 

  * Ignore the problem: Colour glitches are accepted, and are minimized (see below)
  * Work arround the problem: Levels use only a single palette (as implemented by _Wizard & Warriors II_), or have a level design where the colour scheme repeats itself vertically each screen. This seriously reduces the graphical possibilities.
  * Fix the problem: Overlay 8 additional pixels of BG with sprites. This can be either solid-colour sprites designed to hide bachground graphics (as in the game _Alfred Chicken_), or sprites reusing BG graphics and correct colours instead of the incorrect colour from the attribute table. Very few games do this because it reduces the number of (actual) sprites per scanline to 7 and wastes a lot of OAM space (roughly 1/4 in 8x16 pixel sprite mode).



If glitches are accepted they appear to at least 7 pixels due to techincal reasons (assuming left-clip is enabled via $2001). Several strategies exist however to make them less blatant: 

  * Make the various BG palettes use the same luminosity pattern and only changing the hues, that way the area will be wrong colored will at least have correct luminosity, and as such, look less wrong.
  * Make the glitches in the direction the player is not facing (as seen in _Kirby's Adventure_).
  * Share the glitches on both borders, so the total area of graphics showing with the wrong colour will be minimized. For example 4 pixels on the right side and 3 pixels on the left side. On PAL consoles, this will reduce to 2 and 3 respectively.



Games with such glitches typically does a poor job at minimizing the amount of glitches, so they give a bad impression as to what is actually possible. 

This diagram shows how the glitches can be minimized for a 16-pixel horizontal scroll cycle, each line representing a different fine horizontal scroll value. 
    
    
    Fine
     HScroll
      0   --------|------------------------
      1   #-------|------------------------
      2   ##------|------------------------
      3   ###-----|------------------------
      4   ####----|------------------------
      5   #####---|------------------------
      6   ######--|------------------------
      7   #######-|------------------------
      8   ~~~~~~~~|------------------------  (NT update)
      9   #~~~~~~~|~-----------------------
     10   ##~~~~~~|~~----------------------
     11   ###~~~~~|~~~---------------------
     12   xxxx----|--------------------~~~~  (AT update)
     13   xxxxx---|---------------------~~~
     14   xxxxxx--|----------------------~~
     15   xxxxxxx-|-----------------------~
      0   --------|------------------------  (NT update)
          ^^^^^^^^                       ^^--- Hidden on PAL machines
          Hidden part (by hardware through $2001)
      
     - Correct tile displayed with correct colorus
     ~ Correct tile displayed with wrong colours
     x Wrong tile displayed with correct colours
     # Wrong tile displayed with wrong colours
    

### Vertical scrolling with vertical [mirroring](Mirroring.xhtml "Mirroring")

Because data that is on the top/bottom of the screen will immediately show up on the other side, a clever use of NTSC [overscan](Overscan.xhtml "Overscan") can make it glitch-less multidirectional scrolling, but glitches will appear on PAL televisions (and NTSC televisions with a overscan range which is a little off). Because the vast majority of commercial games were developped in NTSC countries, they tend to consider the vertical overscan as a safe buffer zone for scroll updates. 

However, as with the case of horizontal scrolling with horizontal mirroring, it is desirable to minimize the unavoidable glitches. The best possible way to hide glitches is to make 4 pixels with wrong tiles and 4 additional pixels with wrong color on both sides. Games with such glitches typically does a poor job at minimizing the amount of glitches, so they give a bad impression as to what is actually possible. 

Perfectionist programmers could use raster split to hide glitches, and possibly also provide more blanking time to update VRAM as well as hiding the unavoidable sprite poping in the top border. This is seen in the games _Jurassic Park_ and _M.C. Kids_ , but it was rarely done because it complicates the code a lot for little benefits. 

This diagram shows how the glitches can be minimized for a 16-pixel vertical scroll cycle, each column representing a different fine vertical scroll value. 
    
    
    Fine
     VScroll 0123456789ABCDEF0
            -###~~~~-xxx----- < Hidden by
            --##~~~~--xx----- < NTSC average
            ---#~~~~---x----- < Overscan
            ----~~~~--------- <
            -----~~~--------- <
            ------~~--------- <
            -------~--------- <
            -----------------
            -----------------
            -----------------
            -----------------
            -----------------
            -----------------
            -----------------
            -----------------
            ----------------- <
            --------~-------- <
            --------~~------- <
            --------~~~------ <
            ----x---~~~~#---- <
            ----xx--~~~~##--- < Hidden by
            ----xxx-~~~~###-- < NTSC average
            ----xxxx~~~~####- < Overscan
    	    ^(NT update)
    	        ^(AT update)
    		    ^(NT update)
    
      - Correct tile displayed with correct colorus
      ~ Correct tile displayed with wrong colours
      x Wrong tile displayed with correct colours
      # Wrong tile displayed with wrong colours
    

### Status bars and single-screen mirroring

Single-screen [mirroring](Mirroring.xhtml "Mirroring")'s main advantage is that it allows using a status bar at the top or bottom of the screen while also allowing the playfield to extend equally in any direction. Typically the status bar is stored in one nametable and the playfield in the other. Scrolling parameters and nametables are switched the appropriate screen location during rendering. 

Due to the screen layout being entierely linearly adressable, the calculation of PPU addresses of name and attribute tables updates are significantly simpler, leading to typically faster, more efficient scrolling routines. A variable sized or self-scrolling status bar can relatively easily be created thanks to 1-screen mirroring. 

The same graphical glitches as discussed above under horizontal mirroring will happen. However, as long as there is a status bar, glitches are fully avoidable vertically since the data that falls off the bottom (or the top) of the screen will come in the area that is "hidden" by the status bar, regardless of overscan factors. 

## Mirroring chart

This table lists the most common mirroring and scrolling techniques. There are a huge variety of more complicated techniques. For a more comprehensive survey, see: [List of games by mirroring technique](List_of_games_by_mirroring_technique.xhtml "List of games by mirroring technique")

Scrolling Type | Mirroring | Example Games | Comment   
---|---|---|---  
None  | Any  | _Donkey Kong_ , _Tennis_ | With only a single fixed screen, any mirroring type can be used.   
Horizontal Only  | Vertical  | _Super Mario Bros._ , _Gimmick!_ | A [status bar](https://www.nesdev.org/w/index.php?title=Status_bar&action=edit&redlink=1 "Status bar \(page does not exist\)") at the top is easy to accomplish with a [sprite-0 hit](PPU_OAM.xhtml "Sprite-0 hit") (see _Super Mario Bros._).   
Vertical Only  | Horizontal  | _Ice Climber_ , _Gun.Smoke_ | Without a status bar, horizontal mirroring is the best choice for vertical-only scrolling. With a status bar, vertical or single-screen mirroring gives a place in the nametable to render the status bar, and the scrolling seam should be hidden under the bar.   
Alternating Horizontal/Vertical  | Mapper switches H/V  | _Metroid_ , _Air Fortress_ | Motion is limited to a single axis at any given time, and the direction can only change when a new screen is reached.   
Limited Bidirectional  | Horizontal/Vertical  | _Super Mario Bros. 3_ , _Fire Emblem_ | By limiting one of the scrolling axes to only 2-screens wide, this makes unlimited scrolling in the other axis simpler. With unlimited horizontal scrolling there will be unavoidable attribute glitches at one side of the screen (see _Super Mario Bros. 3_), but with unlimited vertical scrolling this can be hidden by [overscan](Overscan.xhtml "Overscan") in NTSC regions (see _Fire Emblem_).   
Unlimited Bidirectional  | Various  | _Castlevania II_ , _Battletoads_ , _Crystalis_ , _Final Fantasy_ | Unlimited scrolling in both axes at once is an advanced technique requiring a game-specific solution.   
  
The best way to understand the mirroring techniques used in a game, use a debugging emulator to look at the nametables. [Status bars](https://www.nesdev.org/w/index.php?title=Status_bar&action=edit&redlink=1 "Status bar \(page does not exist\)") typically require a scrolling split at a timed location on the screen. This can be done most easily with a mapper based [IRQ](IRQ.xhtml "IRQ"), but can also be accomplished with a [sprite-0 hit](PPU_OAM.xhtml "Sprite-0 hit") or other techniques. 
