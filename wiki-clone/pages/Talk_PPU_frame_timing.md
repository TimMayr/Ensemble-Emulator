# Talk:PPU frame timing

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APPU_frame_timing) | View [other pages](Special_AllPages.xhtml#Talk_PPU_frame_timing)
    
    
    	   PPU      29780.7   29780.7  29780.7  29780.7
    	-C--P-------C--P-------C-P-------CP-------*
    	CPU       29781      29781     29781    29781
    	
    * The above synchronization will result in an exact PPU/CPU alignment.
    	
    	 -|--.--V--|-     -|--V--.--|-     -V--.--.--|- 
    	Read             Read             Read
    	                                 Loop will stop here
    

Is the first C--P missing a '-' there? 

## Merge with [PPU rendering](PPU_rendering.xhtml "PPU rendering")?

There is a wonderful diagram at [PPU rendering](PPU_rendering.xhtml "PPU rendering"), as well as lots of information about timing there. I think this article should be merged into that one. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:02, 24 March 2015 (MDT) 
