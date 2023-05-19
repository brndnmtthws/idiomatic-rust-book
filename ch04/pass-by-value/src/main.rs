fn reverse(s: String) -> String {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    String::from_iter(v.iter())
}

fn main() {
    let s = String::from("😜🤖🪼");
    println!("Forward: {s}");
    let s = reverse(s);
    println!("Reverse: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!("rotator", reverse(String::from("rotator")));
        assert_eq!("abcdefg", reverse(String::from("gfedcba")));
    }
}
