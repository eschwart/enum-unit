# enum-unit

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