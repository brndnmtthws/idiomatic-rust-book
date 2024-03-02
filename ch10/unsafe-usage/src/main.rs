unsafe fn unsafe_function() {
    libc::printf(
        "calling C's printf() within unsafe_function()\n\0".as_ptr()
            as *const i8,
    );
}

unsafe trait UnsafeTrait {
    fn safe_method(&self);
    unsafe fn unsafe_method(&self);
}

struct MyStruct;

unsafe impl UnsafeTrait for MyStruct {
    fn safe_method(&self) {
        println!("calling println!() within UnsafeTrait::safe_method()");
    }
    unsafe fn unsafe_method(&self) {
        libc::printf(
            "calling C's printf() within UnsafeTrait::unsafe_method()\n\0"
                .as_ptr() as *const i8,
        );
    }
}

fn main() {
    unsafe {
        unsafe_function();
    }

    let my_struct = MyStruct;
    my_struct.safe_method();
    unsafe {
        my_struct.unsafe_method();
    }
}
