# Talk:CPU interrupts

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACPU_interrupts) | View [other pages](Special_AllPages.xhtml#Talk_CPU_interrupts)

Putting some Visual 6502 links here so I don't lose them: 

Smallest IRQ assertion interval that will trigger an IRQ for LDA #FF: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq&a=0&d=58A9FF1890FE&irq0=5&irq1=6&steps=20](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq&a=0&d=58A9FF1890FE&irq0=5&irq1=6&steps=20)

Ditto for LSR $AB: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq&a=0&d=5846AB1890FE&irq0=11&irq1=12&steps=20](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq&a=0&d=5846AB1890FE&irq0=11&irq1=12&steps=20)

Ditto for LSR $AB with NMI instead of IRQ: [http://visual6502.org/JSSim/expert.html?logmore=Execute,nmi&a=0&d=58461890FE&nmi0=11&nmi1=12&steps=20](http://visual6502.org/JSSim/expert.html?logmore=Execute,nmi&a=0&d=58461890FE&nmi0=11&nmi1=12&steps=20)

* * *

Look to me, if it is necessary, a code using BRK that might coincide with NMI might be able to check the B flag at the end of the NMI handler code to determine what to do next (although this will make it slow). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 08:37, 8 September 2018 (MDT) 
