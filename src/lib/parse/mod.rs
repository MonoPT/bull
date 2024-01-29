use std::{cell::RefCell, fs, path::Path, rc::Rc};

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
            
            let (tag_or_text, id, classes, properties, node_type, is_default_self_closed, inline_text) = check_line_type(&line);

            match node_type {
                NodeType::Element => {
                    let node = handle_new_element(&last_element, tag_or_text, id, classes, properties, identation, is_default_self_closed, inline_text);
                    last_element = Rc::clone(&node);
                },
                NodeType::Text => {
                    let node = handle_new_text_node(&last_element, identation, tag_or_text);
                    last_element = Rc::clone(&node);
                }
                _ => ()
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

fn handle_new_element<'a>(last_child: &Rc<RefCell<Node<'a>>>, tag: String, id: String, classes: Vec<String>, attributes: Vec<(String, String)>, ident: usize, is_default_self_closed: bool, inline_text: String) -> Rc<RefCell<Node<'a>>> {
    let ident = ident;

    let mut is_self_closed = false;

    let self_closed_tags = ["br", "img", "area", "base", "col", "embed", "hr", "input", "link", "meta", "param", "source", "track", "wbr"];

    if is_default_self_closed || self_closed_tags.contains(&tag.as_str()) {
        is_self_closed = true;
    }
    
    if last_child.borrow().identation < ident {
        return Node::new_html_element(&tag, Some(last_child), ident, classes, &id, is_self_closed, attributes, inline_text);
    } else if last_child.borrow().identation == ident {
        match last_child.borrow().get_parent() {
            None => panic!("Something went wrong at handle new element function"),
            Some(parent) => {
                return Node::new_html_element(&tag, Some(&parent), ident, classes, &id, is_self_closed, attributes, inline_text); 
            }
        }
    } else {
        let mut last_node = Rc::clone(&last_child);

        loop {
            if last_node.borrow().identation < ident {
                return Node::new_html_element(&tag, Some(&last_node), ident, classes, &id, is_self_closed, attributes, inline_text); 
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

fn handle_new_text_node<'a>(last_child: &Rc<RefCell<Node<'a>>>, ident: usize, text: String) -> Rc<RefCell<Node<'a>>> {
    let ident = ident;
    
    if last_child.borrow().identation < ident {
        return Node::new_text_element(&text, Some(last_child), ident);
    } else if last_child.borrow().identation == ident {
        match last_child.borrow().get_parent() {
            None => panic!("Something went wrong at handle new element function"),
            Some(parent) => {
                return Node::new_text_element(&text, Some(&parent), ident); 
            }
        }
    } else {
        let mut last_node = Rc::clone(&last_child);

        loop {
            if last_node.borrow().identation < ident {
                return Node::new_text_element(&text, Some(&last_node), ident); 
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

fn check_line_type(line: &str) -> (String, String, Vec<String>, Vec<(String, String)>, NodeType, bool, String) { //class, id, line sem identificador, tipo de elemento, ou throw erro
    let mut node_type = NodeType::Element;
    // Encontrar comentário inline em line

    let mut is_defined_as_self_closed = false;

    //
    let mut splited_line: Vec<&str> = line.split(" ").collect();

    let regex = Regex::new(r"^[a-zA-Z0-9]+(?:[_-]?[a-zA-Z0-9]+)*$").unwrap();
    
    let mut idetifier = splited_line[0].to_owned();

    let mut other = String::new();

    if splited_line.len() > 1 {
        splited_line.remove(0);
        other = splited_line.join(" ").to_string();
        
    }

    let mut id = String::new();

    let mut classes: Vec<String> = vec![];

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
    classes = temp.split(".").map(|s| s.to_string()).collect();

    idetifier = classes[0].to_owned();

    

    let mut properties = String::new();

    if idetifier.chars().last().unwrap_or_default() == '/' {
        is_defined_as_self_closed = true;

        let mut chars = idetifier.chars().collect::<Vec<char>>();
        chars.remove(chars.len() - 1);

        idetifier = chars.into_iter().collect::<String>();
    }

    let mut attributes: Vec<(String, String)> = vec![];

    if regex.is_match(&idetifier) { //Tag é válida
        classes.remove(0);

        let mut inline_text = String::new();

        //Handle attributes !voltar aqui
        if other.len() > 0 {
            let mut is_inside_string_block = false;
            let mut string_closing_char = '"';
            

            let mut key = String::new();
            let mut pair = String::new();

            

            for char in other.chars() {
                let mut closed_this_loop_pair = false;
                if is_inside_string_block && char == string_closing_char {
                    is_inside_string_block = false;
                    closed_this_loop_pair = true;
                }

                let mut started_loop_pair = false;
                if !is_inside_string_block && !closed_this_loop_pair {
                    if char == '\'' || char == '"' || char == '`' {
                        string_closing_char = char;
                        is_inside_string_block = true;
                        started_loop_pair = true;
                    }
                }

                if closed_this_loop_pair {

                    if key == "class" {
                        classes.push(pair.clone());
                    } else {
                        if key.len() > 0 {
                            attributes.push((key.clone(), pair.clone()));
                        } else {
                            inline_text += &format!("{pair} ");
                        }
                        
                    }

                    
                    pair = String::new();
                    key = String::new();
                } else if is_inside_string_block && !started_loop_pair {
                    pair += &char.to_string();
                }

                if !is_inside_string_block && char != ' ' && char != '=' && !closed_this_loop_pair {
                    key += &char.to_string();
                }
            }   
        }

        match classes.last() {
            None => (),
            Some(class) => {
                if idetifier == "div" {
                    if class.chars().last().unwrap_or(' ') == '/' {
                        is_defined_as_self_closed = true;
                    }
                }
            }
        }

        let mut classes_output: Vec<String> = vec![];
        for class in classes {
            if class.trim().len() > 0 {
                classes_output.push(class.replace("/", "").to_owned());
            }
        }

        splited_line.remove(0);

        properties = splited_line.join(" ");

        return (idetifier, id, classes_output, attributes, node_type, is_defined_as_self_closed, inline_text);
    } else {
        //Check if is only comment
        if line.starts_with("//") {
            node_type = NodeType::Comment;

            let text = line.strip_prefix("//").unwrap().to_owned();

            return (text, "".to_owned(), vec![], attributes, node_type, is_defined_as_self_closed, "".to_string());
        }

        if line.starts_with('"') || line.starts_with('\'') || line.starts_with('`') {
            let mut line_chars = line.chars().map(|s| s.to_string()).collect::<Vec<String>>();
            line_chars.remove(0);

            if line.ends_with('"') || line.ends_with('\'') || line.ends_with('`') {
                line_chars.remove(line_chars.len() - 1);
            }

            let line: String = line_chars.iter().map(|s| s.to_owned()).collect();

            node_type = NodeType::Text;

            return (line, "".to_owned(), vec![], attributes, node_type, false, "".to_string());
        }

        panic!("could not parse line: {}", line);
    }
    
}