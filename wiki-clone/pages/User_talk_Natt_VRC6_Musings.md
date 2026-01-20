# User talk:Natt/VRC6 Musings

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3ANatt/VRC6_Musings) | View [other pages](Special_AllPages.xhtml#User_talk_Natt_VRC6_Musings)

## Mode 3

The original VRC6 documents made a big deal about how mode 3 was good for using an extra RAM to get four screens of nametables, by setting up registers 6 and 7 at right angles to the A10 override. Presumably, even without the extra RAM this would be useful for four screens of ROM nametables. Although, just how useful four screens of ROM nametables would be is another question. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:40, 23 January 2014 (MST) 

  
My analysis concluded that you couldn't do 4 screen ROM nametables with D=3. I'm not sure if I was right. Going from what's on the wiki page: 

Choose N=1 and D=3. The 1K bank mapping of PPU 0000:1FFF is: 
    
    
    01234455
    

If M=0 or M=2, the 1K bank mapping of PPU 2000:2FFF is: 
    
    
    6677
    

If M=1 or M=3, the 1K bank mapping of PPU 2000:2FFF is: 
    
    
    6767
    

Either way, we clearly need P=1 in order to give possibility of 4 screen ROM nametables. Using the P substitution table, look at the possibilities: 
    
    
    MMDD Banks    P-Substitution    Final Mapping in PPU 2000:FFFF
    0011  6677    PPU A11            (r6 & ~1),(r6 & ~1), (r7 | 1), (r7 | 1)
    0111  6767    PPU A10            (r6 & ~1), (r7 | 1),(r6 & ~1), (r7 | 1)
    1011  6677    Vcc                 (r6 | 1), (r6 | 1), (r7 | 1), (r7 | 1)
    1111  6767    Gnd                (r6 & ~1),(r7 & ~1),(r6 & ~1),(r7 & ~1)
    

So in any of these cases, you only get two distinct values for the 4 nametables. You would need to have a 6677 pattern with PPU A10 override, or 6767 pattern with PPU A11 override, but in the chart of substitutions given, it doesn't seem that that ever happens? [Natt](User_Natt.xhtml "User:Natt") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Natt&action=edit&redlink=1 "User talk:Natt \(page does not exist\)")) 16:29, 23 January 2014 (MST) 

* * *

Hm. I wonder what's gone wrong here? The original document provides the following examples: 
    
    
    [$B003] = $23, R6 = 0, R7 = 4 : nametables are 0,4,1,5
    [$B003] = $27, R6 = 1, R7 = 4 : nametables are 0,1,4,5
    [$B003] = $2B, R6 = 2, R7 = 5 : nametables are 3,3,5,5
    [$B003] = $2F, R6 = 2, R7 = 5 : nametables are 2,2,4,4
    

The corresponding tests I asked bootgod to do (R6 and R7 were $71 and $78) showed the following: 
    
    
    23   70   78   71   79
    27   70   71   78   79   
    2B   71   79   71   79   
    2F   70   70   78   78
    

I must have wrote this up wrong. 

Ok, so what did I mean to say? From both Konami's and BootGod's data above, CHRA10 is connected to (A11 = M=00), (A10 = M=01), (Vcc = M=10), (Gnd = M=11) And the register map is (6767 = M=00/10), (6677 = M=01/11)... Yeah, there's the typo. Hm. did I screw up 0/4, also? 
    
    
    20  70   71   78   79   
    24  70   78   71   79   
    28  70   70   78   78   
    2C  71   79   71   79
    

Augh, yes. That's embarrassing. Fixing that. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:34, 23 January 2014 (MST) 
