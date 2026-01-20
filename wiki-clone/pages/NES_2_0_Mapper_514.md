# NES 2.0 Mapper 514

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_514) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_514)

NES 2.0 Mapper 514 is used for the game _小霸王 卡拉OK (Subor Karaoke)_. It banks PRG-ROM in 32 KiB amounts, has 8 KiB WRAM, and 2x4 KiB of CHR-RAM that can be bankswitched according to the current nametable offset during rendering. 

## Mode/PRG-ROM Bank Register ($8000)
    
    
    Bit 7654 3210
        BMPP PPPP
        ||++-++++- Select 32 KiB PRG-ROM bank at CPU $8000
        |+-------- Select nametable arrangement
        |           0: Vertical
        |           1: Horizontal
        +--------- Select CHR-RAM banking mode (during PPU read accesses only)
                    0: 8 KiB CHR-RAM at PPU $0000-$1FFF
                    1: 4 KiB CHR-RAM at PPU $0000-$0FFF automatically switched, 4 KiB CHR-RAM at PPU $1000-$1FFF fixed to second half of CHR-RAM
    

## Notes

  * When Bit 7 is set, PPU $0000-$0FFF will point to the first half of CHR-RAM while the PPU renders from the first CIRAM nametable, and the second half of CHR-RAM while the PPU renders from the second CIRAM nametable (offset $2400/$2C00 with horizontal arrangement, or offset $2800/$2C00 with vertical arrangement). The game uses this feature to have a CHR bankswitch in the middle of the screen by setting the scroll value such that the position at which the CHR bank needs to be switched is the seams between the two CIRAM nametables. This works even in the horizontal direction, providing a simple way for a mid-scanline bankswitch. Note that write accesses to CHR-RAM are never bankswitched.


  * Note that this game requires Dendy video timing and will produce graphical glitches with NTSC or regular PAL timing.


  * [NES 2.0 Mapper 518](NES_2_0_Mapper_518.xhtml "NES 2.0 Mapper 518") extends this mapper with a selectable 16 or 32 KiB PRG-ROM bank size.


