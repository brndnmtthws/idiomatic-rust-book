fn some_or_none<T>(option: &Option<T>) {
    match option {
        Some(v) => println!("is some!"),
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
            println!("The whole tuple is ({first}, {middle}, {last}")
        }
    }
}

fn match_with_guard(value: i32) {
    match value {
        v if v == 1 => println!("This value is equal to 1"),
        v if v < 10 => println!("This value is less than 10"),
        _ => println!("This value is at least 10, or less than 1"),
    }
}

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

enum CatColour {
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
    colour: CatColour,
}

fn match_on_black_cats(cat: &Cat) {
    match cat {
        Cat {
            name,
            colour: CatColour::Black,
        } => println!("This is a black cat named {name}"),
        Cat { name, colour: _ } => println!("{name} is not a black cat"),
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
        Ok(()) => println!("Write suceeded"),
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
            _ => println!("Write suceeded"),
        },
        Err(err) => println!(
            "There was an error opening the file: {}",
            err
        ),
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_to_write_file() {
        try_to_write_to_file();
    }
}
