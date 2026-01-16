# PPU nametables

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PPU_nametables) | View [other pages](Special_AllPages.xhtml#PPU_nametables)

A **nametable** is a 1024 byte area of memory used by the PPU to lay out backgrounds. Each byte in the nametable controls one 8x8 pixel character cell, and each nametable has 30 rows of 32 tiles each, for 960 ($3C0) bytes; the 64 ($40) remaining bytes are used by each nametable's [attribute table](PPU_attribute_tables.xhtml "PPU attribute tables"). With each tile being 8x8 pixels, this makes a total of 256x240 pixels in one map, the same size as one full screen. 
    
    
         (0,0)     (256,0)     (511,0)
           +-----------+-----------+
           |           |           |
           |           |           |
           |   $2000   |   $2400   |
           |           |           |
           |           |           |
    (0,240)+-----------+-----------+(511,240)
           |           |           |
           |           |           |
           |   $2800   |   $2C00   |
           |           |           |
           |           |           |
           +-----------+-----------+
         (0,479)   (256,479)   (511,479)
    

    _See also:[PPU memory map](PPU_memory_map.xhtml "PPU memory map")_

## Mirroring

    _Main article:[Mirroring](Mirroring.xhtml "Mirroring")_

The NES has four logical nametables, arranged in a 2x2 pattern. Each occupies a 1 KiB chunk of PPU address space, starting at $2000 at the top left, $2400 at the top right, $2800 at the bottom left, and $2C00 at the bottom right. 

But the NES system board itself has only 2 KiB of VRAM (called CIRAM, stored in a separate SRAM chip), enough for two physical nametables; hardware on the cartridge controls address bit 10 of CIRAM to map one nametable on top of another. 

  * Vertical mirroring: $2000 equals $2800 and $2400 equals $2C00 (e.g. _Super Mario Bros._)
  * Horizontal mirroring: $2000 equals $2400 and $2800 equals $2C00 (e.g. _Kid Icarus_)
  * One-screen mirroring: All nametables refer to the same memory at any given time, and the mapper directly manipulates CIRAM address bit 10 (e.g. many [Rare](Rare.xhtml "Rare") games using [AxROM](AxROM.xhtml "AxROM"))
  * Four-screen mirroring: The cartridge contains additional VRAM used for all nametables (e.g. _Gauntlet_ , _Rad Racer 2_)
  * Other: Some advanced mappers can present arbitrary combinations of CIRAM, VRAM, or even CHR ROM in the nametable area. Such exotic setups are rarely used.



## Background evaluation

    _Main article:[PPU rendering](PPU_rendering.xhtml "PPU rendering")_

Conceptually, the PPU does this 33 times for each scanline: 

  1. Fetch a nametable entry from $2000-$2FFF.
  2. Fetch the corresponding attribute table entry from $23C0-$2FFF and increment the current VRAM address within the same row.
  3. Fetch the low-order byte of an 8x1 pixel sliver of pattern table from $0000-$0FF7 or $1000-$1FF7.
  4. Fetch the high-order byte of this sliver from an address 8 bytes higher.
  5. Turn the attribute data and the pattern table data into palette indices, and combine them with data from [sprite data](PPU_sprite_evaluation.xhtml "PPU sprite evaluation") using [priority](PPU_sprite_priority.xhtml "PPU sprite priority").



It also does a fetch of a 34th (nametable, attribute, pattern) tuple that is never used, but some [mappers](Mapper.xhtml "Mapper") rely on this fetch for timing purposes. 

## See also

  * [PPU attribute tables](PPU_attribute_tables.xhtml "PPU attribute tables")


