# Talk:INES Mapper 116

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_116) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_116)

"However, it seems important that the $8xxx, $9xxx, $Axxx regs be mapped to the entire $1000 region (**unlike stock VRC2 which is supposedly strictly answering to $8000-$8003, etc.**)" \- this is at odds with the actual [VRC2](VRC2_and_VRC4.xhtml "VRC2") page (as well as its pinout), which states that it only only monitors A0-A1 and A12-A15. --[Quietust](User_Quietust.xhtml "User:Quietust") 09:29, 24 June 2012 (PDT) 

this seems like the kind of thing more likely to be understood today than historically. it sounds plausible to me. I'm willing to do some cursory tests to sanity check that it still emulates ok and then update the VRC2 and m116 docs, if someone will second it. --[Zeromus](User_Zeromus.xhtml "User:Zeromus") 10:07, 24 June 2012 (PDT) 
