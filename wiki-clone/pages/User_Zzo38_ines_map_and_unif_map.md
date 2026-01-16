# User:Zzo38/ines.map and unif.map

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/ines.map_and_unif.map) | View [other pages](Special_AllPages.xhtml#User_Zzo38_ines_map_and_unif_map)

This file describes the format of `ines.map` and `unif.map` files used with [DotFami](User_Zzo38_DotFami.xhtml "User:Zzo38/DotFami"). 

The file is a plain ASCII file which is preprocessed with the C preprocessor and uses a C-like syntax. All constant operators (other than `sizeof`) are the same as in C, including the same precedence. 

All data is signed 64-bit integers. ASCII character constants can be used, and you can use decimal, hexadecimal, and octal constants, same as in C. 

## Contents

  * 1 Keywords
  * 2 Outer Codes
  * 3 Inner Codes
  * 4 Cases
  * 5 Variables
  * 6 Procedures
  * 7 Functions
  * 8 Expressions
  * 9 iNES Flags
  * 10 UNIF Flags
  * 11 Miscellaneous
  * 12 Errors



## Keywords
    
    
    crc
    default
    export
    flags
    function
    if
    import
    local
    mapper
    parameter
    procedure
    rom
    romsize
    submapper
    switch
    use
    

## Outer Codes

At the top (outer) level of the file, the following statements are allowed: 

  * variable `=` expression `;`
  * `parameter` expression `=` expression `;`
  * `use` string `;`
  * `if(` condition `)` statement
  * `if(` condition `)` statement `else` statement
  * `{` statements `}`
  * `switch(` expression `) {` cases `}`
  * `import` string `(` parameters `);`
  * `function` name `(` parameters `) {` statements `}`
  * `procedure` name `(` parameters `) {` statements `}`
  * procedurename `(` parameters `);`
  * export `(` names `);`



## Inner Codes

Where statement(s) are expected at inner level, the following is acceptable: 

  * variable `=` expression `;`
  * `parameter` expression `=` expression `;`
  * `use` string `;`
  * `if(` condition `)` statement
  * `if(` condition `)` statement `else` statement
  * `{` statements `}`
  * `switch(` expression `) {` cases `}`
  * `import` string `(` parameters `);`
  * `local` variables (only inside a `{}` block)
  * procedurename `(` parameters `);`



## Cases

A case in a switch is formatted as either a string literal or a constant expression, followed by `{` statements `}`

The last case must be `default` instead of an expression and contains statements if none of the other cases match. 

## Variables

  * Each variable may only be assigned once and must be assigned before used.
  * Some variables can be local to a block, which case they are accessible only within that block.
  * If a condition or switch statement contains blocks which assign variables not local to those blocks, then all cases must assign the same set of variables (not counting variables local to those blocks).
  * A procedure may also assign global variables.
  * Variables store 64-bit values.
  * Parameters to procedures and functions are also acting like local variables which are already assigned.
  * Strings can also be assigned to variables although they cannot be used as a number if they do.



## Procedures

  * Procedures must be declared before used.
  * Recursive calls are not allowed.
  * Same rules for variables, `use` statements, and `parameter` statements apply to procedures, just as if its contents are placed where the procedure is called from.



## Functions

  * Functions must be declared before used.
  * Recursive calls are not allowed.
  * A function call is made inside of an expression.
  * Only local variables may be assigned in a function.
  * Procedure calls are not allowed in a function.
  * The `use` and `parameter` statements are not allowed in a function.
  * The last statement (even inside a `if` or `switch`) should be an expression by itself which is the return value of this function.



## Expressions

  * `mapper`: For iNES, the mapper number. For UNIF, the mapper name (as a string). Negative one for NSF.
  * `submapper`: NES 2.0 submapper number (0 for old iNES files and for UNIF). For NSF, the contents of the extra sound chip support byte (address $07B in the .NSF header).
  * `flags`: Used in both iNES and UNIF; includes flags such as battery RAM, mirroring, and so on. (For NSF, always zero.)
  * `romsize` expression: The expression a number 0 to 15 for PRG ROM, or 16 to 31 for CHR ROM. This indicates the size in bytes. (iNES uses only 0 and 16 and all other ROM sizes will be zero; NSF uses only 0.)
  * `rom` expression: The expression is address (starting at PRG ROM 0, ..., CHR ROM 15) and returns the data byte at that address. (For NSF, negative values allow access to the header, where -128 accesses the first byte of the header (the value will always be 78).)
  * `crc` expression: Expression is same as for `romsize` but result in CRC32 of ROM.
  * Strings can be compared using `==` and `!=` only, and arithmetic may not be performed on strings.
  * name `(` parameters `)`: Make a function call.



## iNES Flags

  * bit3-bit0: Low four bits of iNES Flags 6.
  * bit7-bit4: Low four bits of iNES Flags 7. If the two unused bits of iNES are 10 then use NES 2.0 format; if 01 or 11 then treat this entire byte and all following as zero (so, also the remaining iNES flags in ines.map); if 00 then use old iNES format, using Flags 7 but ignoring the rest of the header.
  * bit11-bit8: Non-battery PRG RAM size (NES 2.0 RAM size lookup table).
  * bit15-bit12: Battery PRG RAM size (NES 2.0 RAM size lookup table).
  * bit19-bit16: Non-battery CHR RAM size (NES 2.0 RAM size lookup table).
  * bit23-bit20: Battery CHR RAM size (NES 2.0 RAM size lookup table).



## UNIF Flags

  * bit7-bit0: Mirroring setting.
  * bit8: Presence of UNIF "`BATR`" chunk.
  * bit9: Presence of UNIF "`VROR`" chunk.
  * bit10: Presence of UNIF "`MIRR`" chunk.



## Miscellaneous

The `parameter` statement is optional and the first expression is the parameter number (0 to 255 only); each parameter can be assigned a maximum of one time, but it is not necessary that if it is assigned in one case that it must be assigned in the other cases as well. 

The `use` statement must occur exactly once, using the same restrictions as assignment to variables. The string parameter can be a variable storing a string, or a string literal, which specifies the filename for a `.cart` file. 

The `import` statement is considered to be a `use` statement for the purpose of where it can occur. The string parameter can be a variable storing a string, or a string literal, which specifies the filename for another file having the same format as `ines.map` or `unif.map` files. The contents of the file is executed, but in its own namespace. The parameters are assigned to the variables in the imported file which are specified using the `export` statement. 

The `export` statement declares variables, and must occur exactly once in a imported file, and none in the `ines.map` and `unif.map` file itself. The number of variables declared must match the ones specified in the `import` statement. 

## Errors

In case of division by zero or anything that violates the specifications here, there is no guarantee of what it will do, including it might crash. However, it might just display an error message instead. 
