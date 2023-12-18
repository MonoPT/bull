
use std::rc::Rc;
use std::cell::RefCell;
use uuid;

use super::node_type::HtmlElement;

pub struct Node<'a>
{
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub parent: Option<Rc<RefCell<Node<'a>>>>,
    pub internal_id: String,
    pub self_closing: bool,
    pub node_type: Box<dyn NodeType<'a> + 'a>    
}

#[derive(Debug)]
pub struct Tag {
    pub open: String,
    pub close: String
}

pub trait NodeType<'a> {
    fn html(&self) -> Tag;
}

impl<'a> Node<'a>
 {
    pub fn new(parent_node: Option<&Rc<RefCell<Node<'a>>>>, node_type: impl NodeType<'a> + 'a) -> Rc<RefCell<Self>> {
        
        let node = Rc::new(RefCell::new(Self {
            children: Vec::new(),
            parent: None,
            internal_id: uuid::Uuid::new_v4().to_string(),
            self_closing: false,
            node_type: Box::new(node_type)
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

    pub fn new_html_element(tag: &str, parent_node: Option<&Rc<RefCell<Node<'a>>>>) -> Rc<RefCell<Self>> {
        //let html_element: HtmlElement<'a, T> = HtmlElement::new(tag);

        fn make_element<'a, T>(tag: &str) -> HtmlElement<'a, T> {
            HtmlElement::new(tag)
        }

        let html_element: HtmlElement<'static, String> = make_element::<String>(tag);

        Node::new(parent_node, html_element)
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

    pub fn html(&self) -> Tag {
        self.node_type.html()
    }

}

