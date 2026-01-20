# Talk:NES expansion port pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_expansion_port_pinout) | View [other pages](Special_AllPages.xhtml#Talk_NES_expansion_port_pinout)

## In/out, and musing all over the place

Audio in/out, $4016 outs, and video out seem clear enough, and, if I understand correctly, EXP* are whatever direction the hardware guys choose between cart and expansion. But how is the expansion port situated related to the controllers, as it seems to only have one connection to each controller pin? 

Is it possible to make an expansion-port [NES] extra-controller adapter that's compatible with, say, FourScore software, with only two ports? Or would one have to have four ports on the expansion device in order to be able to serialize one controller with the other (and signature)? (Famicom-expansion style, putting in upper bits seems much easier, though obviously less useful thanks to it not being a Famicom.) Hmm, adding a SNES controller port would be nice too… [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 10:35, 15 July 2016 (MDT) 

    The data lines are just directly shared with the input from the jacks on the front, so there's technically going to be a bus conflict between a plugged-in controller and whatever tries to drive it via the expansion port. Which doesn't explain why $4017.d0 is available on the [Famicom expansion port](Expansion_port.xhtml "Famicom expansion port pinout")... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:40, 15 July 2016 (MDT)

## Connector Dimensions

This is the only page on the expansion port and it doesn't have anything about what kind of connector it actually is, dimensions, or anything. Are compatible plugs/cables available? 

I suppose I could dig out my front loader and look, but it seems like someone must already have the info. — [Karatorian](User_Karatorian.xhtml "User:Karatorian") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Karatorian&action=edit&redlink=1 "User talk:Karatorian \(page does not exist\)")) 17:20, 23 May 2017 (MDT) 

    It's "standard" 0.1" pin spacing, but that's the only standard aspect to it. Some people have successfully cut apart ISA slots and grafted the halves onto new boards to match the row spacing. Nothing mass manufactured is available, and the only small-run thing was Chykn's ENI/O board. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:30, 23 May 2017 (MDT)
