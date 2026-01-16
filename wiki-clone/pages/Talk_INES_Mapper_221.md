# Talk:INES Mapper 221

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_221) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_221)

### Something wrong

Mapper #221 on this page and mapper #221 emulated by fceux (<https://github.com/TASVideos/fceux/blob/master/src/boards/n625092.cpp>) are two totally different mappers. I dumped "NEW MIXED GAME CARD 40000 IN 1" using this code: 
    
    
               dumper.WriteCpu((ushort)(0x8000 | ((bank & 0b11111000) << 2)), 0x00);
               dumper.WriteCpu((ushort)(0xFFF0 | (bank & 0b00000111)), 0x00);
               data.AddRange(dumper.ReadCpu(0x8000, 0x4000));
    

It works in fceux but seems like the fceux emulation code is not correct too because this dumping code is not working: 
    
    
               dumper.WriteCpu((ushort)(0x8000 | (bank << 2)), 0x00);
               dumper.WriteCpu(0xFFF0, 0x00);
               data.AddRange(dumper.ReadCpu(0x8000, 0x4000));
    

[Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 14:06, 14 August 2021 (UTC) 
