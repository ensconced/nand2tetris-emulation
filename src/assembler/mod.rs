// use std::fs;

// struct Parser<'a> {
//     source: String,
//     current_command: Option<&'a str>,
// }

// enum CommandType {
//     ACommand,
//     CCommand,
//     LCommand,
// }

// impl<'a> Parser<'a> {
//     fn new(file_path: &str) -> Self {
//         Self {
//             source: fs::read_to_string(file_path).unwrap(),
//             current_command: None,
//         }
//     }
//     fn has_more_commands(&self) -> bool {}
//     fn advance(&mut self) {}
//     fn command_type(&self) {}
//     fn symbol(&self) -> &str {}
//     fn dest(&self) -> &str {}
//     fn comp(&self) -> &str {}
//     fn jump(&self) -> &str {}
// }

fn remove_comments(line: &str) -> &str {
    line.split("//").nth(0).unwrap()
}

fn get_commands(source: &str) -> impl Iterator<Item = &str> {
    source
        .lines()
        .map(remove_comments)
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
}

pub fn assemble(source: String) -> Vec<i16> {
    let machine_code = Vec::new();
    let commands = get_commands(&source);
    for command in commands {
        if command.starts_with("@") {
            // is an A-command
        } else if command.starts_with("(") {
            // is an L-command
        } else {
            // should be a C-command
        }
        println!("{}", command);
    }
    machine_code
}
