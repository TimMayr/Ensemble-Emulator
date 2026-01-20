# Talk:Stack

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AStack) | View [other pages](Special_AllPages.xhtml#Talk_Stack)

# Mismatched comments on stack pulling example
    
    
    _pullstack:
      pla		; Pull the value $ff off the stack, and put it into the accumulator.
      tax		; S now becomes $FD.
    
      pla		; Pull the next value ($bb) off the stack, and put it into the X register.
      tay		; S now becomes $FE.
    
      pla		; Pull $e0 off the stack, and put it into the Y register.
    		; S now becomes $FF -- which is where we started!
    

I'm not exactly well-versed in 6502 assembly, but the comments seem mismatched. `PLA` pops the stack content as intended, although afterwards `TAX` moves the accumulator to the X register, which doesn't seem to be indicated in the comment. Likewise, the next stack value is again popped, but then it is transferred to Y, despite the comment indicating that that the value would end up in the X register instead. Nothing is done to the last popped value, although the comment indicates the value ending up on the Y register. 

[Tevo](https://www.nesdev.org/w/index.php?title=User:Tevo&action=edit&redlink=1 "User:Tevo \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Tevo&action=edit&redlink=1 "User talk:Tevo \(page does not exist\)")) 10:43, 4 August 2020 (MDT) 

  * They were indeed mismatched. I've adjusted the comments so they actually make sense. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 14:53, 4 August 2020 (MDT)


