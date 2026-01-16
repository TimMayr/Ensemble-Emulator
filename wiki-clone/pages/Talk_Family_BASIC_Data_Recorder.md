# Talk:Family BASIC Data Recorder

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AFamily_BASIC_Data_Recorder) | View [other pages](Special_AllPages.xhtml#Talk_Family_BASIC_Data_Recorder)

## Example codes

Can someone please add some example codes for how you would record data on the tape and play it back? It might help if someone want to use it. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 18:11, 23 November 2013 (MST) 

    The source for [TapeDump](http://www.chrismcovell.com/TapeDump_Controls.html) and [TapeDump's FDS writer](http://forums.nesdev.org/viewtopic.php?f=2&t=9909) are representative, and could easily be adapted by replacing the writes to $4011 with writes to $4016.0 and reads from $4016.2 with reads from $4017.1
    _(At some point I though "It'd be cool to implement something more sophisticated than KCS, like MFM or RLL2,7" but then I realized ... really, when is anyone going to use it? The best bit density we can reasonably expect to work would be small—maybe 3600bps. For hysterical raisins, everything in an emulator more easily pretends it has battery-backed RAM, and for new manufacture, using Flash PRG or CHR or a serial EEPROM and storing save data in them is easier. And KCS has the advantage that lots of software decoder/encoders already exist.)_ —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:32, 23 November 2013 (MST) 

    Ah, OK, such source-codes may help. However, about your smaller text comment about the battery RAM, you still might want to have separate data files rather than the save file in the cartridge, in several cases, especially if the software also involves the keyboard (for example, if there is a built-in level editor which operates using the keyboard, and saves on tapes, that you can copy the tapes and so on; the main game may not require the keyboard or tape though, and just use the ROM levels instead); in addition, you cannot easily use disks for this purpose either since no emulators I know of support .QDI disk image formats (whose purpose is so that you can create new disks, copy individual disks, and share disks between programs and so on)! --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 13:16, 24 November 2013 (MST)
