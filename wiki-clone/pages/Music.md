# Music

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Music) | View [other pages](Special_AllPages.xhtml#Music)

Music is available for the NES. It is most often stored in NSF files, which package the data for one or more songs along with 6502 machine code to play it back. NSF files can be played on a NES or on a PC with an appropriate player which emulates the NES sound hardware and CPU. 

The music from a large number of NES games has been "ripped" into NSF format, and people are actively creating [new NES music](http://www.2a03.org/) with sequencing tools. 

Refer to the [Music tools](Tools.xhtml#Music_tools "Tools") wiki page for details. 

Some of these general-purpose music engines are designed for making NSFs where the only thing executing is the music player. This means they can be fairly large, and in an [NROM](NROM.xhtml "NROM") game, their features might not justify their size. 

If you want to write your own music engine that targets the NES [APU](APU.xhtml "APU"), you'll first need a lookup table for the note periods. This means you'll need the frequencies that correspond to various pitches. See [APU period table](APU_period_table.xhtml "APU period table"), [Wikipedia:Pitch (music)](https://en.wikipedia.org/wiki/Pitch_\(music\) "wikipedia:Pitch \(music\)"), [Wikipedia:Semitone](https://en.wikipedia.org/wiki/Semitone "wikipedia:Semitone"), and [Wikipedia:Equal temperament](https://en.wikipedia.org/wiki/Equal_temperament "wikipedia:Equal temperament"). 

## NES Audio

  * [NES APU](APU.xhtml "NES APU")



Categories: [Audio](Category_Audio.xhtml)
