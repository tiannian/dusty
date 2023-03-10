# DustyLIR

Dusty Lower IR is a subset of Dusty.

## Syntax

### Statement

#### Bound / Move

```go
a := 2

a: u8 = 2

b := $a

b := &a

b = 1
```

#### FunctionCall

```
b := function1(a)
```

### NewTypeStruct

#### Define

```
Simple { _: u8 }
```

#### Initialization

```
simple: Simple = { _: u8 }
```

### Function

```
add_u8(a: u8, b: u8) -> u8 {
  inline_c! { a + b }
}
```

### Array

```
a := [0; 9]
```

### Flow Control

#### If

```
eq_u8(a, 1) {} {}

a {
  1: {}
  _: {}
}

```

#### Loop

```
;; {}
;lt(a, 8); {}
```

### Return

```
<= 1
```

#### Break

```
<- 3
```

### Module

#### Declare

```
string {}
```

#### Import

```
=> core::abc::{c, d};
```





