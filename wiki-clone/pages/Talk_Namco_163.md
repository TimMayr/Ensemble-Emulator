# Talk:Namco 163

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANamco_163) | View [other pages](Special_AllPages.xhtml#Talk_Namco_163)

## Wither 106?

I see that Namco 106 has been eradicated from the text of the wiki. I don't know where the number 106 came from originally, so I'm not defending its existence at all, however, it deserves some mention, at least on this page, because it is a commonly used name, even if incorrect. Having no mention of it at all will make it confusing for those who are trying to look it up. Some older programs, like VirtuaNSF or PPMCK/MML use the name 106, for instance. 

I'll add a note to this page's lead, but I'd appreciate if someone who knows more about the issue would revise it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:04, 11 February 2013 (MST) 

## Extra 128 bytes of PRG RAM

What is this? Could you please explain it in the article somewhere? (I can't find the information.) Adding it to just the infobox is hella confusing. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:01, 23 April 2015 (MDT) 

    Is this referring to the internal audio RAM? I don't think that should be considered as PRG-RAM, since it's not on the PRG address bus, but written through a port. (It's no more PRG-RAM than CHR-RAM is.) If you want to explain that it can be used as RAM if needed, this needs a proper description in the article. (Presumably you have to disable audio via $E000 to do this? I know the audio hardware normally writes to its own channel registers while operating.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:24, 23 April 2015 (MDT)

    

    Yes, the internal audio RAM. I don't know that the internal postincrement is fast enough to keep up with execution or DMA, but several games use it for saving, so even given its indirect addressing, it seems important to mention. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:50, 23 April 2015 (MDT)
