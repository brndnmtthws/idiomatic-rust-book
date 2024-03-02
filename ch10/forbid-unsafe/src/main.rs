#![forbid(unsafe_code)]

fn main() {
    // unsafe {
    //   libc::printf("Hello, world!\n".as_ptr() as *const _);
    // }
    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.insert(0, "orange");
}
