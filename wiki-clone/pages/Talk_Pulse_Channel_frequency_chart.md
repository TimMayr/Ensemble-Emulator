# Talk:Pulse Channel frequency chart

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APulse_Channel_frequency_chart) | View [other pages](Special_AllPages.xhtml#Talk_Pulse_Channel_frequency_chart)

## Merge

Isn't this the same as [APU period table](APU_period_table.xhtml "APU period table")? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 20:27, 14 October 2013 (MDT) 

Sort of, just a little more user-friendly. It's a simple reference for people who want to know which two bytes to plug into $4006-7 (et al.) in order to obtain a desired musical pitch, without getting bogged down in tables, calculations, formulas, ASCII diagrams, etc. Every reference I've been able to find seems to consist heavily of the latter, when all I want are those two little bytes! --[Dr. Floppy](User_Dr__Floppy.xhtml "User:Dr. Floppy") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Dr._Floppy&action=edit&redlink=1 "User talk:Dr. Floppy \(page does not exist\)")) 22:30, 14 October 2013 (MDT) 

    In my experience, someone developing a music engine doesn't want _a_ pitch but instead a _set_ of pitches to use for all the notes that will be played over the course of the program. Does your music engine store pitches in musical phrases as two bytes (a literal period) or as one (a note number)? I understand that the latter is more common in real music engines. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 07:58, 15 October 2013 (MDT)

    

    Yes, my engine (which officially became "real" over the weekend) uses a complete set of pitches from A-1 to B-7, invoked by single bytes. My lookup tables are arranged in a nonstandard fashion due to the aesthetic design choice I made early on: #$C4 indicates "play Note-C in the fourth octave", so the two relevant bytes need to be the C4th entries in their respective data tables. It was this nonlinear approach that had me wishing for a cut-and-dry reference like the one I eventually created and submitted. --[Dr. Floppy](User_Dr__Floppy.xhtml "User:Dr. Floppy") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Dr._Floppy&action=edit&redlink=1 "User talk:Dr. Floppy \(page does not exist\)")) 23:37, 17 October 2013 (MDT) 

    Approaches that I've used to transcribe music in a readable text file involve either constants and macros (e.g. N_A0 = 0, N_AS0 = N_BB0 = 1, N_B0 = 2, N_C1 = 3, N_C5 = 51) or a domain-specific language translated by a PC-side program into music sequence source code. How would you do sharps and flats in your format? Or transpositions of a musical phrase? Or transpositions in an arpeggio envelope? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 13:33, 18 October 2013 (MDT) 

    I, too, want to know how to do the black keys (are they 1,2,3,4,5?). However from what I can tell, Retrodriver is done in entirely hex coding and does not use any assembler or MML or whatever. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:23, 19 October 2013 (MDT)

    

    

    

    

    I assigned the five accidentals ("black keys") according to my synaesthesia. High nybbles 4-, 5-, 6-, 7-, 8- correspond to C-sharp, E-flat, F-sharp, A-flat and B-flat (respectively). As always, the low-nybble determines the octave. Rhythm offsets utilize high nybbles 2- and 3-, allowing for 32 different note-durations for any given song. The $1x-block is used for selective repeats within a part, and the $0x-block is reserved for special control codes (d7-d3 of the fourth register and timbre/volume changes for the pulse channels, hard rests, soft rests and jumps). --[Dr. Floppy](User_Dr__Floppy.xhtml "User:Dr. Floppy") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Dr._Floppy&action=edit&redlink=1 "User talk:Dr. Floppy \(page does not exist\)")) 21:59, 19 October 2013 (MDT)

    What purpose is this user-friendly for? This table isn't good for someone looking to code an NES music engine. It can't be copy-pasted into code, and typing all of them by hand would be rather tedious and error prone. The [other page](APU_period_table.xhtml "APU period table") provides both a ready-made code block and also a program to generate them for you, both of which are directly useful for that purpose. Having a table like this is maybe useful if you want to look up a single pitch at a time for some reason, but who would want to do this? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 08:27, 15 October 2013 (MDT)

    

    It's useful for those who are using a non-linear format for their frequency storage in order to make actual music _coding_ easier. To illustrate, the C-Major scale in Retrodriver™ format is C4-D4-E4-F4-94-A4-B4-C5. --[Dr. Floppy](User_Dr__Floppy.xhtml "User:Dr. Floppy") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Dr._Floppy&action=edit&redlink=1 "User talk:Dr. Floppy \(page does not exist\)")) 23:37, 17 October 2013 (MDT)

    

    

    I'm not sure how that makes it useful though. Any driver will need to represent the entire scale. With the table on this page you will need to copy-paste 84 individual values to make your driver. It is much better to write a program to generate the table for you- it reduces the time needed and likelihood of human error. At any rate, I would highly recommend revising your table into a grid with the octaves in rows or columns, which would make it much quicker to look up individual values. After doing that, I would suggest we merge that table into [APU period table](APU_period_table.xhtml "APU period table"). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:47, 18 October 2013 (MDT)

    Another one to potentially merge with as well: [Celius NTSC table](Celius_NTSC_table.xhtml "Celius NTSC table") \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 08:25, 17 October 2013 (MDT)

## Tuning

If you're going to have a chart like this you should declare the nature of the tuning used. Presumably this is equal temperament using A-440Hz but this should be explicitly stated, as it is an arbitrary choice. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 08:23, 15 October 2013 (MDT) 

    Also, when generating your table, consider rounding to the nearest integer, rather than rounding only down. The resulting error vs. target pitch will be smaller. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:06, 2 November 2013 (MDT)
