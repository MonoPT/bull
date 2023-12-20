use std::{rc::Rc, cell::RefCell, path::Path, fs};

use regex::Regex;

use super::node::Node;

enum NodeType {
    Comment,
    Element,
    Text,
    Root
}

impl<'a> Node<'a> {
    pub fn parse_string(string: String) -> Rc<RefCell<Node<'a>>> {
        let root = Node::new_root_element();

        let mut last_element: Rc<RefCell<Node>> = Rc::clone(&root);

        for line in string.lines() {
            let line = line.replace("\t", "     ");
            let identation = get_identation(&line);
            let line = line.trim();

            if line.len() < 1 {
                continue;
            }

            //check if is string and handle it (' " `)

            //handle other kinds of elements
            
            let (tag_or_text, id, classes, properties, node_type) = check_line_type(&line);

            match node_type {
                NodeType::Element => {
                    let node = handle_new_element(&last_element, tag_or_text, id, classes, properties, identation);
                    last_element = Rc::clone(&node);
                },
                NodeType::Comment => {

                },
                _ => {

                }
            }            
        }

        root
    }

    pub fn parse_file(path_p: &str) -> Rc<RefCell<Node<'a>>>  {
        let path = Path::new(path_p);

        if !path.exists() {
            panic!("Could not find the file {}", path_p);
        }

        let data = fs::read_to_string(path).expect("Unable to read file");

        Node::parse_string(data)
    }
}

fn handle_new_element<'a>(last_child: &Rc<RefCell<Node<'a>>>, tag: String, id: String, classes: Vec<String>, properties: String, ident: usize) -> Rc<RefCell<Node<'a>>> {
    let mut ident = ident;
    
    if (last_child.borrow().identation < ident) {
        return Node::new_html_element(&tag, Some(last_child), ident, classes, &id);
    } else if last_child.borrow().identation == ident {
        match last_child.borrow().get_parent() {
            None => panic!("Something went wrong at handle new element function"),
            Some(parent) => {
                return Node::new_html_element(&tag, Some(&parent), ident, classes, &id); 
            }
        }
    } else {
        let mut last_node = Rc::clone(&last_child);

        loop {
            if last_node.borrow().identation < ident {
                return Node::new_html_element(&tag, Some(&last_node), ident, classes, &id); 
            }

            let mut temp = Rc::clone(&last_node);

            match last_node.borrow().get_parent() {
                None => panic!("Something went wrong at handle new element function"),
                Some(parent) => {
                    temp = Rc::clone(&parent);
                }
            }

            last_node = temp;
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

fn check_line_type(line: &str) -> (String, String, Vec<String>, String, NodeType) { //class, id, line sem identificador, tipo de elemento, ou throw erro
    let mut node_type = NodeType::Element;
    // Encontrar comentário inline em line


    //
    let mut splited_line: Vec<&str> = line.split(" ").collect();

    let regex = Regex::new(r"^[a-zA-Z0-9]+(?:[_-]?[a-zA-Z0-9]+)*$").unwrap();
    
    let mut idetifier = splited_line[0].to_owned();

    let mut id = String::new();

    let mut classes: Vec<&str> = vec![];

    if idetifier.starts_with(".") {
        idetifier = "div".to_owned() + &idetifier;
        //idetifier = idetifier.strip_prefix(".").expect("an error has ocurred while parsing tag identifier").to_string();
    }

    if idetifier.starts_with("#") {
        idetifier = "div".to_owned() + &idetifier;
        //idetifier = idetifier.strip_prefix("#").expect("an error has ocurred while parsing tag identifier").to_string();
    }

    let mut temp = idetifier.clone();

    //Handle ids
    let mut split = idetifier.split("#").collect::<Vec<&str>>();

    if split.len() > 1 {
        id = split[1].to_owned();

        split.remove(1);

        temp = split.join(".");
    }

    //handle classes
    classes = temp.split(".").collect();
    idetifier = classes[0].to_owned();

    let mut properties = String::new();

    if regex.is_match(&idetifier) { //Tag é válida
        classes.remove(0);

        let mut classes_fixes: Vec<&str> = vec![];

        let mut classes_output: Vec<String> = vec![];

        for class in classes {
            if class.trim().len() > 0 {
                classes_output.push(class.to_owned());
            }
        }

        classes = classes_fixes;

        splited_line.remove(0);

        properties = splited_line.join(" ");

        return (idetifier, id, classes_output, properties, node_type);
    } else {
        //Check if is only comment
        if line.starts_with("//") {
            node_type = NodeType::Comment;

            let text = line.strip_prefix("//").unwrap().to_owned();

            return (text, "".to_owned(), vec![], "".to_owned(), node_type);
        }

        panic!("could not parse line: {}", line);
    }
    
}