# Talk:Init code

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AInit_code) | View [other pages](Special_AllPages.xhtml#Talk_Init_code)

## Prophylactic read of $2002

Is this bit of code necessary? I'm having trouble finding any original NES game that does it. Are there any known examples of a startup problem caused by this? 
    
    
    ; Clear the vblank flag, so we know that we are waiting for the
    ; start of a vertical blank and not powering on with the
    ; vblank flag spuriously set
    bit $2002
    

If it isn't a problem, does anyone have an explanation as to why it isn't? If it is, can anyone point to an example of the problem? (Maybe I'll write a test if I have time later...) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 12:09, 28 May 2015 (MDT) 

    Yes, it's necessary, and a problem, see [PPU power up state](PPU_power_up_state.xhtml "PPU power up state"). There's a thread on the forum of at least one homebrew that failed to boot sometimes because it would immediately fall through the first wait and not wait long enough after the second wait before it started trying to write to PPU registers. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:49, 28 May 2015 (MDT)

    

    The power up state article doesn't have enough information to answer my question. There is "The PPU comes out of reset at the top of the picture" but it's unclear whether this applies to power on. It says $2002 often powers on set, but is unclear whether this is because the PPU starts in vblank. "The VBL flag (PPUSTATUS bit 7) is usually clear at power, and unchanged by reset. It is next set around 27384, then around 57165." Is that consistent? Why "around"? Is "ignored if earlier than ~29658 CPU" an upper bound, or an average?

    

    I'm trying to understand why it doesn't seem to be a problem for commercial games. So far I haven't found any that don't just do 2x $2002 poll loops, but also all of them seem to do a bit of extra initialization work before they start hitting the PPU. If it's the case that the $2002 vblank flag powers on randomized, and the next set is always at 27384 consistently, then the margin of error for failure is <3000 cycles?

    

    Trying to think of a good way to test this and demonstrate the failure. Can you remember anything else about the homebrew in question that might help me search for it? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:04, 28 May 2015 (MDT)

    

    

    The only thing I remember is someone saying something like ~and now I will always wait for $2002.7 _three_ times in my init code~.

    

    

    Anyway, order of events: 

  1. PPU always comes out of reset at Y=0
  2. PPUSTATUS.7 may or may not be set when it comes out of reset
  3. CPU can only detect when Y=240, because that's when the PPU sets PPUSTATUS.7
  4. PPU is not available for pertinent writes until Y=262=0
  5. Waiting for PPUSTATUS.7 twice without clearing the register on startup could therefore sometimes have the CPU think the PPU is ready when Y=240, instead of Y=262+240 —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:18, 28 May 2015 (MDT)



Found this post by Blargg claiming to have tested it and basically verifying what I just asked. [http://forums.nesdev.org/viewtopic.php?f=2&t=3958](http://forums.nesdev.org/viewtopic.php?f=2&t=3958) i.e. the idea that there is a 3000 cycle margin of failure if you don't pre-clear. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:19, 28 May 2015 (MDT) 
