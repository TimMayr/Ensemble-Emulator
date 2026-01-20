# Drawing terrain

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Drawing_terrain) | View [other pages](Special_AllPages.xhtml#Drawing_terrain)

A summary of ideas presented in "My Tileset Workflow (Pixel Art & Gamedev Tutorial)" by MortMort[[1]](https://www.youtube.com/watch?v=btnH0x7_1g8)

Some parts are easier in an image editor with an automatic tessellation (tiling) view that repeats a file three or nine times in the editing window. This may require a proprietary commercial editor focused on game pixel art. 

Center tile
    Make this in a separate file using an image editor with a tessellation view, possibly representing strata within a cross-section of bulk material, then paste the result onto your complete tile sheet.
Top tile
    Duplicate the center tile, set up horizontal tessellation, then build up layers from bottom to top using random-ish strokes with progressively smaller brushes and lighter colors (black to dark cyan to green to lighter, warmer green), then shadow some of the grass strokes with darker value green strokes, then add shadow between the grass and dirt. paste onto tile sheet
Bottom tile
    Duplicate the center tile, set up horizontal tessellation, drop random black cracks in the bottom half of the tile, shade inside cracks and/or blend cracks into bulk. possibly add sky pixels on bottom row. paste onto tile sheet
Left and right sides
    Duplicate the center tile, set up vertical tessellation, repeat cracking and blending for bottom on left side. paste onto tile sheet. repeat for right side
Corners
    These start as copies of the top and bottom sides. Copy the cracks in the bottom half of the side tiles to the top corners and the cracks in the top half of the side tiles to the bottom corners. Then make a curve that extends from the side cracks into the grass on top and the cracks on bottom, blend them in, and light the edges.
Isolated slabs
    Copy the left half of the left side and the right half of the right side to make a 1-cell-wide slab; likewise with the top and bottom half tiles to make a 1-cell-tall slab and a single isolated cell.

By 11 minutes in, a basic tile set is done. Some additions flesh it out: 

Inside turns
    Copy top, bottom, left, and right to bottom, top, right, and left of new 3x3-tile file. Paste center tile in each corner. Copy right quarter of bottom (old top) to right part of bottom left and left quarter of bottom to left part of bottom right, and add shadow transition as with old top tile. Repeat with cracks at top corners. Copy corner tiles to main sheet.
Long slopes (1 rise per 2 run)
    Make 4x2-tile file. Copy top and center and paste three across into new file and a fourth offset a tile down. In the center 2x2 tiles, erase the dirt below the shadow; consider using color select tool. Then select the center 2x2 tiles and use shear tool, or drag each 2-pixel rectangle down an additional pixel. Blend or redraw the grass. In a new 2x2-tile file, paste four copies of the center tile, and erase a part roughly corresponding to the slope. Copy the center 2x2 tiles, paste on top of this, blend the shadow, and copy back to main sheet. Repeat for the opposite direction.
Steep slopes (1 rise per 1 run)
    Technique is similar to long slope, just with a 3x2-tile file and a steeper shear. More clean-up on the grass will be required.

After having made these tiles once, it is an exercise for the reader to create underside slopes, transitions to a cave (in case you treat platforms over platforms as overhangs), distinguishing solid slabs vs. one-way platforms, and the like. Likewise with autotiler rules that combine adjacent tiles of like material into slabs. 
