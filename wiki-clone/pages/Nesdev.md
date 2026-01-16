# Nesdev

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Nesdev) | View [other pages](Special_AllPages.xhtml#Nesdev)

[![Nesdev1 Tokumaru.png](../wiki-images/Nesdev1_Tokumaru.png)](File_Nesdev1_Tokumaru_png.xhtml)  


  
This is a historical representation of the old NESDev main page.  
All of this material is out of date and unmaintained.  


Please visit the [Wiki main page](Nesdev_Wiki.xhtml "Nesdev Wiki") for current information.

## Contents

  * 1 Notices
  * 2 Text files
    * 2.1 NES
    * 2.2 6502
    * 2.3 Famicom Disk System
    * 2.4 Etc.
    * 2.5 Emulation
    * 2.6 Dr. PC Jr.
    * 2.7 SNES
  * 3 NES Programs
  * 4 NES Development Tools
    * 4.1 PC Programs
      * 4.1.1 6502 Tools
      * 4.1.2 Graphic Tools
      * 4.1.3 Misc.
      * 4.1.4 Sound Tools
    * 4.2 Amiga Programs
      * 4.2.1 6502 Tools
      * 4.2.2 Graphic Tools
      * 4.2.3 Misc tools
  * 5 NES Hardware Information
    * 5.1 console hardware
    * 5.2 cartridge hardware
  * 6 NES Hardware Projects
  * 7 Links
    * 7.1 Music
    * 7.2 NES-related
    * 7.3 misc. tech
    * 7.4 home pages
    * 7.5 6502
    * 7.6 misc 6502



## Notices

  * **Do not download full copies of the site through the webserver. Use the[FTP mirror](ftp://ftp.parodius.com/pub/nesdev/nesdev_weekly.zip)** or the ED2K mirror
  * [The messageboards](http://forums.nesdev.org/) are open. The [old boards](http://nesdev.org/cgi-bin/wwwthreads/wwwthreads.pl) are still readable.
  * Consider making a donation of any amount to help pay for hosting costs (which includes every site hosted on Parodius). The paypal link will be found on [Parodius' main page](http://www.parodius.co).
  * An NES dev cartridge is in production by Membler Industries. More details to come.

[![Mbar.gif](../wiki-images/Mbar.gif)](File_Mbar_gif.xhtml)

## Text files

### NES

General information about the Nintendo Entertainment System, or Famicom. 

  * [Nintendo Entertainment System Documentation](http://nesdev.org/NESDoc.pdf) v1.0 by Patrick Diskin.
  * [NinTech](http://nesdev.org/NinTech.txt) by Blue Hawk.
  * [2A03 technical reference](http://nesdev.org/2A03%20technical%20reference.txt) First release (4-23-2004), by Brad Taylor. Covers everything related to the NES's CPU, including sound. The linear counter section needs to be written, if anyone experienced with this is interested in documenting it, please post on the forum.
  * [NTSC 2C02 technical reference](http://nesdev.org/2C02%20technical%20reference.TXT) First release (4-23-2004), by Brad Taylor. Lots of information about the NTSC version of the NES PPU.
  * [NES APU Sound Hardware Reference](http://nesdev.org/apu_ref.txt) 2004.1.30 by Blargg
  * [NES 101](http://nesdev.org/NES101.zip) by Michael Martin. An NES programming tutorial for those who know 6502 assembly.
  * [Programming that 8-bit beast of power, the NES](http://nesdev.org/NESprgmn.txt) v.80.666 by joker21@earthlink.net.
  * [NES Technical FAQ](http://nesdev.org/NESTechFAQ.htm) v1.4 by Chris Covell.
  * [NES Tech doc (French)](http://nesdev.org/nesdoc2.txt) by Crispysix.
  * [Nintendo Entertainment System documentation](http://nesdev.org/ndox200.zip) v2.0 by Jeremy Chadwick.
  * [Nintendo Entertainment System documentation (Chinese)](http://nesdev.org/nestech_cn.txt) v2.0 by Jeremy Chadwick, translation by Blue Potato.
  * [Nintendo Entertainment System documentation (Dutch)](http://nesdev.org/nestechd.txt) v2.0 by Jeremy Chadwick, translation by A.A.J. Bouwmans.
  * [Nintendo Entertainment System documentation (Japanese)](http://nesdev.org/NESTECJ.TXT) v.53 by Jeremy Chadwick, translation by (?).
  * [NES System Architecture](http://fms.komkon.org/EMUL8/NES.html) v2.4 by Marat Fayzullin.
  * [NES System Architecture (Russian)](http://dendy.city.tomsk.net/arhit.htm) v1.4 by Marat Fayzullin, partial translation by (?)
  * [NES System Architecture (Chinese)](http://nesdev.org/nes_c.txt) v2.2 by Marat Fayzullin, translation by Wu Jian.
  * [NES System Architecture (Japanese)](http://nesdev.org/NES_J.TXT) v1.4 by Marat Fayzullin, translation by Bero.
  * [NES Programming Guide](http://nesdev.org/nesguide.zip) v.4 by Bokudono and Waseiyakusha.
  * [The Skinny on NES Scrolling](http://nesdev.org/loopyppu.zip) by loopy.
  * [The Skinny on NES Scrolling (Spanish)](http://nesdev.org/nesscroll-sp.pdf) by loopy, translated by Hyde.
  * [The Skinny on NES Scrolling (Portuguese)](http://nesdev.org/nesscroll-pt.pdf) by loopy, translated by Hyde.
  * [Detailed DMC Operation](http://blargg.8bitalley.com/nes-emu/dmc/) by blargg. Information about how the DMA uses the sample buffer. Off-site link.
  * [How NES Graphics Work](http://nesdev.org/nesgfx.txt) A document describing the basics.
  * [How NES Graphics Work (Dutch)](http://nesdev.org/nesgfx-d.txt) A document describing the basics. Translation by A.A.J.Bouwmans.



### 6502

Information about the 6502. The NES's 2A03 CPU is a modified 6502. 

  * [Assembleur sur NES (French)](http://nesdev.org/nesdoc1.txt) v0.9 by Crispysix.
  * [Rockwell 650x and 651x specs](http://nesdev.org/r650x.zip) Official specifications for the 6502 family.
  * [6502 Microprocessor info](http://nesdev.org/6502.txt) Excerpt from the Commodore 64 Programmers Reference Manual.
  * [6502 Microprocessor info (Chinese)](http://nesdev.org/6502_cn.txt) From the Commodore 64 Progammers Reference Manual, translation by Blue Potato.
  * [NMOS 65xx Instruction Set](http://nesdev.org/6502_cpu.txt) From a C64 emulator and devkit.
  * [Assembly in one step](http://nesdev.org/6502guid.txt) A brief guide to programming the 6502 in assembly language.
  * [6502 Instruction Summary](http://nesdev.org/6502jsm.zip) A list of opcodes and addressing modes. Originally by RTK, modified by CricketNE
  * [6502 Opcodes](http://nesdev.org/opcodes.txt) In japanese. By famtasiaに添付されてたと思った (?)
  * [6502 bugs](http://nesdev.org/6502bugs.txt) A list of known bugs in the 6502.
  * [mathpac](http://nesdev.org/mathv1r_lib.txt) by Matt Malone. Some math routines in 6502.
  * [DR6502 Docs](http://nesdev.org/dr6502-docs.zip) by M.J.Malone. This documentation for a 6502 simuator contains extensive 6502 information.
  * [Undocumented Opcodes](http://nesdev.org/undocumented_opcodes.txt) v3.0 by Freddy Offenga. Based on the Atari 8-bit 6502.
  * [Extra Instructions of the 65XX Series CPU](http://nesdev.org/extra_instructions.txt) 11-27-96 by Adam Vardy.
  * [Circuit-level 6502 description (russian)](http://wiki.breaknes.com/6502) ([google translate](http://translate.google.ru/translate?sl=ru&tl=en&u=http%3A%2F%2Fwiki.breaknes.com%2F6502&act=url))



### Famicom Disk System

Information about the Famicom's floppy disk add-on. 

  * [Famicom Disk System technical reference](http://nesdev.org/FDS%20technical%20reference.txt) 3rd release (4-23-2004), by Brad Taylor. Comprehensive FDS technical docs.
  * [Famicom Disk System Loader](http://nesdev.org/fdsloadr.zip) by Brad Taylor. This is the software and documentation for a hardware project to emulate the FDS's disk drive using a PC disk drive via parallel port. Can also copy FDS disk contents to your PC.
  * [FDS Copy Tool](http://nesdev.org/fdscopy.zip) info from a book in Japanese. Contributed by Tomy.
  * [FDS copy tool 2](http://nesdev.org/fds-copytool2.zip) Japanese docs, scanned by Tomy.
  * [Nintendo Disksystem Sound](http://nesdev.org/FDSSOUND_v1.1.txt) v1.1 by Norix. In Japanese.
  * [FDS Sound](http://nesdev.org/FDS.txt) 07/14/2004 by Disch.
  * [Famicom Disk System info](http://nesdev.org/fds-nori.txt) by Nori. (in Japanese)
  * [Famicom Disk System](http://nesdev.org/fds-e.txt) by Nori, translated by Ki.
  * [Famicom Disk System info](http://nesdev.org/disksystem.txt) by goroh.
  * [Famicom Disk System info](http://nesdev.org/diskspec.txt) by goroh, translated by Sgt. Bowhack.
  * [Disk Info](http://nesdev.org/diskinfo.txt)



### Etc.

Information about the NES/Famicom's add-ons, or anything that doesn't pertain to another category. 

  * [SH657X Toolkit](http://nesdev.org/SH657XTool_release.zip) A huge amount of info, tools, and demos, for an NES clone system using the 657x IC.
  * [NES to Famicom Adapter](http://nesdev.org/NES_ADAPTER.txt) 72 to 60-pin wiring info.
  * [Connecting NES Zapper to Famicom](http://nesdev.org/zapper_to_famicom.txt) Wiring info.
  * [VS UniSystem Information](http://nesdev.org/vsdoc.txt) v1.0 by Fx3.
  * [Nintendo Playchoice 10 Hardware Description](http://nesdev.org/pc10doc.txt) v0.2 by Oliver Achten.
  * [New Playchoice 10 BIOS](http://nesdev.org/pc10bios.zip) v0.1 by Oliver Achten. Allows the Playchoice 10 arcade machine to run NES carts.
  * [Megaman 1 ROM Tech](http://nesdev.org/megm1rom.txt) v0.13 by AlphaBeta. Info on Megaman 1's ROM data structure.
  * [Famicom Four-Player Adapter](http://nesdev.org/ffpa.txt) v1.0 by Richard Hoelscher. This adapter works differently than the ones for NES.
  * [Excite Boxing](http://nesdev.org/Excite_Boxing-english.txt) by goroh, english translation by Ki.
  * [Family Computer Gun](http://nesdev.org/Light_Gun-english.txt) by goroh, english translation by Ki.
  * [Famicom keyboard info](http://nesdev.org/keyboard.txt) by goroh.
  * [Reverse Engineering the Keyboard of Family Computer](http://nesdev.org/Keyboard-english.txt) by goroh, english translation by Ki.
  * [Mapper Information for Emulation](http://nesdev.org/NESHDRJ.TXT) by ???. In Japanese.
  * [Power Pad information](http://nesdev.org/powerpad.txt) v1.2 by Tennessee Carmel-Veilleux.
  * [Family Trainer](http://nesdev.org/famitra.txt) by goroh.
  * [NESdev Mailing list](http://nesdev.org/nesdevma.zip) An archive of postings from Jan 22 1998, to March 25 1999. Go to [Yahoo Groups](http://gamesource.groups.yahoo.com/group/nesdev/) to subscribe. (No activity since February 2006.)
  * [NES Game Genie Format](http://nesdev.org/nesgg.txt) v.071 by Benzene.
  * [NES 4 Player Adapter Documentation](http://nesdev.org/nes4play.txt) v.9 by Fredrik Olsson.
  * [Marat's doc randomized](http://nesdev.org/random.txt) not of any use, but slightly amusing.



### Emulation

Information related to the emulation of the NES/Famicom. 

  * [NES emulator development guide](http://nesdev.org/NES%20emulator%20development%20guide.txt) 4th release (4-23-2004), by Brad Taylor. Lots of information and techniques about emulating the NES.
  * UNIF File format specification An upcoming new format for NES roms.
  * [iNES Header Format](http://nesdev.org/iNES.txt) march 2000 by rvu.
  * [iNES Header Format](http://nesdev.org/neshdr20.txt) v2.0 by VmprHntrD.
  * [NES Palette](http://nesdev.org/nespalette.zip) v1.1 by merp. NTSC and PAL versions converted from BMF's and Matrixz's NES palettes in photoshop, paint shop pro, and microsoft format.
  * [NES palette generator](http://nesdev.org/kevin_palette.txt) 4-08-00 by Kevin Horton. Written in BASIC.
  * [NES Palette](http://nesdev.org/nespal.txt) by Matt Conte.
  * [NES Palette](http://nesdev.org/pal.txt) by Loopy.
  * [.STA format](http://nesdev.org/staform.txt) NESticle .43 save state format info. By goroh.



### Dr. PC Jr.

The Doctor PC Jr. is a Chinese computer based off the NES/Famicom. Visit [Dr. PC Jr. Development Page](http://mypage.direct.ca/c/ccovell/nesdev.html) for extended information. 

  * [BIOS ROM](http://nesdev.org/DrPCJrBIOS.zip) A dump of the BIOS ROM from the Dr. PC Jr.
  * [DOS disk](http://nesdev.org/DrPCJrDOS.zip) The files from the OS disk of the Dr. PC Jr.
  * [CATalogue](http://nesdev.org/CATalogue.zip) v1.0 by Chris Covell. Uses the 'Cue-Cat'.



### SNES

The well-known sequel to the NES, both castrated and enhanced. Only stuff unavailable elsewhere will be featured here. 

  * [SNESdev starter kit](http://nesdev.org/SNES-starterkit.zip) by neviksti. Includes an assembler, example programs, docs, and tools.
  * [SNES Cartridge Reader](http://nesdev.org/sreader.zip) by Lucas Siudym.
  * [SNES Cart Hacking Info](http://nesdev.org/eprcart4.zip) v0.4 by Lucas Siudym. Includes pin-outs and diagrams showing how to modify SNES cartridges to use EPROMs.
  * [SNES Central Development Page](http://en.wikibooks.org/wiki/Super_NES_Programming/Initialization_Tutorial) Off-site link.
  * [Zophar's Domain - SNES Tech Docs](http://www.zophar.net/documents/snes.html) Offsite link. More SNES stuff.
  * [Zophar's Domain - 65816 Docs](http://www.zophar.net/tech/65816.html) Offsite link. Docs about the SNES's CPU. An enhanced, yet backwards compatible, 16-bit version of the 6502.



## NES Programs

See [Projects](Projects.xhtml "Projects"). 

## NES Development Tools

### PC Programs

#### 6502 Tools

  * [NESICIDE](http://github.com/christopherpow/nesicide) v.0.2 by Christopher S. Pow. The NES "Incredibly Cool" Integrated Development Environment, which aims to encompass the full lifecycle of NES ROM development. Off-site link.
  * [6502 hex-to-mnemonix](http://nesdev.org/6502mnemonix.zip) by juicy_killa@hotmail.com. A convenient reference program for viewing hex codes for 6502 opcodes. Email the author if you have any corrections or suggestions.
  * [FASM](http://nesdev.org/fasm10.zip) v1.0 by Toshi Morita. Quote from the docs: FASM was written as a quick replacement for the 2500 AD assembler for Nintendo 8-bit development. GPL license.
  * [Interactive Disassembler](http://nesdev.org/idafw.zip) freeware version, by DataRescue. The commercial version is better, but it's expensive.
  * NESrev v.013 by Kent Hansen. This disassembler detects the differences between code and data, and aims to produce output that can be reassembled. Works with 16KB programs. [[Java source](http://nesdev.org/NESrev.java)]
  * [nbasic](http://bobrost.com/nes/resources.php#devtools) A high-level programming language for 8-bit NES development, by Bob Rost. Off-site link.
  * [P65 Assembler](http://hkn.berkeley.edu/~mcmartin/P65/) A portable 6502 assembler written in Perl. Off-site link.
  * [WLA DX](http://www.villehelin.com/wla.html) A portable GB-Z80/Z80/6502/6510/65816 macro assembler. Linux and MS-DOS versions available. Off-site link.
  * [CC65](http://www.cc65.org/) A portable 6502/65c02/65c816 assembler, linker, and C compiler. Off-site link.
  * [X816](http://nesdev.org/x816112f.zip) An assembler for 6502/65c816. By minus.
  * [6502 SDK](http://nesdev.org/s6502b10.zip) Quoted from the docs: _The kit is an Integrated Development Environment (IDE) similar to Borland ones, which allows you to edit, compile and (hopefully) debug your Assembly code for 65xx processors. It includes a (still rudimental) emulator and a project manager for multiple file applications._ Requires Win3.11 or higher. Here's the [source code](http://nesdev.org/s6502src.zip).
  * [Nintendo Assembler](http://nesdev.org/nesasm.zip) v.1 by Charles Doty, David Michel, and J.H. Van Ornum.
  * [Minachun Disassembler for 6502 Famicom(NES)](http://nesdev.org/md6502.zip) v.04a. Features Japanese language and mapper support. Source included, with an English language option.
  * [TRaCER](http://nesdev.org/tracer.zip) A disassembler for 6502/65c02/65c816. By [koitsu/Jeremy Chadwick](User_Koitsu.xhtml "User:Koitsu")
  * [6502 Simulator](http://nesdev.org/6502sim.zip) by Dan Boris.
  * [DASM](http://nesdev.org/dasm32.zip) v2.12 by Mathew Dillon
  * [neslisp](http://sf.net/projects/neslisp) v0.0.17 by erana (Commodore 64 native version at <http://sf.net/projects/neslispc64> v0.0.17 and up)
  * [ados-nes](http://sf.net/projects/ados-os-nes) v0.1.13 by erana
  * [arm-lisp](http://sourceforge.net/projects/arm-lisp) v0.2.73 by erana (ARMv7 and 6502 target included.)
  * [Ophis](http://michaelcmartin.github.io/Ophis/) 6502/65C02 assembler for Windows/Mac/Linux by Michael Martin



#### Graphic Tools

  * [NES Screen Tool](https://shiru.untergrund.net/software.shtml) by Shiru.
  * [Tile Molester](http://nesdev.org/tilemolester-0.16.zip) v0.16 by Kent Hansen. This tile editor requires [Java](http://java.sun.com/j2se/1.4.2/download.html), and supports NES as well as other console formats.
  * [YY-Chr](http://www.zophar.net/utilities/graphutil/yy-chr.html) by YY. A multi-format tile editor. Available in Japanese and English.
  * [BMPNES](http://nesdev.org/bmpnes.zip) v1.8 by ninjasuperk. Spanish BMP to NES format converter. Includes some utils by Chis Covell.
  * [Tile Layer Pro](http://nesdev.org/tlp10.zip) v1.0 by SnowBro. Tile editor.
  * [CHR-ROM Creator](http://nesdev.org/CHR_Creator.zip) v1.1 by Stefan Fausser. An NES tile editor. [VB40032.DLL](http://nesdev.org/vb40032.zip) is required.
  * [Open tUME](http://opentume.co.cc/) A 2D level editor designed for game development, check it out!
  * [NES Screen Arranger](http://nesdev.org/nsa01b.zip) Uses NES graphics from a .CHR file and let's you set up the name and attribute tables, with optional RLE compression. By SnowBro.
  * [Tile Layer](http://nesdev.org/tlay050b.zip) v0.50b. Let's you edit graphics from roms for various consoles. By SnowBro.
  * [BMP2NES](http://nesdev.org/bmp2nes.zip) by [7h1460](mailto:q7h1460@hotmail.com). Converts BMP graphics to the NES's format.
  * [RAW2CHR](http://nesdev.org/raw2chrDOS.zip) by Chris Covell. Converts graphics from RAW to the NES's format.
  * [CHARlie](http://nesdev.org/charlieDOS.zip) by Chris Covell. Optimizes graphics by removing redundant tiles.
  * [CHR2NAM](http://nesdev.org/chr2namDOS.zip) by Chris Covell. Creates a nametable from an image file.



#### Misc.

  * [uCON64](http://ucon64.sourceforge.net/) (offsite link) ROM management utility. Can convert between file formats, split iNES images into PRG and CHR ROMs (use Pasofami format to do this), and much more.
  * [Game Genie Code Coverter](http://nesdev.org/ggconv.zip) v4.0 by Zazer. Create new, or decode existing Game Genie codes for NES and all other GG-platforms.
  * [Roller Coaster](http://nesdev.org/rollcost.zip) by Pan/ATX. This program can generate various sine and non-repeating random data tables to use in your games/demos.
  * [Famicom Disk System Loader](http://nesdev.org/fdsloadr.zip) by Brad Taylor. This is the software and documentation for a hardware project to emulate the FDS's disk drive using a PC disk drive via parallel port. Can also copy FDS disk contents to your PC.
  * [Legacy of the Wizard map viewer](http://nesdev.org/legwizar.rar) by Brad Taylor. Programs to view the maps from this game. Includes a bitmap graphic of the entire map.
  * [FDSList](http://nesdev.org/FDSListWIN.zip) v1.2 by Chris Covell. Lists and extracts files from Famicom disk images. Supports FDS format.
  * [Decrom](http://nesdev.org/decrom20.zip) v2.0 by Fx3. Converts VROM graphics into ascii.
  * [VRC-VII Emulator](http://nesdev.org/vrc7test.zip) By Kevin Horton. VRC7 is a Konami mapper/sound-chip, with FM synthesis.
  * [VRC-VI Emulator](http://nesdev.org/vrcvibas.zip) Written in qbasic, by Kevin Horton.
  * [Hexposure](http://nesdev.org/hexpo215.zip) v0.215. A Hex editor by SnowBro.



#### Sound Tools

  * [Nerdtracker 2](http://nesdev.org/nt2/) beta version by Bananmos. A DOS/Win9X NES music tracker. The NT2 site also has replay source code for NES, an assortment of songs made by NT2 users, and more.
  * [Nijuu](http://dutycyclegenerator.com/nijuu/nijuu.html) v0.1b by Neil Baldwin. A NES music engine that converts tracks defined in text files into NES or NSF files. Off-site link.
  * [MCK](http://www.geocities.co.jp/Playtown-Denei/9628/) by Izumi. A sound driver using MML format. Off-site link.
  * [NED2NSF](http://nesdev.org/ned2nsf10.zip) v1.0 by Matrixz. Windows program that can make a single-song NSF from a NED file (Nerdtracker 2 format). Does not support NEDs with DMC samples, currently.
  * [MIDIMML converter](http://nesdev.org/midimml1.0.zip) v1.0 by Tom Murphy 7. [[link](http://www-2.cs.cmu.edu/%7Etom7/midimml/)] This program converts MIDI files (with some special annotations) into MML. The output wouldn't be as optimal as hand-crafted MML (in most cases), but it's very interesting and even useful for MIDI users, nonetheless.
  * [MCKC: MCK > MML Converter](http://nesdev.org/mckc-e.txt) by Manbow-J, translated by virt. This document shows you how to use MML to create NES music.
  * [MCK/MML Beginners Guide](http://nesdev.org/mck_guide_v1.0.txt) v1.0 by Nullsleep/8bitpeoples.
  * [MCK/MML Beginners Guide (Chinese)](http://nesdev.org/mck-mml%20Beginners%20Guide%20v1.0_cn.txt) v1.0 by Nullsleep, translated by Tong Yun Weng.
  * [DPCM HowTo](http://nesdev.org/DPCM_HowTo.txt) by nullsleep. How to use DPCM samples with MCK.
  * [Making NSFs under UNIX](http://wiw.org/%7Etek/muzak/#info) by Julian Squires. Info concerning MCK. Off-site link.
  * [nsf2midi (Japanese)](http://nesdev.org/nsf2midi0130.lzh) v.130 by GIGO. [NSF2MIDI (english)](http://nesdev.org/nsf2midi005aE.zip) v.05a Translated by Tatt and Yura.
  * [WAV2NES](http://nesdev.org/wav2nes.zip) by David de Regt. Converts a .wav file to an .NES rom, but uses an obscure mapper.
  * [dmc converter (Japanese)](http://nesdev.org/dmcconv005.zip) v.05 by Norix. [english doc](http://nesdev.org/DPCM_HowTo.txt) by Nullsleep.
  * [sample converters](http://nesdev.org/81.zip) by Bananmos. Includes 8bit-to-1bit and 1bit-to-8bit programs.
  * [sample converter](http://nesdev.org/81v2.zip) by Damian Yerrick. Converts 8-bit samples to NES's 1-bit format, also scales the volume and oversamples the sound.
  * [NES Sound Driver & Library (NSD.Lib)](http://shaw.la.coocan.jp/nsdl/) by S.W. A sound driver. This include the library for cc65/ca65 and MML compiler(This can make NSF and assembler source). ([GitHub](https://github.com/Shaw02/nsdlib/))



### Amiga Programs

#### 6502 Tools

  * [WLA DX](http://www.villehelin.com/wla.html) A GB-Z80/Z80/6502/6510/65816 macro assembler.
  * [DASM v2.0 by Mathew Dillon. Source code is included. [http://nesdev.org/dasm32.zip v.212](http://nesdev.org/dasm.zip) is also available, but does not include an Amiga executable.



#### Graphic Tools

  * [Raw2CHR](http://nesdev.org/raw2chr.zip) by Chris Covell. Converts graphics from RAW to the NES's format. C source code is included.
  * [CHARlie](http://nesdev.org/charlie.zip) by Chris Covell. Optimizes graphics in CHR roms by removing duplicate tiles. C source code is included.
  * [CHR2NAM](http://nesdev.org/chr2nam.zip) by Chris Covell. Makes a nametable from your CHR rom. C source code is included.



#### Misc tools

  * [FDSList](http://nesdev.org/FDSList.zip) v1.2 by Chris Covell. Lists and extracts files from Famicom disk images. Supports FDS format.
  * [AmiGenie](http://nesdev.org/AmiGenie.lha) by Chris Covell. Converts between Game Genie codes and hex addresses.



## NES Hardware Information

  * Index of US [patents](Patents.xhtml "Patents") related to the NES/Famicom.



### console hardware

  * [NES hardware development guide](http://nesdev.org/NES%20hardware%20development%20guide.txt) First release (4-23-2004), by Brad Taylor. Detailed NES hardware info, and several project ideas.
  * [Family Computer schematic](http://nesdev.org/Ntd_8bit.jpg) by Nintendo? File provided by Ubaldo Prones.
  * [Nintendo Family Computer 回路の解析](http://nesdev.org/famicom_kairo.txt) by goroh.
  * [Nintendo Family Computer circuitry RE](http://nesdev.org/famicom-circuitry.txt) by goroh, english translation by Ki.
  * [2A03 Pinout](http://nesdev.org/2A03_pinout.txt) by Eli Dayan.
  * [RP2C02G (aka: the NES's PPU)](http://nesdev.org/nes_ppu.txt) by Kevin Horton.
  * [NES Audio Path](http://nesdev.org/NESAudio.gif) by Kevin Horton.
  * [Electronic Sound Synthesizer](http://nesdev.org/4783812-sound_synth.pdf) A patent for the Famicom Disk System's sound hardware.
  * [Digital Sound Apperatus and External Memory Cartridge Used Therefor"](http://nesdev.org/5317714-digital_sound.pdf) A patent for the NES's sample playing hardware.
  * [74LS373N](http://nesdev.org/74HCT373.pdf) Octal D-Type transparent latch; 3-state ([HTML page](http://www.kingswood-consulting.co.uk/giicm/74300.html#74373))
  * [74HCU04P](http://nesdev.org/74HC_HCT04_CNV_2.pdf) Hex inverter ([HTML page](http://www.kingswood-consulting.co.uk/giicm/7400.html#7404))
  * [74HC368](http://nesdev.org/74HC_HCT368_CNV_2.pdf) Hex buffer/line driver; 3-state; inverting ([HTML page](http://www.kingswood-consulting.co.uk/giicm/74300.html#74368))
  * [74LS139](http://nesdev.org/74HC_HCT139_CNV_2.pdf) Dual 2-to-4 line decoder/demultiplexer ([HTML page](http://www.kingswood-consulting.co.uk/giicm/74100.html#74139))
  * [Disabling the NES's lockout chip](http://nesdev.org/nlockout.txt) rev 0.4 by Mark Knibbs.
  * [Playchoice 10 Schematics](http://nesdev.org/Playchoice.pdf) 1986 Nintendo of America Inc.
  * [VS UniSystem Kit Manual](http://nesdev.org/VS_UniSystem.pdf) by Nintendo of America Inc. Instructions on changing the game in an NES arcade cabinet, part list, etc.
  * [VS UniSystem Wiring Diagram](http://nesdev.org/VS_Wiring.pdf) 1984/1985 by Nintendo of America Inc.
  * [VS UniSystem Schematic](http://nesdev.org/VSSCHEM.pdf) by Nintendo Co. Ltd.
  * ["NES on a chip" pinout](http://nesdev.org/nes-on-a-chip.txt) v1.0 by Kevin Horton.
  * [NT6578 NES "on a chip" pinout](http://nesdev.org/6578_pinout.pdf) by Ubaldo Prones.



### cartridge hardware

_Note: Goroh's docs are in japanese_

  * [NES ROM Pinouts](http://nesdev.org/NES%20ROM%20Pinouts.txt) by Drk. Covers all PRG, CHR, and RAM chips used in NES cartridges.
  * [NES EPROM Conversions](http://nesdev.org/NES%20EPROM%20Conversions.txt) by Drk. Instructions on how to modify certain boards to use EPROMs.
  * [EPROM Pinouts](http://nesdev.org/EPROM%20Pinouts.txt) by Drk.
  * [Famicom Cartridge Connector Pinout](http://nesdev.org/fam_pinout.txt) by Siudym. [DOC version](http://nesdev.org/fam_pinout.doc) available.
  * [Super Mario Bros. 2 Pinout](http://nesdev.org/smb2_pinout.txt) by Siudym. Board is NES-TSROM-07.
  * [Super Mario Bros. 3 Pinout](http://nesdev.org/smb3_pinout.txt) by Siudym. Board is NES-TSROM-08.
  * [Solstice Pinout](http://nesdev.org/solo_pinout.txt) by Siudym. Board is NES-ANROM-XX.
  * [Wizards and Warriors 2: Ironsword Pinout](http://nesdev.org/ww2_pinout.txt) by Siudym. Board is NES-AOROM-03.
  * [Donkey Kong Classics Pinout](http://nesdev.org/dkc_pinout.txt) by Siudym. Board is NES-CNROM-07. Comments in Polish.
  * [goroh's docs 1](http://nesdev.org/goroh1.zip) by goroh, translated by Sgt. Bowhack.
  * [goroh's docs 2](http://nesdev.org/goroh2.zip) by goroh, translated by Sgt. Bowhack.
  * [NES Cart Types](http://nesdev.org/rom.txt) by Kevin Horton.
  * [Namcot 106](http://nesdev.org/namco106.txt) by goroh, fix by ZW4 and nori, english translation by nori.
  * [Preliminary Maxi-15 Mapper Hardware Description](http://nesdev.org/maxi15.txt) by Mark.
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) v0.80 by \Firebug\\. Includes viewer program by Troy McLeod.
  * [Mapper 90 Information](http://nesdev.org/map90v20.txt) v2.0 by Fx3.
  * [Caltron / Myriad Games 6-in-1](http://nesdev.org/caltron.txt) v1.0 by The Mad Dumper.
  * [Konami VRC-VII Chip Info](http://nesdev.org/vrcvii.txt) by Kevin Horton.
  * [Konami VRC-VI Chip Info](http://nesdev.org/vrcvi.txt) by Kevin Horton.
  * [Konami VRC-VI](http://nesdev.org/vrc6-j.txt) by goroh. Sound info is inaccurate.
  * [Nintendo MMC1](http://nesdev.org/mmc1.txt) by Matthew J. Richey.
  * [Nintendo MMC2](http://nesdev.org/mmc2.txt) 01/29/98 by Jim Geffre.
  * [Nintendo MMC3](http://nesdev.org/mmc3.txt) by goroh.
  * [Nintendo MMC4](http://nesdev.org/mmc4.txt) by ???
  * [Nintendo MMC5 Bankswitching](http://nesdev.org/mmc5_bank_switch.txt) by Kevin Horton.
  * [Nintendo MMC5](http://nesdev.org/mmc5-e.txt) by goroh, translated by Sgt. Bowhack.
  * [Mapper 6](http://nesdev.org/mapper6.txt) Info on the FFE mapper. By FanWen Yang.
  * [Bandai Mapper](http://nesdev.org/16.txt) by goroh.
  * [Bandai Mapper](http://nesdev.org/bandai-e.txt) About Bandai's NES mapper. By goroh, translated by Sgt. Bowhack.
  * [Jaleco Mapper](http://nesdev.org/jaleco.txt) by goroh.
  * [Konami Mapper](http://nesdev.org/konami-e.txt) by goroh, translated by Sgt. Bowhack. [Here](http://nesdev.org/konami-j.txt) is the japanese version.
  * [Taito Mapper](http://nesdev.org/taito-j.txt) by goroh.
  * [Namco Mapper](http://nesdev.org/namco.txt) by goroh.
  * [Sunsoft Mapper](http://nesdev.org/sunsoft.txt) by goroh, translated by Sgt. Bowhack.
  * [SMB2j aka "The Lost Levels" Mapper #40 Info](http://nesdev.org/40.txt) 2.08.2000 by The Mad Dumper.
  * [Pirate game "Mario Baby" Mapper #42 Info](http://nesdev.org/42.txt) 5.29.2000 by The Mad Dumper.
  * [iNES mapper 225](http://nesdev.org/225.txt) by ???
  * [iNES mapper 226](http://nesdev.org/226.txt) by Mark.
  * ["Mario Party" 7 in 1](http://nesdev.org/Mari7in1.txt) by The Mad Dumper.



## NES Hardware Projects

  * [PC NES transfer cable](http://nesdev.org/lptnes.zip) by sepi. Connects a PC's parallel port to the NES control port. It allows you control a game with a PC keyboard to log and play back button presses.
  * [Famicom Disk System Loader](http://nesdev.org/fdsloadr.zip) by Brad Taylor. This is the software and documentation for a hardware project to emulate the FDS's disk drive using a PC disk drive via parallel port. Can also copy FDS disk contents to your PC.
  * [CopyNES](http://www.tripoint.org/kevtris/Projects/copynes/) by Kevin Horton. NES modification that allows reading and writing of cartridges.
  * [ENIO](http://enio.chykn.com/) by Pete Brown. Goal is to give the NES ethernet access to a game server over the Internet.
  * [Breaking NES](http://breaknes.com) by org. NES chips exploration down to silicon level.
  * [Battery holder](Battery_holder.xhtml "Battery holder") Processes to replace NES cartridge batteries.



## Links

### Music

  * [NES Music](http://nesdev.org/music/) NES music in module format.
  * [Sounds of the 2A03](http://nesdev.org/2A03/) New, original NES music!
  * [NES Music Ripping Guide](http://nesdev.org/NESAudioRipping.zip) v1.4 by Chris Covell.
  * [The NED File Format](http://nesdev.org/nt2re.zip) by Damian Yerrick. A reverse-engineering of NerdTracker 2 file-format.
  * [Sound test codes](http://nesdev.org/sound.txt) by me. If you know one that isn't in there, tell [me](mailto:5010.0951@tcon.net). Thanks to Bananmos for getting this back to me after my harddisk died.
  * [NES Music Author List](http://nesdev.org/authors.htm) v3.11 Information from various sources, compiled by Memblers. A never-ending research project that needs your [contributions](mailto:5010.0951@tcon.net).
  * [NES Music Author List](http://fripper.fc2web.com/nes_composer.html) Japanese translation by HAS. Off-site link.
  * [Skate or Die 2 (title theme)](http://nesdev.org/sk8ordie.mp3) Recorded by Memblers, through the audio output of an NES. This Rob Hubbard tune uses raw PCM output ($4011) that few (if any) emulators play correctly.
  * [Comic Bakery](http://nesdev.org/comic_bakery.mp3) Recorded by Chris Covell. This is an MP3 of how this cover of a Martin Galway song plays on a real NES (The one in the Stars SE demo). It doesn't sound too good, due to some scarcely documented um.. features of the NES's sound hardware. Since it does a great job of highlighting these features, it may be be useful for emulator authors who want to improve their sound emulation accuracy. This MP3 is in stereo, the square waves are in the left channel, the triangle is in the right one. The noise channel isn't used. If you're wondering, these features were compensated for in Bananmos's newest sound code (as used in Solar Wars), and it is now 100% cool on real NES. =)
  * [2A03.org](http://2a03.org/) An archive of new NES music in NSF format.
  * [NSF Collection](http://www.tripoint.org/kevtris/nes/nsf.html) Here you can get a sorted collection of NSFs, the only bad thing being that the sound effects are removed.
  * [wayfar.net](http://wayfar.net/) Makers of MIDINES, a MIDI interface cartridge for NES.
  * [VGMIX](http://www.vgmix.com/) A videogame music remix site, with an excellent interface for musicans to add their own works.
  * [VORC](http://www.vorc.org/) A Japanese/English chip and game music news site.
  * [Zophar's Domain NSF page](http://www.zophar.net/nsf/) The most complete NSF archive around.
  * [Chibi-Tech's page](http://www.nanjamonja.com/) Tracked chip music, including NES-style stuff.
  * [Dropoff 7](http://www.parodius.com/%7Ememblers/dropoff7.htm) Original music, plus NES music remixes and a few real NES tunes recorded with analogue effects.
  * [The Minibosses](http://www.minibosses.com/) A band that covered some NES songs, their MP3s available for download here.
  * [OverClocked ReMix](http://www.ocremix.org/) A site with remixes (mostly MP3 format) of music from games of many systems.
  * [The Best Game Music!](http://nesmusic.zophar.net/) Check it out whether you read Japanese or not, it has music and original content. Also the home of the MCK sound driver.
  * [Video Game Jam](http://www.kontek.net/vgjam/) Guitar and bass tablature for game music.
  * [Chiptune.com](http://www.chiptune.com/) Chip music in various formats.



### NES-related

  * [NESdev mailing list](http://www.egroups.com/group/nesdev) A mailing list for NES programmers.
  * [NESdev messageboard](http://forums.nesdev.org/) The messageboard for this site.
  * [NES HQ](http://www.neshq.com/) A site with a bit of hardware info, a few cart/box scans, and other stuff.
  * [Hardware Console Design](http://graphics.stanford.edu/%7Eianbuck/proj/Nintendo/) NES Hardware simulator done in VHDL.
  * [NESp](http://www.classicgaming.com/nestable/index.shtml) Website for a portable NES project.
  * [Dr. PC Jr. Development Page](http://mypage.direct.ca/c/ccovell/nesdev.html) A site with technical information and support files for the Dr. PC Jr. (A Chinese computer based on the NES/Famicom)
  * [Nintendo, America!](http://www.valesh.com/%7Ejon/computers/nintendo.html) A page by Jon Valesh, who worked at Color Dreams.
  * [NES World](http://nesworld.com/) An excellent NES site.
  * [tsr's NES archive](http://atarihq.com/tsr/) Another good NES site.



### misc. tech

  * [RetroZone](http://www.retrousb.com/) Manufacturer of the [PowerPak](PowerPak.xhtml "PowerPak") device for NES and SNES, controller adapters, and custom cartridges.
  * [Tototek](http://www.tototek.com/) A supplier of FlashROM-based cartridges for various systems, among other things.
  * [RomLabratory](http://www.romlab.prv.pl/) Tons of tech info about cartridges for NES/FC, SNES/SFC, and Sega Genesis/MegaDrive.
  * [SMS Flash devcart and Motherboard Flash Method](http://smsflash.8m.com/) How to program a Flash-ROM with a PC motherboard, and SMS devcart instructions.
  * [Cuttle Cart](http://www.schells.com/cuttlecart.shtml) A RAM cartridge for 2600 with bankswitching support. Loads a ROM through audio input.
  * [SMS Power](http://www.smspower.org/) A page with some tech info, demos and sources and stuff for the Sega Master System.
  * [GamesX](http://www.gamesx.com/) A page with technical info on various consoles.
  * [Atari 2600 Programming Page](http://www.io.com/%7Enickb/atariprg.htm) You'd have to be a maniac to program this console.
  * [Atari 7800 Developer's Page](http://quadrun.tripod.com/) Info on building a devkit/copier for 2600/7800.



### home pages

  * [Game Development for the 8-bit NES](http://bobrost.com/nes/) Bob Rost's site. Lots of interesting NES projects.
  * [NES Sound Emulation](http://blargg.8bitalley.com/nes-emu/) Blargg's site.
  * [Siudym's Homepage](http://www.siudyms.prv.pl/) Tech info, and other stuff. In Polish and English.
  * [Chewbone Software](http://chewbonesoftware.cjb.net/)
  * [Strangulation Games](http://www.parodius.com/%7Ememblers/games/) Unlicensed NES game developer.
  * [Damian Yerrick's page](http://pineight.com/) NES and PC games and demos.
  * [Chris Covell's page](http://www.disgruntleddesigner.com/chrisc/) Some interesting stuff, and even an explanation of how to convert your NES to have stereo sound!
  * [BlueTech](http://www.tripoint.org/kevtris) Kevin Horton's page. Tech info, pictures and descriptions of some amazing home-made devices.
  * [Mark Knibb's page](http://wwwkenya.freeuk.com/markk/) Some console tech info, plus some Amiga and C64 stuff.
  * [Memblers' page](http://memblers.com/) Info about Dropoff 7 (the band I'm in), and downloadable music in MP3, NSF, and iNES format.



### 6502

  * [6502 Cross-Development Languages and Tools](http://www.npsnet.com/danf/cbm/cross-development.html)
  * [The Fridge](http://www.ffd2.com/fridge/) 6502 source code archive.
  * [Dan Boris's 6502 Page](http://atarihq.com/danb/6502.shtml)
  * [6502.org](http://6502.org/)
  * [MOS 6502 programming manual](http://users.telenet.be/kim1-6502/6502/proman.html)



### misc 6502

  * [Magnified pictures of a 6502](http://micro.magnet.fsu.edu/chipshots/mos/index.html)

[![Mbar.gif](../wiki-images/Mbar.gif)](File_Mbar_gif.xhtml) graphics were ripped from other pages.

Horde soldier: "Surrender, citizen!"  
Bo: (laughs) "I'm not a citizen, I'm a rebel."  
-from the He-Man cartoon, The Secret of the Sword. 
