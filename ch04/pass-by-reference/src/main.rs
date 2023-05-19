fn reverse(s: &str) -> String {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    String::from_iter(v.iter())
}

fn reverse_inplace(s: &mut String) {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    s.clear();
    v.into_iter().for_each(|c| s.push(c));
}

fn main() {
    let s = "😜🤖🪼";
    println!("Forward: {s}");
    let s = reverse(s);
    println!("Reverse: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!("rotator", reverse("rotator"));
        assert_eq!("abcdefg", reverse("gfedcba"));
        assert_eq!("race car", reverse(&String::from("rac ecar")));
    }

    #[test]
    fn test_reverse_inplace() {
        let mut rotator = String::from("rotator");
        reverse_inplace(&mut rotator);
        assert_eq!("rotator", rotator);
        let mut abcdefg = String::from("gfedcba");
        reverse_inplace(&mut abcdefg);
        assert_eq!("abcdefg", abcdefg);
        let mut racecar = String::from("rac ecar");
        reverse_inplace(&mut racecar);
        assert_eq!("race car", racecar);
    }
}
