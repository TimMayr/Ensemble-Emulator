# PPU memory map

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PPU_memory_map) | View [other pages](Special_AllPages.xhtml#PPU_memory_map)

### PPU memory map

The [PPU](PPU.xhtml "PPU") addresses a 14-bit (16kB) address space, $0000-$3FFF, completely separate from the CPU's address bus. It is either directly accessed by the PPU itself, or via the CPU with [memory mapped registers](PPU_registers.xhtml "PPU registers") at $2006 and $2007. 

Address range | Size | Description | Mapped by   
---|---|---|---  
$0000-$0FFF | $1000 | [Pattern table](PPU_pattern_tables.xhtml "PPU pattern tables") 0 | Cartridge   
$1000-$1FFF | $1000 | Pattern table 1 | Cartridge   
$2000-$23FF | $0400 | [Nametable](PPU_nametables.xhtml "PPU nametables") 0 | Cartridge   
$2400-$27FF | $0400 | Nametable 1 | Cartridge   
$2800-$2BFF | $0400 | Nametable 2 | Cartridge   
$2C00-$2FFF | $0400 | Nametable 3 | Cartridge   
$3000-$3EFF | $0F00 | Unused | Cartridge   
$3F00-$3F1F | $0020 | [Palette RAM](PPU_palettes.xhtml "PPU palettes") indexes | Internal to PPU   
$3F20-$3FFF | $00E0 | Mirrors of $3F00-$3F1F | Internal to PPU   
  
## Hardware mapping

The NES has 2kB of RAM dedicated to the PPU, usually mapped to the nametable address space from $2000-$2FFF, but this can be rerouted through custom cartridge wiring. The mappings above are the addresses from which the PPU uses to fetch data during rendering. The actual devices that the PPU fetches pattern, name table and attribute table data from is configured by the cartridge. 

  * $0000-1FFF is normally mapped by the cartridge to a [CHR-ROM or CHR-RAM](CHR_ROM_vs__CHR_RAM.xhtml "CHR ROM vs. CHR RAM"), often with a bank switching mechanism.


  * $2000-2FFF is normally mapped to the 2kB NES internal VRAM, providing 2 nametables with a [mirroring](Mirroring.xhtml#Nametable_Mirroring "Mirroring") configuration controlled by the cartridge, but it can be partly or fully remapped to ROM or RAM on the cartridge, allowing up to 4 simultaneous nametables.


  * $3000-3EFF is usually a mirror of the 2kB region from $2000-2EFF. The PPU does not render from this address range, so this space has negligible utility.


  * $3F00-3FFF is not configurable, always mapped to the internal palette control.



## OAM

In addition, the PPU internally contains 256 bytes of memory known as [Object Attribute Memory](PPU_OAM.xhtml "PPU OAM") which determines how sprites are rendered. The CPU can manipulate this memory through [memory mapped registers](PPU_registers.xhtml "PPU registers") at [OAMADDR](PPU_registers.xhtml "OAMADDR") ($2003), [OAMDATA](PPU_registers.xhtml "OAMDATA") ($2004), and [OAMDMA](PPU_registers.xhtml "OAMDMA") ($4014). OAM can be viewed as an array with 64 entries. Each entry has 4 bytes: the sprite Y coordinate, the sprite tile number, the sprite attribute, and the sprite X coordinate. 

Address Low Nibble | Description   
---|---  
$0, $4, $8, $C | Sprite Y coordinate   
$1, $5, $9, $D | Sprite tile #   
$2, $6, $A, $E | Sprite attribute   
$3, $7, $B, $F | Sprite X coordinate 
