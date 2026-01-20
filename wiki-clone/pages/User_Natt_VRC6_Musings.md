# User:Natt/VRC6 Musings

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ANatt/VRC6_Musings) | View [other pages](Special_AllPages.xhtml#User_Natt_VRC6_Musings)

## Contents

  * 1 This page was created from incorrect data and is not relevant. Will redo later.
    * 1.1 Step 1: Choose your pattern table mapping choice:
    * 1.2 Step 2: Choose CIRAM nametables (N = 0) or CHRROM nametables (N = 1)
    * 1.3 Step 3: Consider your options
    * 1.4 Scratch



# This page was created from incorrect data and is not relevant. Will redo later.

(This isn't really finished or ready for general consumption) 

Assume here that the VRC6 has not been rewired at all (although something like Mapper 076 might make sense in some cases.) 
    
    
    7  bit  0
    ---- ----
    W.PN MMDD
    

## Step 1: Choose your pattern table mapping choice:
    
    
    D = 0  01234567
    D = 1  00112233
    D =2,3 01234455
    

## Step 2: Choose CIRAM nametables (N = 0) or CHRROM nametables (N = 1)

## Step 3: Consider your options

  * D=0,N=0: The standard usage. Nothing is gained by setting P = 0, so set P = 1 and use M to choose V\H\1sca\1scb.
  * D=0,N=1: Your options will be very limited here, as r6 and r7 will be simultaneously setting pattern and nametables. Nothing to be done about this; a limitation of the mapper. Use the low bit of M to choose 6677 vs 6767 in nametables. P = 0 or P = 1 will make no difference to pattern tables, so you can use either P = 0, or P = 1 with toggling the high bit of M to let you change the low bit of r6 and 67 in the nametables. The only use I could see for this is having r6 and r7 point to 2KiB areas with a nametable in one KiB and matching patterns in the other KiB.
  * D=1: P=0 makes no sense, so choose P=1. With N=0, you can choose any CIRAM mapping, including L and Diagonal. With N=1, you have full freedom to do 4screen CHRROM nametables. M has no effect in any case.
  * D=2 or 3,N=0: There is no reason to use these modes. You do not gain any CIRAM nametable mappings beyond those in D=0,N=0, regardless of other bits, and you lose pattern table mappings.
  * D=2,N=1: As in D=0,N=1, use the low bit of M to choose 6677 vs 6767 in the nametables. You can freely choose P=0 or P=1 while only affecting the pattern tables. The high bit of M is irrelevant.
  * D=3,N=1: This mode doesn't appear to gain anything over D=2,N=1, so should not be used. You can use P=1 to change low bits of r6,r7; but those registers are only used in nametables, and so can freely be specified already.



  
That's all a bit of a mess and needs to be clarified some, but it seems like the only options with any real value are: 

  * D=0,N=0: Standard
  * D=1,N=0: If you MUST have L-mirroring or Diagonal mirroring
  * D=1,N=1: CHRROM nametables with full 4-screen nametables
  * D=2,N=1: 2 screens of CHRROM nametables, but more pattern banking options than D=1,N=1



The rest are redundant or bad. One thing to note is that you can never have CHRROM nametables with the A10 override not "matching" the nametable configuration (6677 with CHR A10 replaced by PPU A11 or 6767 with CHR A10 replaced by PPU A10). This is probably a limitation in the silicon, and extra gates would be needed to allow this. The only way to get that level of control is to use the D=1 mode. 

  


## Scratch
    
    
    ppu  DD=0 DD=1 DD=2,3
    0000  r0   r0   r0
    0400  r1   r0   r1
    0800  r2   r1   r2
    0c00  r3   r1   r3
    1000  r4   r2   r4
    1400  r5   r2   r4
    1800  r6   r3   r5
    1c00  r7   r3   r5
    
    
    
    000 6767 A
    11? 6767 B
    01? 6677 C
    100 6677 D
    ?01 4567
    
    
    
    replacements:
    MMDD nbit bank x
    0000 0101 6767 A 'V'
    0111 0101 6767 B 'V'
    0011 0011 6677 C 'H'
    0100 0011 6677 D 'H'
    1000 0000 6767 A 'A'
    1111 0000 6767 B 'A'
    1011 1111 6677 C 'B'
    1100 1111 6677 D 'B'
    
