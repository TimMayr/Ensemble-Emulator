# User talk:Sour

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3ASour) | View [other pages](Special_AllPages.xhtml#User_talk_Sour)

## Contents

  * 1 CPU Tests
  * 2 PPU Tests
  * 3 APU Tests
  * 4 Mapper-specific Tests
  * 5 Input Tests
  * 6 Misc Tests



## CPU Tests

Name  | Author  | Description  | Resources   
---|---|---|---  
[branch_timing_tests](http://www.slack.net/~ant/nes-tests/branch_timing_tests.zip) | blargg | These ROMs test timing of the branch instruction, including edge cases |   
[cpu_dummy_reads](https://github.com/christopherpow/nes-test-roms/raw/master/cpu_dummy_reads/cpu_dummy_reads.nes) | blargg | Tests the CPU's dummy reads | [thread](https://forums.nesdev.org/viewtopic.php?p=31629)  
[cpu_dummy_writes](http://bisqwit.iki.fi/src/nes_tests/cpu_dummy_writes.zip) | bisqwit | Tests the CPU's dummy writes | [thread](https://forums.nesdev.org/viewtopic.php?t=8738)  
[cpu_exec_space](http://bisqwit.iki.fi/src/nes_tests/cpu_exec_space.zip) | bisqwit | Verifies that the CPU can execute code from any possible memory location, even if that is mapped as I/O space | [thread](https://forums.nesdev.org/viewtopic.php?t=8755)  
[cpu_flag_concurrency](http://bisqwit.iki.fi/kala/test1.zip) | bisqwit | Unsure what results are meant to be, see thread for more info. | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=8757)  
[cpu_interrupts_v2](http://blargg.8bitalley.com/parodius/nes-tests/cpu_interrupts_v2.zip) | blargg | Tests the behavior and timing of CPU in the presence of interrupts, both IRQ and NMI; see CPU interrupts. | [thread](https://forums.nesdev.org/viewtopic.php?f=2&t=6510)  
[cpu_reset](http://blargg.8bitalley.com/parodius/nes-tests/cpu_reset.zip) | blargg | Tests CPU registers just after power and changes during reset, and that RAM isn't changed during reset. |   
[cpu_timing_test6](http://blargg.8bitalley.com/parodius/nes-tests/cpu_timing_test6.zip) | blargg | This program tests instruction timing for all official and unofficial NES 6502 instructions except the 8 branch instructions (Bxx) and the 12 halt instructions (HLT) | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=3831)  
[instr_misc](http://blargg.8bitalley.com/parodius/nes-tests/nes_instr_misc.zip) | blargg | Tests some miscellaneous aspects of instructions, including behavior when 16-bit address wraps around, and dummy reads. |   
[instr_test_v5](http://blargg.8bitalley.com/nes-tests/instr_test-v5.zip) | blargg | Tests official and unofficial CPU instructions and lists which ones failed. It will work even if emulator has no PPU and only supports NROM, writing a copy of output to $6000 (see readme). This more thoroughly tests instructions, but can't help you figure out what's wrong beyond what instruction(s) are failing, so it's better for testing mature CPU emulators. |   
[instr_timing](http://blargg.8bitalley.com/parodius/nes-tests/instr_timing.zip) | blargg | Tests timing of all instructions, including unofficial ones, page-crossing, etc. |   
[nestest](http://nickmass.com/images/nestest.nes) | ? | fairly thoroughly tests CPU operation. This is the best test to start with when getting a CPU emulator working for the first time. Start execution at $C000 and compare execution with a log from Nintendulator, an emulator whose CPU is known to work correctly (apart from some details of the power-up state). |   
  
## PPU Tests

Name  | Author  | Description  | Resources   
---|---|---|---  
[blargg_ppu_tests_2005.09.15b](http://www.slack.net/~ant/nes-tests/blargg_ppu_tests.zip) | blargg | Miscellaneous PPU tests (palette ram, sprite ram, etc.) | [thread](https://forums.nesdev.org/viewtopic.php?t=567)  
[full_palette](http://blargg.8bitalley.com/parodius/nes-code/full_palette.zip) | blargg | Displays the full palette with all emphasis states, demonstrates direct PPU color control | [thread](https://forums.nesdev.org/viewtopic.php?p=10658)  
[nmi_sync](http://blargg.parodius.com/nes-code/nmi_sync.zip) | blargg | Verifies NMI timing by creating a specific pattern on the screen (NTSC & PAL versions) | [thread](https://forums.nesdev.org/viewtopic.php?t=6589)  
[oam_read](http://blargg.8bitalley.com/parodius/nes-tests/oam_read.zip) | blargg | Tests OAM reading ($2004), being sure it reads the byte from OAM at the current address in $2003. | [thread](https://forums.nesdev.org/viewtopic.php?t=6424)  
[oam_stress](http://blargg.8bitalley.com/parodius/nes-tests/oam_stress.zip) | blargg | Thoroughly tests OAM address ($2003) and read/write ($2004) | [thread](https://forums.nesdev.org/viewtopic.php?t=6424)  
[ppu_open_bus](http://blargg.8bitalley.com/parodius/nes-tests/ppu_open_bus.zip) | blargg | Tests behavior when reading from open-bus PPU bits/registers |   
[ppu_read_buffer](http://bisqwit.iki.fi/src/nes_tests/ppu_read_buffer.zip) | bisqwit | Mammoth test pack tests many aspects of the NES system, mostly centering around the PPU $2007 read buffer | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=8847&start=0)  
[ppu_sprite_hit](http://blargg.8bitalley.com/parodius/nes-tests/ppu_sprite_hit.zip) | blargg | Tests sprite 0 hit behavior and timing | [thread](https://forums.nesdev.org/viewtopic.php?t=626)  
[ppu_sprite_overflow](http://blargg.8bitalley.com/parodius/nes-tests/ppu_sprite_overflow.zip) | blargg | Tests sprite overflow behavior and timing | [thread](https://forums.nesdev.org/viewtopic.php?t=1308)  
[ppu_vbl_nmi](http://blargg.8bitalley.com/parodius/nes-tests/ppu_vbl_nmi.zip) | blargg | Tests the behavior and timing of the NTSC PPU's VBL flag, NMI enable, and NMI interrupt. Timing is tested to an accuracy of one PPU clock. | [thread](https://forums.nesdev.org/viewtopic.php?t=730)  
[scanline](http://www.qmtpro.com/~nes/demos/scanline.zip) | Quietust | Displays a test screen that will contain glitches if certain portions of the emulation are not perfect. |   
[sprite_hit_tests_2005.10.05](http://www.slack.net/~ant/nes-tests/sprite_hit_tests.zip) | blargg | Generally the same as ppu_sprite_hit (older revision of the tests - ppu_sprite_hit is most likely better) | [thread](https://forums.nesdev.org/viewtopic.php?t=626)  
[sprite_overflow_tests](http://www.slack.net/~ant/nes-tests/sprite_overflow_tests.zip) | blargg | Generally the same as ppu_sprite_overflow (older revision of the tests - ppu_sprite_overflow is most likely better) | [thread](https://forums.nesdev.org/viewtopic.php?t=1308)  
[sprdma_and_dmc_dma](http://blargg.parodius.com/temp/sprdma_and_dmc_dma.zip) | blargg | Tests the cycle stealing behavior of the DMC DMA while running Sprite DMAs | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=6100)  
[tvpassfail](http://pics.pineight.com/nes/tvpassfail.zip) | tepples | NTSC color and NTSC/PAL pixel aspect ratio test ROM | [thread](https://forums.nesdev.org/viewtopic.php?t=3393)  
[vbl_nmi_timing](http://blargg.8bitalley.com/parodius/nes-tests/ppu_vbl_nmi.zip) | blargg | Generally the same as ppu_vbl_nmi (older revision of the tests - ppu_vbl_nmi is most likely better) | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=3953)  
  
## APU Tests

Name  | Author  | Description  | Resources   
---|---|---|---  
[apu_mixer](http://blargg.8bitalley.com/parodius/nes-tests/apu_mixer.zip) | blargg | Verifies proper operation of the APU's sound channel mixer, including relative volumes of channels and non-linear mixing. recordings when run on NES are available for comparison, though the tests are made so that you don't really need these. | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=4911)  
[apu_phase_reset](https://forums.nesdev.org/download/file.php?id=7496) | Rahsennor | Tests the correct square channel behavior when writing to $4003/4007 (reset the duty cycle sequencers but not the clock dividers) | [thread](https://forums.nesdev.org/viewtopic.php?f=2&t=15346)  
[apu_reset](http://blargg.8bitalley.com/parodius/nes-tests/apu_reset.zip) | blargg | Tests initial APU state at power, and the effect of reset. |   
[apu_test](http://blargg.8bitalley.com/parodius/nes-tests/apu_test.zip) | blargg | Tests many aspects of the APU that are visible to the CPU. Really obscure things are not tested here. |   
[blargg_apu_2005.07.30](http://www.slack.net/~ant/nes-tests/blargg_apu_2005.07.30.zip) | blargg | Tests APU length counters, frame counter, IRQ, etc. |   
[dmc_dma_during_read4](http://blargg.8bitalley.com/parodius/nes-tests/dmc_dma_during_read4.zip) | blargg | Tests register read/write behavior while DMA is running |   
[dmc_tests](https://github.com/christopherpow/nes-test-roms/tree/master/dmc_tests) | ? | ? |   
[dpcmletterbox](http://pics.pineight.com/nes/dpcmletterbox.zip) | tepples | Tests accuracy of the DMC channel's IRQ |   
[pal_apu_tests](http://www.slack.net/~ant/nes-tests/pal_apu_tests.zip) | blargg | PAL version of the blargg_apu_2005.07.30 tests |   
[square_timer_div2](http://blargg.8bitalley.com/parodius/nes-tests/older/square_timer_div2.zip) | blargg | Tests the square timer's period |   
[test_apu_2 (1-10)](https://forums.nesdev.org/download/file.php?id=1494) [(11)](https://forums.nesdev.org/download/file.php?id=5699) | x0000 | 11 tests that verify a number of behaviors with the APU (including the frame counter) | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=11174)  
[test_apu_env](http://blargg.8bitalley.com/parodius/nes-tests/older/test_apu_env.zip) | blargg | Tests the APU envelope for correctness. |   
[test_apu_sweep](http://blargg.8bitalley.com/parodius/nes-tests/older/test_apu_sweep.zip) | blargg | Tests the sweep unit's add, subtract, overflow cutoff, and minimum period behaviors. |   
[test_apu_timers](http://blargg.8bitalley.com/parodius/nes-tests/older/test_apu_timers.zip) | blargg | Tests frequency timer of all 5 channels |   
[test_tri_lin_ctr](http://blargg.8bitalley.com/parodius/nes-tests/older/test_tri_lin_ctr.zip) | blargg | Tests triangle's linear counter and clocking by the frame counter |   
[volume_tests](http://pics.pineight.com/nes/volume_tests.zip) | tepples | Plays tones on all the APU's channels to show their relative volumes at various settings of $4011. Package includes a recording from an NES's audio output for comparison. |   
  
## Mapper-specific Tests

Name  | Author  | Description  | Resources   
---|---|---|---  
[31_test](https://forums.nesdev.org/download/file.php?id=3772) | rainwarrior | Tests for mapper 31 (with various NES 2.0 headers for PRG ROM sizes) | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=13120)  
[BNTest](https://forums.nesdev.org/download/file.php?id=2253) | tepples | Tests how many PRG banks are reachable in BxROM and AxROM. | [thread](https://forums.nesdev.org/viewtopic.php?p=79826#p79826)  
[bxrom_512k_test](https://forums.nesdev.org/download/file.php?id=2247) | rainwarrior | Similar to the BxROM test in BNTest above. | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=12085)  
[exram](http://www.qmtpro.com/~nes/demos/mmc5exram.zip) | Quietust | MMC5 exram test |   
[fme7acktest-r1](https://forums.nesdev.org/download/file.php?id=2759) | tepples | Checks some IRQ acknowledgment behiaviors of Sunsoft FME-7 that emulators were getting wrong in 2015. | [thread](https://forums.nesdev.org/viewtopic.php?f=2&t=12436)  
[fme7ramtest-r1](https://forums.nesdev.org/download/file.php?id=2800) | tepples | Checks how much work ram the Sunsoft FME-7 can access | [thread](https://forums.nesdev.org/viewtopic.php?f=9&t=12467)  
[Holy Diver Batman](https://pineight.com/nes/holydiverbatman-bin-0.01.7z) | tepples | Detects over a dozen mappers and verifies that all PRG ROM and CHR ROM banks are reachable, that PRG RAM and CHR RAM can be written and read back without error, and that nametable mirroring, IRQ, and WRAM protection work. | [thread](https://forums.nesdev.org/viewtopic.php?f=22&t=10640)  
[mmc3bigchrram](https://forums.nesdev.org/download/file.php?id=5166) | tepples | MMC3 test for large 32kb CHR RAM with NES 2.0 headers | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=13890)  
[mmc3_test](http://blargg.8bitalley.com/parodius/nes-tests/mmc3_test_2.zip) | blargg | MMC3 scanline counter and IRQ generation tests. |   
[serom](https://forums.nesdev.org/download/file.php?id=3753) | lidnariq | Tests the constraints of SEROM / SHROM / SH1ROM variations of the MMC1 boards. | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=9350&start=90#p153298)  
Submapper Tests | rainwarrior | [2_test](https://forums.nesdev.org/download/file.php?id=4299) \- Mapper 2, Submappers 0, 1 and 2 | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=9350&start=90#p157804)  
rainwarrior | [3_test](https://forums.nesdev.org/download/file.php?id=4302) \- Mapper 3, Submappers 0, 1 and 2 | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=9350&start=90#p154555)  
rainwarrior | [7_test](https://forums.nesdev.org/download/file.php?id=4301) \- Mapper 7, Submappers 0, 1 and 2 | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=9350&start=90#p157804)  
rainwarrior | [34_test](https://forums.nesdev.org/download/file.php?id=3771) \- Mapper 34, Submappers 1 and 2 | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=9350&start=90#p153334)  
[test28](https://forums.nesdev.org/download/file.php?id=158) | tepples | Tests for mapper 28 | [thread](https://forums.nesdev.org/viewtopic.php?p=102693#p102693)  
[vrc6test](http://www.mediafire.com/download/6hvuj53omv8y3fn/vrc6test.zip) | natt | VRC6 mirroring tests | [thread](https://forums.nesdev.org/viewtopic.php?t=11028)  
  
## Input Tests

Name  | Author  | Description  | Resources   
---|---|---|---  
[allpads](https://github.com/pinobatch/allpads-nes/raw/master/allpads.nes) | tepples | Multiple controller test supporting NES controller, Super NES controller, Famicom microphone, Four Score, Zapper, NES Arkanoid controller, and Super NES Mouse | [thread](https://forums.nesdev.org/viewtopic.php?f=2&t=12549) [GitHub](https://github.com/pinobatch/allpads-nes)  
[dma_sync_test_v2](https://forums.nesdev.org/download/file.php?id=5905) | Rahsennor | Tests DMC DMA read corruption | [thread](https://forums.nesdev.org/viewtopic.php?f=2&t=14319)  
mousetest |  | Test for a Super Famicom mouse wired in a Famicom expansion port as controller 3 |   
[PaddleTest3](https://github.com/christopherpow/nes-test-roms/raw/master/PaddleTest3/PaddleTest.nes) | 3gengames | Test for the Arkanoid controller | [thread](https://forums.nesdev.org/viewtopic.php?t=7929)  
[read_joy3](http://blargg.8bitalley.com/parodius/nes-code/read_joy3.zip) | blargg | Various NES controllers tests, including read corruption due to DMC DMA | [thread](https://forums.nesdev.org/viewtopic.php?f=2&t=4124&start=0)  
[Zap Ruder](https://github.com/pinobatch/zap-ruder/raw/master/ruder.nes) | tepples | Test for the Zapper, including dual wield but not the serial Vs. variant | [thread](https://forums.nesdev.org/viewtopic.php?t=8108) [GitHub](https://github.com/pinobatch/zap-ruder)  
[spadtest-nes](https://forums.nesdev.org/download/file.php?id=5356) | tepples | Super Nintendo controller test (when connected to a NES) | [thread](https://forums.nesdev.org/viewtopic.php?p=167288)  
[vaus_test](http://pics.pineight.com/nes/vaus-test-0.02.zip) | tepples | Another test for the Arkanoid controller | [thread](https://forums.nesdev.org/viewtopic.php?p=120455)  
  
## Misc Tests

Name  | Author  | Description  | Resources   
---|---|---|---  
[characterize-vs](https://forums.nesdev.org/download/file.php?id=1415) | lidnariq | VS System tests | [thread](https://forums.nesdev.org/viewtopic.php?f=3&t=10276&start=15#p127592)  
[NEStress](http://nesdev.org/NEStress.zip) |  | Old test - some of the tests are supposed to fail on real hardware. |   
[oc-r1a](https://forums.nesdev.org/download/file.php?id=2826) | tepples | Detects and displays accurate clock rate of the NES | [thread](https://forums.nesdev.org/viewtopic.php?f=22&t=12499)
