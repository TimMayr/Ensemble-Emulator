# M50805 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/M50805_pinout) | View [other pages](Special_AllPages.xhtml#M50805_pinout)

Mitsubishi M50805: 22-pin 0.4" PDIP (Used in [one](http://bootgod.dyndns.org:7777/profile.php?id=3953) [mapper 3](CNROM.xhtml#Notes "INES Mapper 003") game) 
    
    
                     .--\/--.
               NC -- |01  22| -- NC
               NC -- |02  21| -- NC
               NC -- |03  20| -- NC
        DREQ/TEST <- |04  19| <- /D0
      MODE \  MS1 -> |05  18| <- /D1
    SELECT /  MS0 -> |06  17| <- /D2,DTIN
             XOUT <- |07  16| -> BUSY
              XIN -> |08  15| <- /SYNC
              /CE -> |09  14| -> DA1 \ DA
           /RESET -> |10  13| -> DA0 / OUTPUT
              +5V -- |11  12| -- GND
                     '------'
    

Sources: [[1]](https://forums.nesdev.org/viewtopic.php?f=6&t=16129), [[2]](https://forums.nesdev.org/viewtopic.php?f=9&t=9449)

The [die shot](http://seanriddledecap.blogspot.com/2017/07/blog-post.html) shows that pins 1-3 and 20-22 are genuinely no connection - there's nothing to be wire bonded to. 

Categories: [Audio](Category_Audio.xhtml), [Pinouts](Category_Pinouts.xhtml)
