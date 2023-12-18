use std::marker::PhantomData;

use super::node::{NodeType, Tag};

pub struct HtmlElement<'a, T> {
    id: String,
    classes: Vec<String>,
    tag: String,
    phantom: PhantomData<&'a T>
}

impl<'a, T> NodeType<'a> for HtmlElement<'a, T> {
    fn html(&self) -> Tag {

        Tag {
            open: format!("<{}>", self.tag),
            close: format!("</{}>", self.tag)
        }
    }
}

impl<'a, T> HtmlElement<'a, T> {
    pub fn new(tag: &str) -> Self {
        HtmlElement {
            id: String::new(),
            classes: vec![],
            tag: tag.to_string(),
            phantom: PhantomData
        }
    }
}
