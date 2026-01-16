# Bad Apple

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Bad_Apple) | View [other pages](Special_AllPages.xhtml#Bad_Apple)

**Bad Apple!! PV-FC 2** is a homebrew NES ROM based on a fan-made shadow art music video for a fan-made cover of the song "Bad Apple!!" from a _Touhou_ game. It uses the [MMC3](MMC3.xhtml "MMC3") and a 512 KiB PRG ROM and 8 KiB CHR RAM. It plays 3 minutes and 39 seconds of 15 fps video at 64x60 pixel resolution, using a blocky video mode that puts four pixels in each tile like the [Game Genie](Game_Genie.xhtml "Game Genie"). 

## Compression

The video format used by this ROM was reverse engineered by [tepples](User_Tepples.xhtml "User:Tepples") on September 30, 2012. 

Code starting at $F557 decodes the slice to a framebuffer at $400-$5DF and expands it to a transfer buffer at $600-$6FF, and then the next NMI handler copies it to memory. There are I-frames (frame type $01) and long chains of P-frames (frame type $02). An I-frame ("intra") or keyframe is just 480 bytes of nibbles; there are fewer than half a dozen in the whole video. Each P-frame ("predicted") or delta frame consists of four slices of 8 tile rows (16 pixels) each. In each slice, a 0 bit means the entire row is unchanged from the previous frame, and a 1 bit means there will be a 2-byte word stating which of the sixteen 4x2-pixel nibble pairs in a row of tiles should be replaced with new data for this frame, followed by as many nibble pairs as there are 1 bits in the word. The bits in a nibble pair are mapped to pixels as follows: 
    
    
    4 5 0 1
    6 7 2 3
    

There are also D-frames ("deleted", frame type $00), which copy the whole frame from the previous frame. These are used for occasional short freezes such as those at the very beginning and end (black), and when Marisa grabs the apple (around frame 221), saving 4 bytes over using a P-frame. Frames never cross 8 KiB bank boundaries; a special terminator code ($03) is used instead of a frame type after the last frame in a bank. 

There is a later 30 fps version that uses 512 KiB PRG ROM, 256 KiB CHR ROM, and 8 KiB PRG RAM, the same configuration as _Kirby's Adventure_. This has not been cracked. 

## External links

  * [Original music video](https://knowyourmeme.com/videos/29502-bad-apple) mirrored on knowyourmeme.com (was originally hosted on Niconico, which requires registration and was not available in English)
  * [NES version 2 (TGROM) download page](http://littlelimit.net/bad_apple_2.htm)
  * [NES version 2.5 (TSROM) download page](http://littlelimit.net/bad_apple_2_5.htm)
  * [NES version running on a PowerPak](https://www.youtube.com/watch?v=AZehH55i_wg)



Categories: [Homebrew](Category_Homebrew.xhtml)
