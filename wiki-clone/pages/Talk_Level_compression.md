# Talk:Level compression

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ALevel_compression) | View [other pages](Special_AllPages.xhtml#Talk_Level_compression)

Some additional kind of level compressions: 

  * Huffed RLE, used in _Attribute Zone_ (which also stores compressed level data in CHR ROM, to take advantage of the PPU auto-increment). The compression program takes an input file which specifies which kind of tiles that RLE should be applied to. For the other tiles, RLE is not applied.
  * Predictive coding, with preprocessing if applicable. This is a huffed Markov chain. Unreachable areas may be filled with walls during the preprocessing step, in order to reduce the number of Huffman codes so that the codes can be made shorter. In cases where only one tile makes sense, that tile is encoded using zero bits.
  * Preprocessing like described above can be applied even without predictive coding, if this would allow shorter Huffman codes to be used, or longer runs.
  * Rotated/flipped level data. In some cases for some games, it may make sense that a preprocessor might rotate or flip the level data if that would improve compression.
  * If using RLE, boustrophedon might help compression in some cases. For example, see [the first screen of Town of ZZT](http://zzo38computer.org/img_1C/townzzt.png). Boustrophedon would group together the blue parts near the bank, would group together the yellow parts near the armory, etc.



\--[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:07, 4 November 2017 (MDT) 

The page lists Micro Mages as using the most significant bit when offsetting the metameta tiles in their vertical symmetry. While the developer's video shows the most significant bit, in the actual game, the least significant bit is used. 
