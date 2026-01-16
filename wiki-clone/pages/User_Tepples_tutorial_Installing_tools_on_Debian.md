# User:Tepples/tutorial/Installing tools on Debian

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ATepples/tutorial/Installing_tools_on_Debian) | View [other pages](Special_AllPages.xhtml#User_Tepples_tutorial_Installing_tools_on_Debian)

## Contents

  * 1 apt-get
  * 2 Install cc65
  * 3 Install FCEUX for Windows
  * 4 Configure a text editor



## apt-get

Debian GNU/Linux and derivatives include the APT package manager, which makes installing the majority of tools easy. 

Connect to the Internet, paste the following into a terminal, enter your password, and a few 
    
    
    sudo apt-get install build-essential python3-imaging gimp gedit fceux wine
    

This installs several things: 

  * `build-essential` [GNU Make](https://www.gnu.org/software/make/), a program that determines which parts of your project have changed, such as assembly language source code or game graphics, and rebuilds only those parts, and a C compiler, needed for building cc65.
  * `python3-imaging` is Pillow (Python Imaging Library), a component used by Python programs that manipulate image data, such as PNG, JPEG, and BMP files.
  * `gimp` is GIMP (GNU Image Manipulation Program), a paint program distributed as free software. You can use it to create game graphics.
  * `fceux` is FCEUX (Family Computer Emulator Ultra Extended), an emulator that interprets NES programs and displays their output.
  * `wine` is Wine (Wine Is Not an Emulator), which allows use of Windows desktop applications on X11/Linux. You will need this to run the debugging version of FCEUX.
  * `gedit` is the GNOME text editor.



## Install cc65

cc65 is a C compiler targeting the MOS Technology 6502 CPU. In this tutorial, you won't be using the C compiler, but you will be using the assembler (ca65) and the linker (ld65) that come with it. 

Because there is no Debian package for cc65 as of the third quarter of 2015, you'll need to compile it from source. The source code for cc65 is in [a GitHub repository](https://github.com/cc65/cc65). If you are already familiar with Git or Subversion, you can clone the project to your computer. Otherwise, visit the repository, follow the "Download ZIP" link at the right, and unzip it somewhere convenient. 

TODO: make 

TODO: make install, and prefix/PATH issues 

## Install FCEUX for Windows

The version of FCEUX included with Debian unfortunately does not include a debugger. Instead, you can run FCEUX for Windows. 

TODO 

## Configure a text editor

Now we'll configure a text editor so that you can press one key and see your ROM come to life. 

TODO 
