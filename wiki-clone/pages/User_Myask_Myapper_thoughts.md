# User:Myask/Myapper thoughts

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AMyask/Myapper_thoughts) | View [other pages](Special_AllPages.xhtml#User_Myask_Myapper_thoughts)

## Contents

  * 1 8x8 attributes, beginning of 8x1
  * 2 8x1 attributes?
  * 3 Flipping
  * 4 DMA theft
  * 5 Slideshow mirroring
  * 6 Fill Mode
  * 7 Textbox Bank
  * 8 Oekarcade
  * 9 "3d" maze acceleration/rendering
  * 10 Various extra attribute bits
    * 10.1 rotate 90°
    * 10.2 8x8-only
    * 10.3 8x8 page override
    * 10.4 Just 3 extra per-tile address bits
  * 11 CHR-BlockRAM



## 8x8 attributes, beginning of 8x1

[User:Myask/MyaGrafx](User_Myask_MyaGrafx.xhtml "User:Myask/MyaGrafx")

## 8x1 attributes?

It's all in the fine Y and keeping it in mind. The intensive solution is to watch the PPU bus and keep our own copy of PPUSCROLL, which requires knowing the latch and thus watching several of the registers. Pattern-fetch snooping can yield it, but as it is after the attribute fetch, it does not work for the first fetch of a scanline. Triple fetch is after said 2 pre-fetched tiles. Is really a double-fetch duplicating next one normally, but VBLANK is between the last line's double-fetch and the first of pre-render, which is interrupted by whatever the program wants to access. One could count tiles, then: 
    
    
    if(this_fetch == last_fetch) reset(counter);
    if(is_AT(this_fetch) && is_NT(last_fetch)) counter += 1;
    if(counter == (32? 33? 34?)) //override the usual pattern, we're doing first prefetch
    ///...after which we can just let the pattern fetch give us the information
    

But is this any simpler than register-snooping? This segues nicely into a scanline counter interrupt, though for some reason I have the thought of a nametable-relative Y-based interrupt instead (which is just a different frame of reference on the same thing.) Resetting to "scanline 0" if no triple-fetch detected in 128 CPU clocks seems workable. 

## Flipping

BG tileflipping is pretty easy. 
    
    
    chr_a[2:0] = ppu_a[2:0] ^ {3{is_chr_access & vert_flip}};
    ppu_d[7:0] = (is_chr_access) ? (horiz_flip ? chr_d[0:7] : chr_d[7:0]) : something_else);
    

If one stored the attributes in the same byte you can piggyback that to set the flip bits. But what to use the other four bits for? Just do two tiles per byte? Allow MMC5-like extended tile index allowance? Allow swapping of two colors in the attribute? 
    
    
    wire invert_cur = ppu_a[4] ? invert_plane1 : invert_plane0;
    ppu_d[7:0] = (is_chr_access) ? (invert_cur ^ (horiz_flip ? chr_d[0:7] : chr_d[7:0])) : something_else);
    

Rotation would require orthogonal accesses, all-but requiring in-chip CHR. Note: one should only do this if things are following (AT,NT,PT,PT) pattern. If not, one is fetching sprite patterns. While it is of mild interest that there are 64 sprites filling out the 32x32 count, OAM already deals with that better. 

Though, one could use the garbage NT fetches (which are a double fetch, can leech off logic detecting the triple fetch!) to add extended per-sprite banking, though detecting which sprite is which is Another Problem, requiring either duplicating/spying OAM's tile entry, or requiring some specific pattern of sprite arrangement (useful for static images?). This also allows some odd bits, like changing sprite tile mid-sprite (vertically). 

## DMA theft

If you watch for a $4014 write, you can then watch the DMA happen...and thus have another destination to copy it to. This could alleviate the VBLANK crunch a bit (for e.g. the 8x1 mapper), as DMA copies much faster than program can. Obviously bringing it into the chip 'à la' MMC5 means one could allow access to both buses, but that costs a lot of chip resources. 

## Slideshow mirroring
    
    
    PPUDOUT[0:7] = PPUDIN[0:7] & {8,{~PPUA[11]&PPUA[9]}};//causes all tiles to be 0 and also be BG color
    CIRAMA10 = PPUA[10];
    //ab
    //00
    

Of course, the slideshow effect could be done without any special hardware just by using a render-disable raster effect instead of a scroll. 

## Fill Mode

…is an expansion of the above: 
    
    
    wire is_fill_mode = (nt_mapping[PPU_A[11:10]] == FILL_MODE) && is_nt_fetch;
    PPUDOUT[0:7] = is_fill_mode ? fill_chr[0:7] : chrdin[0:7];
    

Actually, now I think it, "CIRAM0, CIRAM1, disable, fill mode" isn't a bad set of 4. Catch is, of course, that that would mean cartside NT0-3 makes it 12 bits of mirroring to allow full control, two writes. That leaves four bits of unused write… 

## Textbox Bank

Sort of an expanded MMC4, switch NT(+AT) as well as CHR bank on detecting certain tile-reads. 

Note that this means you don't have to align the textbox to a 16x16 as the switch will also switch on per-tile where the palette comes from. 

This lets you arbitrarily block-group off tile blocks of the screen for certain segments (see: Fire Emblem textboxes). However, you might need to use sprites to mark the corners; otherwise you need extra viable start- and end-tile designations in the mapper, eating up your usable tilespace. 

It need not be for a textbox, a portrait or some sub-part of the screen that tends to be CHR-greedy. For instance, consider the Macventure ports (**Shadowgate** et al.) and their boxing-off of the screen: one could expand the Current Scene viewpoint to a full 16x16 tiles with just one bank. (It's presently 14x14, and looking at the Sphinx image showed that the two most common tiles were 61 of the 196 tiles available,) Of course, this has the drawback of reducing the overdraw fraction of such a viewport available to increase colordepth from sprite palettes. 

## Oekarcade

One need not bank the CHR on vertical quarters of the screen like Oeka Kids. One could arbitrarily section the screen, like so: 

Example 1 |  | Example 2 |  | Example 3   
---|---|---|---|---  
1 | 1 | 1 | 1 |  | 1 | 2 | 3 | 1 |  | 1 | 1 | 2 | 2   
2 | 4 | 4 | 2 |  | 3 | 4 | 4 | 2 |  | 1 | 1 | 2 | 2   
2 | 4 | 4 | 2 |  | 2 | 4 | 4 | 3 |  | 3 | 3 | 4 | 4   
3 | 3 | 3 | 3 |  | 1 | 3 | 2 | 1 |  | 3 | 3 | 4 | 4   
  
Remember, any quartering of the screen is sufficient to allow every 8x8 to have its own unique CHR tile. 

Named "Oekarcade" for that Example 1 would allow an arcade-cabinet screenborder with score display in 1, hands/buttons in 4. 

## "3d" maze acceleration/rendering

only for "midimaze-plus": no height differences, no textures, but arbitrary wall angles. in Blockram: a 256-element height count (+1 color bit) (quick-fill by DMA repurpose) read eight bytes of pixel heights 
    
    
    if ((scanline.d7 ? ~scanline : scanline) < (height[x]/2)) px = 2+colorbit
    elif ((scanline.d7 ? ~scanline : scanline) == (height[x]/2)) px = LINECOLOR (1)
    else px = BG (0)
    

output the two planes to another blockram as CHR (need up to 512 bits) for next scanline timing a bit tricky thanks to hblank More sophisticated logic may be added to 8x1sliver palette? Already genning the colors; shouldn't take too much more. I don't want to think about color-overflow-prevention logic. 

Palette as just scanline.d7 does a simple horizon. 

Height may be generated by a division table of Z->Y, though I suspect software table-lookup preferable. In a full MIDImaze-like maze, deriving Z seems simpler; converting the map to a format allowing a LUT on "is there an edge at each of these grid-edges for this ray" seems like the hard part. 

## Various extra attribute bits

Require: DMA spying (obviously), some mode of determining when each sprite will render (duplicate sprite-processing thread?)…though one could cut a corner here and just have it apply to all sprites with that same tile. 

### rotate 90°

(requires 1K RAM) …rotation by -90° is just by doing this and also hitting the flip bits. Would have glitches if tiles of sprites changed since last frame by my first method of watching them be read to populate the blockRAM to rotate the tile. (A cart where the mapper interposed all CHR signals would be able to just do its own DMA…but actually, a DMA is when this is learned to be relevant, so one could…except, no, PPU doesn't stop driving A, R/W for anything, so it'd still require interposed chip.) Doesn't work for 8x16 in useful ways, mostly, without… 

### 8x8-only

requires PPUCTRL($2000).d5 spying to know when 8x16 is on. Force the lower tile of an 8x16 sprite to retrieve all zeros, yielding transparency. 0T_ILET_ILE1_xxxx = 0 

### 8x8 page override

requires PPUCTRL.d5 spying. Have an 8x8 sprite pattern's be fetched from the other side (flip its A12). Or… 

### Just 3 extra per-tile address bits

Header says it all. 

## CHR-BlockRAM

…if you're just going to have all 64 sprites use a RAM-type mode, the 90° rotation becomes easier. However, it'd require 2K if you were using 8x16, and that's a lot slower to update than a tile index. 
