# Talk:VRC4

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AVRC4) | View [other pages](Special_AllPages.xhtml#Talk_VRC4)

This table was generated in trying to find the mythical VRC4f board. This is the union of all VRC2 and VRC4 in both Nestopia's DB and NesCartDB. The names of games in Nestopia but not in NesCartDB were found using GoodNES-3.1.4. 

Game | IC | PCB | iNES | variant | crc32   
---|---|---|---|---|---  
Mad City (prototype) |  | not in nescartdb | 23 | 2b | AA9F9765   
? |  | not in nescartdb | 25 | 4b | F880E010   
Pizza Pop Mario (unl) |  | not in nescartdb | 25 | 4d | 7075BF5E   
Goal!! |  | not in nescartdb | 25 | 4d | D6D2E486   
? |  | not in nescartdb | 23 | 4f | 51392C01   
Ganbare Pennant Race | VRC2 | 351618 | 22 | 2a | 90F6FA33   
TwinBee 3 | VRC2 | 351618 | 22 | 2a | D7FABAC1   
Contra | VRC2 | LROG009-00 | 23 | 2b | B27B8CF4   
Dragon Scroll | VRC2 | 350603 | 23 | 2b | AC9895CC   
Ganbare Goemon 2 | VRC2 | 350926 | 23 | 2b | 0CC9FFEC   
Getsufuu Maden | VRC2 | 350636 | 23 | 2b | 49123146   
Jarinko Chie | VRC2 | 351179 | 23 | 2b | 39B68AA3   
Wai Wai World | VRC2 | 350926 | 23 | 2b | 8A96E00D   
Ganbare Goemon Gaiden | VRC2 | 351948 | 25 | 2c | EB92B32A   
Wai Wai World 2 | VRC4 | 352398 | 21 | 4a | 8B03F74D   
Gradius 2 | VRC4 | 351406 | 25 | 4b | 5ADBF660   
Bio Miracle | VRC4 | 351406 | 25 | 4b | 6DC28B5A   
Racer Mini Yonku | VRC4 | 351406 | 25 | 4b | F6271A51   
Ganbare Goemon Gaiden 2 | VRC4 | 352889 | 21 | 4c | 286FCD20   
TMNT2 | VRC4 | 352400 | 25 | 4d | 490E8A4C   
TMNT | VRC4 | 352400 | 25 | 4d | 4A601A2C   
Tiny Toon Adventures | VRC4 | 352396 | 23 | 4e | 91328C1D   
Akumajou Special | VRC4 | 352396 | 23 | 4e | C1FBF659   
Parodius Da | VRC4 | 352396 | 23 | 4e | D467C0CC   
Crisis Force | VRC4 | 352396 | 23 | 4e | FCBF28B1   
  
It is plausible that "VRC4f" is supposed to be [World Hero](INES_Mapper_027.xhtml "INES Mapper 027")

  


## registers

do the "lower" two address lines for prg registers matter at all? like for vrc4a PRG Select 0, it looks like $8000/2/4/6 all do the same thing? versus say CHR selects where each address in the sets of four means a specific thing. If that's true can we add an explanation of that? As is it's not obvious why those four addresses are specified. 
