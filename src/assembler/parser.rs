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

pub fn assemble(source: String) -> Vec<i16> {
    let result = Vec::new();
    let lines = source
        .lines()
        .map(|line| line.split("//").nth(0).unwrap())
        .filter(|line| line.len() > 0);
    for line in lines {
        println!("{}", line);
    }
    result
}
