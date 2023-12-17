//Had to move this functions to a different file because rust analyzer was complaining it couldnt finf children while in the same file as the struct

use std::{rc::Rc, cell::RefCell};

use super::node::Node;

impl<'a> Node<'a> {
    pub fn set_parent(parent_node: Rc<RefCell<Node<'a>>>, current_node: Rc<RefCell<Node<'a>>>) {
        Node::push_child_node(Rc::clone(&parent_node), Rc::clone(&current_node));
        current_node.borrow_mut().parent = Some(parent_node);
    }
    
    fn push_child_node(parent_node: Rc<RefCell<Node<'a>>>, current_node: Rc<RefCell<Node<'a>>>) {

        match &current_node.borrow().parent {
            None => (),
            Some(parent) => {
                if parent.borrow().internal_id == parent_node.borrow().internal_id {
                    return;
                }
            }
        }

        parent_node.borrow_mut().children.push(current_node);
    }

    pub fn reparent_node(new_parent: &Rc<RefCell<Node<'a>>>, child: &Rc<RefCell<Node<'a>>>, insert_children: bool) {
        let child_rc: Rc<RefCell<Node<'a>>> = Rc::clone(&child);
        let old_parent_ref = child_rc.borrow().get_parent();
    
        child_rc.borrow_mut().parent = Some(Rc::clone(new_parent));

        if !insert_children {
            return;
        }
    
        match old_parent_ref {
            None => (),
            Some(parent) => {
                let mut new_children_vec: Vec<Rc<RefCell<Node>>> = vec![];
    
                for child in &parent.borrow_mut().children {
                    let new_ref = Rc::clone(child);
                        
                    if !Rc::ptr_eq(&child_rc,&new_ref) {
                        new_children_vec.push(new_ref);
                    }
                }
    
                parent.borrow_mut().children = new_children_vec;
            }
        }
    
        new_parent.borrow_mut().children.push(child_rc);
    }

    pub fn insert_child_at_index(index: usize, child: &Rc<RefCell<Node<'a>>>, parent: &Rc<RefCell<Node<'a>>>) { //Inserts child at given index. Can even clone node
        let parent = Rc::clone(&parent);
        
        let child = Rc::clone(&child);
        
        if index >= parent.borrow_mut().children.len() {
            Node::reparent_node(&parent, &child, true);
            return;
        }

        Node::reparent_node(&parent, &child, false);
        parent.borrow_mut().children.insert(index, child);
    }

    pub fn print_tree(&self) {
        let mut ident = 0;
        println!("{}", &self.id);

        fn func<'a>(ident: i32, current_parent: &Rc<RefCell<Node<'a>>>) {
            let ident = ident + 1;

            let mut s = String::new();

            for _ in 0..ident {
                s += "  ";
            }

            println!("{s}{}", &current_parent.borrow().id);

            for child in &current_parent.borrow().children {
                let child_ref = Rc::clone(&child);

                func(ident, child);
            }
        }

        for child in &self.children {
            func(ident, child);
        }

    }
}