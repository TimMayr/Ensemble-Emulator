# Sample RAM map

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sample_RAM_map) | View [other pages](Special_AllPages.xhtml#Sample_RAM_map)

Documents about programming for systems using the 6502 CPU often refer to RAM in 256-byte "pages". As described in [CPU memory map](CPU_memory_map.xhtml "CPU memory map"), the NES has a 2048 byte RAM connected to the CPU, which provides eight such pages at $0000-$07FF. The optional PRG RAM chip on some cartridge boards is an 8192 byte SRAM that provides 32 pages at $6000-$7FFF. 

It's up to you to find uses for this memory, within certain restrictions imposed by the NES's architecture. Indirect addressing modes on 6502 rely on the "zero page" (or "direct page"), which lies at $0000-$00FF. Some other addressing modes can read or write the zero page slightly faster. The stack instructions (PHA, PLA, PHP, PLP, JSR, RTS, BRK, RTI) always access the "stack page", which lies at $0100-$01FF. But you can use the parts of the stack page that those instructions aren't using. 

Here's a sketch of a memory map that may work for your programs. Feel free to adapt it to fit your needs. 

Addresses | Size | What can go there   
---|---|---  
$0000-$000F | 16 bytes | Local variables and function arguments   
$0010-$00FF | 240 bytes | Global variables accessed most often, including certain pointer tables   
$0100-$019F | 160 bytes | Data to be copied to [nametable](PPU_nametables.xhtml "PPU nametables") during next vertical blank (see [The frame and NMIs](The_frame_and_NMIs.xhtml "The frame and NMIs"))   
$01A0-$01FF | 96 bytes | Stack   
$0200-$02FF | 256 bytes | Data to be copied to [OAM](PPU_OAM.xhtml "OAM") during next vertical blank   
$0300-$03FF | 256 bytes | Variables used by sound player, and possibly other variables   
$0400-$07FF | 1024 bytes | Arrays and less-often-accessed global variables 
