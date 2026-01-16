# User:Qbradq/MMC1 Development Cart

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AQbradq/MMC1_Development_Cart) | View [other pages](Special_AllPages.xhtml#User_Qbradq_MMC1_Development_Cart)

## Contents

  * 1 NOTICE
  * 2 Goals and Project Outline
  * 3 Parts and Materials
    * 3.1 Notes



# NOTICE

This page describes a work-in-progress. At this time my ROM programmer is not cooperating with me, so I do not know if the below parts list is correct. 

# Goals and Project Outline

The goal of this project is to create a development cartridge for NES homebrew development. This cartridge will support the popular MMC1 mapper, 256KB of PRG-ROM, 8KB CHR-RAM and 8KB of battery-backed PRG-RAM. This configuration is commonly known as SNROM. For reference, this is the same configuration used in The Legend of Zelda, but with double the ROM space. 

The cartridge described here is not the kind you plug into your computer to update the code. Rather you remove the ROM chip from the cartridge and use an external chip programmer to place the code onto the ROM chip. This provides a slower cycle time compared to other methods. The benefit of this cartridge is that it can be made from readily available parts with only basic soldering skills. 

# Parts and Materials

The following basic tools and materials will be required for this project: 

  * A soldering iron, 15W-30
  * Solder
  * De-Soldering Braid (optional)
  * A Number 1 Philips Head Screw Driver
  * Electrical Tape (optional)



The following parts are required for construction: 

Part Name  | Count  | Supplier Used  | Supplier Link  | Cost Per Unit   
---|---|---|---|---  
ReproPak MMC1  | 1  | RetroZone  | [http://www.retrousb.com/product_info.php?cPath=24&products_id=43](http://www.retrousb.com/product_info.php?cPath=24&products_id=43) | $9.00   
CIClone  | 1  | RetroZone  | [http://www.retrousb.com/product_info.php?cPath=24&products_id=37](http://www.retrousb.com/product_info.php?cPath=24&products_id=37) | $4.00   
Blue Case (or any other color you like)  | 1  | RetroZone  | [http://www.retrousb.com/product_info.php?cPath=24&products_id=39](http://www.retrousb.com/product_info.php?cPath=24&products_id=39) | $4.00   
6462P-12 8Kx8 SRAM  | 2  | Jameco  | [https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=42930](https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=42930) | $2.49   
28F020-12 256Kx8 Flash ROM  | 1  | Jameco  | [https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=242616](https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=242616) | $7.95   
28-Pin Low-Profile ZIF Socket  | 1  | Jameco  | [https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=102745](https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=102745) | $7.95   
General-Purpose Diode (Sold in Packs of 10) (Optional)  | 2  | Jameco  | [https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=655613](https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=655613) | $0.02   
BR2330 Memory Battery (or any other 3V coin battery) (Optional)  | 1  | Jameco  | [https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=2113691](https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=2113691) | $2.45   
1KOhm Carbon Film Resistor (Sold in Packs of 100) (Optional)  | 1  | Jameco  | <https://www.jameco.com/webapp/wcs/stores/servlet/Product_10001_10001_690865_-1> | $0.02   
28-Pin DIP Socket (Sold in Packs of 10) (Optional)  | 2  | Jameco  | [https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=112272](https://www.jameco.com/webapp/wcs/stores/servlet/ProductDisplay?storeId=10001&langId=-1&catalogId=10001&productId=112272) | $0.22   
  
_All prices are in USD and are accurate as of 03/31/2011_

In addition to the parts and materials you will need a chip programmer compatible with standard 29F020 Flash ROM chips. Something like the Willem GQ-3X or TOP853 should work. I have a Willem PCB 5.0, which is an obsolete parallel programmer. They can still be found on eBay for under $50 shipped, but you have to have a (real) parallel port. 

## Notes

The Diodes, Battery and Resistor are only needed if you want battery-backed PRG-RAM (that is if you want save files like The Legend of Zelda, not passwords like Metroid). 

If you can find a 3V coin battery with solder leads on it that would be ideal. These instructions include soldering leads onto the battery, which is certainly not ideal. 

The 28-Pin DIP Sockets are only needed if you are super paranoid like me and want to be able to easily replace your RAM chips. Note that this will not allow you to swap in CHR-ROM latter. For that you will have to de-solder the 28-Pin DIP Socket used for CHR-RAM. 
