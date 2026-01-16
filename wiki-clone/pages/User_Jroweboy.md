# User:Jroweboy

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AJroweboy) | View [other pages](Special_AllPages.xhtml#User_Jroweboy)

Staging location for a new Programming guide/article on Banking and Interrupt safe banking for various common mappers. 

Banking (aka [Bank switching](https://en.wikipedia.org/wiki/Bank_switching "wikipedia:Bank switching")) is a technique that many mappers provide to expand the total capacity of a cartridge. Inside of a cartridge, there are one or more Memory Banks (aka banks) and a dedicated hardware register that will reroute reads to the correct bank. By banking a certain region of memory, the address that the CPU reads will remain the same, but the data that the CPU reads will depend on the current selected bank. The size of a bank is dependent on the 

## Contents

  * 1 The Problem that Banking Solves
  * 2 How Does Banking Work
  * 3 Banking Instructions for Common Mappers
  * 4 Interrupt Safe Banking



## The Problem that Banking Solves

The NES uses a 6502 CPU that only supports 16 bits of [CPU_memory_map:addressable space](https://www.nesdev.org/w/index.php?title=CPU_memory_map:addressable_space&action=edit&redlink=1 "CPU memory map:addressable space \(page does not exist\)"), meaning that any code, data, or register must be within the region [$0000, $FFFF] in order for the CPU to access it. For most carts, only the CPU memory region from [$8000, $FFFF] can be used for code and data and the PPU memory region from [$0000, $2000] can be used for Background Tile and Sprite graphics, leaving a total of 32kb PRG ROM + 8kb CHR ROM = 40kb of ROM space. Realistically, its very challenging to make large scope games with only 40kb of space, and without any dedicated cartridge hardware, there is no way to access any data outside of these regions. 

## How Does Banking Work

Banking is a feature of many mappers that will reroute memory access to a different location of memory. Inside the cartridge, there is a data line that connects the CPU to the memory bank, and on that line is a register that will change which bank that line will read from. By writing to this register, the software is able to affect what data will be 

As an analogy, consider a small desk that can only fit 4 pieces of paper at one time. There's only two different things you can do with these papers, you can read a line from a paper, or you can switch it out with another one. Imagine someone says "Read line 3 on page 4", the line you read back 

## Banking Instructions for Common Mappers

## Interrupt Safe Banking
