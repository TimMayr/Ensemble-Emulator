# Talk:Visual circuit tutorial

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AVisual_circuit_tutorial) | View [other pages](Special_AllPages.xhtml#Talk_Visual_circuit_tutorial)

## Contents

  * 1 Cross-coupled inverter
  * 2 Some possible NMOS clarifications
  * 3 Pictures
  * 4 "Open" and "closed" terminology
  * 5 Steady state and voltage vs. current
  * 6 Redraws
  * 7 PLA
  * 8 Circuit notation
  * 9 Sizing



## Cross-coupled inverter

Is that actually an S-R latch? vis. <http://en.wikipedia.org/wiki/File:R-S_mk2.gif> —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:46, 24 May 2013 (MDT) 

## Some possible NMOS clarifications

It might be possible to make "in NMOS, transistors are placed in a substrate of n-doped semiconductor" clearer. Does "transistor" here refer to the entire diffusion + poly combo (so that diffusion + poly together are placed on the substrate), or does the n-doped semiconductor make up (or interact with) some particular part of the transistor (like the diffusion, or the polysilicon)? -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 23:43, 27 May 2013 (MDT) 

    I misremembered about the substrate; it's a lightly p-doped substrate, vis [wikipedia:File:CMOS fabrication process.svg](https://en.wikipedia.org/wiki/File:CMOS_fabrication_process.svg "wikipedia:File:CMOS fabrication process.svg"). So I fixed that. The transistor is the sum of all its parts: the substrate (or "body"), both the source and drain N-doped diffusion, the layer of quartz insulator beneath the gate and the layer of polysilicon that is the gate. Arguably it's also the metal or polysilicon vias to the source and drain. Also [wikipedia:File:CMOS NAND Layout.svg](https://en.wikipedia.org/wiki/File:CMOS_NAND_Layout.svg "wikipedia:File:CMOS NAND Layout.svg"). Because the gate is placed before the diffusion, the region immediately under the gate is actually not very doped— hence "substrate". —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:51, 28 May 2013 (MDT) 

    I'm happy with the current description then. Gives the general idea without lying or getting bogged down in details (which might be overkill for someone needing to look up "NMOS" in an article like this). -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 01:03, 28 May 2013 (MDT)

## Pictures

Sorry if the page is getting a bit heavy. I guess some of the pictures could be converted to JPEG or a lower bit depth with no big loss. As should be clear from some of the images I'm not exactly a professional illustrator, so recommendations are welcome :P. Thanks for sharpening up those pictures btw! -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 05:41, 28 May 2013 (MDT) 

    I think the Best solution is figuring out how to get SVGs out. Those would compress very well, given the subject matter. Next best (and hopefully slightly more practical?) is to figure out how to get firefox/chrome to stop antialiasing the polygons in Visual2X0X. Without the blurry edges, a palettized PNG will be both easy and compress a lot. Finally, I don't think converting to JPEG will help with this—it's best at large regions of comparatively smoothly fading surfaces.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:57, 28 May 2013 (MDT) 

    I played with it for a bit, and have a few answers. You'll need a locally-downloaded version, because you'll be making a few local changes to the source. 

  * To fix blur on really deep zooms (= disable bilinear filtering), in `expert.css`, in the section "canvas.chip", add this line for firefox: `image-rendering: -moz-crisp-edges;` or the equivalent line mentioned here: <https://developer.mozilla.org/en-US/docs/Web/CSS/image-rendering> for your browser (But! it's not working for me using chrome 26)
  * To reduce blur on diagonal lines (usually polysilicon), in `wires.js`, in the function "setupBackground", add these two lines after the "getContext" call: `ctx.mozImageSmoothingEnabled = false;` and `ctx.webkitImageSmoothingEnabled = false`
  * To ~~increase the maximum zoom possible and~~ even out irregular pixel spacing, in `expertWires.js`, near the top, change "grCanvasSize=5000" to some larger value. (for me 20000 breaks horribly. 10000 seems reasonable for getting pretty screenshots) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 01:43, 29 May 2013 (MDT)



    Thanks for the tips! Planning to add a section on those PLA-like circuits (preferably after I figure out what they're actually called :P), but after that I might do a picture clean-up pass. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 02:52, 29 May 2013 (MDT) 

    I've already increased grCanvasSize multiple times - Visual6502 had it set to 2000, and I initially doubled it to 4000 for 2A03/2C02 (because they're much larger) and then just recently boosted it further to 5000. Increasing it further has the side effect of drastically increasing memory usage - setting it to 6000 will make it use a **gigabyte** of RAM. Also, increasing the actual maximum zoom level is done by adjusting grMaxZoom. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 07:13, 29 May 2013 (MDT) 

    Well, that **would** explain why 20000 caused chromium to simply refuse and firefox to eat All My Memory... For whatever reason, for me using 10000 is "only" causing firefox to take (1.145g-240m) resident and (1.36g-692m) virtual; chromium eats (1.6g-60m) resident and (2.7g-1.6g) virtual (debian sid x86-64) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:55, 29 May 2013 (MDT)
    Another thing that might help, although it does change the appearance a bit, is in "setupBackground", comment out `if((c==0)||(c==6)) ctx.stroke();` (prepend with //). This makes the metal (and polysilicon??) be flat shaded without a border. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:55, 29 May 2013 (MDT)

By the way, what program are you using to add the labels? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:54, 30 May 2013 (MDT) 

    Ad-hoc in Inkscape. I just embed the screenshot and put shapes and text on top of it. I could make an archive of the .svg's if you want; then you'd only need to replace the background image. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 20:28, 30 May 2013 (MDT) 

    I would definitely appreciate that! —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:36, 31 May 2013 (MDT) 

    Here you go: <https://dl.dropboxusercontent.com/u/20047039/vis_tut_svgs.tar.bz2> . I removed the transparent borders some of the images had (I goofed up on those) and tweaked the text size a bit in vis_crossreg.svg. vis_oam_left.svg might be unnecessarily tall too... -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 01:13, 31 May 2013 (MDT)

  


## "Open" and "closed" terminology

I switched it around after some feedback in [http://forum.6502.org/viewtopic.php?f=1&t=2522](http://forum.6502.org/viewtopic.php?f=1&t=2522), but I'm not so sure anymore. What's your take on it? Unless it's "wrong", I prefer the old version (since I'm used to it :P). -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 02:56, 31 May 2013 (MDT) 

    Switched back to the old version for now. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 03:14, 31 May 2013 (MDT)

    It kinda makes the definitions circular, but you could use "conducting" and "nonconducting"; or maybe "in saturation" and "in cutoff" since they're FETs (even if it's rather jargony). Having read the thread at 6502.org, it looks like you can't win with "open" and "closed", though. Alternatively, just define your terms at the beginning and blame the reader when they confuse them :p —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:12, 31 May 2013 (MDT) 

    Or we could borrow [relay](https://en.wikipedia.org/wiki/relay "wikipedia:relay") terminology: a FET can be in a "make" or "break" state. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 14:45, 31 May 2013 (MDT) 

    So for something like "the diffusion area from which current will flow when the gate is <?> is called the source", <?> could be 

  * "open" \- Door analogy.
  * "closed" \- Switch analogy. Kinda confusing (at least for me) given that "gate" is a door analogy.
  * "on" \- Easier to confuse as part of the sentence - "on top of what?". Could use italics, but that's kinda ugly to have to do in lots of places.
  * "conducting" \- Pretty long and specific word to use all over the place. "Open" feels better there.
  * "made" (?) - Sounds a bit weird. :P
  * "high" \- Works decently, but you still need another form for stuff like "when the gate is opened".


    I might be giving this too much thought, but I still kinda like "open". Could just mention the terminology caveat I guess. Can't find a clear winner on the Googles. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 23:22, 31 May 2013 (MDT)

## Steady state and voltage vs. current

Re. the change from 

"if any of the gates in red circles are open (high), the current from the highlighted node will go to ground instead of to the gate in the blue circle on the top"

to 

"if any of the gates in red circles are open (high), the voltage of the highlighted node will be pulled to ground instead of pulled high"

(with comment "in steady state, the current doesn't really 'go' anywhere."): 

Is the first version really factually wrong? I realize that you can model it as the entire node having a single "state" in steady state, but isn't what actually happens as in the first description? As a beginner I find currents simpler to think about than voltages, and beginners are the target audience. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 11:54, 1 June 2013 (MDT) 

    I guess "the current from the power source" might be more precise though. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 12:10, 1 June 2013 (MDT) 

    The current doesn't "go to" the transmission gate. A voltage is present at the transmission gate. A current could go through the transmission gate while it's conducting. But you can't have a current of water "to" a glass meaning "sitting in". —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:20, 1 June 2013 (MDT) 

    Ah, so the problem you had with it was that no current will flow through the pass transistor when it is closed, even if all the grounding transistors are closed? Yeah, that makes the old description a bit iffy for that case at least... -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 12:34, 1 June 2013 (MDT) 

    I guess you have to be careful when talking about current in general here, as you'll only have a current until the steady state is reached in cases where the current doesn't go to ground. Didn't think of that until now. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 13:49, 1 June 2013 (MDT) 

    Should prolly keep the description for the inverter as-is though. Absolutely technically correct might be the enemy of beginner-friendly in that case I think, unless you have any ideas. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 13:59, 1 June 2013 (MDT) 

    Describing things using "current" instead of "voltage" is sufficiently less wrong when you're talking about exclusively NMOS logic that I'm not bothered by it. Transmission gates or CMOS logic are where the pedantry starts actually changing meaning. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:47, 1 June 2013 (MDT)

## Redraws

Thanks for (and sorry about) dropping the rewrite on you with the changed figure (Vis_crossreg.png) — I'd just finished and then had to run out the door. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:47, 1 June 2013 (MDT) 

    No problem. I just assumed you had forgotten (like I tend to do :P). -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 14:49, 1 June 2013 (MDT) 

    I like the combined DRAM picture by the way. Probably would've drawn it that way if I had been less lazy when doing the original ones. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 01:09, 2 June 2013 (MDT)

I think I'm going to revisit some of the highlighting. I showed my partner the images, but they had problems picking out what counted as "highlighted" vs "not". —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:40, 4 June 2013 (MDT) 

## PLA

The length counter lookup table is currently being described as a PLA with an AND and OR plane, but I'm not sure that's really correct - in the forum post on 6502.org (with regards to transistor open/closed terminology), several people pointed out that both "planes" are composed entirely of NOR gates. In reality, I don't think this is a PLA at all: it's just a 5->32 (N -> 2N) decoder followed by a NOR-based mask ROM. The defining characteristic of a PLA is its ability to implement arbitrary combinational logic circuits, and the instances of this pattern in the RP2A03 aren't really used in this manner - there are 3 lookup tables (length counter, noise frequency, PCM frequency), 1 address decoder ($4000-$401F), and a very specific 15->6 decoder (for the frame counter). The RP2C02's H and V decoders are a bit more similar to the state decoder in the 6502, but they still aren't really PLAs, and the others are also just ordinary decoders (2->4 decoder for luma, 4->13 decoder for chroma, and the $2000-$2007 R/W decoder). --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 12:46, 2 June 2013 (MDT) 

    Yeah, wasn't sure of the right terminology here. PLA did seem a bit odd given that they're hardcoded rather than reprogrammable. I'll rewrite it in a while now that I know the correct terminology. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 12:57, 2 June 2013 (MDT)

    DeMorgan's law says a bunch of NOR gates _are_ AND gates with inverted inputs. Since the AND plane is regular (a demultiplexer) instead of the OR plane does imply to me it's a ROM instead of a PAL, but the term "PLA" is apparently intentionally generic enough to mean **both** , and there is an established history of using [ROMs as programmable logic](https://en.wikipedia.org/wiki/Programmable_logic_device#Using_a_ROM_as_a_PLD "wikipedia:Programmable logic device"). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:04, 2 June 2013 (MDT) 

    Yeah, my view when writing it was that it was an AND plane implemented with NOR gates. Perhaps that's not a common way to think about this particular circuitry though (I don't know). -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 13:22, 2 June 2013 (MDT)
    I think I prefer the decoder + ROM mask version, as it introduces more terminology and might be a bit more correct (even if "PLA" might be colloquial). I could also point out the similarity with PLAs in a rewrite. The only downside is that the labels on the first image would need to be changed to say "5-to-32 decoder" and "Mask ROM". Would that be a lot of work? -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 13:40, 2 June 2013 (MDT) 

    Could send the image to gmail at ulfalizer (reverse :P) if you decide to redraw it, so it can be synched with the update to the text. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 14:01, 2 June 2013 (MDT)

    

    

    It _is_ still an AND plane feeding into a NOR plane, regardless of where on the ROM/PLA/PAL spectrum it is. All the ROM word line selectors I have ever seen have all been implemented that way—anything else would be wasteful. Anyway, I've uploaded to [File:Vis_len_rom.png](File_Vis_len_rom_png.xhtml "File:Vis len rom.png") a version with labels "Decoder" (5-to-32 wouldn't fit) and "Mask ROM". —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:40, 2 June 2013 (MDT)

## Circuit notation

I really like those equivalent schematic displays, as tying things back to electronics notation was one of the things I felt were missing. If there's a particular name for the notation used (with e.g. triangles for transistors), that would be good to mention too. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 04:59, 4 June 2013 (MDT) 

    Wikipedia calls them ["ANSI schematic symbols"](https://en.wikipedia.org/wiki/Logic_gate#Symbols "wikipedia:Logic gate")… the triangle with a line coming in the side is the symbol for a tristateable buffer, which is vaguely misleading (because no buffering occurs). But it is the closest digital symbol equivalent. I originally drew that part with another multiplexer, but since apu_clk1 and w400e aren't strictly complements I thought better of it. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:40, 4 June 2013 (MDT)

## Sizing

A high-level overview of diffusion and gate sizing would be nice to have towards the end of the article (after section six or seven or so). Often you see signals invert into others where it seems it wouldn't be necessary from a purely digital perspective, and mentioning things like that would be nice too (guessing it has to do with boosting the signal). 

I'm moving on to some other stuff for a while, but feel free to jump in. :) -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 04:51, 5 June 2013 (MDT) 

    Could you give me a few node numbers/names to talk about? I haven't spent enough time staring at the simulators to have places in mind. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:18, 6 June 2013 (MDT) 

    Here's some off the top of my head: 

  * CPU: The PCM mixer. Looking at **pcm_out0** and **pcm_out1** , both have just one strip of powered diffusion, but the gate on **pcm_out0** is thicker (presumably to create more resistance).
  * PPU: The superbuffers that **pclk0_en** and **pclk1_en** connect to, used to generate **pclk0** and **pclk1**.
  * PPU: The thing used to introduce the different colors at the beginning of the article, also from the PPU, and also related to **pclk0** and **pclk1**. Maybe it's to make really sure the signals stay nice inverses.


    I don't have any examples from random logic off the top of my head unfortunately, as I haven't paid much attention while browsing around. At least the first one would probably make a good example at least. Might think of more later.
    
    Some formulas would be okay too, if you know them and they're reasonably simple (stuff like how resistance relates to width and breadth of poly, etc.). -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 15:27, 6 June 2013 (MDT)
