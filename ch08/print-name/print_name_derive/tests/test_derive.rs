use print_name::PrintName;
use print_name_derive::PrintName;
use std::marker::PhantomData;

#[test]
fn test_derive() {
    #[derive(PrintName)]
    struct MyStruct;

    #[derive(PrintName)]
    struct MyGenericStruct<T> {
        phantom: PhantomData<T>,
    }

    assert_eq!(MyStruct::name(), "MyStruct");
    MyStruct::print_name();
    assert_eq!(MyGenericStruct::<i64>::name(), "MyGenericStruct");
    assert_eq!(MyGenericStruct::<i128>::name(), "MyGenericStruct");
    assert_eq!(MyGenericStruct::<String>::name(), "MyGenericStruct");
    MyGenericStruct::<i128>::print_name();
}
