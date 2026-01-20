# Talk:FDS BIOS

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AFDS_BIOS) | View [other pages](Special_AllPages.xhtml#Talk_FDS_BIOS)

## Invalid Disk ID structure

Info on the page: 
    
    
       offset	size	error#	description
       ------	----	------	-----------
        0       1       $04     game manufacturer code
        1       4       $05     game ASCII name string
        5       1       $06     game version
    

Isn't a game name 3 bytes long? [https://www.nesdev.org/wiki/FDS_disk_format#Disk_info_block_(block_1)](FDS_disk_format.xhtml#Disk_info_block_\(block_1\)) Should be: 
    
    
       offset	size	error#	description
       ------	----	------	-----------
        0       1       $04     game manufacturer code
        1       3       $05     game ASCII name string
        4       1       $05?    game type code (usually space, $20)
        5       1       $06     game version
    

[Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 19:23, 15 September 2023 (UTC) 

    The BIOS treats the game type code as part of the game name when checking the disk ID structure, so the article should reflect that. There are other inconsistencies as well, so I'll correct them. --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 11:40, 18 September 2023 (UTC)
