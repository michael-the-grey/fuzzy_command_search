use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::env;
use std::path;

pub enum Action {
    Add(char),
    Delete,
    Select,
}

impl Action {
    pub fn parse(input: u8) -> Action {
        match input {
            10 => Action::Select,
            127 => Action::Delete,
            c => Action::Add(c as char),
        }
    }
}

pub struct CommandSearch {
    pub command: String,
    pub matches: Vec<String>,
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
            matches: Vec::new(),
            history: history
        }
    }

    pub fn change(&mut self, action: Action) {
        match action {
            Action::Add(c) => self.command.push(c),
            Action::Delete => {
                self.command.pop();
            }
            Action::Select => (),
        }
        self.search();
    }

    fn search(&mut self) {
        let commands = self.history.clone();

        self.matches = commands.into_iter()
            .flat_map(|x| self.score(x))
            .collect();
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
