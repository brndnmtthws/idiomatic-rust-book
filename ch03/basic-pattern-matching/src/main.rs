fn some_or_none<T>(option: &Option<T>) {
    match option {
        Some(_v) => println!("is some!"),
        None => println!("is none :("),
    }
}

fn some_or_none_display<T: std::fmt::Display>(option: &Option<T>) {
    match option {
        Some(v) => println!("is some! where v={v}"),
        None => println!("is none :("),
    }
}

fn what_type_of_integer_is_this(value: i32) {
    match value {
        1 => println!("The number one number"),
        2 | 3 => println!("This is a two or a three"),
        4..=10 => println!("This is a number between 4 and 10 (inclusive)"),
        _ => println!("Some other kind of number"),
    }
}

fn destructure_tuple(tuple: &(i32, i32, i32)) {
    match tuple {
        (first, ..) => println!("First tuple element is {first}"),
    }
    match tuple {
        (.., last) => println!("Last tuple element is {last}"),
    }
    match tuple {
        (_, middle, _) => println!("The middle tuple element is {middle}"),
    }
    match tuple {
        (first, middle, last) => {
            println!("The whole tuple is ({first}, {middle}, {last})")
        }
    }
}

fn match_with_guard(value: i32, choose_first: bool) {
    match value {
        v if v == 1 && choose_first => {
            println!("First match: This value is equal to 1")
        }
        v if v == 1 && !choose_first => {
            println!("Second match: This value is equal to 1")
        }
        v if choose_first => {
            println!("First match: This value is equal to {v}")
        }
        v if !choose_first => {
            println!("Second match: This value is equal to {v}")
        }
        _ => println!("Fell through to the default case"),
    }
}

fn unreachable_pattern_match(value: i32) {
    match value {
        1 => println!("First match: This value is equal to 1"),
        1 => println!("Second match: This value is equal to 1"),
        _ => println!("This value is not equal to 1"),
    }
}

// Does not compile!
// fn invalid_generic_matching<T>(value: &T) {
//     match value {
//         "is a string" => println!("This is a string"),
//         1 => println!("This is an integral value"),
//     }
// }

enum DistinctTypes {
    Name(String),
    Count(i32),
}

fn match_enum_types(enum_types: &DistinctTypes) {
    match enum_types {
        DistinctTypes::Name(name) => println!("name={name}"),
        DistinctTypes::Count(count) => println!("count={count}"),
    }
}

enum CatColor {
    Black,
    Red,
    Chocolate,
    Cinnamon,
    Blue,
    Cream,
    Cheshire,
}

struct Cat {
    name: String,
    color: CatColor,
}

fn match_on_black_cats(cat: &Cat) {
    match cat {
        Cat {
            name,
            color: CatColor::Black,
        } => println!("This is a black cat named {name}"),
        Cat { name, color: _ } => println!("{name} is not a black cat"),
    }
}

enum ErrorTypes {
    IoError(std::io::Error),
    FormatError(std::fmt::Error),
}

struct ErrorWrapper {
    source: ErrorTypes,
    message: String,
}

impl From<std::io::Error> for ErrorWrapper {
    fn from(source: std::io::Error) -> Self {
        Self {
            source: ErrorTypes::IoError(source),
            message: "there was an IO error!".into(),
        }
    }
}

fn write_to_file() -> Result<(), ErrorWrapper> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("filename")?;
    file.write_all(b"File contents")?;
    Ok(())
}

fn try_to_write_to_file() {
    match write_to_file() {
        Ok(()) => println!("Write succeeded"),
        Err(err) => println!("Write failed: {}", err.message),
    }
}

fn write_to_file_without_result() {
    use std::fs::File;
    use std::io::prelude::*;

    let create_result = File::create("filename");
    match create_result {
        Ok(mut file) => match file.write_all(b"File contents") {
            Err(err) => {
                println!("There was an error writing: {}", err)
            }
            _ => println!("Write succeeded"),
        },
        Err(err) => {
            println!("There was an error opening the file: {}", err)
        }
    }
}

fn main() {
    some_or_none(&Some(()));
    some_or_none::<()>(&None);

    some_or_none_display(&Some(1));
    some_or_none_display::<i32>(&None);

    what_type_of_integer_is_this(1);
    what_type_of_integer_is_this(2);
    what_type_of_integer_is_this(10);
    what_type_of_integer_is_this(42);

    destructure_tuple(&(1, 2, 3));

    match_with_guard(1, true);
    match_with_guard(1, false);
    match_with_guard(42, true);
    match_with_guard(42, false);

    unreachable_pattern_match(1);
    unreachable_pattern_match(10);

    match_enum_types(&DistinctTypes::Name("Alice".into()));
    match_enum_types(&DistinctTypes::Name("Bob".into()));
    match_enum_types(&DistinctTypes::Count(10_000));

    let black_cat = Cat {
        name: String::from("Henry"),
        color: CatColor::Black,
    };
    let cheshire_cat = Cat {
        name: String::from("Penelope"),
        color: CatColor::Cheshire,
    };
    match_on_black_cats(&black_cat);
    match_on_black_cats(&cheshire_cat);

    try_to_write_to_file();
    write_to_file_without_result();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_to_write_file() {
        try_to_write_to_file();
    }
}
