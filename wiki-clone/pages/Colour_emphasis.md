# Colour emphasis

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Colour_emphasis) | View [other pages](Special_AllPages.xhtml#Colour_emphasis)

Colour emphasis on the NES is accomplished via bits 5, 6, and 7 of [PPUMASK](PPU_registers.xhtml "PPUMASK"), for red, green, and blue respectively. These bits enable/disable an "emphasis" or "tint" effect visually on screen. 

  * Bit 5 emphasizes red on the NTSC PPU, and green on the PAL & Dendy PPUs.
  * Bit 6 emphasizes green on the NTSC PPU, and red on the PAL & Dendy PPUs.
  * Bit 7 emphasizes blue on the NTSC, PAL, & Dendy PPUs.
  * Each bit emphasizes 1 color while darkening the other two. Setting all three emphasis bits will darken colors $00-$0D, $10-$1D, $20-$2D, and $30-$3D (see [PPU palettes](PPU_palettes.xhtml "PPU palettes")). Note that $1D black is affected by color emphasis, but $0F black is not.
  * The emphasis bits are applied independently of greyscale (bit 0 of [PPUMASK](PPU_registers.xhtml "PPUMASK")), so they will still tint the color of the grey image.



On Dendy and PAL PPUs, the purpose of bits 5 and 6 are reversed (red for green, and green for red).[1][2]

See [NTSC video](NTSC_video.xhtml "NTSC video") for a description of how bits 5-7 work on NTSC and PAL PPUs. 

The [RGB PPU](Vs__System.xhtml "Vs. System") used by PlayChoice and some other systems treat the emphasis bits differently. Instead of darkening other RGB components, it forces one component to maximum brightness. [A few games](Colour_emphasis_games.xhtml "Colour-emphasis games"), which set all three tint bits to darken all colors, are unplayable on these PPUs because they instead turn the entire screen white. 

## See also

  * [NTSC video](NTSC_video.xhtml "NTSC video")
  * [PPU palettes](PPU_palettes.xhtml "PPU palettes")
  * [PPU registers](PPU_registers.xhtml "PPU registers")
  * [Colour-emphasis games](Colour_emphasis_games.xhtml "Colour-emphasis games")
  * [Color $0D games](Color__0D_games.xhtml "Color $0D games")
  * [Talk:Myths#All_three_color_emphasis_bits_with_color_0D](Talk_Myths.xhtml#All_three_color_emphasis_bits_with_color_0D "Talk:Myths")



## References

  1. ↑ PAL PPU swaps green and red emphasis bits: <http://forums.nesdev.org/viewtopic.php?p=131889#p13188>
  2. ↑ Dendy PPU swaps green and red emphasis bits: <http://forums.nesdev.org/viewtopic.php?p=155513#p155513>


