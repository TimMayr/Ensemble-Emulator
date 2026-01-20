# Talk:FDS disk format

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AFDS_disk_format) | View [other pages](Special_AllPages.xhtml#Talk_FDS_disk_format)

## Removed Price field description

[This edit](https://www.nesdev.org/w/index.php?title=FDS_disk_format&curid=221&diff=20450&oldid=20399) by Fiskbit removed this Price field description with the comment: "Removes 'Price' section, which seems to have no backing."

    

    When the **Disk rewrite count** field is $00, the **Price** field represents the cost of a "new/original" disk. In this situation, a value of $01 means 3400円 (3400 yen), and a value of $03 also means 3400円 but includes peripherals. The example given by Enri in his Famicom Disk System documentation mentions the game [とびだせ大作戦 (Tobidase Daisakusen/3D Worldrunner)](http://ja.wikipedia.org/wiki/%E3%81%A8%E3%81%B3%E3%81%A0%E3%81%9B%E5%A4%A7%E4%BD%9C%E6%88%A6), where both sales of the game with and without the [とびだせメガネ (Tobidase 3D glasses)](http://homepage3.nifty.com/poco-pen/s-te.htm) cost 3400円. The reason for the delineation within the actual **Price** field is unknown.
    When the **Disk rewrite count** field is $01, the **Price** field represents incurred costs of disk rewriting (such as that done by a Disk Writer kiosk). A value of $00 means 500円 (which appears to be specific to Mario Brothers via an in-box advertisement promising that all disk rewrites would only cost 500円), while a value of $01 means 600円.

This was added by Koitsu in [this edit](https://www.nesdev.org/w/index.php?title=Family_Computer_Disk_System&diff=2948&oldid=2947), with a comment that references [this forum discussion](https://forums.nesdev.org/viewtopic.php?t=11519). 

I'm not making an argument for restoring this, at this point, but I wanted to document the source of it in case anyone thinks it's worth following up on. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 03:07, 23 February 2023 (UTC) 

Ah, I see what the source is now. It's a direct translation of the description in [Enri's FDS document](http://www43.tok2.com/home/cmpslv/Famic/Famdis.htm). (ブロック コードの説明 section, _block code description_) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 03:11, 23 February 2023 (UTC) 

    Thanks for sourcing that. I see that page calls the field "デバグバージョンまたは、プライス", translating to 'debug version or price'. Nintendo called this field 'Disk version', distinct from 'Game version'. For some games, we have multiple disk versions which differ in the ROM contents and this field. So, either this is just a disk version field, or a dual-purpose field that also means price sometimes. I'm open to adding price back if we find evidence for it, but I don't think what we currently know supports it. - [Fiskbit](User_Fiskbit.xhtml "User:Fiskbit") ([talk](User_talk_Fiskbit.xhtml "User talk:Fiskbit")) 15:32, 23 February 2023 (UTC)
