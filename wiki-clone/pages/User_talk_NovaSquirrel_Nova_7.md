# User talk:NovaSquirrel/Nova-7

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3ANovaSquirrel/Nova-7) | View [other pages](Special_AllPages.xhtml#User_talk_NovaSquirrel_Nova_7)

## CHR banking

Where do the 512-byte banks get the extra bits to specify a full address in CHR? They need an extra bit from somewhere (maybe they're fixed at fixed at 0 or 1 or A9 ?) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:06, 14 October 2015 (MDT) 

The 512-byte banks would be limited to the first 128 KB. The mapper is really intended to be used with a 32 KB or 128 KB SRAM for CHR, and 128 KB is already a crazy amount of CHR RAM so I think this limit is okay. --[NovaSquirrel](User_NovaSquirrel.xhtml "User:NovaSquirrel") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:NovaSquirrel&action=edit&redlink=1 "User talk:NovaSquirrel \(page does not exist\)")) 16:31, 14 October 2015 (MDT) 
