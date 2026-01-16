# Registers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Registers) | View [other pages](Special_AllPages.xhtml#Registers)

Registers provide internal state storage for a device. 

These may store a number, like the accumulator register in the [CPU](CPU.xhtml "CPU"), or they may store other behaviour states, like the colour output of the [PPU](PPU.xhtml "PPU"). The various cartridge [mappers](Mapper.xhtml "Mapper") have registers that control [banking](https://www.nesdev.org/w/index.php?title=Banking&action=edit&redlink=1 "Banking \(page does not exist\)"), as well as other devices (e.g. [IRQ](IRQ.xhtml "IRQ") timer). 

A register might be entirely internal, or it might have a memory mapped address to provide external access via the CPU. Such registers are often referred to by this write address. 

See: 

  * [CPU registers](CPU_registers.xhtml "CPU registers")
  * [PPU registers](PPU_registers.xhtml "PPU registers")
  * [APU registers](APU_registers.xhtml "APU registers")
  * [2A03 register map](2A03.xhtml "2A03")


