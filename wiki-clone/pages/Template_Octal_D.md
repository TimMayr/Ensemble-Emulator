# Template:Octal D

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3AOctal_D) | View [other pages](Special_AllPages.xhtml#Template_Octal_D)

In theory, it would be possible to implement the bank select register with a [74HC377](74377.xhtml "74377") octal D latch, allowing up to {{{1}}}, but due to mask [ROM](ROM.xhtml "ROM") cost in the NES era, no non-pirate NES cart used this much memory. [About this template](Template_Octal_D_doc.xhtml "Template:Octal D/doc"): 

A lot of pages about [74HC161](74161.xhtml "74HC161")-based discrete logic mappers have similar wording. 

Examples: 
    
    
    {{octal D|2 megabytes of PRG ROM}}
    
    {{octal D|
    512 KB of PRG ROM and 128 KB of PRG ROM|
    whynot=but licensed games used an ASIC mapper instead for smaller banks}}
    
