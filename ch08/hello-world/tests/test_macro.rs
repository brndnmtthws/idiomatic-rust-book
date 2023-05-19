#[cfg(test)]
mod tests {
    #[test]
    fn print_hello_world() {
        use hello_world::say_hello_world;
        say_hello_world!();
    }
}
