# FDS disk format

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FDS_disk_format) | View [other pages](Special_AllPages.xhtml#FDS_disk_format)

The [Famicom Disk System](Family_Computer_Disk_System.xhtml "Famicom Disk System") uses disks that are a modified version of Mitsumi Quick Disk hardware, with a custom data format stored on the disk. 

Each side can hold more than 64KB of data. 

For archive and emulation, dumps of these disks are often preserved using the [FDS file format](FDS_file_format.xhtml "FDS file format") (.FDS). 

## Contents

  * 1 FDS Disk Side format
    * 1.1 Gaps
    * 1.2 CRCs
    * 1.3 True disc capacity
  * 2 Block format
    * 2.1 Disk info block (block 1)
      * 2.1.1 Date format
    * 2.2 File amount block (block 2)
    * 2.3 File header block (block 3)
    * 2.4 File data block (block 4)
  * 3 See also
  * 4 References



## FDS Disk Side format

Each disk side must be structured into block as follows : 

1, 2, 3, 4, 3, 4, ...., 3, 4 

The 3, 4 pattern should be repeated once per file present on the disk. 

Blocks type 1 and 2 represent information about the disk and how many files it has, and each block type 3 and 4 pair represents a single file. See _Block format_ below. 

### Gaps

Physically on the disc, there are "gaps" of 0 recorded between blocks and before the start of the disc. Length of the gaps are as follows: 

  * Before the start of the disc : At least 26150 bits, 28300 typical.
  * Gap between blocks : At least 480 bits, 976 bits typical.



Gaps are teminated by a single '1' bit. In terms of bytes, it would be $80, as the data is stored in little endian format. 

### CRCs

At the end of each block, a 16-bit CRC is stored. On loading, the CRC is *not* calculated by the 6502 in the BIOS, but by the RAM adapter, which monitors disc transfers and calculates the CRC. It will automatically send an error code if both CRCs doesn't match. 

The CRC used is the common [CRC-16/KERMIT](https://reveng.sourceforge.io/crc-catalogue/16.htm#crc.cat.crc-16-kermit) algorithm. The '1' bit at the end of the gap is included in the calculation. 

### True disc capacity

In the commonly used [FDS file format](FDS_file_format.xhtml "FDS file format"), its disk image size is limited to 65500 bytes, but does not contain CRCs or gaps. 

The actual disk capacity is somewhat larger, and there are a few variant disk formats that may have even more capacity. 

## Block format

### Disk info block (block 1)

Offset | Length (bytes) | Description | Details   
---|---|---|---  
$00 | 1 | Block code | Raw byte: $01   
$01 | 14 | Disk verification | Literal ASCII string: `*NINTENDO-HVC*`  
Used by BIOS to verify legitimate disk image   
$0F | 1 | Licensee code | See [Licensee codes](Licensee_codes.xhtml "Licensee codes")  
$10 | 3 | Game name | 3-letter ASCII code per game (e.g. ZEL for _The Legend of Zelda_)   
$13 | 1 | Game type | $20 = `" "` — Normal disk  
$45 = `"E"` — Event (e.g. Japanese national [DiskFax](http://ja.wikipedia.org/wiki/%E3%83%95%E3%82%A1%E3%83%9F%E3%83%AA%E3%83%BC%E3%82%B3%E3%83%B3%E3%83%94%E3%83%A5%E3%83%BC%E3%82%BF_%E3%83%87%E3%82%A3%E3%82%B9%E3%82%AF%E3%82%B7%E3%82%B9%E3%83%86%E3%83%A0#.E3.83.87.E3.82.A3.E3.82.B9.E3.82.AF.E3.83.95.E3.82.A1.E3.82.AF.E3.82.B9) tournaments)  
$4A = `"J"` — Unknown. May indicate [Family Computer Network Adapter](Family_Computer_Network_Adapter.xhtml "Family Computer Network Adapter")  
$52 = `"R"` — Reduction in price via advertising   
$14 | 1 | Game version | Starts at $00, increments per revision   
$15 | 1 | Side number | $00 = Side A, $01 = Side B. Single-sided disks use $00   
$16 | 1 | Disk number | First disk is $00, second is $01, etc.   
$17 | 1 | Disk type (FMC) | $01 for FMC blue-disk releases, and $00 otherwise [1]  
$18 | 1 | Unknown | $00 in all known games   
$19 | 1 | Boot read file code | Refers to the file code/file number to load upon boot/start-up   
$1A | 5 | Unknown | Raw bytes: $FF $FF $FF $FF $FF   
$1F | 3 | Manufacturing date | See Date format  
$22 | 1 | Country code | $49 = Japan   
$23 | 1 | Unknown | Raw byte: $61. Speculative: Region code?   
$24 | 1 | Unknown | Raw byte: $00. Speculative: Location/site?   
$25 | 2 | Unknown | Raw bytes: $00 $02   
$27 | 5 | Unknown | Speculative: some kind of game information representation?   
$2C | 3 | "Rewritten disk" date | See Date format. It's speculated this refers to the date the disk was formatted and rewritten by something like a [Disk Writer kiosk](http://www.nintendolife.com/news/2014/03/weirdness_this_footage_of_the_famicom_disk_writer_kiosk_is_a_bit_awesome).  
In the case of an original (non-copied) disk, this should be the same as Manufacturing date  
$2F | 1 | Unknown |   
$30 | 1 | Unknown | Raw byte: $80   
$31 | 2 | Disk Writer serial number |   
$33 | 1 | Unknown | Raw byte: $07   
$34 | 1 | Disk rewrite count | Value stored in [BCD](https://en.wikipedia.org/wiki/Binary-coded_decimal "wikipedia:Binary-coded decimal") format. $00 = Original (no copies).   
$35 | 1 | Actual disk side | $00 = Side A, $01 = Side B   
$36 | 1 | Disk type (other) | $00 = yellow disk, $FF = blue disk, $FE = prototype, sample, or internal-use (white or pink) disk. Some prototype disks have been observed with value $00, but this may indicate the field was simply not filled in.   
$37 | 1 | Disk version | Unknown how this differs from game version ($14). Disk version numbers indicate different software revisions. Speculation is that disk version incremented with each disk received from a licensee.   
$38 | 2 | CRC | (Omitted from .FDS files)   
  
The `*NINTENDO-HVC*` string, stored in ASCII, proves that this is a FDS disk. If the string doesn't match, the BIOS will refuse to read the disk further. 

If the FDS is started with a disk whose side number and disk number aren't both $00, it will be prompted to insert the first disk side. However, some games make this number $00, even for the second disk to make it bootable too. 

All files with IDs smaller or equals to the _boot read file code_ will be loaded when the game is booting. 

#### Date format

All bytes are stored in BCD format, in order of year, month, and day. The year is usually represented as the 1-indexed number of years since the start of the current [Japanese calendar](https://en.wikipedia.org/wiki/Japanese_calendar "wikipedia:Japanese calendar") era, either the Shōwa era (1926) or Heisei era (1989). Values observed are $61-63 for 1986-1988 and $01-03 for 1989-1991. Disk Writer kiosks continued to use Shōwa era dates beyond the end of the Shōwa era, so disks written by the service up to its discontinuation in 2003 may have year values as high as $78. 

A small number of games used the last two digits of the current year instead of a Japanese calendar date. Values observed are $85-$86 for 1985-1986. 

### File amount block (block 2)

This block contains the total number of files recorded on disk. 

Offset | Length (bytes) | Description | Details   
---|---|---|---  
$00 | 1 | Block code | Raw byte: $02   
$01 | 1 | File Amount |   
$02 | 2 | CRC | (Omitted from .FDS files)   
  
More files might exist on the disk, but the BIOS load routine will ignore them, those files are called "hidden" files. Some games have a simple copy protection this way: They have their own loading routine similar to the one from the BIOS but hard-code the file amount to a higher number, which will allow for loading hidden files. This also allows the game to load faster because the BIOS will stop reading the disc after the last non-hidden file. 

### File header block (block 3)

Offset | Length (bytes) | Description | Details   
---|---|---|---  
$00 | 1 | Block code | Raw byte: $03   
$01 | 1 | File Number |   
$02 | 1 | File ID | The ID specified at disk-read function call.   
$03 | 8 | File Name |   
$0B | 2 | File Address | 16-bit little-endian. The destination address when loading.   
$0D | 2 | File Size | 16-bit little-endian.   
$0F | 1 | File Type | 0: Program (PRAM)  
1: Character (CRAM)  
2: Name table (VRAM)   
$10 | 2 | CRC | (Omitted from .FDS files)   
  
The file Number must go in increasing order, first file is 0. File IDs can be freely assigned, and this is the number which will decide which file is loaded from the disk (instead of the file number). An ID smaller than the boot number means the file is a boot file, and will be loaded on first start up. 

File names are uppercase ASCII. 

### File data block (block 4)

Offset | Length (bytes) | Description | Details   
---|---|---|---  
$00 | 1 | Block code | Raw byte: $04   
$01 | \-- | Disk data |   
\-- | 2 | CRC | (Omitted from .FDS files)   
  
## See also

  * [FDS file format](FDS_file_format.xhtml "FDS file format") (**.FDS**)
  * [FDS technical reference.txt](https://nesdev.org/FDS%20technical%20reference.txt) by Brad Taylor
  * [Enri's Famicom Disk System page](http://cmpslv3.stars.ne.jp/Famic/Famdis.htm) (Japanese)
  * [Enri's Famicom Disk System page](https://web.archive.org/web/20091023182159/http://www2.odn.ne.jp/~haf09260/Famic/Famdis.htm) (Japanese) (old/outdated)
  * [Fantasy's FDS Maniacs page](http://park19.wakwak.com/~fantasy/fds/) (Japanese). Technical information is in the [ディスクシステム資料室](http://park19.wakwak.com/~fantasy/fds/document/document.htm) section.
  * [FDS Study site](https://web.archive.org/web/20080515232015/http://fdsstudy.hp.infoseek.co.jp/) via archive.org (Japanese). The FDSStudy program can still be found [here](https://w.atwiki.jp/famicomall/pages/589.html).
  * [fds-nori.txt](https://nesdev.org/fds-nori.txt) \- FDS reference in Japanese by Nori
  * [Forum post](https://forums.nesdev.org/viewtopic.php?p=194867#p194867): .fds format: Can checksums be heuristically detected? - Includes a CRC implementation in C.



## References

  1. ↑ [FamicomWorld:](http://famicomworld.com/workshop/articles/product-codes-fmc-and-fsc/) FMC and FSC product codes


