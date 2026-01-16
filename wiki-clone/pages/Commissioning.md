# Commissioning

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Commissioning) | View [other pages](Special_AllPages.xhtml#Commissioning)

Some people are better at graphical or level design than programming, and their game concept may not fit the NESmaker engine well. So they might seek to hire a programmer on NESdev, VGS, or elsewhere to develop a game, whether as volunteer work or as paid work. To make the most of your employee or contractor, it's best to plan ahead somewhat to avoid problems down the road. 

## Avoiding premature engine limits

If the producer asks the programmer to write a program to display a particular set of assets, the programmer will write a program to display those assets. To simplify the job, the programmer will [do the simplest thing that could possibly work](http://ronjeffries.com/xprog/articles/practices/pracsimplest/), and this can cause the engine to include tacit assumptions related to those particular assets. For example: 

  * If all the graphics that the artist presents are red, the programmer is likely to make assumptions that simplify the program but limit it to displaying only red graphics.
  * If they all have a particular pixel size, the programmer is likely to make assumptions that simplify the program but limit it to displaying only graphics of that size.
  * If there's only 12K of dialogue, the programmer is likely to make assumptions that simplify the program but limit it to only one 16K PRG bank of dialogue.
  * If each level has only 256 tiles, the programmer is unlikely to consider scroll-driven tile replacement in CHR RAM or a mid-screen CHR bank switch to break this limit.
  * If each level has only certain enemy types near each other, the programmer is unlikely to consider arbitrary combinations of enemy cels in sprite video memory.
  * If there's no rough draft of an object's appearance, the programmer is unlikely to code its behavior.
  * If the sample music uses only discrete pitch, the programmer is unlikely to add portamento support to a music player.



If the producer later asks for display of assets that break the engine's limits, even if they don't break the [limits of the hardware on which the game runs](Limitations.xhtml "Limitations"), the changes won't necessarily be "real quick" to make. Some extensions can be hacked in after a project's halfway point, but in the worst case, undoing the simplifications can take as much time as starting much of the engine over from scratch. A rapid apparent completion of an early milestone based on an oversimplification could throw off estimates of programmer productivity and thus of the overall project's time requirements, which could cause a project to miss deadlines. It can also prove financially costly if the producer is paying the programmer hourly or if the producer relies on sales associated with Christmas or with the holiday around which the game is themed. 

Therefore it's best to present a representative set of assets to your programmer early on. Even if you don't have all assets for the entire game ready when you ask the programmer to start working on the engine and its art pipeline, do your best to let the programmer know how much there'll be and show diverse assets that exercise all features of the engine that you expect to use later. In particular, try to get one complete chapter of your game playable and feature-complete before the schedule's halfway point in order to ensure that the engine can handle the demands that other levels are likely to put on it. This way your team won't be caught off guard later by limits that you didn't anticipate. 

That or [allow enough time to build one to throw away](http://wiki.c2.com/?PlanToThrowOneAway). 
