# Talk:Input devices

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AInput_devices) | View [other pages](Special_AllPages.xhtml#Talk_Input_devices)

I guess the discussion at the beginning could be NES-specific. Could someone fill in the details for Famicom? -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 04:39, 14 April 2013 (MDT) 

* * *

Isn't $4016:2 connected to controller 2 on the Famicom for the microphone? -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 00:01, 15 April 2013 (MDT) 

    Yes, but it didn't fit fluidly into the description, and debatably is unnecessary on the general "input devices" page. The more places we describe something, the more places we have a chance of accidentally contradicting ourselves.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 01:37, 15 April 2013 (MDT)

## "Usage of port pins by hardware type" table

In the "Usage of port pins by hardware type" table, I believe I've spotted some errors, but I want to make sure I understand correctly: 

  * The Power Pad and Family Trainer Mat use writes to $0416 (OUT2/1/0), not reads from $0416 ($0416 D2/1/0).
  * Devices that use sequential reads to get bit data (Arkanoid controllers) or button data (Power Pad~~/Family Trainer Mat~~) also use the corresponding CLK signal.



\--[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 11:02, 31 December 2021 (UTC) 

Edit: Upon re-reading the [Family Trainer Mat](Power_Pad.xhtml#Family_Trainer_Mat "Power Pad") protocol, my current understanding is that it doesn't use the CLK signal. --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 20:36, 1 January 2022 (UTC) 

### Draft revision to table

I've [drafted a revision to table](User_Bavi_H_Input_devices_table.xhtml "User:Bavi H/Input devices table") that makes these changes: 

  * putting the output columns on the left so that a left-to-right reading order matches up with an output-then-input sequence of operations
  * putting clock columns (/OE1 and /OE2) to the left of their respective data columns (D4 to D0), for the same reason
  * adding an initial row that explains how to access the signals ("write $4016", "read $4016", "read $4017")
  * adding more initial rows that explain which signals are available at which ports.



Any comments? --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 20:36, 1 January 2022 (UTC) 

    Well, if nothing else, I like your formatting better. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 04:21, 7 January 2022 (UTC)

I boldly posted the draft version. Feel free to discuss, modify, revert(?). --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 14:01, 10 February 2022 (UTC) 
