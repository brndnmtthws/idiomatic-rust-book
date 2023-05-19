use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadLineArgument(usize),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

fn read_nth_line(path: &Path, n: usize) -> Result<String, Error> {
    if n < 1 {
        return Err(Error::BadLineArgument(0));
    }
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(path)?;

    let mut reader_lines = BufReader::new(file).lines();
    reader_lines
        .nth(n - 1)
        .map(|result| result.map_err(|err| err.into()))
        .unwrap_or_else(|| Err(Error::BadLineArgument(n)))
}

fn main() -> Result<(), Error> {
    let path = Path::new("Cargo.toml");
    println!(
        "The 4th line from Cargo.toml reads: {}",
        read_nth_line(path, 4)?
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_read_cargotoml() {
        let third_line = read_nth_line(Path::new("Cargo.toml"), 3)
            .expect("unable to read third line from Cargo.toml");
        assert_eq!("version = \"0.1.0\"", third_line);
    }

    #[test]
    fn test_not_a_file() {
        let err = read_nth_line(Path::new("not-a-file"), 1)
            .expect_err("file should not exist");
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_bad_arg_0() {
        let err = read_nth_line(Path::new("Cargo.toml"), 0)
            .expect_err("0th line is invalid");
        assert!(matches!(err, Error::BadLineArgument(0)));
    }

    #[test]
    fn test_bad_arg_too_large() {
        let err = read_nth_line(Path::new("Cargo.toml"), 500)
            .expect_err("500th line is invalid");
        assert!(matches!(err, Error::BadLineArgument(500)));
    }
}
