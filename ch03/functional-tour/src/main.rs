use std::collections::LinkedList;

fn main() {
    let vec = vec![1, 2, 3, 4];
    println!("{:?}", vec);
    let vec: Vec<_> = vec.iter().map(|v| v.to_string()).collect();
    println!("{:?}", vec);

    let linkedlist: LinkedList<i32> =
        vec.iter().flat_map(|v| v.parse::<i32>()).collect();
    println!("{:?}", linkedlist);

    let vec = vec!["duck", "1", "2", "goose", "3", "4"];
    let (successes, failures): (Vec<_>, Vec<_>) = vec
        .iter()
        .map(|v| v.parse::<i32>())
        .partition(Result::is_ok);
    println!("successses={:?}", successes);
    println!("failures={:?}", failures);

    let successes: Vec<_> =
        successes.into_iter().map(Result::unwrap).collect();
    let failures: Vec<_> =
        failures.into_iter().map(Result::unwrap_err).collect();
    println!("successses={:?}", successes);
    println!("failures={:?}", failures);

    let popular_dog_breeds = vec![
        "Labrador",
        "French Bulldog",
        "Golden Retriever",
        "German Shepherd",
        "Poodle",
        "Bulldog",
        "Beagle",
        "Rottweiler",
        "Pointer",
        "Dachshund",
    ];

    let ranked_breeds: Vec<_> = popular_dog_breeds
        .into_iter()
        .enumerate()
        .map(|(idx, breed)| (idx + 1, breed))
        .rev()
        .collect();

    println!("{:?}", ranked_breeds);
}
