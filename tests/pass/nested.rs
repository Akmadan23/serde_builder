use serde_builder::SerdeBuilder;

#[derive(Debug, Default, PartialEq, SerdeBuilder)]
struct MyNestedStruct {
    n_field1: i32,
    n_field2: String
}

#[derive(Debug, Default, PartialEq, SerdeBuilder)]
struct MyStruct {
    field1: i32,
    field2: f64,
    field3: bool,
    field4: String,
    field5: Option<u8>,
    #[use_builder]
    field6: MyNestedStruct
}

fn main() {
    let b = MyStructBuilder {
        field1: Some(42),
        field2: None,
        field3: Some(true),
        field4: None,
        field5: Some(16),
        field6: Some(MyNestedStructBuilder {
            n_field1: None,
            n_field2: Some(String::from("test"))
        })
    };

    assert_eq!(b.build(), MyStruct {
        field1: 42,
        field2: 0.0,
        field3: true,
        field4: String::new(),
        field5: Some(16),
        field6: MyNestedStruct {
            n_field1: 0,
            n_field2: String::from("test")
        }
    });
}
