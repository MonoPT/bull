use regex::Regex;

use super::node::Node;

enum NodeType {
    Comment,
    Element,
    Text
}

impl<'a> Node<'a> {
    pub fn parse_string(string: &str) {
        let root = Node::new_html_element( "root", Option::None);

        for line in string.lines() {
            let line = line.replace("\t", "     ");
            let identation = get_identation(&line);


            check_line_type(&line);

            let line = line.trim();

            println!("{line}, {identation}");
        }
    }
}

fn get_identation(line: &str) -> usize {
    let identation: usize = line.chars()
    .take_while(|ch| ch.is_whitespace() && *ch == ' ')
    .map(|ch| ch.len_utf8())
    .sum();

    identation + 1
}

fn check_line_type(line: &str) -> bool { //class, id, line sem identificador, tipo de elemento, ou throw erro
    let node_type = NodeType::Element;


    let regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*[a-zA-Z0-9]$").unwrap();
    

    regex.is_match(line)
}