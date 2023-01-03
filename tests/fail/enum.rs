use serde_builder::SerdeBuilder;

#[derive(SerdeBuilder)]
enum MyEnum {
    A,
    B,
    C
}

fn main() {}
