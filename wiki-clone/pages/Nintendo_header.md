# Nintendo header

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Nintendo_header) | View [other pages](Special_AllPages.xhtml#Nintendo_header)

Around a third of licensed Nintendo NES games have at least part of a header present before their CPU vectors.[1] The [FamicomBox](FamicomBox.xhtml "FamicomBox") reads this header to determine the validity and title of an inserted game. Homebrew games can include this header for compatibility with FamicomBox consoles that have been modified to bypass the CIC. A script is able to generate valid headers.[2]

The info in these headers is frequently incomplete and/or inaccurate. Value $00 or $FF is often used for empty fields. 

  * **$FFE0-$FFEF** : Title of the game, right-justified and in the specified encoding. See title encoding and title length below. Left-justifying, padding on the right with spaces, and setting a max length is acceptable, but nonstandard. The title is often abbreviated, a code, wrongly-justified, or incorrect.
  * **$FFF0-$FFF1** : PRG checksum (NROM/CNROM/UNROM: the whole PRG-ROM; GNROM: each 32 KiB bank; MMC: the last 16 KiB). Sum of all bytes in the relevant area, excluding the two PRG checksum bytes. Big-endian.
  * **$FFF2-$FFF3** : CHR checksum. $0000 if RAM. Big-endian.
  * **$FFF4** : Data sizes and type. 
    * D7-D4: PRG size (0 = 64 KiB, 1 = 16 KiB, 2 = 32 KiB, 3 = 128 KiB, 4 = 256 KiB, 5 = 512 KiB)
    * D3: CHR type (0 = CHR ROM, 1 = CHR RAM)
    * D2-D0: CHR size (0 = 8 KiB, 1 = 16 KiB, 2 = 32 KiB, 3 = 64 or 128 KiB, 4 = 256 KiB)
  * **$FFF5** : Board type. 
    * D7: Nametable [arrangement](Mirroring.xhtml "Mirroring") (0 = Horizontal arrangement, 1 = Vertical arrangement). Inverse of [iNES mirroring bit](INES.xhtml#Flags_6 "INES"). If mapper-controlled, this is supposed to be 0, but in practice is often 1, perhaps indicating a primary arrangement.
    * D6-D0: Mapper (0 = NROM, 1 = CNROM, 2 = UNROM, 3 = GNROM, 4 = MMC (any)). Determines which area to checksum.
  * **$FFF6** : Title encoding (0 = No title entered, 1 = ASCII ($20-3F and $41-5A allowed), 2 = [JIS X 0201](https://en.wikipedia.org/wiki/JIS_X_0201) (possibly [Shift JIS](https://en.wikipedia.org/wiki/Shift_JIS)?)). The JIS X 0201 encoding appears to support at least the ASCII encoding plus single-byte katakana characters.
  * **$FFF7** : Title length - 1 (0 = No title entered, 1-15 = 2-16 bytes). Unknown if 1-byte titles are supported, but using a 2-byte title padded on the right with a space is equivalent. Title lengths are frequently off by one.
  * **$FFF8** : [Licensee code](Licensee_codes.xhtml "Licensee codes") (0 = No code entered).
  * **$FFF9** : Header validation byte. 8-bit checksum of $FFF2-$FFF9 should = 0.



## FamicomBox compatibility

The FamicomBox is the only device known to use the Nintendo header. Its firmware cartridge validates game cartridges using the header validation byte and the PRG checksum. If these are both correct, the game is marked as valid and the header title is copied, and otherwise, the game is looked up in the firmware cartridge's database for a match. Titles are assumed to use ASCII encoding. 

As part of validation, the FamicomBox performs writes to the game cartridge's mapper. It does this by searching forward from $8000-FFFF, ANDing each byte by a write mask and comparing against the write value. If they match, the value at that ROM address is read and then written back. This is done to avoid bus conflicts. If no match is found, $0000 is read and written, instead. 

The sequence of operations for validating a cartridge are as follows: 

  1. The slot is checked for whether it contains a cartridge by checking 17 addresses for any non-$FF value. These addresses are: $0000, $8000, $8001, $8002, $8004, $8008, $8010, $8020, $8040, $8080, $8100, $8200, $8400, $8800, $9000, $A000, $C000. If only $FF is found, the slot is skipped. Note that the check on $0000 appears to result in false positives and so it is the CIC that normally identifies an empty slot.
  2. The mapper is written with value $00 and mask $FF.
  3. The header is validated by verifying that $FFF2-FFF9 sum to 0. If they do not, the validation process is stopped and the database is checked.
  4. The PRG checksum is calculated based on the cartridge's mapper type. The bytes at $FFF0 and $FFF1 are each subtracted from this sum and the result is compared to $FFF0-FFF1 (big endian). If they do not match, the validation process is stopped and the database is checked. Note that the bank is not changed between completing the sum and using the header's PRG checksum values. 
     * **NROM/CNROM** : The PRG size is determined by looking for the largest range containing unique data ($E000-FFFF if $C000-DFFF and $E000-FFFF match, $C000-FFFF if $8000-BFFF and $C000-EFFF match, and $8000-FFFF otherwise). This range is summed.
     * **UNROM** : A single checksum is created by summing 8 banks together. The 8 banks are loaded by writing $00-07 with mask $07 to the mapper. For each bank, $8000-BFFF is summed into the combined total.
     * **GNROM** : Each of 4 banks is individually checked, so each requires its own checksum at $FFF0-FFF1. The 4 banks are loaded by writing $00, $10, $20, and $30 with mask $30 to the mapper. For each bank, $8000-FFFF is summed.
     * **MMC** : $C000-FFFF is summed.
     * **Other** : The match automatically fails.
  5. The title length at $FFF7 is checked. If it isn't in the range $01-0F, the title is considered invalid, so a 16-byte title of value $20 (space) is used and the slot is marked as valid with an invalid title. Otherwise, the right-aligned title is copied from $FFE0-FFEF and stored left-aligned, the remaining space on the right is padded with value $20, and the slot is marked as valid with a valid title. Note that 1-character titles will be seen as invalid, but this can be fixed by incrementing the length and suffixing the title with a space. Any bytes to the left of the used portion of the title are free for general-purpose use.



For database lookups, $00 with mask $FF is written to the mapper and then the NROM/CNROM-style checksum is calculated. $FFF0-FFF1 are not subtracted from this checksum. 

### Caveats for particular mappers

UNROM
    The checksum routine avoids bus conflicts by searching ROM for the first byte in the currently-loaded $8000-FFFF region whose low 3 bits match the target bank number, reading that byte, and writing it back. This causes problems with variants supporting more than 8 banks, such as UOROM, because the remaining bits will depend on the value that was found by the search, so a checksum generator must perform this search to find which of the 8 banks are visited when calculating the checksum.
    For 64 KiB UNROM, such as [240p Test Suite](https://github.com/pinobatch/240p-test-mini), the checksum routine reads each of the four banks twice. Thus, it double counts every byte, but still subtracts the checksum bytes only once. The checksum value must account for this. A potential strategy is to generate a checksum, and then reduce the value of an unused byte by half the sum of the two checksum bytes.
AxROM and BxROM
    The header does not support these mapper types. The Japanese release of Battletoads uses AOROM and has a header that claims to be UNROM, but its checksum does not adhere to UNROM rules. These are best supported using the NROM or MMC types and placing the header information in the first bank.
MMC1
    The mapper is not reset before the checksum is read, so the last bank may not be fixed to $C000-FFFF. Therefore, every bank that could be mapped there at power-on needs its own header with its own checksum.
MMC3, most VRCs, FME-7
    These mappers have a fixed bank at $E000-$FFFF and an unpredictable switchable bank at $C000-$DFFF. To get the checksum to match, either the $00 mapper write needs to be used to make this region predictable, or the sum of all 8 KiB banks that can be mapped there must be identical. For MMC3, placing $00 at the start of the second-last bank will send this write to the [bank select register](MMC3.xhtml#Bank_select_\(%248000-%249FFE,_even\) "MMC3") at $8000 if the mapper starts in PRG ROM bank mode 1, fixing $C000-DFFF to the second-last bank. Note that power-on state may be consistent for variants of these mappers, and Super Chinese 2 for FamicomBox (Sharp MMC3B) relies on starting in PRG ROM bank mode 0.

In general, the MMC mapper type is likely the best fit for any given project, with the header and checksum included in any place that can be loaded after a write of $00 to first of any address that can contain $00. 

## References

  1. ↑ [Forum post:](https://forums.nesdev.org/viewtopic.php?p=57051#p57051) Survey of headers in commercial games
  2. ↑ [GitHub:](https://github.com/pinobatch/240p-test-mini/blob/master/nes/tools/sssfix.py) PinoBatch's Nintendo header script


