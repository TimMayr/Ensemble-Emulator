# Visual 2C02

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Visual_2C02) | View [other pages](Special_AllPages.xhtml#Visual_2C02)

**Note: Visual 2A03 and Visual 2C02 are hosted on a limited uplink, so please avoid e.g. shift-reloading them frequently. Normal reloading should work fine and should properly catch any updates to the data files.**

[Visual 2C02](https://www.qmtpro.com/~nes/chipimages/visual2c02/) is a transistor-level simulator of the NTSC PPU by Quietust, using the same core as the [Visual 6502](http://visual6502.org/) project. It can be used to figure out exactly how the PPU operates under various circumstances. 

## Contents

  * 1 Manual
    * 1.1 Section overview
      * 1.1.1 Running (section 1)
      * 1.1.2 Register access (section 2)
      * 1.1.3 Memory (section 3)
      * 1.1.4 Video waveform (section 4)
      * 1.1.5 Picture display (section 5)
      * 1.1.6 Navigation (section 6)
      * 1.1.7 Tracing (section 7)
        * 1.1.7.1 Finding nodes to trace
    * 1.2 Improving performance
    * 1.3 Tutorial: Outputting some pixels
    * 1.4 Some things to look out for



# Manual

_This page covers the basic user interface and tracing functionality. For making sense of the circuit display, see[this tutorial](Visual_circuit_tutorial.xhtml "Visual circuit tutorial")._

## Section overview

[![caption](../wiki-images/Visual_2C02_sections.jpeg)](File_Visual_2C02_sections_jpeg.xhtml "caption")

### Running (section 1)

The controls in this section start, stop, and reset the simulation. The _Scanline:_ and _Pixel:_ status displays are based on internal PPU counters and should be self-explanatory (see [this timing diagram](File_Ppu_svg.xhtml "File:Ppu.svg") for what operation is carried out at each dot). The starting state to use when resetting the simulation can be selected with the radio buttons in section 6. 

Currently supported reset states: 

Power-on
    Immediately after pulsing the PPU's RESET input. Note that chip operation will be [severely limited](PPU_power_up_state.xhtml "PPU power up state") in this state, and sprite RAM will be in an indeterminate state (neither 0 nor 1) until it is properly initialized.
Pre-render scanline, even frame
    The chip is powered on, all sprite and palette RAM has been initialized, and the chip has been simulated for 260 complete scanlines, leaving it at the very beginning of the pre-render scanline.
Pre-render scanline, odd frame
    Same as the above, but the pre-render scanline will end one cycle early.
Post-render scanline, odd frame
    The above state after 241 scanlines of simulation, leaving it 1 scanline before VBLANK begins.

### Register access (section 2)

This is a list of register accesses to be carried out, going in sequence from top to bottom. (The simulated 2C02 isn't attached to any other simulated devices.) 

For example, _W 1 1e_ decodes as _write $1E to $2001_. Reads are included as they can be significant for some registers - in this case, the "value" column will be filled in with whatever was read from the register. 

Register accesses can be removed by clicking on the '-' and added by clicking on the '+'. A '-' in the R/W column indicates a delay (use the numpad to input the '-'), where e.g. 1 23 as the numeric values means 0x123 master half-cycles. The '*' shows the current access, and can be clicked to jump to that point in the sequence. 

### Memory (section 3)

Memory display. Can also be used to modify memory. 

  * 3F00-3F1F — Palettes. Some of the cells are mirrors. Only visible if "Show palette RAM contents" is checked in section 6.
  * S000-S11F — OAM. For example, S000 holds the y position for sprite 0. Only visible if "Show sprite RAM contents" is checked in section 6. 
    * S000-S0FF — Primary OAM.
    * S100-S11F — Secondary OAM (normally not directly accessible).
  * 0000-03FF — Pattern tables. This 1KB segment is mirrored eight times to fill out the entire CHR space.
  * 2000-23FF — Nametables. The simulation uses a kind of "one-screen low" mirroring, and the data here is mirrored to fill out the entire nametable space. 
    * This layout for pattern and name tables happens to be the same as the PPU A13 variant of [iNES Mapper 218](INES_Mapper_218.xhtml "INES Mapper 218").



Select a byte of memory by clicking on it, then type the desired new value in hexadecimal. You can use the arrow keys to navigate to adjacent cells. 

### Video waveform (section 4)

Video output waveform display, based on the _vid__ node. You will at least see some level changes and squiggly stuff here near the end of each scanline, which is the NTSC hsync and colorburst. 

### Picture display (section 5)

This contains the picture being rendered by the 2C02, done by extracting the pixel values being fed into the NTSC luma/chroma generators and drawing them using the PlayChoice-10 RGB palette (with extra grays added in column 0xD). Note that this does _not_ yet simulate color emphasis. 

### Navigation (section 6)

Pretty self-explanatory. Node numbers or node names (e.g. "spr0_hit") can be entered in the _Find:_ edit box to locate them in the diagram. 

If you want to manipulate an external pin, you can enter its node ID in the "Modify" box (or simply click on it) and press the "High"/"Low"/"Float" buttons to the right. Using this with internal nodes is not recommended, since it may result in unexpected behavior as the High/Low states will _persist_ until you select them and choose "Float" (except for nodes connected to [power sources](Visual_circuit_tutorial.xhtml#Power_sources "Visual circuit tutorial"), which must be set to "High" in order to function correctly). 

### Tracing (section 7)

Tracing of node values. Additional nodes to trace can be added in the _Trace these too:_ edit box, separated by either spaces or commas (e.g. "spr0_hit tile_l vid_" or "spr0_hit,tile_l,vid_"). 

The cycle column is based on the master clock, which the PPU divides by four. Each line in the trace corresponds to a half-cycle, so there are 4*2 = 8 lines per PPU tick. 

#### Finding nodes to trace

A list of nodes can be found in <https://www.qmtpro.com/~nes/chipimages/visual2c02/nodenames.js> . For nodes that have many bits, e.g. _finex0_ , _finex1_ , _finex2_ , you can trace all of them at once by using "finex" as the node name. 

## Improving performance

Turning off tracing and unticking _Animate during simulation_ and _Show sprite RAM contents_ can massively speed up the simulation. 

## Tutorial: Outputting some pixels

1\. Put 81 at pattern table address 0000. This will make the palette index for each pixel of the first row of the first tile, in order, '10000001'. (Putting 81 at 0008 as well would make it '30000003', etc.) 

(Since the nametables are initialized to 0 by default, this is the tile that will be used for all the background tiles by default.) 

2\. Change the value at 3F01, which is the BG palette entry that will be used. 20 seems to work fine. 

3\. Run the simulation (and note the Performance section). The first line is the pre-render line, so nothing will be seen here. At scanline 1, you should see some pixels being output in the waveform display corresponding to the 81 pattern. 

## Some things to look out for

  * Note that the default register writes might move around sprite 0 and do other stuff, so you might have to remove some of them or manually modify memory later to get the state you want.


  * There's a bunch of sprites sitting at (0,0). If sprites are enabled and all use a black tile, this means you will see black for the first 8 pixels of scanlines 1-8. (Sprites don't start drawing until scanline 1 at the earliest since the y OAM coordinate is one less than the actual position.)


