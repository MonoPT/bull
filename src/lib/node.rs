use std::rc::Rc;
use std::cell::RefCell;
use uuid;

pub struct Node<'a> {
    pub id: String,
    pub tag: String,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub parent: Option<Rc<RefCell<Node<'a>>>>,
    pub internal_id: String
}

impl<'a> Node<'a> {
    pub fn new(tag: &str, id: &str, parent_node: Option<&Rc<RefCell<Node<'a>>>>) -> Rc<RefCell<Node<'a>>> {
        
        let node = Rc::new(RefCell::new(Node {
            id: id.to_string(),
            tag: tag.to_string(),
            children: Vec::new(),
            parent: None,
            internal_id: uuid::Uuid::new_v4().to_string()
        }));

        match parent_node { //Check if parent node was provided
            None => (),
            Some(parent_node) => {
                let parent_node = Rc::clone(&parent_node);

                //Add parent
                Node::set_parent(parent_node, Rc::clone(&node));
            }
        }

        node
    }

    //Request relation between nodes
    pub fn get_parent(&self) -> Option<Rc<RefCell<Node<'a>>>> {
        self.parent.clone()
    }

    pub fn get_child_by_index(&self, index: usize) -> Option<Rc<RefCell<Node<'a>>>> {
        let children = &self.children;

        if children.len() == 0 || index > children.len() - 1 {
            return None;
        }

        Some(Rc::clone(&children[index]))
    }

    pub fn next_node(&self) -> Option<Rc<RefCell<Node<'a>>>> {
        let internal_id = self.internal_id.clone();

        match &self.get_parent() {
            None => return None,
            Some(parent) => {
                let mut return_next = false;

                for child in &parent.borrow_mut().children {
                    if return_next {
                        return Some(Rc::clone(child));
                    }


                    if child.borrow().internal_id == internal_id {
                        return_next = true;
                    }
                }

                return None;
            }
        }
    }

    pub fn index_as_child(&self) -> Option<usize> {
        
        let internal_id = self.internal_id.clone();

        match &self.get_parent() {
            None => return None,
            Some(parent) => {
                let mut count = 0;

                for child in &parent.borrow_mut().children {
                    if child.borrow().internal_id == internal_id {
                        return Some(count);
                    }

                    count = count + 1;
                }

                return None;
            }
        }
    }

    pub fn previous_node(&self) -> Option<Rc<RefCell<Node<'a>>>>{
        match self.index_as_child() {
            None => return None,
            Some(index) => {

                if index < 1 {
                    return None;
                }

                match self.get_parent() {
                    None => return  None,
                    Some(parent) => {
                        return Some(Rc::clone(&parent.borrow().children[index - 1]));
                    }
                }
            }
        }
    }

}

