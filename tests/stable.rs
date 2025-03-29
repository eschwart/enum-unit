use enum_unit::*;

#[allow(dead_code)]
#[derive(EnumUnit)]
enum Example {
    A,
    B(u8),
    C { id: &'static str },
}

#[test]
fn test_stable() {
    let a = Example::A;
    let b = Example::B(69);
    let c = Example::C { id: "Unit" };

    assert_eq!(a.kind(), ExampleUnit::A);
    assert_eq!(b.kind(), ExampleUnit::B);
    assert_eq!(c.kind(), ExampleUnit::C);
}

#[test]
#[cfg(feature = "bitflags")]
fn test_bitflags() {
    let ab = ExampleUnit::A | ExampleUnit::B;
    let ac = ExampleUnit::A | ExampleUnit::C;
    let bc = ExampleUnit::B | ExampleUnit::C;

    let a = ab - ExampleUnit::B;
    let b = bc - ExampleUnit::C;
    let c = ac - ExampleUnit::A;

    assert_eq!(a, ExampleUnit::A);
    assert_eq!(b, ExampleUnit::B);
    assert_eq!(c, ExampleUnit::C)
}

#[test]
#[cfg(feature = "serde")]
fn test_serde() {
    let a = Example::A.kind();

    let serialized = serde_json::to_string(&a).unwrap();
    let deserialized: ExampleUnit = serde_json::from_str(&serialized).unwrap();

    assert_eq!(a, deserialized);
}

#[test]
#[cfg(all(feature = "bitflags", feature = "serde"))]
fn test_serde_bitflags() {
    let ab = Example::A.kind() | Example::B(2).kind();

    let ab_serialized = serde_json::to_string(&ab).unwrap();
    let ab_deserialized: ExampleUnit = serde_json::from_str(&ab_serialized).unwrap();

    assert_eq!(ab, ab_deserialized)
}
