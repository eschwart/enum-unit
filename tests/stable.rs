#![allow(dead_code)]
#![allow(unused_imports)]

use enum_unit::*;

// === Types under test ===

#[derive(EnumUnit)]
enum ExampleEnum {
    A,
    B(u8),
    C { id: &'static str },
}

#[derive(EnumUnit)]
struct ExampleNamedStruct {
    a: (),
    b: u8,
    c: &'static str,
}

#[derive(EnumUnit)]
struct ExampleUnnamedStruct((), u8, &'static str);

// This is a valid derive but needs no test.
#[derive(EnumUnit)]
struct ExampleUnitStruct;

// === Test constants / constructors ===

const EX_ENUM_A: ExampleEnum = ExampleEnum::A;
const EX_ENUM_B: ExampleEnum = ExampleEnum::B(69);
const EX_ENUM_C: ExampleEnum = ExampleEnum::C { id: "Unit" };

// === Tests ===

mod enum_tests {
    use super::*;

    #[test]
    fn test_kind() {
        assert_eq!(EX_ENUM_A.kind(), ExampleEnumUnit::A);
        assert_eq!(EX_ENUM_B.kind(), ExampleEnumUnit::B);
        assert_eq!(EX_ENUM_C.kind(), ExampleEnumUnit::C);
    }

    #[test]
    #[cfg(feature = "bitflags")]
    fn test_bitflags() {
        let ab = ExampleEnumUnit::A | ExampleEnumUnit::B;
        let ac = ExampleEnumUnit::A | ExampleEnumUnit::C;
        let bc = ExampleEnumUnit::B | ExampleEnumUnit::C;

        let a = ab - ExampleEnumUnit::B;
        let b = bc - ExampleEnumUnit::C;
        let c = ac - ExampleEnumUnit::A;

        assert_eq!(a, ExampleEnumUnit::A);
        assert_eq!(b, ExampleEnumUnit::B);
        assert_eq!(c, ExampleEnumUnit::C);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde() {
        let a = EX_ENUM_A.kind();

        let serialized = serde_json::to_string(&a).unwrap();
        let deserialized: ExampleEnumUnit = serde_json::from_str(&serialized).unwrap();

        assert_eq!(a, deserialized);
    }

    #[test]
    #[cfg(all(feature = "bitflags", feature = "serde"))]
    fn test_serde_bitflags() {
        let ab = EX_ENUM_A.kind() | EX_ENUM_B.kind();

        let serialized = serde_json::to_string(&ab).unwrap();
        let deserialized: ExampleEnumUnit = serde_json::from_str(&serialized).unwrap();

        assert_eq!(ab, deserialized);
    }
}

mod struct_named_tests {
    use super::*;

    #[test]
    #[cfg(feature = "bitflags")]
    fn test_bitflags() {
        let ab = ExampleNamedStructUnit::A | ExampleNamedStructUnit::B;
        let ac = ExampleNamedStructUnit::A | ExampleNamedStructUnit::C;
        let bc = ExampleNamedStructUnit::B | ExampleNamedStructUnit::C;

        let a = ab - ExampleNamedStructUnit::B;
        let b = bc - ExampleNamedStructUnit::C;
        let c = ac - ExampleNamedStructUnit::A;

        assert_eq!(a, ExampleNamedStructUnit::A);
        assert_eq!(b, ExampleNamedStructUnit::B);
        assert_eq!(c, ExampleNamedStructUnit::C);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde() {
        let b = ExampleNamedStructUnit::B;

        let serialized = serde_json::to_string(&b).unwrap();
        let deserialized: ExampleNamedStructUnit = serde_json::from_str(&serialized).unwrap();

        assert_eq!(b, deserialized);
    }

    #[test]
    #[cfg(all(feature = "bitflags", feature = "serde"))]
    fn test_serde_bitflags() {
        let ab = ExampleNamedStructUnit::A | ExampleNamedStructUnit::B;

        let serialized = serde_json::to_string(&ab).unwrap();
        let deserialized: ExampleNamedStructUnit = serde_json::from_str(&serialized).unwrap();

        assert_eq!(ab, deserialized);
    }
}

mod struct_unnamed_tests {
    use super::*;

    #[test]
    #[cfg(feature = "bitflags")]
    fn test_unnamed_struct_bitflags() {
        // Combine the synthetic flags into a bitflags variable
        let f0f1 = ExampleUnnamedStructUnit::F0 | ExampleUnnamedStructUnit::F1;
        let f1f2 = ExampleUnnamedStructUnit::F1 | ExampleUnnamedStructUnit::F2;
        let f0f2 = ExampleUnnamedStructUnit::F0 | ExampleUnnamedStructUnit::F2;

        // Remove flags to isolate individual ones
        let f0 = f0f1 - ExampleUnnamedStructUnit::F1;
        let f1 = f1f2 - ExampleUnnamedStructUnit::F2;
        let f2 = f0f2 - ExampleUnnamedStructUnit::F0;

        assert_eq!(f0, ExampleUnnamedStructUnit::F0);
        assert_eq!(f1, ExampleUnnamedStructUnit::F1);
        assert_eq!(f2, ExampleUnnamedStructUnit::F2);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_unnamed_struct_serde() {
        let f0 = ExampleUnnamedStructUnit::F0;

        // Serialize the value
        let serialized = serde_json::to_string(&f0).unwrap();
        let deserialized: ExampleUnnamedStructUnit = serde_json::from_str(&serialized).unwrap();

        // Ensure that deserialization works as expected
        assert_eq!(f0, deserialized);
    }

    #[test]
    #[cfg(all(feature = "bitflags", feature = "serde"))]
    fn test_unnamed_struct_serde_bitflags() {
        let f0f1 = ExampleUnnamedStructUnit::F0 | ExampleUnnamedStructUnit::F1;

        // Serialize the combined flags
        let serialized = serde_json::to_string(&f0f1).unwrap();
        let deserialized: ExampleUnnamedStructUnit = serde_json::from_str(&serialized).unwrap();

        // Ensure the combined flags are correctly serialized and deserialized
        assert_eq!(f0f1, deserialized);
    }
}
