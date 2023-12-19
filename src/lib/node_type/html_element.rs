use std::marker::PhantomData;

use super::super::node::{NodeType, Tag};

pub struct HtmlElement<'a, T> {
    id: String,
    classes: Vec<String>,
    tag: String,
    phantom: PhantomData<&'a T>,
    text: String
}

impl<'a, T> NodeType<'a> for HtmlElement<'a, T> {
    fn html(&self) -> Tag {

        Tag {
            open: format!("<{}>", self.tag),
            close: format!("</{}>", self.tag)
        }
    }

    fn node_tag(&self) -> String {
        self.tag.clone()
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, _text: &str) -> Result<String, String> {
        Err("Element is not a text node".to_owned())
    }
}

impl<'a, T> HtmlElement<'a, T> {
    pub fn new(tag: &str) -> Self {
        HtmlElement {
            id: String::new(),
            classes: vec![],
            tag: tag.to_string(),
            text: String::new(),
            phantom: PhantomData
        }
    }
}
