# UM6582 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UM6582_pinout) | View [other pages](Special_AllPages.xhtml#UM6582_pinout)

United Microelectronics UM6582: 16-pin 0.3" PDIP (Used in the Stylandia's IQ-502 joypads) 
    
    
               .---v---.
      TURBO <- |01   16| -> TURBO/2
         /B -> |02   15| -> TURBO/4
     /START -> |03   14| <- /A
    /SELECT -> |04   13| -- VCC
      /DOWN -> |05   12| -- GND
     /RIGHT -> |06   11| <- STROBE
        /UP -> |07   10| <- CLK
      /LEFT -> |08   09| -> D0
               '-------'
    

Notes: 

  * TURBO, TURBO/2, TURBO/4 can be used as turbo input for button A/B (on the tested pad, TURBO was used and other pins were NC),
  * They toggle on the every / every second / every fourth rising edge of the (CLK and not STROBE) signal respectively and there is around 82ns delay between the transition
  * After 8 edges of CLK signal, D0 goes low (on most pads, it goes high)



[![](../wiki-images/UM6582_waveforms.png)](File_UM6582_waveforms_png.xhtml)

[](File_UM6582_waveforms_png.xhtml "Enlarge")

Waveforms

Source: [[1]](https://maciejewski.com/pl/pegasus-stylandia-iq-502-z-gniazdem-sluchawkowym/)

Categories: [Pinouts](Category_Pinouts.xhtml)
