# User talk:Blargg

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3ABlargg) | View [other pages](Special_AllPages.xhtml#User_talk_Blargg)

The main page should describe the site's content, have a Nesdev forum link, and links to other information. It should have links to the main information one will be _repeatedly_ referring to. For example, CPU instruction reference, PPU registers, APU registers, MMC1, MMC3, etc. These links can also give a more graphical idea of the information on the site. Something more information-rich than a simple hierarchical list may be better, like a table with blocks for the various subsystems. In this case, fitting a good number of links without scrolling seems useful. 

The main page should not be chatty or try to engage the reader in a conversational way. The reader should not have to classify himself in order to relate with the information, so nothing like "Newbie? Head on over here!". The information content should be clear, so that the reader can just naturally go to what he is interested in. This will take some work, but I strongly want something of a refined level. This is one reason icons don't work to classify things, because they impose a shallow distinction and distract from the more subtle aspects of the information structure. It's sort of the layout equivalent to animated images on a web page. 

After quite a bit of brainstorming, it's clear that pages will generally fall into only one of the following categories, which each call for a different style of presentation. Again, these are mainly concepts that help guide the design of the pages. 

Tutorials: 

  * Increasing set of assumptions of student's knowledge, so that he can do them in order.
  * Aren't intended to be re-read, so they can be verbose.
  * Programs included should be complete and easy to build and run.
  * Text should generally explain things as they are introduced, without requiring the student to read lots of other pages on the site that assume more than he knows at that point.



Techniques: 

  * Assume decent familiarity with system.
  * Code examples don't need to be complete programs. Could have short snippets on the page only.
  * Descriptions can make extensive use of links to other Wiki pages that further describe elements.
  * Not a place to document quirks, but techniques will definitely use them.



Documentation: 

  * Describes operation of things for normal use, without getting bogged down with tiny details. For example, the PPU palette description doesn't need to cover how $3F04, $3F08, and $3F0C can actually be used if rendering is turned off (but it shouldn't say flat out that they are never used, just that they aren't normally used).
  * Can assume general knowledge of the system.
  * Should not try to cover techniques, just what the hardware gives you. So the PPU scrolling registers would be covered, with mention that split-screen can be performed, along with a link to a technique page that describes how.
  * I can't figure any decent way to have separate "details" pages in a separate organization, so it seems perhaps we can have "PPU registers" cover normal usage, and some kind of "PPU register details" or something that it links to, covering all the details in a non-verbose way.



Hardware: 

  * Pinouts, voltages, waveforms, ways of hooking up hardware, etc. Basically anything outside of the programmer model of the system. The main reason for the division is I think most readers are only going to want software information.


