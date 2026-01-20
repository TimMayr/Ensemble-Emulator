# NESdev IRC channel

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NESdev_IRC_channel) | View [other pages](Special_AllPages.xhtml#NESdev_IRC_channel)

## Contents

  * 1 Connecting
  * 2 Topic
  * 3 Conduct
  * 4 Bot



## Connecting

Our IRC channel is located on the [EFnet](http://www.efnet.org/) network. [A list of servers](http://www.efnet.org/?module=servers) is available. 

Find yourself an [IRC client](https://en.wikipedia.org/wiki/List_of_IRC_clients "wikipedia:List of IRC clients") and drop by to [#nesdev](irc://irc.efnet.org/nesdev) for assistance with your code or hardware related inquiries or interests. 

If you would like to access the NESdev IRC channel via the browser, you may do so by clicking [here](https://widget01.mibbit.com/?server=irc.servercentral.net&channel=%23nesdev&nick=_nesdev_)

Occasionally, EFnet's round-robin DNS may put you on a server that has blocked (G-lined or K-lined) your IP address for some server-specific reason, or it might have the annoying "/QUOTE PONG :cookie" CAPTCHA. You may want to connect to a [specific server](http://www.efnet.org/?module=servers) instead, such as the following: 

  * North America: [irc.servercentral.net](irc://irc.servercentral.net/nesdev), [irc.blessed.net](irc://irc.blessed.net/nesdev), [irc.prison.net](irc://irc.prison.net/nesdev), [irc.igs.ca](irc://irc.igs.ca/nesdev)
  * Europe: [irc.efnet.ch](irc://irc.efnet.ch/nesdev), [efnet.xs4all.nl](irc://efnet.xs4all.nl/nesdev), [irc.swepipe.se](irc://irc.swepipe.se/nesdev), [irc.efnet.ru](irc://irc.efnet.ru/nesdev)
  * Middle East: [irc.inter.net.il](irc://irc.inter.net.il/nesdev)
  * Africa: [irc.ac.za](irc://irc.ac.za/nesdev)



Tip: Some IRC servers won't K-line you if you get ident (RFC1413/931) working. [Ident](https://en.wikipedia.org/wiki/Ident "wikipedia:Ident") is a protocol to associate an IRC connection to a specific user name/account on a computer. Many IRC clients (ex. [mIRC](http://www.mirc.com/)) have built-in ident servers that run on TCP port 113; make sure that's enabled. If you have a firewall or NAT router, these will probably block ident requests until you tell your firewall/router to allow them/port forward them. On the router, you can use use port triggering (outbound TCP 6667 -> inbound TCP 113) or set up a static port forward to your computer (TCP port 113). If using a software firewall, allow your IRC client to accept inbound connections to TCP port 113. 

## Topic

The channel attempts to follow a standard
    Voiced (+v) individuals are either friends of operators, respected regulars, or known to have an understanding of the NES/Famicom and its architecture. Not all of these people will be able to answer questions, but questions will eventually get answered.
Code or hardware related questions are better suited for most operators
    The operators (or ops, +o) of the channel -- like most channels -- were chosen for specific reasons by other operators; mainly that they are trusted with the authority of managing the channel, keeping the bot (LittleMac) opped, having a basic to advanced understanding of 6502, NES/Famicom architecture, and knowledge of other platforms.
    #nesdev is **not the official support channel** for [PowerPak](PowerPak.xhtml "PowerPak") and other [RetroZone](http://www.retrousb.com/) products, but when its inventor is on (under nick bunnyboy or retrousb), you can bring up your support issues in channel or through a private message.

In summary: #nesdev as a channel exists as a community to casually discuss hardware, software, and NES/Famicom related topics (mainly pertaining to development or hacking). Off-topic conversation is not discouraged, so long as it does not affect any conversation in the channel related to these topics. 

## Conduct

As a channel we try to stay casual, informative, and helpful.
    Users that undesiredly use the help of others as a crutch or talk off-topic much too often will receive negative attention. ...Just because disapproval isn't publicly expressed, doesn't mean it does not exist or matter intensely. So don't be [Mr. "I vant to suck your help."](http://slash7.com/2006/12/22/vampires/) TL;DR - people are flexible, within reason.
Please be considerate of the majority of our users that log or lurk until insightful discussion has germinated.
    We don't have rigid rules for bannable offenses; operators may do as they please.
#nesdev is not considered a safe for work environment.
    As discussions started by voices or ops are capable of running to any which direction; unless moderation is necessary. Please use your own discretion.

## Bot

We're on our third bot. "LittleMac" is an Eggdrop bot that maintains user privileges. 

To set your password (only necessary after an user account has been added): 
    
    
    /msg LittleMac pass <password>
    

To identify yourself from a new host: 
    
    
    /msg LittleMac ident <password>
    

To get ops (if you have +o rights on the bot): 
    
    
    /msg LittleMac op <password>
    

To gain help: 
    
    
    /msg LittleMac help
    

Welcome! 
