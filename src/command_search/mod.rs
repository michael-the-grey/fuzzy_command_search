use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::env;
use std::path;

pub struct CommandSearch {
    pub command: String,
    history: HashSet<String>
}

impl CommandSearch {
    pub fn new() -> CommandSearch {
        let mut contents = String::new();
        let home = env::var("HOME").expect("HOME variable must be set");
        let mut zsh_history = path::PathBuf::from(home.as_str());
        zsh_history.push(".zsh_history");

        File::open(zsh_history)
            .expect("Couldn't read history file")
            .read_to_string(&mut contents)
            .expect("Couldn't read history file");

        let history: HashSet<String> = contents.lines()
            .map(|x| x.splitn(2, ";").collect())
            .map(|x: Vec<&str>| x.last().unwrap().to_string())
            .collect();

        CommandSearch {
            command: String::new(),
            history: history
        }
    }

    pub fn input(&mut self, action: u8) -> Vec<String> {
        match action {
            127 => {
                self.command.pop();
            }
            c => self.command.push(c as char),
        }
        self.search()
    }

    fn search(&self) -> Vec<String> {
        let matches = self.history.clone();

        matches.into_iter()
            .flat_map(|x| self.score(x))
            .collect()
    }

    fn score(&self, command: String) -> Option<String> {
        let pattern = self.command.as_str();
        let size = command.len();
        let mut likeness = 0;
        let mut index = 0;

        for c in pattern.chars() {
            let remaining = command.chars()
                .skip(index)
                .skip_while(|&x| x != c)
                .collect::<Vec<_>>()
                .len();

            index = size - remaining;
            if index != size {
                likeness += 1;
                index += 1;
            } else {
                break;
            }
        }

        match likeness == pattern.len() {
            true => Some(command),
            false => None,
        }
    }
}
