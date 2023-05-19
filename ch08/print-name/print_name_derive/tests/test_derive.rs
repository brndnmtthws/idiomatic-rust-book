use print_name::PrintName;
use print_name_derive::PrintName;

#[test]
fn test_derive() {
    #[derive(PrintName)]
    struct MyStruct;

    assert_eq!(MyStruct::name(), "MyStruct");
    MyStruct::print_name();
}
