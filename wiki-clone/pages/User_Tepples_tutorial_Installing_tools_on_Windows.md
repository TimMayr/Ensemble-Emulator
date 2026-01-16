# User:Tepples/tutorial/Installing tools on Windows

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ATepples/tutorial/Installing_tools_on_Windows) | View [other pages](Special_AllPages.xhtml#User_Tepples_tutorial_Installing_tools_on_Windows)

Unlike Debian, Windows lacks a centralized package manager for desktop applications, and the Windows Store introduced in Windows 8 is not oriented toward software developer tools. 

## Contents

  * 1 Install Python
  * 2 Install Pillow
  * 3 Install cc65 from binary package
  * 4 Install MSYS
  * 5 Install FCEUX
  * 6 Install GIMP
  * 7 Install and configure Notepad++



## Install Python

Many data conversion tools used in this tutorial are written in [Python](https://www.python.org/), an interpreted programming language maintained by the Python Software Foundation. To run Python programs, you'll need a Python interpreter. As of the first quarter of 2016, the latest version of Python is 3.5.1. 

**Download[Python 3.5.1 (Windows x86)](https://www.python.org/ftp/python/3.5.1/python-3.5.1.exe) and install it.** It's a Windows executable installer; you may need somebody with administrative privileges on the local PC to enter a password to authorize the installation. 

## Install Pillow

Pillow (Python Imaging Library) is a component used by Python programs that manipulate image data, such as PNG, JPEG, and BMP files. 

To install Pillow, open a Command Prompt and enter the following command: 
    
    
    C:\Python35\Scripts\pip.exe install Pillow
    

Then test the Pillow installation: 

  1. From the Start menu, inside All Programs, inside Python, choose IDLE (Python GUI).  
or: From the Start menu or Windows 8 Start screen, type IDLE and then choose IDLE (Python GUI).
  2. (If IDLE's window is titled "Python 3.x.x Shell", skip this step.)  
From the Run menu, choose Python Shell.
  3. Paste in each of the two following lines, pressing Enter after each one:


    
    
    from PIL import Image
    Image.new('RGB', (100, 100), (0, 191, 0)).show()
    

This should cause a small green box to appear on the screen. If it does not, or you see a bunch of red text in the IDLE window, there may be a problem with your Python or Pillow installation. 

## Install cc65 from binary package

  1. From <http://cc65.github.io/cc65/> download the Windows Snapshot.
  2. Unzip it to a folder in your home directory.
  3. TODO: Add this folder to your PATH



## Install MSYS

[MSYS](http://mingw.org/wiki/MSYS) is a port of parts of the free GNU operating system to Windows. It includes [GNU Make](https://www.gnu.org/software/make/), a program that determines which parts of your project have changed, such as assembly language source code or game graphics, and rebuilds only those parts. 

  1. Download and install mingw-get-setup.exe from <http://mingw.org/wiki/Getting_Started>
  2. TODO: Choose MSYS packages



## Install FCEUX

FCEUX (Family Computer Emulator Ultra Extended) is an emulator, or an application that interprets NES programs and displays their output. 

  1. Download FCEUX (win32 binary) from [FCEUX download page](http://www.fceux.com/web/download.html).
  2. Unzip it somewhere convenient in your home directory.
  3. TODO: configure controls



## Install GIMP

GIMP, the GNU Image Manipulation Program, is a paint program distributed as free software. You can use it to create game graphics. 

  1. On the [download page](http://www.gimp.org/downloads/), click "Download GIMP 2.8 directly".
  2. Install the MSI.
  3. TODO: Set default brush to 1 pixel and default grid to 8 pixels, then save tool options



## Install and configure Notepad++

To edit source code, you need a text editor. Windows Notepad is included with Windows and will work in a pinch, but there is better. 

TODO: notepad-plus-plus.org 

TODO: Configure for 6502 syntax highlighting 
