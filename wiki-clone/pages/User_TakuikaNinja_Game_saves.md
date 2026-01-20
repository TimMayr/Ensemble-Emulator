# User:TakuikaNinja/Game saves

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ATakuikaNinja/Game_saves) | View [other pages](Special_AllPages.xhtml#User_TakuikaNinja_Game_saves)

Longer games can benefit from being able to save and resume sessions without needing to leave the console on with the cartridge inserted. Genres such as action-adventure, metroidvania, and RPGs would not exist in their current form without the ability to save and load game progress! While savestate features in [emulators](Emulators.xhtml "Emulators") and [flashcarts](Category_Flash_Cartridge.xhtml "Flashcart") have somewhat reduced the demand for comprehensive in-game save features, they should be considered when producing physical versions. 

This article lists the commonly used methods for saving game progress. All of the methods share the following limitations/drawbacks: 

  * The contents of save data on the first boot is not guaranteed to be valid. Games should check for uninitialised save data and set its contents to a known state on the first boot. Ideally, the player should be notified of this.
  * Data corruption via intentional (e.g. save editing) or unintentional (e.g. power faults) means can put save data into an invalid state. Games should implement/perform integrity checks to prevent loading corrupted data (or even correct it), and notify the player in the event that save data was lost.
  * Game revisions/updates have the risk of making save data from older versions incompatible. Care should be taken to ensure forwards and backwards compatibility.



## Contents

  * 1 Passwords
  * 2 Disk saving
  * 3 Battery-backed SRAM
  * 4 Self-flashing
  * 5 Other methods
  * 6 References



## Passwords

Passwords are one of the cheapest and earliest forms of saving. The game encodes the current game state into a human-readable password and presents it to the player to transcribe/memorise. Later, an entry screen is provided to accept and decode the password back into the game state. The human-readable nature means that passwords can often be shared between players, whether it be in-person or online. 

[The simplest thing that could possibly work](http://c2.com/xp/DoTheSimplestThingThatCouldPossiblyWork.html) would be to have hard-coded passwords at certain points of the game (stage, chapter, etc). However, this is prone to brute-forcing and basic reverse-engineering. More complex games with password systems implement software routines to encode/decode a bit stream containing variables with [fixed bit length encoding](Fixed_Bit_Length_Encoding.xhtml "Fixed Bit Length Encoding") (especially 1-bit flags). A constant value or a checksum is often shuffled into the bit stream to provide protection against entry errors and brute-forcing. The password's complexity (length, character set size) correlates with the amount of data to save,[1] and should be balanced appropriately. Games using Latin character sets may remove vowels (and possibly some confusing number/letter pairs such as 0/O & 5/S) to prevent inappropriate words from appearing in valid passwords. 

See [here](https://pineight.com/nes/#password_save) for an example program which encodes and decodes passwords for 32 bits (4 bytes total) of data. 

Benefits: 

  * It can be implemented on any mapper, as it does not require extra hardware.
  * It can be easier to share/test certain game states.



Limitations: 

  * Password encoding/decoding routines are often tailored for a specific game, even when a generic scheme (such as the example above) is used. As such, documented code examples are limited.
  * Larger data sizes require more complex passwords, which are harder to implement and are more prone to transcription/entry errors.
  * Players are likely to input invalid passwords, whether it be by mistake or by modifying known valid passwords.



## Disk saving

The [Famicom Disk System](Family_Computer_Disk_System.xhtml "Famicom Disk System") [BIOS](FDS_BIOS.xhtml "FDS BIOS") provides an API to save and load disk files. Therefore, save data can be saved to and loaded from disk. The rewritable nature of the medium means that this process directly alters the disk contents. The FDS calculates and verifies a CRC16 checksum for each disk file, providing basic protection against data corruption. Disk saves can potentially be shared between copies of the same game through disk swapping. (_Exciting Baseball_ is one such example) 

There is example code to [save](FDS_BIOS.xhtml#Example_code_to_save_a_file "FDS BIOS") and [load](FDS_BIOS.xhtml#Example_code_to_load_files "FDS BIOS") disk files. 

Benefits: 

  * Accessible by all disk programs, as the BIOS ROM is always mapped.



Limitations: 

  * Only the last file of a disk side can be modified due to the disk format used. If multiple save files are required, they must be concatenated into a single disk file (and therefore must occupy a contiguous section of memory at some point).
  * Disk errors returned by the BIOS must be accounted for.
  * Write protections added in later disk drives will prevent large files from being written to disk. (TODO: measure the minimum data size that can trigger it)
  * The [file amount block](FDS_disk_format.xhtml#File_amount_block_\(block_2\) "FDS disk format") must accurately reflect the number of disk files for the BIOS routines to function correctly. This can be worked around by using custom disk I/O routines (as seen in some games with "hidden" files) but is not recommended due to the complexity of handling disk I/O registers. 
    * Programs which [bypass the approval check](FDS_BIOS.xhtml#Bypass_method "FDS BIOS") but wish to use disk I/O must include a large final file (e.g. the save data itself could be used) to stall the disk seeking process instead of presenting a false number in the file amount block.
  * Editing disk contents is fairly easy, even on original hardware. (i.e. via _Tonkachi Editor_)
  * Because the widely used [FDS file format](FDS_file_format.xhtml "FDS file format") omits CRC values, some emulators may not fully implement the CRC calculation and therefore fail to detect corrupted disk files.
  * Many emulators implement disk saving by creating and applying IPS patches based on the filename of the disk image, in order to avoid altering the original file. Since the IPS format by itself cannot compare/validate the source and target disk images, altering the contents of the original disk image without updating the IPS patch (e.g. disk version updates, development builds) can lead to unexpected behavior.



## Battery-backed SRAM

Some mapper/board configurations allow for a section of memory (typically 8KiB mapped to $6000-$7FFF) to be powered by a battery while outside of the console.[2] Modern reproductions of such boards are readily available. (e.g. Broke Studio multi mapper board) 

Battery-backed SRAM is the simplest save method to implement in program code, as the save data can be accessed just like system memory. There is usually enough SRAM allocated to include multiple save files. Games should protect against uninitialised or corrupted SRAM contents by including redundant copies of save data and verifying internal checksum values. 

Benefits: 

  * SRAM can be used as extra PRG-RAM, not just save data. For example, _The Legend of Zelda_ places common code/data in unused parts of its SRAM.
  * Mappers with support for battery-backed SRAM include [MMC1](MMC1.xhtml "MMC1") and [MMC3](MMC3.xhtml "MMC3"), which are popular/common choices for homebrew development.



Limitations: 

  * SRAM contents are uninitialised on the first boot. The [init code](Init_code.xhtml "Init code") must account for this.
  * It is prone to data corruption, especially when powering off the console. (Hence messages such as "Hold RESET while powering off the system") Some mappers may have toggle-able write protections to mitigate this, but verifying save data integrity is still recommended.
  * Batteries will lose their charge after around 10 years (under normal conditions). Replacing batteries without losing save data is not trivial.



## Self-flashing

Homebrew mappers such as [UNROM 512](UNROM_512.xhtml "UNROM 512") and [GTROM](GTROM.xhtml "GTROM") have the ability to erase/rewrite their PRG-ROM flash chip sectors at runtime. Therefore, sectors can be allocated to contain save data. Self-flashing code must run outside of the flash chip - the [required code](https://www.nesdev.org/w/images/default/f/fa/Flash.asm.txt "Flash.asm.txt") is loaded into RAM and executed from there. There are active [publishers and boardmakers](Publishers_and_boardmakers.xhtml "Publishers and boardmakers") for these mappers. 

Benefits: 

  * Flash chips can last significantly longer than battery-backed solutions. (around 100 years)
  * The supported mappers are often cheaper to produce, as the self-flashing is an inherent feature of the board configuration.



Limitations: 

  * Limited number and feature sets of mappers which support self-flashing.
  * Limited self-flashing support by emulators and flashcarts. 
    * These often implement self-flashing by creating and applying IPS patches based on the filename, much like disk saving on the FDS. Altering the contents of the original file without updating the IPS patch (e.g. version updates, development builds) can lead to unexpected behavior, for the same reason.
  * Since the flash chip contains both the PRG-ROM and save data, it is possible to irreversibly erase/overwrite program code.
  * Flash chip sectors have limited endurance in terms of erase cycles (often 100000). Games should minimise the amount of sector erases/rewrites (e.g. by only saving when data has changed) in order to maximise the lifetime of the chip. Games could also include failsafes in case self-flashing no longer works.[3]



## Other methods

These methods are less common and require Famicom-exclusive peripherals: 

  * The [Family BASIC Data Recorder](Family_BASIC_Data_Recorder.xhtml "Family BASIC Data Recorder") uses cassette tape as the storage medium. Games other than _Family BASIC_ can support this. Example: _Excitebike_
  * Some games support using external SRAM/EEPROM save devices ([Battle Box](Battle_Box.xhtml "Battle Box"), [Datach](INES_Mapper_157.xhtml "INES Mapper 157"), [Turbo File](Turbo_File.xhtml "Turbo File"), etc). Cross-game/cross-system save sharing is also possible with a select few of them.



## References

  1. ↑ <https://nerdlypleasures.blogspot.com/2015/06/types-of-nes-passwords.html>
  2. ↑ "Why Your Game Paks Never Forget". _Nintendo Power_ #20 (March 1991), pp. 28-31.
  3. ↑ ["CrossPaint (NES Homebrew) - UNROM512 Flash Saving" by CutterCross](https://youtu.be/Pd4_HFvFqBo) (7 minutes)


