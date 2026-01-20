# User:Myask/MyaGrafx

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AMyask/MyaGrafx) | View [other pages](Special_AllPages.xhtml#User_Myask_MyaGrafx)

"Perhaps someone should mock up a specification for a CPLD that only provides 8x8 attributes and nothing else."\--[Tepples](http://forums.nesdev.org/viewtopic.php?f=21&t=11497&start=165#p145690)

## Contents

  * 1 Sources
  * 2 Basic
  * 3 Basic Implementation
  * 4 Less basic



## Sources

[PPU rendering](PPU_rendering.xhtml "PPU rendering"), [Cartridge connector](Cartridge_connector.xhtml "Cartridge connector"), [6264](6264_static_RAM.xhtml "6264")

## Basic

As the cart only has CIRAM A10 piped through, one can't just remap part of CIRAM to supply the 256 bytes of attribute one needs for page 0. 

So, for the basicmost case... PPU: --10 NT11 11yy yxxx is where attributes normally reside. To our advantage, the nametable byte precedes the attribute table byte. This allows at least the 8x8 tile to be detected: NT-fetch is PPU: --10 NTYY YyyX XXxx. (A0, A5 are what we want to know, as they're not in the AT fetch; A1,6 are the 16-px accounted for by which two bits within an attribute byte.). For this simple one, instead of storing the attribute data per-tile, it's arranged as usual: four tiles in a 32x32 are specified each byte, just that the game pak will return different byte depending on the X and Y tile evenness. 

## Basic Implementation
    
    
     /* NES 8x8(8x1?)-Attribute graphics mapper
       Created by "Myask", April 2015
       8x8-mapper:
         Accepts input at ppu_a == 14'b__11_NTx0_xxxx_xxxx 
            (ppu_a[8] == 0 to avoid conflict with palettes)
            where ppu_a[7:4] are the 16px-tile level Y
            and ppu_a[3:0] are the 16px-tile level X.
            Bitpairs are organised as normal, just at 16x16 instead.
         (genuinely indifferent to ppu_a[9] on write.)
                      and cpu_a == 16'b1xxx_xxxx_xxxx_xxxx
            to enable,    cpu_d ==  8'bxxxx_xxx1
            to disable,   cpu_d ==  8'bxxxx_xxx0
         Outputs at       ppu_a == 14'b__10_NT11_11YY_YXXX
           using the previous nametable fetch to select which of four possible
           attribute byte to return.
     */
     module mya_at_mapper{
       system_clk,
       m2,
       cpu_rw, //high=r
       cpu_a,
       cpu_d,
       romsel_n, //traditionally _n signifies active-low
       irq_n,
       exp,
       ppu_wr_n,
       ppu_rd_n,
       ppu_a,
       ppu_d,
       ppu_a13_n,
       ciram_ce_n,
       ciram_a10,
       mya_atram_a,
       mya_atram_d,
       mya_atram_we_n,
       mya_atram_oe_n,
       mya_atram_cs1_n,
       mya_atram_cs2
     };
     //first, cart-edge signals
     input         system_clk;
     input         m2;
     input         cpu_rw;
     input  [14:0] cpu_a;
     inout   [7:0] cpu_d;
     input         romsel_n;
     
     output        irq_n;
     
     inout   [9:0] exp;
     
     input         ppu_wr_n;
     input         ppu_rd_n;
     input  [13:0] ppu_a;
     inout   [7:0] ppu_d;
     input         ppu_a13_n;
     
     output        ciram_ce_n;
     output        ciram_a10;
     //then cart-internals: first, the (probably 6264)
     output [12:0] mya_atram_a;
     inout   [7:0] mya_atram_d;
     output        mya_atram_we_n;
     output        mya_atram_oe_n;
     output        mya_atram_cs1_n;
     output        mya_atram_cs2;
     //then make all the variable names (sigh) Cart-external:
     wire          system_clk;
     wire          m2;
     wire          cpu_rw;
     wire   [14:0] cpu_a;
     wire    [7:0] cpu_d;
     wire          romsel_n;
     wire          irq_n;
     wire    [9:0] exp;
     
     wire          ppu_wr_n;
     wire          ppu_rd_n;
     wire   [13:0] ppu_a;
     wire    [7:0] ppu_d;
     wire          ppu_a13_n;
     
     wire          ciram_ce_n;
     wire          ciram_a10;
     //cart-internal:
     wire   [12:0] mya_atram_a;
     wire    [7:0] mya_atram_d;
     wire          mya_atram_we_n;
     wire          mya_atram_oe_n;
     wire          mya_atram_cs1_n;
     reg           mya_atram_cs2;
     //chip-internal:
     reg     [1:0] at_8x;
     reg     [1:0] at_8y; 
     wire          ul;
     //reg     [2:0] at_finey;
     
     always @(negedge ppu_rd_n) 
       if (ppu_a[13] & (~& ppu_a[9:8])) begin //trap nt-fetch
         at_8x[1:0] <= ppu_a[1:0]; //and store the 8&16px-level X
         at_8y[1:0] <= ppu_a[6:5]; //and Y-coordinates.
       end //trap nt-fetch
     
     always @(posedge m2)
       if (~cpu_rw & ~romsel_n) 
         mya_atram_cs2 <= cpu_d[0]; 
     //only have one visible register bit, so little decoding necessary: CPU$8xxx.
     
     assign mya_atram_a[7:5] = ppu_a[5:3]; 
     assign mya_atram_a[3:1] = ppu_a[2:0];
     assign mya_atram_a[0] = (ppu_a[12] ? ppu_a[6]: at_8x[1]); 
     assign mya_atram_a[4] = (ppu_a[12] ? ppu_a[7]: at_8y[1]);
     assign mya_atram_a[9:8] = ppu_a[11:10]; 
     //ppu_a[11:10] = NT-select. also don't need to be routed through CPLD
     assign mya_atram_a[12:10] = 3'b000; //Doing fineY-AT'd need a new write port
     assign ul = 1'b0; //~(at_8x | at_8y); 
     //assign ul with 0 to not bother using ciram for any attributes at all
     assign mya_atram_oe_n = ~( ppu_a[13]
       & (& ppu_a[9:6]) //Nametable: @PPU 16'b0010_xx11_11xx_xxxx
       & ~ul & ~ppu_rd_n);
     assign mya_atram_we_n = ~( ~ppu_wr_n & (&ppu_a[13:12]) & ~ppu_a[8] );
       //Write-port: @PPU 14'b11_NT?0_YYYY_XXXX: ~a[8] to avoid palettespace
     assign mya_atram_cs1_n = 1'b0;
     //assign mya_atram_cs2 = mya_atram_enable;
     assign ciram_ce_n = ~(ppu_a[13]    //nt/at only
       & (~ppu_a[12]) // | (& ppu_a[11:8]))  (palette ram cares not for CIRAM/CE
       & ((~& ppu_a[9:6]) ? ul : 1'b1) );//enable for the ul AT- and all NT- fetches.
     
     always @ (mya_atram_d or mya_atram_oe_n) begin//ppu_d mux logic
       //note: may need additional always@ args, (and combining) 
       //as-is might do stupid things like making ppu_d a latch?
       if (mya_atram_oe_n) //that is, when not reading...
         ppu_d[7:0] = mya_atram_d[7:0];
       else begin //when reading by AT, mux out the correct bitpair
         case ({at_8y[0],at_8x[0]})
           2'b00: ppu_d = {4{mya_atram_d[1:0]}}; //upper-left
           2'b01: ppu_d = {4{mya_atram_d[3:2]}}; //upper-right
           2'b10: ppu_d = {4{mya_atram_d[5:4]}}; //lower-left
           2'b11: ppu_d = {4{mya_atram_d[7:6]}}; //lower-right
           default: ppu_d[7:0] = mya_atram_d[7:0];
         endcase
       end //
     end //ppu_d mux logic
     
     endmodule //mya_at_mapper
    

## Less basic

This mode of writing does not work if we want to extend to 8x1 attributes; there are three bits of attribute space to add and we only have three choices (00, 01, 10) of PPUADDR 8-9 for NT3. Even in two-screen mirroring, there is a small problem: but as we are relying on CIRAM for the first sliver of each section, one does not need to have duplicate write-access to those. One could remap $38** to what would have been in $3F**. Four-screen proves more problematic. Also problematic is determining the fine-Y. Brute-force method is to snoop for writes to PPU_SCROLL, as well as reads from PPU_STATUS and writes to PPU_ADDR to know the high-byte latch status. If we don't want to allow raster effects, which seems like a short-sighted decision, perhaps one could somehow divine where to begin from the dummy-fetch prerender scanline. In any case, it would basically require a scanline counter, at which point one would just add a few more bits of state to get a useful scanline-type interrupt, though if it shares the low three bits with the rendering portion it would be more of a NT-relative Y-coordinate interrupt... 
