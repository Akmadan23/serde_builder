use serde_builder::SerdeBuilder;

#[derive(Debug, Default, PartialEq, SerdeBuilder)]
struct MyStruct {
    field1: i32,
    field2: bool,
    field3: String,
    field4: Option<u8>
}

fn main() {
    let b1 = MyStructBuilder {
        field1: None,
        field2: None,
        field3: None,
        field4: None
    };

    let b2 = MyStructBuilder {
        field1: Some(42),
        field2: Some(true),
        field3: Some(String::from("test")),
        field4: Some(16)
    };

    let s1 = b1.build();
    let s2 = b2.build();

    assert_eq!(s1, MyStruct {
        field1: 0,
        field2: false,
        field3: String::new(),
        field4: None
    });

    assert_eq!(s2, MyStruct {
        field1: 42,
        field2: true,
        field3: String::from("test"),
        field4: Some(16)
    });
}
