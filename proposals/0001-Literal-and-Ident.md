# Identifiers, Encoding, and Literals

## Identifiers

In DustLang, identifiers must meet the following requirements:

- Identifiers cannot contain ASCII symbols.
- The first character of an identifier cannot be an ASCII digit.
- The first character of an identifier can be an underscore `_`.
- Identifiers can contain underscores `_`.
- Identifiers cannot contain Unicode whitespace characters.
- Apart from the above restrictions, identifiers can use any Unicode characters.

## Encoding

DustLang files are encoded in UTF-8 format, which means literals are also compiled into UTF-8 format.

## Literals

Literals are values directly written in the source code. These values, except for numbers, will be compiled into the global static variable area.

> Even numbers can be written to the global static variable area depending on the implementation on different platforms.

DustLang supports Compiler Macro Interfaces(CMI) to extend various forms of literals.

> Note: These CMI are available at least at the bootstrap level 1.

### Numeric Literals

Numeric literals start with an ASCII digit. If the first character is 1-9, it represents a regular decimal number. If the first character is 0, there are several possibilities:

- If the second character is a decimal point `.` it represents a floating-point number.
- If the second character is `x`, it represents a hexadecimal integer.
- If the characters following the second character form an identifier, it is determined by the injected code logic of the compiler library interface.

Numeric literals can have `_` as a separator, which does not affect the semantics of the integer.

> Bootstrap 0 does not support separators.

#### Integers

Apart from characters related to integer bases, no other characters are allowed.

- Decimal: Only `0-9_` are allowed.
- Hexadecimal: `Only 0-9A-Fa-f_` are allowed.

#### Floating-Point Numbers

> Some backends may not support floating-point number types, and compilation will result in an error.

Floating-point numbers come in two forms: decimal and scientific notation.

- Decimal form allows only `0-9._`.
- Scientific notation form allows only `0-9._Ee`.

### String Literals

String literals start with an identifier followed by a double quote, or they can be directly enclosed in double quotes.

#### UTF-8 Strings

UTF-8 strings are directly enclosed in double quotes:

```
"这是一个UTF-8字符串"
"Emoji 😀"
```

#### Byte Strings

Byte strings are identified with a b prefix or an x prefix.

    ASCII byte strings start with the b prefix. Only ASCII characters are allowed within double quotes.
    Hexadecimal strings start with the x prefix. Only 0-9a-zA-Z characters are allowed within double quotes.
    Apart from these two prefixes, new identifiers can be injected through the compiler library interface.

### Character Literals

Character literals start with an identifier followed by a single quote, or they can consist of a single quote. The single quotes enclose a UTF-8 character.

```
'a'
'好'
```

> Similarly, character literals support extensions by placing an identifier before the quote.

## Pest Grammar

```
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }

// ident
ident_first = _{ LETTER | "_" }
ident_inner = _{ ident_first | NUMBER }
ident       =  { ident_first ~ ident_inner* }

// number
num_literal_inner = _{ LETTER | NUMBER | "." | "_" }
num_literal  =  { NUMBER ~ num_literal_inner*}

// string
literal_inner_char = _{ !"\"" ~ ("\\\"" | ANY) }
literal_inner      =  { literal_inner_char* }
string_literal = { ident? ~ "\"" ~ literal_inner ~ "\"" }

// char
char_literal = { "'" ~ LETTER? ~ "'" }

literal = { string_literal | num_literal }
```

### Compiler Macros Interfaces

- `@cii::literal::int`: Integer
- `@cii::literal::str`: String
- `@cii::literal::char`: Character
