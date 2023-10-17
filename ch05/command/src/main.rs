use std::fs::File;
use std::io::{BufRead, BufReader, Error, Seek, Write};

trait Command {
    fn execute(&self) -> Result<(), Error>;
}

struct ReadFile {
    receiver: File,
}

impl ReadFile {
    fn new(receiver: File) -> Box<Self> {
        Box::new(Self { receiver })
    }
}

impl Command for ReadFile {
    fn execute(&self) -> Result<(), Error> {
        println!("Reading from start of file");
        let mut reader = BufReader::new(&self.receiver);
        reader.seek(std::io::SeekFrom::Start(0))?;

        for (count, line) in reader.lines().enumerate() {
            println!("{:2}: {}", count + 1, line?);
        }

        Ok(())
    }
}

struct WriteFile {
    content: String,
    receiver: File,
}

impl WriteFile {
    fn new(content: String, receiver: File) -> Box<Self> {
        Box::new(Self { content, receiver })
    }
}

impl Command for WriteFile {
    fn execute(&self) -> Result<(), Error> {
        println!("Writing new content to file");
        let mut writer = self.receiver.try_clone()?;

        writer.write_all(self.content.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("file.txt")?;

    let commands: Vec<Box<dyn Command>> = vec![
        ReadFile::new(file.try_clone()?),
        WriteFile::new("file content\n".into(), file.try_clone()?),
        ReadFile::new(file.try_clone()?),
    ];

    for command in commands {
        command.execute()?;
    }

    Ok(())
}
