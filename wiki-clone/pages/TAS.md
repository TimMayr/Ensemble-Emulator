# TAS

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/TAS) | View [other pages](Special_AllPages.xhtml#TAS)

A **TAS** (short for _Tool-Assisted Speedrun_) is an input recording for a video game. When the game is run in real time, the input is fed to the game, and the game reacts to the input as if the player was playing it. However, the input has been crafted at the author's leisure, and may involve maneuvers that are way too difficult, or outright impossible, for a real human player to perform, by requiring abilities such as premonition or perfect accuracy. This creates [entertainment that appeals to people for a number of reasons](http://tasvideos.org/WhyAndHow.html). Personally, I take it as a form of art. 

For this to be possible, however, an emulator must pass [certain requirements](http://tasvideos.org/EmulatorResources/Requirements.html). Most importantly, the emulator must be completely deterministic. The emulator must contain no random components, such as fluctuation in thread synchronization. From the perspective of the game running in the emulator, the emulator must run identically on every single invokation, when the same input is provided. For the most part, real consoles are also deterministic, and indeed, some TASes have been [replayed on an actual NES console](http://tasvideos.org/ConsoleVerifiedMovies.html). Then there are [features which make the creation of TASes significantly easier](http://tasvideos.org/EmulatorResources/Features.html). If you are an emulator author, please try to ensure your emulator fulfills most of these requirements. 

If your emulator is accurate enough, you can use TAS videos as a test material to automate your emulator testing with games. The TAS inputs go quickly through large amounts of the game's code and data, invoking bordercase behavior within the games, making very good testcases. To use, at the onset of each NMI, just read the next frame from the movie file, and use that input for the response whenever the game polls the input. Note that you may need to delete or add a few idle frames in the beginning of the movie before sync is achieved. 

Note that not all TAS videos have been created on an accurate emulator. For the movie to sync, you must replicate the same timings as the emulator the TAS was made on. Some games, such as Super Mario Bros., are not really picky, but most of them require fairly accurate emulation. 

As a general principle, the newer the TAS is, the more accurate the emulator it was played on. That is, if you aimed for accuracy, you should try to replay these newer TASes. However, the newer TASes also often push the edge, invoking an esoteric race condition within the game or timing-sensitive buffer overflow glitches, meaning that even a difference of a single cycle in sprite-0-hit flag may translate to the difference between a TAS that syncs and one that does not. The TASes that depend on such level of faithful reproduction of timing are very few, though. An example of a TAS with such glitch is [NES Mega Man (JPN) in 12:23.34 by Shinryuu & FinalFighter](http://tasvideos.org/1686M.html) made with FCEUX with "_old ppu_ " setting. Its predecessor, [NES Mega Man (JPN/USA) in 15:29.27 by Deign](http://tasvideos.org/1103M.html) is considerably more likely to sync on any fairly accurate emulator (though so far, attempts to sync it on _actual NES_ have been unsuccessful). 

## External links

  * [TASVideos - a website that collects, produces and publishes TAS videos.](http://tasvideos.org/)
  * [TASVideos: Emulator feature checklist: The minimums for emulator being TAS-production capable](http://tasvideos.org/EmulatorResources/Requirements.html)
    * [TASVideos: Emulator feature wishlist: TAS authors wish for these features in any emulator they work with.](http://tasvideos.org/EmulatorResources/Features.html)
  * [TASVideos: List of console-verified movies. Your emulator should be accurate enough to run these TASes as well.](http://tasvideos.org/ConsoleVerifiedMovies.html)
  * [NESmock: A tool for converting NES input recording files between different emulator formats.](http://bisqwit.iki.fi/source/nesmock.html)
    * [Links to pages with description of each known emulator's movie format.](http://tasvideos.org/EmulatorResources.html)



## Authors

  * [Bisqwit](User_Bisqwit.xhtml "User:Bisqwit")


