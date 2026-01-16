# Talk:FDS audio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AFDS_audio) | View [other pages](Special_AllPages.xhtml#Talk_FDS_audio)

## Envelope Speed

The "Envelope Speed" section has this quote: 

    _BIOS or game code appears to set this to $FF._

I've recently checked the FDS bios disassembly, and it seems to actually set the value to $E8, as seen at $ee72 in the BIOS: 
    
    
    $EE72	LDA #$e8
    $EE74	STA $408A

FDS games may set the envelope speed to $FF; However, I haven't checked any of them, so I wouldn't be able to tell you. --[Freem](User_Freem.xhtml "User:Freem") 20:47, 12 January 2013 (MST) 
