#![feature(test, linked_list_remove)]
extern crate test;

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::collections::{HashSet, LinkedList};
    use test::Bencher;

    #[bench]
    fn vec_find(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let nums: Vec<i32> = (0..1_000_000).collect();
        b.iter(|| {
            for _ in 0..1_000 {
                let n = rng.gen_range(0..1_000_000);
                assert_eq!(nums.iter().find(|&&x| x == n), Some(&n));
            }
        });
    }

    #[bench]
    fn list_find(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let nums: LinkedList<i32> = (0..1_000_000).collect();
        b.iter(|| {
            for _ in 0..1_000 {
                let n = rng.gen_range(0..1_000_000);
                assert_eq!(nums.iter().find(|&&x| x == n), Some(&n));
            }
        });
    }

    #[bench]
    fn set_find(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let nums: HashSet<i32> = (0..1_000_000).collect();
        b.iter(|| {
            for _ in 0..1_000 {
                let n = rng.gen_range(0..1_000_000);
                assert_eq!(nums.get(&n), Some(&n));
            }
        });
    }

    #[bench]
    fn vec_append(b: &mut Bencher) {
        b.iter(|| {
            let mut nums: Vec<i32> = Vec::new();
            for n in 0..1_000_000 {
                nums.push(n);
            }
        });
    }

    #[bench]
    fn list_append(b: &mut Bencher) {
        b.iter(|| {
            let mut nums: LinkedList<i32> = LinkedList::new();
            for n in 0..1_000_000 {
                nums.push_back(n);
            }
        });
    }

    #[bench]
    fn set_append(b: &mut Bencher) {
        b.iter(|| {
            let mut nums: HashSet<i32> = HashSet::new();
            for n in 0..1_000_000 {
                nums.insert(n);
            }
        });
    }

    #[bench]
    fn vec_remove(b: &mut Bencher) {
        b.iter(|| {
            let mut nums: Vec<i32> = (0..100_000).collect();
            for _ in 0..1_000 {
                nums.remove(nums.len() / 2);
            }
        });
    }

    #[bench]
    fn list_remove(b: &mut Bencher) {
        b.iter(|| {
            let mut nums: LinkedList<i32> = (0..100_000).collect();
            for _ in 0..1_000 {
                nums.remove(nums.len() / 2);
            }
        });
    }

    #[bench]
    fn set_remove(b: &mut Bencher) {
        b.iter(|| {
            let mut nums: HashSet<i32> = (0..100_000).collect();
            for n in 0..1_000 {
                nums.remove(&n);
            }
        });
    }
}
