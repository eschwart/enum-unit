# enum-unit
[![Crate](https://img.shields.io/crates/v/enum-unit.svg)](https://crates.io/crates/enum-unit)

## Description
Generates unit-variant enums from existing enums.

## Example
```rust
use enum_unit::*;

#[derive(EnumUnit)]
enum Example {
    A(i16),
    B { id: u8 },
}

fn main() {
    let a = Example::A(-420);
    assert_eq!(ExampleUnit::A, a.into());

    let b = Example::B { id: 69 };
    assert_eq!(ExampleUnit::B, b.into());
}
```

## Optional Features
| Name | Description |
| ---- | ----------- |
| [bitflags](https://crates.io/crates/bitflags) | Units have [bitflag](https://docs.rs/bitflags/2.8.0/bitflags/#generating-flags-types) functionality. |
| [serde](https://crates.io/crates/serde) | Units implement `Serialize` and `Deserialize`. |
| `full` | `bitflags` + `serde`