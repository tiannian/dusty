# C lite

C-lite is a subset of C. It will be used as an IR.

## Definition

### Basic Type

#### Integer

```
u8 { _: ilc!(uint8_t) }

add_u8_u8(x u8, y u8) u8 {
  ilc!(x + y)
}
```

- uintX_t: 8 / 16 / 32 / 64
- intX_t: 8 / 16 / 32 / 64

#### Float

```
f32 { _: ilc!(float) }

add_float_float(x f32, y f32) f32 {
  ilc!(x + y)
}
```

- float / double

#### Size Type

```
usize { _: ilc!(size_t) }
isize { _: ilc!(ptrdiff_t) }
```

#### Char and Str

```
char { _: ilc!(const char*) }
str { _: ilc!(const char*) }
```

## Syntax

### Bound



## Target

### Host

### Freestanding

