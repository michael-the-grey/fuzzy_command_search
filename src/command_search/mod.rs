use std::fs::File;
use std::io::Read;

pub struct History {
    commands: Vec<String>
}

impl History {
    pub fn new() -> History {
        let mut contents = String::new();
        let path = "/home/michael/.zsh_history";

        File::open(&path)
            .expect("Couldn't read history file")
            .read_to_string(&mut contents)
            .expect("Couldn't read history file");

        let commands: Vec<String> = contents.lines()
            .map(|x| x.splitn(2, ";").collect())
            .map(|x: Vec<&str>| x.last().unwrap().to_string())
            .collect();

        History {
            commands: commands
        }
    }

    pub fn search(&self, input: &str) -> Vec<String> {
        let matches = self.commands.clone();

        matches.into_iter()
            .take(input.len())
            .collect()
    }
}
