fn reverse(s: String) -> String {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    String::from_iter(v.iter())
}

fn reverse_and_uppercase(s: String) -> (String, String) {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    let reversed = String::from_iter(v.iter());
    let uppercased = reversed.to_uppercase();
    (reversed, uppercased)
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

    #[test]
    fn test_reverse_and_uppercase() {
        assert_eq!(
            reverse_and_uppercase("rotator".to_string()),
            ("rotator".to_string(), "ROTATOR".to_string()),
        );
        assert_eq!(
            reverse_and_uppercase("abcdefg".to_string()),
            ("gfedcba".to_string(), "GFEDCBA".to_string()),
        );
    }
}
