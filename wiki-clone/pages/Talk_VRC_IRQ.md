# Talk:VRC IRQ

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AVRC_IRQ) | View [other pages](Special_AllPages.xhtml#Talk_VRC_IRQ)

## VRC7

From the decapsulated VRC7 image we have available, and with tracings of the lower layers provided by the user SCSR on Discord, I was able to build a preliminary [visual chip simulator](https://www.qmtpro.com/~nes/chipimages/visualvrc7/) and analyze the IRQ counter: 

  * The prescaler is actually implemented as 2 separate counters: one 2-bit that counts up to 2 before resetting and one 7-bit that counts up to 112 or 113 before resetting (with the condition `((B0 | A1) & B4 & B5 & B6)`).
  * The prescaler runs even with the "M" bit is set, but it doesn't do anything.
  * When the "A" bit is set, the IRQ counter reloads once it overflows and then continues counting (e.g. $FD -> $FE -> $FF -> $xx + IRQ -> $xx+1 -> $xx+2 -> ...); when it is clear, it reloads and then turns off (e.g. $FD -> $FE -> $FF -> $xx + IRQ -> $xx -> ...).
  * When the "E" bit is clear, the prescaler and IRQ counter continue to run but no interrupts are generated.
  * Writing to the Control register immediately reloads the IRQ counter and resets the prescaler regardless of the data written.
  * Writing to the Acknowledge register does **not** copy the "A" bit to the "E" bit. This was tested as follows: 
    * Setting E=1 and A=1 and writing to ACK post-IRQ acknowledges it and the counter continues running.
    * Setting E=1 and A=0 and writing to ACK post-IRQ acknowledges it but the counter remains stopped.
    * Setting E=1 and A=0 and writing to ACK **pre-IRQ** does _not_ prevent the IRQ from firing.
    * Setting E=0 and A=1 and writing to ACK, even pre-IRQ, does _not_ not cause interrupts to start happening.



It would be worth rerunning these tests on an actual VRC7, just in case the simulator I used contains errors (SCSR and I have both discovered and fixed multiple errors so far), and the earlier VRC chips should also be retested to see how they behave. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 14:47, 23 June 2022 (UTC) 
