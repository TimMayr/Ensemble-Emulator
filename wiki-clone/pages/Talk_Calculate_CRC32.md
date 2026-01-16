# Talk:Calculate CRC32

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACalculate_CRC32) | View [other pages](Special_AllPages.xhtml#Talk_Calculate_CRC32)

## Checksums/hashes article?

Just putting this here temporarily, until we can write an article on checksums. 

> "<Myria> what would be a good checksum algorithm to use on 6502? a literal check"sum" would not work, because of the nature of the data being checksummed."
> 
> I'm working on this exact problem for multi-line serial transfers using controller ports and the expansion I/O port. That link ( <http://6502.org/source/integers/crc-more.html> ) is to a nice site. 
> 
> [http://forums.nesdev.org/viewtopic.php?f=2&t=13225](http://forums.nesdev.org/viewtopic.php?f=2&t=13225) I had a post specific to parity. I plan on creating a separate topic on the Wiki or Forums with references for good non-parity methods of error-checking on the NES. 
> 
> ADC8 is one of the simplest. You simply use ADC $xxxx,X on a sliding position in a 256-byte array for each page of the file/memory to checksum. This is similar to ADD8 but high bit of last result is added to lowest bit of next sum in the series. 
> 
> XOR8 is stupidly universal and easy to implement. Just use EOR $xxxx,X instead of ADC $xxxx,X. As an added bonus, you can use the associative property as shown here: [http://forums.nesdev.org/viewtopic.php?f=2&t=13225#p154962](http://forums.nesdev.org/viewtopic.php?f=2&t=13225#p154962)
> 
> To create a position-detecting hash, you keep doing something like XOR8 as normal, and then you keep a separate sum that adds the checksum in addition to just the current data byte being verified. You'd have something like "P=P XOR Byte(X): Q=Q + P" in BASIC. Because P _at that time_ varies, if bytes are swapped or missing or inserted, it changes the final value for Q. All that matters is that the position changes the value that you're combining with Q. P could even be Byte(X)+X.

[alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 01:48, 5 October 2015 (MDT) 
