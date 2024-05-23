#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use core::f64::consts::PI;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

struct CargoTomlReader {
    coroutine:
        Pin<Box<dyn Coroutine<Yield = (usize, String), Return = ()>>>,
}

impl CargoTomlReader {
    fn new() -> io::Result<Self> {
        let file = File::open("Cargo.toml")?;
        let mut reader = BufReader::new(file);
        let mut line_number: usize = 0;

        let coroutine = Box::pin(
            #[coroutine]
            move || loop {
                let mut line = String::new();
                line_number += 1;
                match reader.read_line(&mut line) {
                    Ok(0) => return,
                    Ok(_) => yield (line_number, line),
                    _ => return,
                }
            },
        );
        Ok(Self { coroutine })
    }
}

impl Iterator for CargoTomlReader {
    type Item = (usize, String);
    fn next(&mut self) -> Option<Self::Item> {
        match self.coroutine.as_mut().resume(()) {
            CoroutineState::Yielded(val) => Some(val),
            CoroutineState::Complete(()) => None,
        }
    }
}

fn main() -> io::Result<()> {
    let mut yield_pi = #[coroutine]
    || {
        yield PI;
        "Coroutine complete!"
    };

    loop {
        match Pin::new(&mut yield_pi).resume(()) {
            CoroutineState::Yielded(val) => {
                dbg!(&val);
            }
            CoroutineState::Complete(val) => {
                dbg!(&val);
                break;
            }
        }
    }

    let cargo_reader = CargoTomlReader::new()?;
    for (line_number, line) in cargo_reader {
        print!("{line_number}: {line}");
    }

    Ok(())
}
