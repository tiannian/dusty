# Syntax

## Embedded Language

### C

```
inline_c! {
    // C Code here
}
```

## Define

### Type

```
u1 @= inline_c! {uint8_t};
```

### Struct

#### Pure Struct

```
SimpleStruct {
  type: u8,
  type2: u8,
}
```

#### Enum Struct

```
SimpleEnum {
  Enum1,
  Enum2,
}
```

#### Complex Enum Struct

```
ComplexEnum {
  type: u8,
  Type1 { v1: u8 }
  Type2 { v1: u8 }
}
```

### Function
```
func1(arg: u8) -> u8 {
  1
}
```
