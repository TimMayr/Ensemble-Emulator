# Level compression

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Level_compression) | View [other pages](Special_AllPages.xhtml#Level_compression)

**Level compression** refers to techniques that allow fitting more level data into a smaller space. Levels may easily reach several kilobytes of space uncompressed, and with the cartridge size constraints of an NES game, this is most likely unacceptable. These are some general techniques for NES-friendly level compression, and it is often possible to use multiple ones in the same game. 

## Contents

  * 1 Object based
    * 1.1 Two-byte
    * 1.2 Three-byte
    * 1.3 Alternatives to next-screen bit
    * 1.4 Single-byte Clusters
    * 1.5 Meta commands
  * 2 Reducing the grid size
    * 2.1 Vertical columns
    * 2.2 Metametatiles
    * 2.3 Symmetry
  * 3 Compressed tilemaps
    * 3.1 Flipped/Rotated
    * 3.2 Boustrophedon
  * 4 Other techniques
    * 4.1 Repeating backgrounds
    * 4.2 Post-processing
  * 5 External links



## Object based

Object compression represents levels as a series of commands that represent individual elements of the level. Each "Object" generally consists of an object type, an X and Y position within the level, and a width and a height, where some of these may be implied by the type of object. 

### Two-byte

Mario series games tend to use the object level compression technique. There are many ways to go about storing the information for objects. Super Mario Bros 1 in particular (and the homebrew game [Double Action Blaster Guys](https://www.nesdev.org/w/index.php?title=Double_Action_Blaster_Guys&action=edit&redlink=1 "Double Action Blaster Guys \(page does not exist\)")) store a "Length" within the object type, where a given range of values refer to the same object at different sizes. This tends to lead to a format like the following: 
    
    
    nttttttt xxxxyyyy
    |||||||| ||||++++- Y position within a screen
    |||||||| ++++----- X position within a screen
    |+++++++---------- Object type
    +----------------- Next screen flag, moves to encoding the next screen if 1
    

In this scheme, an object's width or height may be the bottom 3 or 4 bits of the Object Type, and some objects that do not need a size (doors, question mark blocks containing powerups) do not need a length. 

### Three-byte

For later games, widths and heights are usually given their own dedicated bits. A typical Super Mario World object looks like the following: 
    
    
    nBByyyyy bbbbxxxx hhhhwwww
    |||||||| |||||||| ||||++++- Width
    |||||||| |||||||| ++++----- Height
    |||||||| ||||++++---------- X position within a screen
    |++|||||-++++-------------- Object type
    |  +++++------------------- Y position within a screen
    +-------------------------- Next screen flag, moves to encoding the next screen if 1
    

Super Mario World dedicates another bit to the level's Y position, as non-vertical levels are two screens tall. 

### Alternatives to next-screen bit

While Mario games typically use "Next screen" bits in their objects, there are ways around it that free up another bit per-object to be used for another purpose, such as the object type. 

Each level in [President](User_Tepples_President.xhtml "User:Tepples/President") contains a list of how many bytes of information each screen contains, with the engine knowing to move to the next screen once the end is reached. This also makes it easier to move backwards through the level data, by allowing the game to easily get to the previous screen's information. 

[Nova the Squirrel](User_NovaSquirrel_Nova_the_Squirrel.xhtml "User:NovaSquirrel/Nova the Squirrel") makes X positions relative, with each object moving forward 0 to 15 metatiles before performing the command. 

Games that don't scroll (like single-screen arcade games with multiple levels) also have no use for a next-screen bit. 

### Single-byte Clusters

For objects that only need an X and Y position and no size information (such as coins), a group of them can be stored with only one byte per object, like so: 
    
    
    rxxxyyyy
    ||||++++- Y position (0-15), absolute
    |+++----- X position (0-7), relative to the previous object
    +-------- Repeat; if 1, this command is followed by another of this type
    

The "repeat" bit could be replaced with another X or Y bit, but that would require a byte to be used up for the length of the group. 

### Meta commands

With the level effectively made up of a series of commands, it is easy to include signals in the level format that do things other than encode level tiles alongside the actual level objects. For example, Super Mario Bros 1 has a set of commands (encoded as objects with high Y positions) that set pointers for pipe destinations and enable various backgrounds. 

## Reducing the grid size

One technique for reducing the amount of space the level takes up is to reduce the width and/or height of the level grid that needs to be stored. This is accomplished by making the chunks that the level is composed of larger, so that one byte encodes more space. 

### Vertical columns

In this scheme, the game stores a list of predefined columns of blocks. Through this method, the level is reduced down to a one dimensional array. 

The _Legend of Zelda_ is one game that stores its map data as a series of vertical columns. _Super Mario Bros 1_ has a library of vertical column "templates" that it can then place other blocks on top of. 

### Metametatiles

Vertical columns may not provide the flexibility needed for games, so another option is to make the level out of chunks that have a width as well as a height. If the level is composed of 32x32 tiles rather than 16x16 ones, the game only needs to store 25% as much information, plus the dictionary to turn larger metatiles into smaller ones. 

_Mega Man_ games that split their maps into 32x32 metametatiles (megatiles?), composed of four 16x16 metatiles. _Blaster Master_ and _Full Quiet_ go even further, using 64x64 chunks that are then composed of 32x32 metametatiles, that are then in turn composed of 16x16 metatiles. Several other Sunsoft games have 256x256 as their largest chunk size.) _Sonic the Hedgehog 2_ uses 128x128 metatiles composed of 16x16 metatiles. Another benefit of 32x32 metametatiles in games that don't scroll vertically is that this is the same size that an attribute table byte covers, so you can store premade color information that is ready to be copied directly into the attribute table. 

### Symmetry

_Micro Mages_ uses [vertical symmetry](https://youtu.be/ZWQ0591PAxM?t=468) as well as metametatiles, reducing the amount of data required for each 256x32 level chunk down to just four bytes. To compensate for the blandness that symmetry could cause, the most significant bit of each metametatile ID encodes an amount to shift the row horizontally. 

A horizontally scrolling game could use horizontal symmetry instead, with vertical shifting. 

## Compressed tilemaps

Another option for level compression is to just start with the final, complete map of the level and use compression algorithms such as run-length encoding, or more advanced options like the LZ77 family. 

Kirby's Adventure's levels are stored this way. This method allows complete flexibility with placing metatiles completely freely (allowing for things like Kirby's complicated backgrounds), but might not compress as well as other methods. 

Another important advantage is that a specialized map editor is not required, and something like [Tiled](https://www.mapeditor.org/) can be used alongside a simple script to convert and compress the output from it. 

### Flipped/Rotated

Some types of compression can benefit from storing the level flipped or rotated, and then fixing it before play. For example, for compression based around horizontal runs, a level with many vertical lines could be stored rotated to change the vertical lines into horizontal ones. 

### Boustrophedon

Having one row go left-to-right, then having the next row go right-to-left, alternating back and forth. Improves compression on areas of the same tile that touch the edge of the map, allowing the data to be encoded with fewer runs. 

## Other techniques

### Repeating backgrounds

Super Mario Bros has a set of backgrounds that get placed onto each screen before tiles get placed on top of it. With the repeating backgrounds, the game can have a background without needing to specify where each cloud or other element goes. 

### Post-processing

This technique can reduce the amount of effort that goes into level creation alongside just making the level smaller. Modern game development has a concept that's sometimes referred to as _auto-tiling_ where a map tile chooses its appearance based on its value as well as the values of the surrounding cells. This means the level designer does not need to manually place down edges, corners, and other details, nor does the map need to spend bytes on encoding them. 

A disadvantage is that it takes some control away from the level designer unless a way to override the auto-tiling is provided, and edges might be desired in more places than just the outsides of contiguous shapes. Also, if the entire level is stored in PRG RAM and the post-processing is run over the entire level after decompression, this also results in the player waiting longer before a level starts, so care should be taken to ensure it is fast. 

_Nova the Squirrel_ is one example game that does this, automatically adding edges/corners to ground tiles, surfaces to water, bottom parts of doors, and other changes. 

_Haunted: Halloween '85_ and its sequel _The Curse of Possum Hollow_ use vertical RLE modified with a Markov chain algorithm, which interprets a "run" as a mapping from each tile to the most common tile below it. 

## External links

[Super Mario World level format](http://smwspeedruns.com/index.php/Level_Data_Format)

[Map Decoding in President](http://pineight.com/mw/index.php?title=Map_decoding_in_President)
