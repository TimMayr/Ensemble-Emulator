# NES 2.0 Mapper 268

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_268) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_268)

**NES 2.0 Mapper 268** denotes cartridges using the **AA6023** ASICs, labelled **SMD132** /**SMD133** when packaged, [MMC3](MMC3.xhtml "MMC3") clones that can address up to 32 MiB of PRG-ROM, 256 KiB of CHR-ROM or -RAM, and 8 KiB of WRAM. The ASIC provides six outer bank registers. Their address is selected by a solder pad labelled "5/6K", whose setting is denoted by NES 2.0 Submapper bit 0. One variant can even address 64 MiB of PRG-ROM. 

Setting | Address Range | NES 2.0 Submapper bit 0 | UNIF MAPR   
---|---|---|---  
right | $6000-$6FFF (submappers except 2/3)/$7000-$7FFF (submappers 2/3) | 0 | **COOLBOY** (without prefix)   
left | $5000-$5FFF | 1 | **MINDKIDS** (without prefix)   
  
NES 2.0 Submapper bits 1-3 distinguish between incompatible revisions and wiring variants: 

NES 2.0 Submappers | PCB Name | Notes   
---|---|---  
0/1 | **COOLBOY** /**MINDKIDS** /**KT-008** | CHR-RAM only, max. 32 MiB PRG-ROM   
**YH2018A** | CHR-RAM only, max. 64 MiB PRG-ROM   
**JTH-813** | CHR-ROM only, max. 8 MiB PRG-ROM, max. 1 MiB CHR-ROM   
**KT-???** | CHR-ROM+CHR-RAM; uses the ASIC's mixed CHR-ROM/RAM functionality selected by register $xxx4  
2/3 | **GH2009_V01** /**SMD173C_60** /**SMD173C_L1** | Later revision of 0/1. Die version **AA6023B** , chip still labelled as **SMD133**  
4/5 | **KP-6022** ,**LD622D** | Max. 4 MiB PRG-ROM   
6/7 | **J-852C** | Max. 128 KiB of CHR-RAM, as CHR A17 selects between two PRG-ROM chips   
8/9 | **SMD72A_V5S_V01** | Max. 2 MiB PRG-ROM, Max. 256 KiB CHR-RAM that can be write-protected   
10/11 | **SMD172C-L1** | Max. 8 MiB PRG-ROM, with single-screen mirroring   
  
## Contents

  * 1 Registers
    * 1.1 $xxx0
    * 1.2 $xxx1
    * 1.3 $xxx2
    * 1.4 $xxx3
    * 1.5 $xxx4
    * 1.6 $xxx5
  * 2 Register interpretation
    * 2.1 Banking mode = $00
    * 2.2 Banking mode = $10
    * 2.3 Banking mode = $40
    * 2.4 Banking mode = $50
  * 3 64 MiB mapping
  * 4 WRAM usage
  * 5 Game List
  * 6 Official documentation
  * 7 Notes



## Registers

In the $6000-$6FFF/$7000-$7FFF setting, the ASIC's registers registers overlay, but are functionally independent from, any PRG-RAM present. 

Masks: $F007 

### $xxx0

Submappers 0/1/2/3: 
    
    
     7  bit  0
     ---- ----
     ABCC DEEE
     |||| ||||
     |||| |+++-- PRG offset (PRG A19, A18, A17)
     |||| +----- Alternate CHR A17
     ||++------- PRG offset (PRG A24, A23)
     ||++------- CHR offset (CHR A19, A18) on CHR-ROM-having boards
     |+--------- PRG mask (PRG A17 from 0: MMC3; 1: offset)
     +---------- CHR mask (CHR A17 from 0: MMC3; 1: alternate)
    

Submappers 4/5: 
    
    
     7  bit  0
     ---- ----
     ABCC DEEE
     |||| ||||
     |||| |+++-- PRG offset (PRG A19, A18, A17)
     |||| |++--- CHR offset (A21, A20)
     |||| +----- Alternate CHR A17
     ||++------- PRG offset (PRG A21, A20)
     ||++------- CHR offset (CHR A19, A18) on CHR-ROM-having boards
     |+--------- PRG mask (PRG A17 from 0: MMC3; 1: offset)
     +---------- CHR mask (CHR A17 from 0: MMC3; 1: alternate)
    

Submappers 6/7: 
    
    
     7  bit  0
     ---- ----
     ABCC DEEE
     |||| ||||
     |||| |+++-- PRG offset (PRG A19, A18, A17)
     |||| +----- PRG chip select
     ||++------- PRG offset (PRG A24, A23), CHR offset (CHR A19, A18)
     |+--------- PRG mask (PRG A17 from 0: MMC3; 1: offset)
     +---------- PRG chip select source (0: from MMC3 CHR A17; 1: from D bit)
    

Both PRG chips always have the same size, so that the first half of a submapper 6/7 ROM file represents the content of the first, and the second half of the second chip. 

Submappers 8/9: 
    
    
     7  bit  0
     ---- ----
     AB.C DEEE
     || | ||||
     || | |+++-- PRG offset (PRG A19, A18, A17)
     || | +----- Alternate CHR A17
     || +------- 1=Write-protect CHR-RAM
     |+--------- PRG mask (PRG A17 from 0: MMC3; 1: offset)
     +---------- CHR mask (CHR A17 from 0: MMC3; 1: alternate)
    

Submappers 10/11: 
    
    
     7  bit  0
     ---- ----
     ABSC DEEE
     |||| ||||
     |||| |+++-- PRG offset (PRG A19, A18, A17)
     |||| +----- Alternate CHR A17
     |||+------- CIRAM A10 if S=0
     ||+-------- 0=1-Screen mirroring via bit C
     ||          1=Normal mirroring via MMC3 register A000
     |+--------- PRG mask (PRG A17 from 0: MMC3; 1: offset)
     +---------- CHR mask (CHR A17 from 0: MMC3; 1: alternate)
    

### $xxx1

Submappers 0/1/6/7/10/11: 
    
    
     7  bit  0
     ---- ----
     GHIJ KKLS
     |||| |||+-- SC0 bit (see below)
     |||| ||+--- GNROM mode bank PRG size (1: 32 KiB bank, PRG A14=CPU A14; 0: 16 KiB bank, PRG A14=offset A14)
     |||+-++---- PRG offset (in order: PRG A20, A22, A21)
     ||+-------- PRG mask (PRG A20 from 0: offset; 1: MMC3)
     |+--------- PRG mask (PRG A19 from 0: offset; 1: MMC3)
     +---------- PRG mask (PRG A18 from 0: MMC3; 1: offset)
    

Submappers 2/3: 
    
    
     7  bit  0
     ---- ----
     GHIL JKKx
     |||| |||
     |||| +++--- PRG offset (in order: A20, A21, A22)
     |||+------- GNROM mode bank PRG size (0: 32 KiB bank, PRG A14=CPU A14; 1: 16 KiB bank, PRG A14=offset A14)
     ||+-------- PRG mask (PRG A20 from 0: offset; 1: MMC3)
     |+--------- PRG mask (PRG A19 from 0: offset; 1: MMC3)
     +---------- PRG mask (PRG A18 from 0: MMC3; 1: offset)
    

Note that both meaning and position of the L bit is flipped between submappers 0/1 and 2/3, and the order of bits A20-A22. 

Submappers 4/5/8/9: 
    
    
     7  bit  0
     ---- ----
     GHIx xxLx
     |||    |
     |||    +--- GNROM mode bank PRG size (1: 32 KiB bank, PRG A14=CPU A14; 0: 16 KiB bank, PRG A14=offset A14)
     ||+-------- PRG mask (PRG A20 from 0: offset; 1: MMC3)
     |+--------- PRG mask (PRG A19 from 0: offset; 1: MMC3)
     +---------- PRG mask (PRG A18 from 0: MMC3; 1: offset)
    

### $xxx2
    
    
     7  bit  0
     ---- ----
     STTT MMMM
     |||| ||||
     |||| ++++-- CHR offset for GNROM mode (CHR A16, A15, A14, A13)
     |+++------- GNROM CHR bitmask (ANDed with CHR A16, A15, A14 in MMMM)
     +---------- GNROM CHR bitmask lock (1: prevents subsequent updates to S or T bits)
    

### $xxx3
    
    
     7  bit  0
     ---- ----
     NPZP QQRS
     |||| |||+-- SC1 bit (see below)
     |||| +++--- PRG offset for GNROM mode (PRG A16, A15, A14)
     |||+------- 1: GNROM mode; 0: MMC3 mode
     ||||         (1: PRG A16...13 from QQ, L, R, CPU A14, A13 + CHR A16...10 from MMMM, PPU A12...10;
     ||||          0: PRG A16...13 from MMC3 + CHR A16...A10 from MMC3 )
     ||+-------- 1: Also enable PRG RAM in $5000-$5FFF
     |+-+------- Banking mode
     |+--------- "Weird MMC3 mode"
     +---------- Lockout (prevent further writes to all registers but the one at $xxx2, only works in MMC3 mode)
    

### $xxx4
    
    
     7  bit  0
     ---- ----
     UUUU UUUV
     |||| ||||
     |||| |||+--- 0: Select CHR-ROM only, 1: Select mixed CHR-RAM/CHR-ROM mode
     ++++-+++---- MMC3 bank register value D1-D7 that select CHR-RAM rather than CHR-ROM
    

This register is only meaningful on cartridges that mount both CHR-ROM and CHR-RAM. By writing the appropriate value to D1-D7, the functionality of [INES Mapper 074](INES_Mapper_074.xhtml "INES Mapper 074") and [INES Mapper 194](INES_Mapper_194.xhtml "INES Mapper 194") is replicated. 

### $xxx5
    
    
     7  bit  0
     ---- ----
     xxxW XXYY
        | ||||
        | ||++--- Functionality unclear without extra hardware on PCB
        | ++----- Functionality unclear without extra hardware on PCB (CHR)
        +-------- 0: Normal MMC3 oversize mode
                  1: mega-UNROM mode; uses CHR A16 and A17 bits as PRG A18 and A19
    

## Register interpretation

The following things are always true, regardless of mode: 

  * The CCKK bits are always connected to PRG A24..A21.
  * The PRG and CHR mask controls (I, H, G, B, & A) always control whether PRG A20..A17 and CHR A17 are connected to the internal MMC3 registers, or the literal values in other controls (J, EEE, & D).



### Banking mode = $00

“Normal” oversize MMC3 mode. 

Supports MMC3 anywhere between 128 KiB and 2 MiB, otherwise identical to ordinary [MMC3](MMC3.xhtml "MMC3"). Goofy mask values will produce goofy memory layouts. 

The combined register of CCKKJEEE could be thought of as a way to select one of 256 different 128 KiB PRG MMC3 games, or CCKK selecting one of 16 different 2 MiB MMC3 games. 

Lockout works in this mode. 

### Banking mode = $10

“Normal” GNROM mode. 

PRG A16 and A15 come from the QQ controls. 

When in 16 KiB PRG mode (L control), PRG A14 comes from the R control. Otherwise, it comes from CPU A14. 

CHR A16, A15, A14, and A13 come from MMMM controls. CHR A12, A11, and A10 come from PPU A12, A11, and A10. 

The combined register of CCKKJEEEQQR could be thought of as a way to select one of 2048 different 16 KiB NROM games. 

Note: The CHR and PRG mask registers STILL AFFECT THIS MODE, allowing weird splicing of MMC3 banking (with 128 KiB granularity) overlaying GNROM-style banking. 

Lockout DOES NOT work in this mode. 

### Banking mode = $40

“Weird” MMC3 mode. 

If [MMC3 PRG ROM bank mode](MMC3.xhtml#Bank_select_.28.248000-.249FFE.2C_even.29 "MMC3") is "normal" (8+8+16F), then the PRG banks at 0xC000 and 0xE000 are fixed to 8 KiB bank 0, instead of 0xFE and 0xFF. Supposedly this effect is completely ignored when MMC3 PRG ROM bank mode is inverted (8F+8+8+8F). 

The second half of each of the 2 KiB CHR banks is replaced with bank 0, and the LSB of MMC3 registers 0 and 1 work (instead of being replaced by PPU A10). 

Otherwise, behaves as “normal” MMC3 mode above. 

Supposedly this was intended be used with extra hardware to enable an MMC4-like mode, perhaps similar to [iNES Mapper 165](INES_Mapper_165.xhtml "INES Mapper 165"). 

### Banking mode = $50

“Weird” GNROM-ish mode. 

Combine the rules mentioned above for “Normal” GNROM mode with “Weird” MMC3 mode. (These descriptions should not be contradictory.) 

Lockout DOES NOT work in this mode. 

# 64 MiB mapping

The **YH2018A** PCB uses the SC0 ($6001 bit 0) and SC1 ($6001 bit 1) bits (see above) to address two halves of 32 MiB for a total of 64 MiB. To do so, the ASIC's SL0 input pin is connected to GND, and the ASIC's OA0 output is connected to the ROM chip's PRG A25 input. This results in the following behaviors: 
    
    
    $xxx1.0  $xxx3.0  Meaning
    0        0        PRG A25=PRG A0, i.e. bytes at even addresses come from the first 32 MiB and bytes at odd addresses come from the second 32 MiB half.
    0        1        PRG A25=1, i.e. second 32 MiB half
    1        x        PRG A25=0, i.e. first 32 MiB half
    

After reset, both bits are cleared, so either the code at the reset vector must be carefully placed with bytes alternating between the two 32 MiB halves, or (as the existing multicarts do) must be duplicated in both 32 MiB halves. 

# WRAM usage

MMC3-mode operation with WRAM in the $6000-$6FFF (submapper 0) setting involves setting up the extra registers with the MMC3's WRAM disabled, setting the Lockout bit, then enabling WRAM again: 
    
    
    LDA #$00
    STA $A001   ; Disable WRAM
    ; initialize PRG/CHR offsets and masks
    ; ...
    LDA #$80
    STA $6003   ; Enable Lockout
    STA $A001   ; Enable WRAM
    

Because Lockout does not work in GNROM mode, it is impossible to have GNROM game use WRAM in the $6000-$6FFF (submapper 0) setting, as any WRAM write would inadvertently change the outer bank registers. The one known Chinese RPG (楚留香新传) running in GNROM mode with WRAM uses the $5000-$5FFF (submapper 1) setting. 

# Game List

Multicarts, mostly using the "Coolboy" or "Mindkids" circuit board: 

  * _218-in-1 Real Game_
  * _MegaMan 8-in-1 (PL-0006)_
  * _Data East All-Star Collection_
  * _PocketGames 150-in-1_
  * _Super 208-in-1_
  * _Super 340-in-1_
  * _Super 360-in-1_
  * _Super 402-in-1_
  * _Super Game 143-in-1_ , a.k.a. _The Best Games of NES_
  * _深圳市仁顺科技 400-in-1 Real Game_
  * _霸王小子 500-in-1_



Chinese RPGs with more than 512 KiB of PRG-ROM: 

  * _大话水浒_ (Dàhuà Shuǐhǔ)
  * _楚留香新传_ (Chǔliúxiāng xīn Zhuàn)
  * _魔道劫_ (Módào Jié)
  * _口袋妖怪: 叶绿 (vol. 1-3)_ (Kǒudài Yāoguài: Yè Lǜ), a.k.a. _Pokémon Leaf-Green_
  * _最终幻想 1 - 黑暗篇_ (Zuìzhōng Huànxiǎng 1 - Hēi'àn Piān), a.k.a. _Final Fantasy X, Part 1_
  * _最终幻想 2 - 光明篇_ (Zuìzhōng Huànxiǎng 2 - Guāngmíng Piān), a.k.a. _Final Fantasy X, Part 2_ , original release only
  * _最终幻想 3 - 黑暗篇_ (Zuìzhōng Huànxiǎng 3 - Zhōngjié Piān), a.k.a. _Final Fantasy X, Part 3_
  * _神奇宝贝_ (Shénqí Bǎobèi), a.k.a. _Pokémon Diamond_



Other games that do not merely use the ASIC as a simple MMC3 clone: 

  * _Gravity Trooper Metal Storm_ (Retro-Bit release)
  * _Holy Diver_ (Retro-Bit release)
  * _Quest Forge_ (Piko Interactive release), although the underlying game is simply NROM



A great number of Chinese cartridges use the ASIC as a simple MMC3 clone and thus are usually found set to [INES Mapper 004](MMC3.xhtml "INES Mapper 004"). They can be recognized by writing $80 to $6003 in their initialization code. 

# Official documentation

  * [File:AA6023Av1 1 2-OK.pdf](File_AA6023Av1_1_2_OK_pdf.xhtml "File:AA6023Av1 1 2-OK.pdf") \- datasheet for AA6023A
  * [File:AA6023Bv1.0-ok.pdf](File_AA6023Bv1_0_ok_pdf.xhtml "File:AA6023Bv1.0-ok.pdf") \- datasheet for AA6023B
  * [File:YBB52LQF064006 SMD132 AA6023A.pdf](File_YBB52LQF064006_SMD132_AA6023A_pdf.xhtml "File:YBB52LQF064006 SMD132 AA6023A.pdf") \- pinout for SMD132
  * [SMD133 (AA6023) pinout](SMD133__AA6023__pinout.xhtml "SMD133 \(AA6023\) pinout")
  * [File:SMD129.pcb](File_SMD129_pcb.xhtml "File:SMD129.pcb") \- PCB layout of one of "COOLBOY" cartridges (for Altium Designer)



# Notes

  * No cart has been discovered so far that makes use of the "weird" mode.
  * Some new cartridges from MINDKIDS have /WE and /OE pins connected to mapper, which allows you to rewrite flash memory without soldering. This also allows console to write data to the cartridge. No cart has been discovered so far that makes use of this feature, but this can be used for homebrew.
  * See also: [COOLGIRL](NES_2_0_Mapper_342.xhtml "COOLGIRL")



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
