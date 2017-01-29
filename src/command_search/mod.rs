use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

pub struct History {
    commands: HashSet<String>
}

impl History {
    pub fn new() -> History {
        let mut contents = String::new();
        let path = "/home/michael/.zsh_history";

        File::open(&path)
            .expect("Couldn't read history file")
            .read_to_string(&mut contents)
            .expect("Couldn't read history file");

        let commands: HashSet<String> = contents.lines()
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
            .flat_map(|x| score(input, x))
            .collect()
    }
}

fn score(pattern: &str, command: String) -> Option<String> {
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
