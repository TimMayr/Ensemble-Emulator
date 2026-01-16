# VRC IRQ

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC_IRQ) | View [other pages](Special_AllPages.xhtml#VRC_IRQ)

## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 IRQ Latch
    * 2.2 IRQ Control
    * 2.3 IRQ Acknowledge
  * 3 Operation



## Overview

Konami's [VRC4](VRC2_and_VRC4.xhtml "VRC4"), [VRC6](VRC6.xhtml "VRC6"), and [VRC7](VRC7.xhtml "VRC7") share the same IRQ system. 

This system is fairly unique, in that it is driven by CPU cycles, yet it can imitate a scanline counter. It does this by having a prescaler that divides the cycles by 113⅔, the number of cycles in each scanline on [Famicom, NTSC NES, and Dendy-style PAL famiclones](Cycle_reference_chart.xhtml#Clock_rates "Cycle reference chart"). The scanline on an authentic PAL NES is slightly shorter (106⁵⁄₁₆); if PAL games were to use a VRC, scanline counts would need to be rescaled by ¹⁵/₁₆. Since it is a CPU cycle counter and not a true scanline counter, when enabled it will run even during VBlank, or even when PPU rendering is disabled. 

## Registers

There are 3 control registers for VRC IRQs: **IRQ Latch** , **IRQ Control** , and **IRQ Acknowledge** (see desired mapper page for which addresses correspond to which register). 

### IRQ Latch
    
    
    7  bit  0
    ---------
    LLLL LLLL
    |||| ||||
    ++++-++++- IRQ Latch (reload value)
    

Note that on VRC4, this register is split across two addresses: one for the high 4 bits, and one for the low 4 bits. 

  


### IRQ Control
    
    
    7  bit  0
    ---------
    .... .MEA
          |||
          ||+- IRQ Enable after acknowledgement (see **IRQ Acknowledge**)
          |+-- IRQ Enable (1 = enabled)
          +--- IRQ Mode (1 = cycle mode, 0 = scanline mode)
    

Any write to this register will acknowledge the pending IRQ and reset the prescaler. If this register is written to with 'E' set, the IRQ counter is reloaded with the latch value. If 'E' is clear, the IRQ counter remains unchanged. The 'A' bit here has no immediate effect, and remains unused until **IRQ Acknowledge** is written to. It can be used to distinguish a one-shot IRQ from a repeated IRQ. 

### IRQ Acknowledge
    
    
    7  bit  0
    ---------
    .... ....
    

Any write to this register will acknowledge the pending IRQ. In addition, the 'A' control bit moves to the 'E' control bit, enabling or disabling IRQs. Writes to this register do not affect the current state of the IRQ counter or prescaler. 

  


## Operation

When in scanline mode ('M' bit clear), a prescaler divides the passing CPU cycles by 114, 114, then 113 (and repeats that order). This approximates 113+2/3 CPU cycles, which is one NTSC scanline. When the prescaler is reset (by writing to the **IRQ Control** register with 'E' set), the sequence is reset and it will be 114 CPU cycles before the IRQ counter will be clocked. 

A simple way to emulate this behavior is to have the prescaler start at 341 and subtract 3 every CPU cycle. When it drops to or below 0, increment it by 341 and clock the IRQ counter. 

When in cycle mode ('M' bit set), the prescaler is effectively bypassed, and the IRQ counter gets clocked every CPU cycle. 

When the IRQ counter is clocked: 

  * If IRQ counter is $FF, reload IRQ counter with latch value 'L', trip IRQ
  * otherwise, increment IRQ counter by 1



If IRQs are disabled ('E' bit clear), neither the prescaler nor IRQ counter gets clocked. 
