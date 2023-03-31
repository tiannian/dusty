# DustyLIR

Dusty Lower IR is a subset of Dusty.

It can direct convert to compile target (C-lite, yul or other assembly).

## Syntax

### Statement

#### Bound / Move

```go
a: u8 = 2;

a = 1;
```

DustyIR don't must declare type.

#### Semicolon

Each expression can use semicolon to convert into statement.

### Expression

Each expression have a return value, can use in Bound or Move statement right value.

#### Bound

Bound in right value, mean this bound has moved.

#### Literal

- Number: `[1-9][0-9,.]+`
- Integer
  - Dec: `[1-9][0-9,]+`
  - Hex: `0x[0-9a-fA-F,]+`
- Character: `.|\\r|\\t|\\n`
- String: `([^"\r\n\\] | '\\' .)*`

#### Function Calll

Call function

```rust
functioncall(1, 2, 3)
```

#### Inital list and Flow control

refer to.

#### Block

Block is made up by Statement. The value of block is the latest expression or break operation.

Lasest expression as VALUE

```rust
{
  // Statements ...
  1
}
```

Break operation as VALUE

```rust
{
  // Statements ...
  <- 1;
  // Other statements ...
}
```

### Types

#### Type with default field

Definition:

```
NewTypeU8 { _: u8 }
```

DustyIR only support `_` as default field.

This means alias for types.

Initialization:

```
value: NewTypeU8 = 1;
```

#### Array

Same value initialize:

```rust
array: [u8; 32] = [0; 32];
```

Element initialize:

```rust
array: [u8; 3] = [1, 2, 3];
```

### Flow Control

#### Match

```
a :u8 = 1;

a => { /* 1 */ }
  => { /* 2 */ }
  => 3 { /* 3 */ }
  => 4 | 5 { /* 4 or 5*/ }
  => _ { /* other value of u8 */ }
```

#### Loop

```
(a: u8 = 0; isequal(&a, 7); incr($a)) {}
```

The first field is Bound; Second must be true.

### Function

Definition:

```
// Without return value
function1(a: u8) {}

// With return value
function2(a: u8) u8 {
  <= 1;
}

// With return value
function3(a: u8) u8 {
  1
}
```

### File Level

Only have this syntax elements:

- function definition
- class definition
- bound as const value
- import

#### Import

```
=> mapping::path1::path2;

=> mapping::{path1, path2};
```


