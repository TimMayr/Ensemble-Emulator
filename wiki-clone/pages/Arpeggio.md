# Arpeggio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Arpeggio) | View [other pages](Special_AllPages.xhtml#Arpeggio)

In chip music, an **arpeggio** is an effect that rapidly alternates a tone generator's period among two to four pitches to create a warbly approximation of a chord. The effect was highly popular on Commodore 64, and it was popular on the NES among European developers who started in the C64 scene. Japanese developers, on the other hand, tended to shy away from arpeggio. 

Arpeggio tends to sound muddy when notes are below middle C, especially for an arpeggio updated at the full 50 or 60 Hz rate of the vertical blanking [NMI](NMI.xhtml "NMI"). When the difference between frequencies of successive pitches is less than the update frequency, there is less than one cycle of phase difference, and the ear doesn't have a chance to positively identify each frequency. In addition, some listeners find arpeggio grating when played with a volume envelope that has no decay. 

"Non-looped arpeggio" is a term used in NerdTracker II for playing the beginning of a note at a different pitch from the rest. FamiTracker, ppMCK, and Pently likewise support an arpeggio envelope that contains a part preceding the loop point. Numerous games play the first frame an octave up or down to add an attack to a triangle channel bass or a "plink" to melodic tracks. 

## Games whose music has plenty of arpeggios

  * _Asterix_
  * Anything by [Tim Follin](https://en.wikipedia.org/wiki/Tim_Follin "wikipedia:Tim Follin")
    * _Pictionary_
    * _Silver Surfer_
    * _Solstice_
  * _Jurassic Park_
  * _M.C. Kids_
  * _The Smurfs_
  * Several games by Codemasters



## External links

  * [Effect 0xy (Arpeggio)](http://famitracker.com/wiki/index.php?title=Effect_0xy) on Famitracker Wiki


