# Make sram

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Make_sram) | View [other pages](Special_AllPages.xhtml#Make_sram)

This program **make_sram.py** creates .sav files for all [iNES](INES.xhtml "INES") format ROMs in a folder and its subfolders that have the battery bit turned on. It's useful for users of [PowerPak](PowerPak.xhtml "PowerPak"), which can't create files by itself. It is written in Python and has been tested on Python 2.6 on Ubuntu Karmic and (to a lesser extent) Python 2.5 on Windows. If you have trouble getting to run under your copy of Python, tell me about it [here](Talk_Make_sram.xhtml "Talk:Make sram") or on the forum. 
    
    
    #!/usr/bin/env python
    """
    make_sram
    See versionText below for copyright information.
    
    Change log:
    0.02 (2009-12-30): fixed a 'with' statement that I missed
    0.01 (2009-12-29): initial release
    """
    import os
    # I'd use python 2.5's with statement, but the last time I posted a
    # python program on pocket heaven, people were complaining about not
    # being able to run my program on python 2.4.
    
    
    helpText = """Usage: make_sram [foldername]
    Makes empty 8192 byte .sav files for .nes files that need it.
    """
    versionText = """make_sram 0.02 (2009-12-30)
    
    Copyright 2009 Damian Yerrick
    Copying and distribution of this file, with or without modification,
    are permitted in any medium without royalty provided the copyright
    notice and this notice are preserved.  This file is offered as-is,
    without any warranty.
    """
    
    def findAllRoms(folder='.'):
        pathnames = []
        for (root, folders, files) in os.walk(folder):
            for filename in files:
                if filename.lower().endswith('.nes'):
                    pathnames.append(os.path.join(root, filename))
        return pathnames
    
    def getINESBatteryBit(filename):
        infp = None
        try:
            infp = open(filename, 'rb')
            infp.seek(6)
            c = ord(infp.read(1))
            return (c & 0x02) != 0
        finally:
            if infp is not None:
                infp.close()
    
    def getSavName(filename):
        (folder, filename) = os.path.split(filename)
        (filename, ext) = os.path.splitext(filename)
        return os.path.join(folder, filename + '.sav')
    
    def processFolder(folder='.', dry=False):
        # First find the corresponding .sav filename for each .nes rom
        # with a battery bit
        savFiles = [getSavName(filename)
                    for filename in findAllRoms(folder)
                    if getINESBatteryBit(filename)]
        # Then find the ones that don exits (like tires)
        savFiles = [filename
                    for filename in savFiles
                    if not os.path.exists(filename)]
        blankSavData = chr(0) * 8192
        for filename in savFiles:
            print filename
            if not dry:
                r = open(filename, 'wb')
                r.write(blankSavData)
                r.close()
    
    def main(argv=None):
        if argv is None:
            import sys
            argv = sys.argv
        if len(argv) > 1:
            folderName = argv[1]
            if folderName in ('--help', '-?', '/?', '-h'):
                print helpText
                return
            elif folderName == '--version':
                print versionText
                return
        else:
            folderName = '.'
        processFolder(folderName)
    
    if __name__=='__main__':
        main()
    
