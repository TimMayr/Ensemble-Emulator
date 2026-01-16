# Installing CC65

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Installing_CC65) | View [other pages](Special_AllPages.xhtml#Installing_CC65)

An **assembler** is a program that translates assembly language source code into machine code. A commonly used assembler that produces machine code for 6502 CPUs is **CA65** , which is distributed as part of the **CC65** package. These instructions tell how to install and run CA65. 

## Contents

  * 1 Installing CC65 on Windows
    * 1.1 Configuring Windows
    * 1.2 Configuring 7-Zip
    * 1.3 Downloading CC65
  * 2 Installing CC65 on Linux
    * 2.1 Installing with Package Managers
    * 2.2 Building from git
  * 3 Installing CC65 on Mac OS X



## Installing CC65 on Windows

### Configuring Windows

Windows is shipped with file name extensions hidden in the File Explorer. This misfeature was originally intended to make Windows 95 look more like Mac OS 7.5, which did not use file name extensions. However, hiding them makes it easier to accidentally create a file name with two extensions. Worse yet, destructive software such as the ILOVEYOU worm ([Wikipedia article](https://en.wikipedia.org/wiki/ILOVEYOU "wikipedia:ILOVEYOU")) are typically shipped with two extensions: one to indicate to the operating system that the file is executable and one to fool the user into thinking that the file is not executable (and thus safe). The first thing we will do is to turn on the display of file name extensions: 

  1. Open Control Panel. (This may be in Start > Control Panel or Start > Settings > Control Panel.)
  2. On Windows XP, if "Pick a category" shows up at top center of the window, click "Switch to Classic View".
  3. Open Folder Options (File Explorer Options from Windows 8 and onwards) and activate the View pane.
  4. In the scrolling list of Advanced Options, make sure that "Hide extensions for known file types" is _not_ checked.
  5. Press OK to put the change into effect.



From Windows 8 and onwards, this setting can also be changed from the View tab in the File Explorer's ribbon interface. (Enable/Show File Extensions) 

### Configuring 7-Zip

Windows comes with a bare-bones archiver that can only handle PKZIP archives (`.zip`), not rar, 7z, tar, gz, or bz2 files. The [7-Zip package](http://www.7-zip.org/) can extract files from all of them, as well as add files to PKZIP and 7-Zip archives. 

### Downloading CC65

  1. Read the [front page of CC65's web site](http://cc65.github.io/cc65/).
  2. Scroll down to "Download". At the bottom of this section is a list of mirrors.
  3. Enter one of the mirrors, and download the `cc65-win32-_#####_.zip` and `cc65-doc-_#####_.zip` packages to your computer. (The _#####_ represents a version number, such as 2.11.0-1, which may change before you read this.) The `cc65-win32` contains the CC65 package compiled for Windows, and `cc65-doc` contains the manual.
  4. Extract `cc65-win32-_#####_.zip` to a new folder.
  5. Open this folder and run the `install.vbs` file as an administrator to copy it to Program Files.
  6. Delete this folder.



Log out and log in to complete the installation. 

## Installing CC65 on Linux

### Installing with Package Managers

The easiest method is to install CC65 using the package manager of your choice. Here are some examples from popular distributions: 

  * Debian-based: apt/apt-get
  * Arch-based: yay (or any other [AUR helper](https://wiki.archlinux.org/title/AUR_helpers))
  * RedHat-based: dnf/yum



### Building from git

If CC65 is not available via your package manager, or you wish to use features/bugfixes not yet present in release versions, you can build it from the source code instead. On Debian-based distributions, open a terminal and enter the following commands. On other distributions, the `apt` command will need to be changed. 
    
    
    sudo apt install build-essential git
    mkdir -p ~/develop
    cd ~/develop
    git clone https://github.com/cc65/cc65.git
    cd cc65
    nice make -j2
    make install PREFIX=~/.local
    which cc65
    

If your account has been configured to run applications built from source and installed for one user, the last step should show `/home/<username>/.local/bin/cc65`. If it does not, add `~/.local/bin` to your `PATH` environment variable: 
    
    
    nano ~/.bashrc
    
    # and add the following at the end of the file
    if [ -d "$HOME/.local/bin" ] ; then
        PATH="$HOME/.local/bin:$PATH"
    fi
    

Press Ctrl+O Enter to save, then Ctrl+X to quit, and the change to `PATH` will take effect the next time you log in. 

## Installing CC65 on Mac OS X

Using Homebrew: On computer with Homebrew installed, open Terminal and type the following command: `brew install cc65`. Everything else should be automatic. 

Using Macports: Alternatively macports can be used for easy and fast installation. Open a terminal and enter the following command: `sudo port install cc65`. This will install cc65, ca65 and ld65 on the computer. 
