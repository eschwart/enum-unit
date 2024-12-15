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

    assert_eq!(ExampleUnit::from(a), ExampleUnit::A);
    assert_eq!(ExampleUnit::from(b), ExampleUnit::B);
    assert_eq!(ExampleUnit::from(c), ExampleUnit::C);

    #[cfg(feature = "bitflag")]
    {
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
}
