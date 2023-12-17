mod lib;

use std::{rc::Rc, cell::RefCell, borrow::BorrowMut};

use lib::node::Node;

//
// Make functions to 
// //  - make a children iterator
//  - Make a function to find if there is infinite recursion and if there is panic!
//  - Passar funções exclusivamente internas para ficheiro à parte de forma a isolar esses métodos (set_parent)
//  - print the tree to the terminal
//  - parse the .bull file


fn main() {
    let root = Node::new("Root", "root", Option::None);

    let child1 = Node::new("Child1", "nod1", Some(&root));
    let child3 = Node::new("Child3", "nod3", Some(&child1));
    let child5 = Node::new("Child5", "nod5", Some(&child1));
    let child4 = Node::new("Child4", "nod4", Some(&root));
    let mut child2 = Node::new("Child2", "node2", Some(&child1));
    let child6 = Node::new("Child6", "nod6", Some(&child4));

    let mut current = Rc::clone(&child2);

    loop {
        let mut parent_node: Option<Rc<RefCell<Node>>> = None;

        match current.borrow().get_parent() {
            Some(parent) => {
                println!("ID: {}, Tag: {:?}", parent.borrow().id, parent.borrow().tag);
                parent_node = Some(parent);
            }
            None => ()
        }

        match parent_node {
            None => break,
            Some(node) => {
                current = node;
            }
        }
    }

    let child = Node::new("nova", "nova", None);

    Node::reparent_node(&child4, &child, true);
    
    let index = child.borrow().index_as_child().unwrap();

    Node::insert_child_at_index(index, &child, &child4);
    Node::reparent_node(&child4, &child3, true);

    let child2 = Node::new("nova2", "nova2", Some(&child));

    let mut next = Some(child1);
    for _ in 0..4 {
        break;
        match next {
            None => println!("nothing"),
            Some(node) => {
                
                println!("\n current node: {}", node.borrow().id);

                match node.borrow().index_as_child() {
                    None => println!("is root"),
                    Some(pos) => {
                        println!("is index {} ", pos);
                    }
                }

                match node.borrow().previous_node() {
                    None => (),
                    Some(prv) => {
                        println!("previous {}", prv.borrow().id)
                    }
                }
    
                next = node.borrow().next_node();
            }
        }
    }

    root.borrow().print_tree();


    
}