# Talk:RAMBO-1

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ARAMBO-1) | View [other pages](Special_AllPages.xhtml#Talk_RAMBO_1)

As for the 2mb PRG, I'm not sure if the cart actually supports that much ROM, but the PRG registers apparently are 8 bits, and 8kb * $100 = 2048kb = 2mb. --[Drag](User_Drag.xhtml "User:Drag") 03:05, 9 November 2011 (UTC) 

## Alternate IRQ timing

This is how to get 4 games working: Klax, Skull&Crossbones, Rolling Thunder and Hard Drivin'. 

  * Address mask: $E001.


    
    
    writes to $C000: irq_latch=data;
    writes to $C001: irq_reload=true; irq_mode=data&1;
    writes to $E000: irq_enable=false; IRQ acknowledge by CPU.
    writes to $E001: irq_enable=true; IRQ acknowledge by CPU.
    

  * When the IRQ is clocked by CPU or scanline modes:


    
    
    If irq_reload == true:
       irq_counter = irq_latch;
       if irq_latch != 0
          irq_counter |= 1;
       irq_reload=false;
    Else if irq_counter == 0:
       irq_counter = irq_latch;
    Else
       irq_counter--;
    If irq_counter == 0 and irq_enable == true
       irq_delay=4 (IRQ will be fired 4 CPU cycles later)
    

## Alternate Method for RAMBO-1 IRQ

Nintendulator-NRS's source file /mappers/src/iNES/mapper064.cpp has an alternative implementation of the IRQ which allows it to work properly with all known games to use it: Klax, Skull & Crossbones and Hard Drivin'. 
