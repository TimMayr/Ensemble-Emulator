# INES Mapper 029

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_029) | View [other pages](Special_AllPages.xhtml#INES_Mapper_029)

This mapper was allocated on 15-nov-2013 to implement some homebrew games, including Glider. FCEUX acquired support on r3029. 

The example board in question was marked as follows: "Sealie Computing", "RET-CUFROM revD", "2/29/08"

The board includes 4x banks worth of full 8KB CHR ram, and 8x 16KB PRG banks. 

It is hard-wired for vertical mirroring, and contains 8KB of WRAM mounted in the usual place. There is no battery, but pads are present and suitable for such a purpose if anyone were so inclined. There are no bus conflicts. 

The board contains logic designed for reprogramming from a CopyNES. If EXP0 and R/W go low, /WR reaches the flash ROM. When EXP0 is high, the internal bankswitching register gets accessed. 
    
    
     Registers:
     ---------------------------
     
     Range,Mask:   $8000-FFFF, $8000
     
       $8000:  [...P PPCC]
         C = CHR RAM bank
         P = PRG ROM bank
     
     PRG Setup:
     ---------------------------
          $8000   $A000   $C000   $E000  
         +-------------------------------+
         |     $8000     |     { -1}     |
         +---------------+---------------+
     
     CHR Setup:
     ---------------------------
           $0000   $1000   
         +-------+-------+
         |     $8000     |
         +-------+-------+
    

Here's a schematic of the board: 
    
    
    377 O0 --- CRAM A13
    
    377 O1 --- CRAM A14
                 ____
    A14 ---------\    \
                  )    >--- ROM A14
    377 O2 ------/____/
    
                 ____
    A14 ---------\    \
                  )    >--- ROM A15
    377 O3 ------/____/
    
                 ____
    A14 ---------\    \
                  )    >--- ROM A16
    377 O4 ------/____/
    
                 ____
    EXP0 --------\    \
                  )    >--- ROM /WE
    R/W ---------/____/
    
              ,--------------- 377 CLK
              |
              +--------------- ROM /CE
              |   ____
    /ROMSEL --+--|    \
                 |     )--,
    M2 ----------|____/   |
              ,-----------'
              |   ____
              `--|    `-.
                 |       \
    A14 ---------|        )o-- RAM /CE
                 |       /
    A13 ---------|____,-'
    
                  ____
    EXP0 --------|    `-.
                 |       \
    +5V ---------|        )o-- 377 /G
                 |       /
          ,------|____,-'
          |
          `------------------,
                  ____       |
    +5V ------+--|    `-.    +-- RAM /OE
              |  |       \   |
              `--|        )o-+-- ROM /OE
                 |       /
    R/W ------+--|____,-'
              |
              `--------------- RAM /WE
    
