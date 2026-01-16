# R.O.B. Firmware

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/R.O.B._Firmware) | View [other pages](Special_AllPages.xhtml#R_O_B__Firmware)
    
    
                     ;R.O.B. firmware disassembly, nocash 2019
                     ;------------------
                     ;dumping info:
                     ;http://www.seanriddle.com/sm590/
                     ;http://www.seanriddle.com/sm590/sm590dump.txt
                     ;------------------
                     ;Component list
                     ;base, with mainboard "(C)1985 Nintendo, RFC-03-CPU"
                     ; IC1 16pin "RFC-CPU10, Nintendo JAPAN, 8644 A" (CPU SM590)
                     ; IC2 8pin  "IR2C25, SHARP 8638 A" motor driver
                     ; IC3 8pin  "IR2C25, SHARP 8638 A" motor driver
                     ; IC4 8pin  "IR2C25, SHARP 8638 A" motor driver
                     ; x   2pin  "CSB455" 455kHz resonator
                     ; SW  5pin  Power switch (on/off)
                     ; Batteries 4x AA
                     ;head, with daughterboard "RFC-03-AMxxx?"
                     ; IC? 9pin  "IR3T07 JAPAN, SHARP 8637 B"
                     ; x   xpin  "43PI" light sensor ;unknown if there is lens, too?
                     ; x   2pin  LED (on separate tiny PCB)
                     ;motors
                     ; There are three motors, each can run forward/back or stop,
                     ;  each motor has a gear with a series of 5 big/small wheels
                     ;  (to reduce the fast spinning motors to slower rotations).
                     ; The left/right motor is in the base, the up/down and
                     ;  open/close motors are in the shoulders.
                     ; The up/down and left/right motors are bundled with switches
                     ;  (for sensing the six up/down and five left/right notches).
                     ; The open/close motor has no such feedback mechanism.
                     ;------------------
                     ;PORT usage
                     ;---
    PORT[0].0 in     light_sensor
    PORT[0].1 in     notch switch (for sensing five left/right step positions)
    PORT[0].2 in     notch switch (for sensing six up/down step positions)
    PORT[0].3 out    LED
    PORT[1].0 out    move down
    PORT[1].1 out    move up
    PORT[1].2 out    arm close
    PORT[1].3 out    arm open
    PORT[2].0 out    rotate right
    PORT[2].1 out    rotate left
    PORT[2].2 -      -
    PORT[2].3 -      -             ;(no external pin; 18pin/20pin package only)
    PORT[3].0 out    debug         ;(no external pin; 18pin/20pin package only)
    PORT[3].1 out    debug         ;(no external pin; 20pin package only)
    PORT[3].2 -      -             ;(no external pin; 20pin package only)
    PORT[3].3 clock? -
                     ;------------------
                     ;RAM usage
                     ;---
    [00h]            command_buffer_msb  (first 4bit)
    [01h]            command_buffer_mid  (middle 4bit)
    [02h]            command_buffer_lsb  (last 4bit)
    [03h]                   ;\delay count's
    [04h]                   ;/
    [05h]            -   ;unused
    [06h]            -   ;unused
    [07h]            -   ;unused
    [08h]            -   ;unused
    [09h]            -   ;unused
    [0Ah]            another_step (0=no, 1=down, 2=up)
    [0Bh]            some offhold ... for LED
    [0Ch]            -   ;unused
    [0Dh]            led_flag (bit0)  ;and another somewhat unused flag in bit1
    [0Eh]                       ;\offhold for led-blink test command
    [0Fh]                       ;/
    [10h]            -   ;unused
    [11h]            -   ;unused
    [12h]            -   ;unused
    [13h]            -   ;unused
    [14h]            -   ;unused
    [15h]            -   ;unused
    [16h]            -   ;unused
    [17h]            -   ;unused
    [18h]            -   ;unused
    [19h]            -   ;unused
    [1Ah]            -   ;unused
    [1Bh]            -   ;unused
    [1Ch]            lopcount_for_calibration_steps
    [1Dh]            lopcount for move/grab
    [1Eh]                     ;\lopcount
    [1Fh]                     ;/
                     ;------------------
                     reset_entrypoint:
    0000 (000) 48     clr    c              ;uh, what for?
                     ;- - - - - --------
                     standy_entrypoint:
    0001 (040) 74     mov    h,00                           ;\
    0002 (060) 20     mov    l,00                           ;
    0003 (070) 7D 00  call   0100 ;zerofill_up_to_16_nibbles; clear 32 ram nibbles
    0005 (07C) 75     mov    h,01                           ;
    0006 (07E) 7D 00  call   0100 ;zerofill_up_to_16_nibbles;/
                     ;- - - - - --------
                     softreset_entrypoint:
    0008 (05F) 22     mov    l,02                           ;\
    0009 (06F) 30     mov    a,00                           ;
                     @@clear_port_lop:                      ; clear port 2,1,0
    000A (077) 46     out    [l],a  ;PORT[2,1,0]=0          ;
    000B (07B) 53     decsk  l                              ;
    000C (07D) F7      jmp   000A ;@@clear_port_lop         ;/
                     ;- - -
    000D (03E) 75     mov    h,01                           ;\
    000E (01F) 2C     mov    l,0C                           ;
    000F (04F) 3C     mov    a,0C  ;aka -4 (num steps)      ;
                     @@rightmost_lop:                       ; rotate 4x to rightmost
    0010 (067) 4A     mov    [hl],a ;[1Ch]                  ; (and up/open)
    0011 (073) 3A     mov    a,0A                   ;\up    ; (until hitting four
    0012 (079) 21     mov    l,01                   ; open  ; horizontal notches)
    0013 (03C) 46     out    [l],a  ;PORT[1]=2+8    ;/      ;
    0014 (05E) 7D 51  call   012E ;move_right               ; this is doing 4 steps
    0016 (057) F5      jmp   0018 ;@@notyet_rightmost       ; (albeit tiny steps if
    0017 (06B) C3     jmp    001D ;@@rightmost_timeout      ; it was already at
                     ;---                                   ; rightmost position)
                     @@notyet_rightmost:                    ;
    0018 (075) 75     mov    h,01                           ;
    0019 (03A) 2C     mov    l,0C                           ;
    001A (01D) 40     mov    a,[hl] ;[1Ch]                  ;
    001B (00E) 01     addsk  a,01                           ;
    001C (007) E7      jmp   0010 ;@@rightmost_lop          ;
                     @@rightmost_timeout:                   ;/
                     ;- - -
    001D (043) 75     mov    h,01                           ;\
    001E (061) 2C     mov    l,0C                           ;
    001F (030) 3E     mov    a,0E  ;aka -2 (num steps)      ;
                     @@center_lop:                          ; rotate 2x to center
    0020 (058) 4A     mov    [hl],a ;[1Ch]                  ; (and up/open)
    0021 (06C) 3A     mov    a,0A                   ;\up    ; (until hitting two
    0022 (076) 21     mov    l,01                   ; open  ; horizontal notches)
    0023 (03B) 46     out    [l],a  ;PORT[1]=2+8    ;/      ;
    0024 (05D) 7D 28  call   012F ;move_left                ; uhm, this is doing
    0026 (017) E5      jmp   0028 ;@@notstill_rightmost     ; only 2 steps (enough
    0027 (04B) A3     jmp    002D ;@@center_timeout         ; to rotate to center)
                     ;---                                   ; (but probably not
                     @@notstill_rightmost:                  ; enough to reach top;
    0028 (065) 75     mov    h,01                           ; unless up/down was
    0029 (032) 2C     mov    l,0C                           ; already near top)
    002A (019) 40     mov    a,[hl] ;[1Ch]                  ;
    002B (00C) 01     addsk  a,01                           ;
    002C (046) D8      jmp   0020 ;@@center_lop             ;
                     @@center_timeout:                      ;/
                     ;- - -
    002D (023) 20     mov    l,00                           ;\
    002E (051) 55     in     a,[l]  ;PORT[0]                ;
    002F (028) 66     testsk a.2    ;bit2:PORT[0]           ;
    0030 (054) 8D      jmp   0034 ;@@notyet_topmost         ; up 1x more
    0031 (06A) 7D 1C  call   015F ;initial_move_up          ;
    0033 (01A) 00      nop                                  ;
                     @@notyet_topmost:                      ;/
    0034 (00D) 74     mov    h,00                           ;\
    0035 (006) 2D     mov    l,0D                           ;
    0036 (003) 60     testsk [hl].0 ;[0Dh]      ;LED.flag   ; LED
    0037 (041) E8      jmp   003A ;@@do_not_enable_led      ;
    0038 (020) 7D 2D  call   0166 ;set_port_0_to_8          ;
                     @@do_not_enable_led:                   ;/
                     ;- - - - - - ------
                     want_fresh_command:   ;MAINLOOP for NEW command
    003A (068) 74     mov    h,00                           ;\
    003B (074) 22     mov    l,02                           ; clear command buffer
                     @@clear_cmdbuf_lop:                    ; [02h,01h,00h]=F:F:F
    003C (07A) 3F     mov    a,0F                           ;
    003D (03D) 43     xchgsk a,[hl-] ;[02h,01h,00h]=Fh      ;
    003E (01E) FA      jmp   003C ;@@clear_cmdbuf_lop       ;/
                     ;- - - - - - ------
                     want_next_cmd_bit:   ;MAINLOOP for NEXT command BIT
    003F (00F) 24     mov    l,04
    0040 (047) 30     mov    a,00      ;inner.lopcount
    0041 (063) 43     xchgsk a,[hl-] ;[04h], L=3, and do NOT skip
    0042 (071) 33      mov   a,03      ;outer.lopcount
                     @@scan_light_outer_lop:                ;\
                     @@scan_light_inner_lop:                ;
    0043 (038) 4A     mov    [hl],a ;[03h] or [04h]         ;
    0044 (05C) 23     mov    l,03              ;\DEBUG info ;
    0045 (06E) 32     mov    a,02              ; 20pin chip ;
    0046 (037) 46     out    [l],a  ;PORT[3]=2 ;/only       ;
    0047 (05B) 7D EC  call   01A1 ;skip_if_dark             ; --> and TEST LIGHT
    0049 (036) A5      jmp   0054 ;@@got_light              ;
    004A (01B) 24     mov    l,04                           ;
    004B (04D) 40     mov    a,[hl] ;[04h]  ;inner.lopcount ;
    004C (026) 0F     addsk  a,0F                           ;
    004D (013) A4      jmp   004F ;@@scan_light_inner_done  ;
    004E (049) B8     jmp    0043 ;@@scan_light_inner_lop   ;
                     ;---                                   ;
                     @@scan_light_inner_done:               ;
    004F (024) 43     xchgsk a,[hl-] ;[04h], L=3            ;
    0050 (052) 40      mov   a,[hl] ;[03h]  ;outer.lopcount ;
    0051 (029) 0F     addsk  a,0F                           ;
    0052 (014) A2      jmp   006A ;@@got_no_light           ;
    0053 (04A) B8     jmp    0043 ;@@scan_light_outer_lop   ;
                     ;---                                   ;
                     @@got_light:                           ;/
                     @@wait_till_end_of_light:              ;\
    0054 (025) 7D EC  call   01A1 ;skip_if_dark             ; wait until dark again
    0056 (009) A5      jmp   0054 ;@@wait_till_end_of_light ;/
    0057 (004) 24     mov    l,04                           ;\
    0058 (042) 3B     mov    a,0B                           ;
    0059 (021) 43     xchgsk a,[hl-] ;[04h], L=3            ;
    005A (010) 33      mov   a,03                           ; post-light delay
                     @@afterlight_outer_delay:              ;
                     @@afterlight_inner_delay:              ;
    005B (048) 4A     mov    [hl],a ;[03h] or [04h]         ;
    005C (064) 7D EC  call   01A1 ;skip_if_dark  ;\delay    ;
    005E (039) 00      nop                       ;/dummy    ;
    005F (01C) 24     mov    l,04                           ;
    0060 (04E) 40     mov    a,[hl] ;[04h]                  ;
    0061 (027) 0F     addsk  a,0F                           ;
    0062 (053) B4      jmp   0064 ;@@afterlight_inner_done  ;
    0063 (069) C8     jmp    005B ;@@afterlight_inner_delay ;
                     ;---                                   ;
                     @@afterlight_inner_done:               ;
    0064 (034) 23     mov    l,03                           ;
    0065 (05A) 40     mov    a,[hl] ;[03h]                  ;
    0066 (02D) 0F     addsk  a,0F                           ;
    0067 (016) C5      jmp   0069 ;@@afterlight_outer_done  ;
    0068 (00B) C8     jmp    005B ;@@afterlight_outer_delay ;
                     ;---                                   ;
                     @@afterlight_outer_done:               ;/
    0069 (045) 49     set    c                              ;<-- set LIGHT flag
                     @@got_no_light:  ;in: cy=light
    006A (022) 74     mov    h,00
    006B (011) 23     mov    l,03                     ;\
    006C (008) 54     testsk c                        ; output DEBUG info
    006D (044) 30      mov   a,00                     ; (18pin/20pin chip only)
    006E (062) 31     mov    a,01                     ; (none such in 16pin retail)
    006F (031) 46     out    [l],a  ;PORT[3]=0/1      ;/
    0070 (018) 22     mov    l,02
                     @@shift_cmdbuf_lop:              ;\
    0071 (04C) 40     mov    a,[hl]  ;[02h,01h,00h]   ; left-shift RAM[02h,01h,00h]
    0072 (066) 72     adc    a,[hl]  ;[02h,01h,00h]   ; with carry-in = LIGHT
    0073 (033) 43     xchgsk a,[hl-] ;[02h,01h,00h]   ;
    0074 (059) CC      jmp   0071 ;@@shift_cmdbuf_lop ;/
    0075 (02C) 79 A7  jmp    01E1 ;process_cmd_if_any ;------>
                     ;------------------
    0077 (02B) 00     nop           ;\
    0078 (055) 00     nop           ;
    0079 (02A) 00     nop           ;
    007A (015) 00     nop           ; unused (8 bytes)
    007B (00A) 00     nop           ;
    007C (005) 00     nop           ;
    007D (002) 00     nop           ;
    007E (001) 00     nop           ;/
                     ;------------------
    007F (07F) 80     jmp    0000
                     ;------------------
                     ;unused, unless this is wakeup vector for standby?
    0080 (080) 78 40  jmp    0001 ;standy_entrypoint
                     ;------------------
                     ;unused, unless this is another exception vector?
    0082 (0E0) 78 5F  jmp    0008 ;softreset_entrypoint
                     ;------------------
                     not_0010_1011_1110_cont:  ;in: HL=01h
    0084 (0F8) 3F     mov    a,0F       ;0010_1111_1110     ;\
    0085 (0FC) 45     cmpsk  a,[hl] ;[01h]                  ;
    0086 (0FE) 87      jmp   009C ;@@not_0010_1111_1110     ;/
    0087 (0BF) DA     jmp    00E5 ;@@to_want_next_cmd_bit   ;--> CMD_0010_1111_1110 --> N/A
                     ;------------------
                     not_0010_xxxx_1110_cont:  ;in: HL=02h
    0088 (0DF) 3A     mov    a,0A       ;0010_xxxx_1010
    0089 (0EF) 45     cmpsk  a,[hl] ;[02h]
    008A (0F7) A2      jmp   00EA ;@@not_0010_xxxx_1010
    008B (0FB) 21     mov    l,01
                     ;mov    a,0A       ;0010_1010_1010
    008C (0FD) 45     cmpsk  a,[hl] ;[01h]
    008D (0BE) CF      jmp   008F ;@@not_0010_1010_1010
    008E (09F) DA     jmp    00E5 ;@@to_want_next_cmd_bit   ;--> CMD_0010_1010_1010 --> N/A
                     @@not_0010_1010_1010:
    008F (0CF) 3E     mov    a,0E       ;0010_1110_1010                    ;\
    0090 (0E7) 45     cmpsk  a,[hl] ;[01h]                                 ; check
    0091 (0F3) 3B      mov   a,0B       ;0010_1011_1010                    ; if 0Eh
    0092 (0F9) 45     cmpsk  a,[hl] ;[01h]                                 ; or 0Bh
    0093 (0BC) 8E      jmp   009B ;@@not_0010_1110_1010_nor_0010_1011_1010 ;/
    0094 (0DE) 1E     cmpsk  a,0E   ;this is either 0Eh or 0Bh             ;\
    0095 (0AF) 32      mov   a,02   ;CMD_0010_1011_1010 --> turn left      ; move
    0096 (0D7) 31     mov    a,01   ;CMD_0010_1110_1010 --> turn right     ; left
    0097 (0EB) 7D 54  call   0130 ;move_left_or_right                      ; right
    0099 (0BA) 00      nop                                                 ;/
    009A (09D) D1     jmp    00AE ;set_led_and_want_fresh_command
                     ;---
                     @@not_0010_1110_1010_nor_0010_1011_1010:
    009B (08E) 3F     mov    a,0F       ;0010_1111_1010
                     @@not_0010_1111_1110:
    009C (087) 3A     mov    a,0A       ;0010_1010_1110
    009D (0C3) 45     cmpsk  a,[hl] ;[01h]
    009E (0E1) 83      jmp   00B6 ;@@not_0010_1111_1010_nor_0010_1010_1110
                     ;- -
                     ;CMD_0010_1111_1010 --> up small step
                     ;CMD_0010_1010_1110 --> down small step
    009F (0B0) 1F     cmpsk  a,0F           ;\direction
                     ;- - - - - - - ----    ;
                     move_down:             ;       ;\
                     @@move_down_lop:       ;       ;
    00A0 (0D8) 31      mov   a,01           ;       ;
                     move_up:               ;       ;
                     @@move_up_lop:         ;       ;
    00A1 (0EC) 32     mov    a,02           ;/      ;
    00A2 (0F6) 7D 27  call   0161 ;move_up_or_down  ;
    00A4 (0DD) 00      nop                          ;
    00A5 (0AE) 74     mov    h,00                   ;
    00A6 (097) 2A     mov    l,0A                   ;
    00A7 (0CB) 30     mov    a,00                   ;
    00A8 (0E5) 41     xchg   a,[hl] ;[0Ah]          ; <-- test if another_step
    00A9 (0B2) 0F     addsk  a,0F                   ;
    00AA (099) D1      jmp   00AE ;set_led_and_want_fresh_command   ;--> no, done
    00AB (08C) 0F     addsk  a,0F                   ; <-- direction of another_step
    00AC (0C6) D8      jmp   00A0 ;@@move_down_lop  ;
    00AD (0A3) EC     jmp    00A1 ;@@move_up_lop    ;/
                     ;---
                     set_led_and_want_fresh_command:
    00AE (0D1) 74     mov    h,00
    00AF (0A8) 2D     mov    l,0D
    00B0 (0D4) 6C     set    [hl].0 ;[0Dh]         ;LED.flag
    00B1 (0EA) 6D     set    [hl].1 ;[0Dh]
                     ;- - -
                     set_led_temporarily_and_want_fresh_command:
    00B2 (0B5) 7D 2D  call   0166 ;set_port_0_to_8
    00B4 (08D) 78 68  jmp    003A ;want_fresh_command
                     ;---
                     @@not_0010_1111_1010_nor_0010_1010_1110:
    00B6 (083) 2D     mov    l,0D
    00B7 (0C1) 6D     set    [hl].1 ;[0Dh]           ??????????
    00B8 (0A0) DA     jmp    00E5 ;@@to_want_next_cmd_bit
                     ;---
                     not_0_0010_xxxx_xxxx_cont:
    00B9 (0D0) 20     mov    l,00
    00BA (0E8) 35     mov    a,05       ;0101_xxxx_xxxx
    00BB (0F4) 45     cmpsk  a,[hl] ;[00h]
    00BC (0FA) 3A      mov   a,0A       ;1010_xxxx_xxxx
    00BD (0BD) 45     cmpsk  a,[hl] ;[00h]
    00BE (09E) C2      jmp   00D8 ;@@not_0101_xxxx_xxxx_nor_1010_xxxx_xxxx
    00BF (08F) 52     incsk  l      ;L=1 (and no skip)
    00C0 (0C7) 45     cmpsk  a,[hl] ;[01h]     ;1010_1010_xxxx or 0101_0101_xxxx
    00C1 (0E3) C2      jmp   00D8 ;@@not_0101_0101_xxxx_nor_1010_1010_xxxx
    00C2 (0F1) 52     incsk  l      ;L=2 (and no skip)
    00C3 (0B8) 45     cmpsk  a,[hl] ;[02h]     ;1010_1010_1010 or 0101_0101_0101
    00C4 (0DC) C2      jmp   00D8 ;@@not_0101_0101_0101_nor_1010_1010_1010
                     ;- - -
                     ;CMD_0101_0101_0101   ;\aka bits toggle on/off repeatedly
                     ;CMD_1010_1010_1010   ;/
    00C5 (0EE) 2E     mov    l,0E                                            ;\
    00C6 (0B7) 48     clr    c                                               ;
    00C7 (0DB) 3F     mov    a,0F                                            ;
    00C8 (0ED) 72     adc    a,[hl]  ;[0Eh]                                  ;
    00C9 (0B6) 42     xchgsk a,[hl+] ;[0Eh], L=0Fh, and do not skip...       ;
    00CA (09B) 31     mov    a,01       ;uhm, why this? that will ADC 1+cy ? ;
    00CB (0CD) 72     adc    a,[hl]  ;[0Fh]                                  ;
    00CC (0A6) 4A     mov    [hl],a  ;[0Fh]                                  ;
    00CD (093) 54     testsk c                                               ;
    00CE (0C9) DA      jmp   00E5 ;@@to_want_next_cmd_bit                    ;/
    00CF (0A4) 7D 11  call   016B ;set_0Bh_to_zero  ;[0Bh]=0
    00D1 (0A9) 2D     mov    l,0D
    00D2 (094) 68     clr    [hl].0  ;[0Dh]         ;LED.flag
    00D3 (0CA) 20     mov    l,00
    00D4 (0A5) 55     in     a,[l]  ;PORT[0]
    00D5 (092) 08     addsk  a,08      ;bit3 to cy (LED)
    00D6 (089) B5      jmp   00B2 ;set_led_temporarily_and_want_fresh_command
    00D7 (084) D3     jmp    00E2 ;@@clr_led_temporarily_and_want_next_cmd_bit
                     ;---
                     @@not_0101_0101_0101_nor_1010_1010_1010:
    00D8 (0C2) 2B     mov    l,0B
    00D9 (0A1) 40     mov    a,[hl]  ;[0Bh]    ;offhold ?
    00DA (090) 01     addsk  a,01
    00DB (0C8) B4      jmp   00E4 ;@@apply_increased_led_offhold_and_want_next_cmd_bit  ;in: L=0Bh, and A=[0Bh]+1
    00DC (0E4) 4A     mov    [hl],a  ;[0Bh]
    00DD (0F2) 2D     mov    l,0D
    00DE (0B9) 6D     set    [hl].1  ;[0Dh]
    00DF (09C) 60     testsk [hl].0  ;[0Dh]         ;LED.flag
    00E0 (0CE) D3      jmp   00E2 ;@@clr_led_temporarily_and_want_next_cmd_bit
    00E1 (0A7) 96     jmp    00E7 ;@@set_led_temporarily_and_want_next_cmd_bit
                     ;---
                     @@clr_led_temporarily_and_want_next_cmd_bit:
    00E2 (0D3) 7D 16  call   0167 ;set_port_0_to_0   ;out: L=0, A=0
                     ;- - -
                     @@apply_increased_led_offhold_and_want_next_cmd_bit:
    00E4 (0B4) 4A     mov    [hl],a ;[00h]=0 ... OR ... [0Bh]=increased   ;uhm?
                     @@to_want_next_cmd_bit:
    00E5 (0DA) 78 0F  jmp    003F ;want_next_cmd_bit
                     ;---
                     @@set_led_temporarily_and_want_next_cmd_bit:
    00E7 (096) 7D 2D  call   0166 ;set_port_0_to_8
    00E9 (0C5) DA     jmp    00E5 ;@@to_want_next_cmd_bit
                     ;---
                     @@not_0010_xxxx_1010:
    00EA (0A2) 3B     mov    a,0B       ;0010_xxxx_1011
    00EB (091) 45     cmpsk  a,[hl] ;[02h]
    00EC (088) DA      jmp   00E5 ;@@to_want_next_cmd_bit ;@@not_0010_xxxx_1011
    00ED (0C4) 21     mov    l,01
    00EE (0E2) 63     testsk [hl].3 ;[01h]  ;0010_1xxx_1011                     ;\
    00EF (0B1) DA      jmp   00E5 ;@@to_want_next_cmd_bit ;@@not_0010_1xxx_1011 ; verify fixed bits
    00F0 (098) 61     testsk [hl].1 ;[01h]  ;0010_1x1x_1011                     ;
    00F1 (0CC) DA      jmp   00E5 ;@@to_want_next_cmd_bit ;@@not_0010_1x1x_1011 ;/
    00F2 (0E6) 79 B6  jmp    01C9 ;is_0010_1x1x_1011        ;-->
                     ;------------------
    00F4 (0D9) 00     nop           ;\
    00F5 (0AC) 00     nop           ;
    00F6 (0D6) 00     nop           ;
    00F7 (0AB) 00     nop           ;
    00F8 (0D5) 00     nop           ; unused (11 bytes)
    00F9 (0AA) 00     nop           ;
    00FA (095) 00     nop           ;
    00FB (08A) 00     nop           ;
    00FC (085) 00     nop           ;
    00FD (082) 00     nop           ;
    00FE (081) 00     nop           ;/
                     ;------------------
    00FF (0FF) 80     jmp    0080
                     ;------------------
                     zerofill_up_to_16_nibbles:   ;in: HL=dest, out: L=0
                     @@lop:               ;fill [xxh..0Fh] or [xxh..1Fh]
    0100 (100) 30     mov    a,00
    0101 (140) 42     xchgsk a,[hl+]  ;[xxh]
    0102 (160) 80      jmp   0100 ;@@lop
    0103 (170) 4C     ret
                     ;------------------
                     cmd_open_arms:
    0104 (178) 38     mov    a,08   ;open
                     ;- - - - - --------
                     cmd_close_arms:
    0105 (17C) 34     mov    a,04   ;close
    0106 (17E) 21     mov    l,01                   ;\apply motor direction
    0107 (13F) 46     out    [l],a  ;PORT[1]=4/8    ;/
    0108 (15F) 75     mov    h,01                   ;\
    0109 (16F) 2D     mov    l,0D                   ;
    010A (177) 3D     mov    a,0D                   ;
                     @@arm_delay_lop:               ;
    010B (17B) 4A     mov    [hl],a ;[1Dh]          ; delay
    010C (17D) 7D 3A  call   0119 ;do_long_delay    ;
    010E (11F) 75     mov    h,01                   ;
    010F (14F) 2D     mov    l,0D                   ;
    0110 (167) 40     mov    a,[hl] ;[1Dh]          ;
    0111 (173) 01     addsk  a,01                   ;
    0112 (179) FB      jmp   010B ;@@arm_delay_lop  ;/
                     ;- - - - - --------
                     stop_all_motors:
    0113 (13C) 30     mov    a,00                   ;\
    0114 (15E) 21     mov    l,01                   ;
    0115 (12F) 46     out    [l],a  ;PORT[1]=0      ; stop ALL motors
    0116 (157) 22     mov    l,02                   ;
    0117 (16B) 46     out    [l],a  ;PORT[2]=0      ;/
    0118 (175) 4C     ret
                     ;------------------
                     do_long_delay:
    0119 (13A) 7D 16  call   0167 ;set_port_0_to_0          ;-
    011B (10E) 75     mov    h,01                           ;\init ONE nibble of TWO-nibble-delay count
    011C (107) 2F     mov    l,0F ;[1Fh]                    ; uhm, but don't init [1Eh] here?
    011D (143) 7D 00  call   0100 ;zerofill_up_to_16_nibbles;/
                     @@lop1:       ;100h cycles             ;\
    011F (130) 24     mov    l,04  ;0Ch cycles     ;\       ;
                     @@lop2:                       ;        ;
    0120 (158) 30     mov    a,00              ;\  ;        ;
                     @@lop3:       ;10h cycles ;   ;        ;
    0121 (16C) 01     addsk  a,01              ;   ;        ;
    0122 (176) EC      jmp   0121 ;@@lop3      ;/  ;        ; long delay
    0123 (13B) 52     incsk  l                     ;        ; (loop 100h*0Ch*10h)
    0124 (15D) D8      jmp   0120 ;@@lop2          ;/       ;
    0125 (12E) 2E     mov    l,0E                       ;\  ;
                     @@carry_to_msb_lop:                ;   ;
    0126 (117) 40     mov    a,[hl]  ;[1Eh,1Fh]         ;   ;
    0127 (14B) 01     addsk  a,01                       ;   ;
    0128 (165) C6      jmp   012C ;@@no_carry           ;   ;
    0129 (132) 42     xchgsk a,[hl+] ;[1Eh,1Fh]         ;   ;
    012A (119) 97      jmp   0126 ;@@carry_to_msb_lop   ;/  ;
    012B (10C) 4C     ret                       ;--> done   ;
                     ;---                                   ;
                     @@no_carry:                            ;
    012C (146) 4A     mov    [hl],a  ;[1Eh,1Fh]             ;
    012D (123) B0     jmp    011F ;@@lop1                   ;/
                     ;------------------
                     move_right:
    012E (151) 31     mov    a,01   ;direction (right)
                     ;- - - - - - ------
                     move_left:
    012F (128) 32     mov    a,02   ;direction (left)
                     ;- - - - - - ------
                     move_left_or_right:
    0130 (154) 49     set    c      ;input bit   (for left/right)
    0131 (16A) 22     mov    l,02   ;output port (for left/right)
                     ;- - - - - - ------
                     move_up_or_down_or_left_or_right:          ;in: A,L,Cy
    0132 (135) 46     out    [l],a  ;PORT[1/2]=1/2  ;-run in desired direction
    0133 (11A) 7D 3A  call   0119 ;do_long_delay    ;-delay <---- this helps to
    0135 (106) 75     mov    h,01                   ;\            get AWAY from
    0136 (103) 2D     mov    l,0D                   ;             the CURRENT notch
    0137 (141) 34     mov    a,04          ;lopcount;
                     @@test_notch_lop:              ; now run until NEXT notch...
    0138 (120) 4A     mov    [hl],a ;[1Dh] ;lopcount;
    0139 (150) 7D 80  call   0180 ;test_notch_switch; --> test notch switch
    013B (174) CD      jmp   014B ;@@reached_notch  ;
    013C (17A) 75     mov    h,01                   ;
    013D (13D) 2D     mov    l,0D                   ;
    013E (11E) 40     mov    a,[hl] ;[1Dh] ;lopcount;
    013F (10F) 01     addsk  a,01                   ;
    0140 (147) A0      jmp   0138 ;@@test_notch_lop ;/
                     ;- - -
                     ;oops, timeout (this may happen if the mechanics are blocked,
                     ;or perhaps if batteries are too low to move the motor)...
    0141 (163) 30     mov    a,00                   ;\
    0142 (171) 21     mov    l,01                   ;
    0143 (138) 46     out    [l],a  ;PORT[1]=0      ; stop (timeout)
    0144 (15C) 22     mov    l,02                   ;
    0145 (16E) 46     out    [l],a  ;PORT[2]=0      ;/
    0146 (137) 74     mov    h,00                   ;\
    0147 (15B) 2A     mov    l,0A                   ; prevent another_step
    0148 (16D) 68     clr    [hl].0 ;[0Ah]          ; upon timeout
    0149 (136) 69     clr    [hl].1 ;[0Ah]          ;/
    014A (11B) 4D     retsk                         ;-return and skip upon timeout
                     ;---
                     @@reached_notch:
    014B (14D) 54     testsk c                      ;\
    014C (126) B4      jmp   0164 ;@@motor_port_1   ; set L=1/2
    014D (113) 22     mov    l,02         ;port_2   ;
                     @@this_motor_port:             ;/
    014E (149) 55     in     a,[l]  ;PORT[1/2]      ;\
    014F (124) 64     testsk a.0                    ; output REVERSE (to move back
    0150 (152) 31      mov   a,01                   ; to center of notch?)
    0151 (129) 32     mov    a,02                   ;
    0152 (114) 46     out    [l],a  ;PORT[1/2]=1/2  ;/
    0153 (14A) 20     mov    l,00  ;outer lopcount  ;\
    0154 (125) 30     mov    a,00  ;inner lopcount  ;
                     @@reverse_delay1: ;10h times   ;
                     @@reverse_delay2: ;10h times   ;
    0155 (112) 00     nop                           ; delay (10h*10h)
    0156 (109) 00     nop                           ;
    0157 (104) 00     nop                           ;
    0158 (142) 00     nop                           ;
    0159 (121) 00     nop                           ;
    015A (110) 01     addsk  a,01                   ;
    015B (148) 92      jmp   0155 ;@@reverse_delay2 ;
    015C (164) 52     incsk  l                      ;
    015D (172) 92      jmp   0155 ;@@reverse_delay1 ;/
    015E (139) BC     jmp    0113 ;stop_all_motors  ;-clear PORT 1/2 and RET
                     ;------------------
                     initial_move_up:
    015F (11C) 32     mov    a,02  ;up
    0160 (14E) 31      mov   a,01  ;down <-- blah/unused
                     ;- - - - - --------
                     move_up_or_down:   ;in; A=1/2
    0161 (127) 48     clr    c    ;input bit   (for up/down)
    0162 (153) 21     mov    l,01 ;output port (for up/down)
    0163 (169) B5     jmp    0132 ;move_up_or_down_or_left_or_right
                     ;------------------
                     @@motor_port_1:
                     ;(weirdly, this two opcodes are part of the function at 0132)
    0164 (134) 21     mov    l,01
    0165 (15A) C9     jmp    014E ;@@this_motor_port
                     ;------------------
                     set_port_0_to_8:
    0166 (12D) 38     mov    a,08
                     ;- - - - - --------
                     set_port_0_to_0:
    0167 (116) 30     mov    a,00
    0168 (10B) 20     mov    l,00
    0169 (145) 46     out    [l],a  ;PORT[0]=0/8    ;LED
    016A (122) 4C     ret
                     ;------------------
                     set_0Bh_to_zero:  ;[0Bh]=0   ;in: H=0
    016B (111) 2B     mov    l,0B
    016C (108) 30     mov    a,00
    016D (144) 4A     mov    [hl],a ;[0Bh]=0
    016E (162) 4C     ret
                     ;------------------
    016F (131) 00     nop           ;\
    0170 (118) 00     nop           ;
    0171 (14C) 00     nop           ;
    0172 (166) 00     nop           ;
    0173 (133) 00     nop           ;
    0174 (159) 00     nop           ;
    0175 (12C) 00     nop           ; unused (16 bytes)
    0176 (156) 00     nop           ;
    0177 (12B) 00     nop           ;
    0178 (155) 00     nop           ;
    0179 (12A) 00     nop           ;
    017A (115) 00     nop           ;
    017B (10A) 00     nop           ;
    017C (105) 00     nop           ;
    017D (102) 00     nop           ;
    017E (101) 00     nop           ;/
                     ;------------------
                     ;hmmm, why is this different as than at 007F/00FF/01FF, and,
                     ;does the rom-dump-self-test even allow to read xx7F/xxFF ?
    017F (17F) FF     jmp    017F
                     ;------------------
                     test_notch_switch:    ;in: Cy=motor (0=up/down, 1=left/right)
    0180 (180) 75     mov    h,01
    0181 (1C0) 2E     mov    l,0E
    0182 (1E0) 30     mov    a,00
    0183 (1F0) 42     xchgsk a,[hl+] ;[1Eh]=0, L=Fh, which will not skip...
    0184 (1F8) 30     mov    a,00
                     @@outmost_test_lop:
    0185 (1FC) 4A     mov    [hl],a  ;[1Fh,1Eh]
    0186 (1FE) 30     mov    a,00         ;number of switch-hits
                     @@test_retry_again:
    0187 (1BF) 5C     mov    x,a          ;number of switch-hits
    0188 (1DF) 20     mov    l,00                           ;\
    0189 (1EF) 55     in     a,[l]  ;PORT[0]                ;
    018A (1F7) 54     testsk c                              ; test port, bit1/bit2
    018B (1FB) E1      jmp   019E ;@@test_bit2    ;cy=0 --> ; --> test bit2 (up/dn)
    018C (1FD) 65     testsk a.1    ;bit1:PORT[0] ;cy=1 --> ; --> test bit1 (lt/rt)
    018D (1BE) 9D      jmp   019A ;@@test_hit               ;
                     @@cont_next:                           ;/
                     ;- - - - -
                     @@outer_delay:                         ;\
    018E (19F) 3D     mov    a,0D                   ;\3x    ; 16x
                     @@inner_delay:                 ; inner ; outer
    018F (1CF) 01     addsk  a,01                   ; delay ; delay
    0190 (1E7) CF      jmp   018F ;@@inner_delay    ;/      ;
    0191 (1F3) 52     incsk  l                              ;
    0192 (1F9) 9F      jmp   018E ;@@outer_delay            ;/
    0193 (1BC) 2E     mov    l,0E                           ;\
                     @@carry_to_msb_lop:                    ;
    0194 (1DE) 40     mov    a,[hl]  ;[1Fh,1Eh]             ; raise 2x4bit
    0195 (1AF) 01     addsk  a,01                           ; lopcount
    0196 (1D7) FC      jmp   0185 ;@@outmost_test_lop       ;
    0197 (1EB) 42     xchgsk a,[hl+] ;[1Fh,1Eh]             ;
    0198 (1F5) DE      jmp   0194 ;@@carry_to_msb_lop       ;/
    0199 (1BA) 4D     retsk         ;RET with SKIP
                     ;---
                     @@test_hit:
    019A (19D) 5D     xchg   x,a          ;number of switch-hits
    019B (18E) 01     addsk  a,01         ;increase
    019C (187) BF      jmp   0187 ;@@test_retry_again   ;not yet 16 switch-hits
    019D (1C3) 4C     ret           ;RET without SKIP   ;okay got 16 switch-hits
                     ;---
                     @@test_bit2:
    019E (1E1) 66     testsk a.2    ;bit2:PORT[0]
    019F (1B0) 9D      jmp   019A ;@@test_hit
    01A0 (1D8) 9F     jmp    018E ;@@cont_next
                     ;------------------
                     skip_if_dark:         ;out: Cy,Skip,FixedExecutionTiming
    01A1 (1EC) 20     mov    l,00
    01A2 (1F6) 55     in     a,[l]  ;PORT[0]        ;\
    01A3 (1BB) 64     testsk a.0                    ;
    01A4 (1DD) 97      jmp   01A6 ;@@has_light_1    ;
    01A5 (1AE) BD     jmp    01BD ;@@got_dark_1     ;
                     ;---                           ;
                     @@has_light_1:                 ;/
    01A6 (197) 55     in     a,[l]  ;PORT[0]        ;\
    01A7 (1CB) 64     testsk a.0                    ;
    01A8 (1E5) 99      jmp   01AA ;@@has_light_2    ;
    01A9 (1B2) 9E     jmp    01BE ;@@got_dark_2     ;
                     ;---                           ;
                     @@has_light_2:                 ;/
    01AA (199) 55     in     a,[l]  ;PORT[0]        ;\
    01AB (18C) 64     testsk a.0                    ;
    01AC (1C6) D1      jmp   01AE ;@@has_light_3    ;
    01AD (1A3) 8F     jmp    01BF ;@@got_dark_3     ;
                     ;---                           ;
                     @@has_light_3:                 ;/
    01AE (1D1) 55     in     a,[l]  ;PORT[0]        ;\
    01AF (1A8) 64     testsk a.0                    ;
    01B0 (1D4) B5      jmp   01B2 ;@@has_light_4    ;
    01B1 (1EA) C7     jmp    01C0 ;@@got_dark_4     ;
                     ;---                           ;
                     @@has_light_4:                 ;/
    01B2 (1B5) 55     in     a,[l]  ;PORT[0]        ;\
    01B3 (19A) 64     testsk a.0                    ;
    01B4 (18D) 83      jmp   01B6 ;@@has_light_5    ;
    01B5 (186) E3     jmp    01C1 ;@@got_dark_5     ;
                     ;---                           ;
                     @@has_light_5:                 ;/
    01B6 (183) 55     in     a,[l]  ;PORT[0]        ;\
    01B7 (1C1) 64     testsk a.0                    ;
    01B8 (1A0) E8      jmp   01BA ;@@has_light_6    ;
    01B9 (1D0) F1     jmp    01C2 ;@@got_dark_6     ;
                     ;---                           ;
                     @@has_light_6:                 ;/
    01BA (1E8) 00     nop         ;delay
    01BB (1F4) 49     set    c    ;CY=1 (light) (continous light on seen 6 times)
    01BC (1FA) 4C     ret         ;ret without skip
                     ;--- --- ---
                     @@got_dark_1:
    01BD (1BD) 3A     mov    a,0A   ;delay count -6
                     @@got_dark_2:
    01BE (19E) 3B     mov    a,0B   ;delay count -5
                     @@got_dark_3:
    01BF (18F) 3C     mov    a,0C   ;delay count -4
                     @@got_dark_4:
    01C0 (1C7) 3D     mov    a,0D   ;delay count -3
                     @@got_dark_5:
    01C1 (1E3) 3E     mov    a,0E   ;delay count -2
                     @@got_dark_6:
    01C2 (1F1) 3F     mov    a,0F   ;delay count -1
                     @@got_dark_delay:              ;\delay (to maintain
    01C3 (1B8) 01     addsk  a,01                   ; fixed execution time)
    01C4 (1DC) B8      jmp   01C3 ;@@got_dark_delay ;/
    01C5 (1EE) 48     clr    c    ;CY=0 (dark)
    01C6 (1B7) 4D     retsk       ;SKIP
                     ;------------------
    01C7 (1DB) 00     nop           ;\unused (2 bytes)
    01C8 (1ED) 00     nop           ;/
                     ;------------------
                     is_0010_1x1x_1011:
    01C9 (1B6) 60     testsk [hl].0 ;[01h]  ;0010_1x11_1011
    01CA (19B) 92      jmp   01D5     ;@@not_0010_1x11_1011
    01CB (1CD) 62     testsk [hl].2 ;[01h]  ;0010_1111_1011
    01CC (1A6) A9      jmp   01D1     ;@@not_0010_1111_1011
                      ;CMD_0010_1111_1011 --> down large step
    01CD (193) 2A     mov    l,0A
    01CE (1C9) 6C     set    [hl].0 ;[0Ah] ;nonzero=request another_step (value=1=down)
    01CF (1A4) 78 D8  jmp    00A0 ;move_down
                     ;---
                     @@not_0010_1111_1011:  ;aka is 0010_1011_1011
                      ;CMD_0010_1011_1011 --> up large step
    01D1 (1A9) 2A     mov    l,0A
    01D2 (194) 6D     set    [hl].1 ;[0Ah] ;nonzero=request another_step (value=2=up)
    01D3 (1CA) 78 EC  jmp    00A1 ;move_up
                     ;---
                     @@not_0010_1x11_1011:  ;aka is 0010_1x10_1011
    01D5 (192) 62     testsk [hl].2 ;[01h]  ;0010_1110_1011
    01D6 (189) C8      jmp   01DB     ;@@not_0010_1110_1011
                     ;CMD_0010_1110_1011 --> turn LED on permanently
    01D7 (184) 2D     mov    l,0D
    01D8 (1C2) 6C     set    [hl].0 ;[0Dh]         ;LED.flag
    01D9 (1A1) 78 0F  jmp    003F ;want_next_cmd_bit
                     ;---
                     @@not_0010_1110_1011:  ;aka is 0010_1010_1011
                     ;CMD_0010_1010_1011 --> reset
    01DB (1C8) 2D     mov    l,0D
    01DC (1E4) 6C     set    [hl].0 ;[0Dh]         ;LED.flag
    01DD (1F2) 78 5F  jmp    0008 ;softreset_entrypoint
                     ;------------------
    01DF (19C) 00     nop           ;\unused (2 bytes)
    01E0 (1CE) 00     nop           ;/
                     ;------------------
                     process_cmd_if_any:
    01E1 (1A7) 20     mov    l,00
    01E2 (1D3) 54     testsk c                              ;\test 13th bit
    01E3 (1E9) DA      jmp   01E5 ;@@is__0_xxxx_xxxx_xxxx   ; (aka carry-out from
    01E4 (1B4) 82     jmp    01FD ;@@not_0_xxxx_xxxx_xxxx   ; 3x4bit cmd buffer)
                     @@is__0_xxxx_xxxx_xxxx:                ;/
    01E5 (1DA) 32     mov    a,02       ;0010_xxxx_xxxx     ;\
    01E6 (1AD) 45     cmpsk  a,[hl] ;[00h]                  ; check start bits
    01E7 (196) 82      jmp   01FD ;@@not_0010_xxxx_xxxx     ;/
    01E8 (18B) 22     mov    l,02                           ;-now end bits
    01E9 (1C5) 3E     mov    a,0E       ;0010_xxxx_1110     ;\
    01EA (1A2) 45     cmpsk  a,[hl] ;[02h]                  ;
    01EB (191) 8A      jmp   01FB ;@@not_0010_xxxx_1110     ;/
    01EC (188) 21     mov    l,01                           ;-now middle bits
                     ;mov    a,0E       ;0010_1110_1110     ;\
    01ED (1C4) 45     cmpsk  a,[hl] ;[01h]                  ;
    01EE (1E2) E6      jmp   01F2 ;@@not_0010_1110_1110     ;/
    01EF (1B1) 7D 78  call   0104 ;cmd_open_arms            ;--> CMD_0010_1110_1110 --> open arms
    01F1 (1CC) AB     jmp    01F7 ;@@to_set_led_and_want_fresh_command
                     ;---
                     @@not_0010_1110_1110:  ;in: HL=01h
    01F2 (1E6) 3B     mov    a,0B       ;0010_1011_1110     ;\
    01F3 (1B3) 45     cmpsk  a,[hl] ;[01h]                  ;
    01F4 (1D9) AA      jmp   01F9 ;@@not_0010_1011_1110     ;/
    01F5 (1AC) 7D 7C  call   0105 ;cmd_close_arms           ;--> CMD_0010_1011_1110 --> close arms
                     @@to_set_led_and_want_fresh_command:
    01F7 (1AB) 78 D1  jmp    00AE ;set_led_and_want_fresh_command
                     ;---
                     @@not_0010_1011_1110:  ;in: HL=01h
    01F9 (1AA) 78 F8  jmp    0084 ;not_0010_1011_1110_cont
                     ;---
                     @@not_0010_xxxx_1110:  ;in: HL=02h
    01FB (18A) 78 DF  jmp    0088 ;not_0010_xxxx_1110_cont
                     ;---
                     @@not_0_xxxx_xxxx_xxxx:
                     @@not_0010_xxxx_xxxx:
    01FD (182) 78 D0  jmp    00B9 ;not_0_0010_xxxx_xxxx_cont
                     ;------------------
    01FF (1FF) 80     jmp    0180
                     ;------------------
    
