# PPU attribute tables

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PPU_attribute_tables) | View [other pages](Special_AllPages.xhtml#PPU_attribute_tables)

An **attribute table** is a 64-byte array at the end of each [nametable](PPU_nametables.xhtml "PPU nametables") that controls which palette is assigned to each part of the background. 

Each attribute table, starting at $23C0, $27C0, $2BC0, or $2FC0, is arranged as an 8x8 byte array: 
    
    
           2xx0    2xx1    2xx2    2xx3    2xx4    2xx5    2xx6    2xx7
         ,-------+-------+-------+-------+-------+-------+-------+-------.
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
    2xC0:| - + - | - + - | - + - | - + - | - + - | - + - | - + - | - + - |
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
         +-------+-------+-------+-------+-------+-------+-------+-------+
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
    2xC8:| - + - | - + - | - + - | - + - | - + - | - + - | - + - | - + - |
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
         +-------+-------+-------+-------+-------+-------+-------+-------+
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
    2xD0:| - + - | - + - | - + - | - + - | - + - | - + - | - + - | - + - |
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
         +-------+-------+-------+-------+-------+-------+-------+-------+
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
    2xD8:| - + - | - + - | - + - | - + - | - + - | - + - | - + - | - + - |
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
         +-------+-------+-------+-------+-------+-------+-------+-------+
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
    2xE0:| - + - | - + - | - + - | - + - | - + - | - + - | - + - | - + - |
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
         +-------+-------+-------+-------+-------+-------+-------+-------+
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
    2xE8:| - + - | - + - | - + - | - + - | - + - | - + - | - + - | - + - |
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
         +-------+-------+-------+-------+-------+-------+-------+-------+
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
    2xF0:| - + - | - + - | - + - | - + - | - + - | - + - | - + - | - + - |
         |   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
         +-------+-------+-------+-------+-------+-------+-------+-------+
    2xF8:|   .   |   .   |   .   |   .   |   .   |   .   |   .   |   .   |
         `-------+-------+-------+-------+-------+-------+-------+-------'
    
    
    
    ,---+---+---+---.
    |   |   |   |   |
    + D1-D0 + D3-D2 +
    |   |   |   |   |
    +---+---+---+---+
    |   |   |   |   |
    + D5-D4 + D7-D6 +
    |   |   |   |   |
    `---+---+---+---'
    

Each byte controls the palette of a 32×32 pixel or 4×4 tile part of the nametable and is divided into four 2-bit areas. Each area covers 16×16 pixels or 2×2 tiles, the size of a [?] block in _Super Mario Bros._ Given palette numbers topleft, topright, bottomleft, bottomright, each in the range 0 to 3, the value of the byte is 
    
    
    value = (bottomright << 6) | (bottomleft << 4) | (topright << 2) | (topleft << 0)
    

Or equivalently: 
    
    
    7654 3210
    |||| ||++- Color bits 3-2 for top left quadrant of this byte
    |||| ++--- Color bits 3-2 for top right quadrant of this byte
    ||++------ Color bits 3-2 for bottom left quadrant of this byte
    ++-------- Color bits 3-2 for bottom right quadrant of this byte
    

Most games for the NES use 16×16 pixel [metatiles](https://www.nesdev.org/w/index.php?title=Metatile&action=edit&redlink=1 "Metatile \(page does not exist\)") (size of _Super Mario Bros._ ? block) or 32x32 pixel metatiles (width of **SMB** pipe) in order to align the map with the attribute areas. 

## Worked example

[![](../wiki-images/Thwaite_bg_with_attr_grid.png)](File_Thwaite_bg_with_attr_grid_png.xhtml)

The background in the game _[Thwaite](User_Tepples_Thwaite.xhtml "User:Tepples/Thwaite")_ , with an overlaid attribute grid.

[![](../wiki-images/Thwaite_attrs.png)](File_Thwaite_attrs_png.xhtml)

Each 16x16 pixel color area has one of four subpalettes assigned to it, and one byte controls four color areas.

[![](../wiki-images/Thwaite_palette_color_sets.png)](File_Thwaite_palette_color_sets_png.xhtml)

The background palette has one backdrop color and four three-color subpalettes.

Consider the byte at $23F2: 
    
    
     ,---- Top left
     3 1 - Top right
     2 2 - Bottom right
     `---- Bottom left
    

The byte has color set 2 at bottom right, 2 at bottom left, 1 at top right, and 3 at top left. Thus its attribute is calculated as follows: 
    
    
    value = (bottomright << 6) | (bottomleft << 4) | (topright << 2) | (topleft << 0)
          = (2           << 6) | (2          << 4) | (1        << 2) | (3       << 0)
          = $80                | $20               | $04             | $03
          = $A7
    

To find the address of an attribute byte corresponding to a nametable address, see: [PPU scrolling: Tile and attribute fetching](PPU_scrolling.xhtml#Tile_and_attribute_fetching "PPU scrolling")

## Glitches

There are some well-known glitches when rendering attributes in NES and Famicom games. 

While an attribute table specifies one of four three-color palettes for each 16x16 pixel region, the left-side clipping window in [PPUMASK ($2001)](PPU_registers.xhtml "PPU registers") is only 8 pixels wide. 

This is the reason why games that use either horizontal or vertical [mirroring](Mirroring.xhtml "Mirroring") modes for arbitrary-direction [scrolling](PPU_scrolling.xhtml "PPU scrolling") often have color artifacts on one side of the screen (on the right side in _Super Mario Bros. 3_ ; on the trailing side of the scroll in _Kirby's Adventure_ ; and at the top and bottom in _Super C_). 

The game _Alfred Chicken_ hides glitches on the left and right sides by using both left clipping and hiding the right side of the screen under solid-colored sprites. To mask the entire 240-scanline height, this approach would occupy 15 entries of 64 in the sprite table in 8x16 sprite mode, or 30 entries in the 8x8 mode. 

## Expansion

There are two bits of memory in the attribute table that control the palette selection for each 16x16 pixel area on the screen. Because the PPU actually fetches that memory redundantly for each 8x1 pixel area as it draws the display, it is possible for a mapper to control this memory and supply different data for each read. The [MMC5](MMC5.xhtml "MMC5") mapper does this for its 8x8 extended attribute mode. 
