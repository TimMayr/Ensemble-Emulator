# User:Tepples/tutorial

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ATepples/tutorial) | View [other pages](Special_AllPages.xhtml#User_Tepples_tutorial)

Project began in [this forum post](http://forums.nesdev.org/viewtopic.php?f=2&t=13113)

  1. Gathering tools 
     1. [On Debian GNU/Linux and derivatives](User_Tepples_tutorial_Installing_tools_on_Debian.xhtml "User:Tepples/tutorial/Installing tools on Debian")
        1. Install prerequisites 
           * `sudo apt-get install build-essential python3-imaging gimp fceux wine`
        2. Compile and install cc65
        3. Configure the editor
        4. Install FCEUX (debugging version) for Windows
     2. [On Windows](User_Tepples_tutorial_Installing_tools_on_Windows.xhtml "User:Tepples/tutorial/Installing tools on Windows")
        1. Install Python 3
        2. Install Pillow using pip
        3. Install cc65 from binary
        4. Install MSYS
        5. Install FCEUX
        6. Install GIMP
        7. Install and configure Notepad++
     3. On Fedora 
        1. Mostly similar to Debian (using `yum` instead of `apt-get`) but will need to either include a section on compiling and installing FCEUX (SDL version) from source or always use FCEUX in Wine because Fedora does not package FCEUX, in turn because Red Hat Inc. fears Nintendo's legal department.
     4. On FreeBSD
     5. On OS X
     6. Building a ROM from source code 
        1. The [project for this step](https://pineight.com/nes/#template) is intentionally a more complex program than the first program of lesson 2 because I want to exercise more of the toolchain (ca65, ld65, Python, Pillow, GNU Make, GNU Coreutils) to prevent problems later.
  2. Hello World 
     1. Turn the screen green (immediate mode and PPU addressing)
     2. Display "HI" (immediate mode)
     3. How to use FCEUX debugger
     4. Display "Hello World" (indexed addressing and looping)
     5. Display "Hello World" at different positions (subroutines)
     6. Display an arrow of "Hello World" (more looping)
     7. Display multiple strings (pointers and indirect indexed addressing)
     8. Display two screens of text (reading the controller using the tutorial's library)
     9. Text file viewer (multiple kinds of delimiters in text)
  3. A bouncing ball 
     1. Display a circle (clearing, filling, and pushing OAM)
     2. Move the circles (position variables and waiting for vertical blanking)
     3. Draw some walls (+32 fill mode)
     4. Limit the circle's movement (position comparison)
     5. Bounce off the walls (velocity variables)
     6. Display pressed buttons (nametable updating during vblank)
     7. Display coordinates (binary to decimal conversion)
     8. Control the circle (react to button presses and move in 2 dimensions)
     9. Add some momentum (acceleration and subpixel math)
  4. Air hockey game 
     1. Creating graphics
     2. Multi-sprite objects
     3. Collision with paddles
     4. Attributes
  5. Brick breaking game 
     1. Collision with a level map
     2. Update buffering
  6. CHR RAM 
     1. iNES header
     2. Linker script
     3. Init code
     4. Loading CHR data
     5. Tile animation
  7. Platforming 
     1. Metasprite table
     2. Gravity
  8. Scrolling
  9. Sound effects 
     1. Making a pitch lookup table generator
  10. Music
  11. Mappers
  12. Explanation of the rest of the library 
     1. pads.s
     2. ppuclear.s


