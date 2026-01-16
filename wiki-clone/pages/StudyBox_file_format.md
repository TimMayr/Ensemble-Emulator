# StudyBox file format

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/StudyBox_file_format) | View [other pages](Special_AllPages.xhtml#StudyBox_file_format)

The StudyBox file format contains the mono audio and decoded data of a single cassette tape for the [Fukutake StudyBox](Fukutake_Study_Box.xhtml "Fukutake StudyBox") in a chunked format. The file extension is ".studybox". It starts with the following file header: 
    
    
    Offset Length          Meaning
    $0     char[4]         "STBX" identifier
    $4     uint32_le       Length of all following fields ($00000004)
    $8     uint32_le       Version number *0x100 (0x100 =1.00)
    

The header must be followed by decoded PAGE data chunks. A page chunk has the following format: 
    
    
    Offset Length          Meaning
    $0     char[4]         "PAGE" identifier
    $4     uint32_le       Length of all following fields
    $8     uint32_le       Audio offset in samples into the audio chunk at which the lead-in section (0-bits followed by a single 1-bit) originally began
    $C     uint32_le       Audio offset in samples into the audio chunk at which the data originally began
    $10    uint8[]         Decoded data for this page (size varies)
    

Each page chunk's audio offset must be greater than the previous page chunk's audio offset. 

The page chunks must be followed by an AUDI chunk representing the audio portion of the cassette tape: 
    
    
    Offset Length          Meaning
    $0     char[4]         "AUDI" identifier
    $4     uint32_le       Length of all following fields
    $8     uint32_le       File type
                           $0: .WAV
                           $1: .flac
                           $2: .ogg
                           $3: .mp3
    $C     uint8[]         Embedded audio file
    

The audio chunk must come after the page chunks. Since the "Audio offset" field in the page chunks refer to samples in this audio, trimming the audio or changing the sampling rate will require changing the audio offsets of all the page chunks. 

Categories: [File formats](Category_File_formats.xhtml)
