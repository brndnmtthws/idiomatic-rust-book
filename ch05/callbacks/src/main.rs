fn callback_fn<F>(f: F)
where
    F: Fn() -> (),
{
    f();
}

fn main() {
    let my_callback = || println!("I have been called back");
    callback_fn(my_callback);
}
