use serde_builder::SerdeBuilder;

#[derive(Debug, Default, PartialEq, SerdeBuilder)]
struct MyStruct1<T> {
    field1: T,
    field2: Vec<T>,
}

#[derive(Debug, Default, PartialEq, SerdeBuilder)]
struct MyStruct2<T1, T2, T3> {
    field1: T1,
    field2: T2,
    field3: T3,
}

fn main() {
    let b1 = MyStruct1Builder {
        field1: Some(42),
        field2: Some(vec![1, 2, 3, 4]),
    };

    assert_eq!(b1.build(), MyStruct1 {
        field1: 42,
        field2: vec![1, 2, 3, 4]
    });

    let b2 = MyStruct2Builder {
        field1: Some(69),
        field2: Some(4.20),
        field3: Some(true)
    };

    assert_eq!(b2.build(), MyStruct2 {
        field1: 69,
        field2: 4.20,
        field3: true
    });
}
