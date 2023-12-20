use std::marker::PhantomData;

use super::super::node::{NodeType, Tag};

pub struct RootElement<'a, T> {
    phantom: PhantomData<&'a T>
}

impl<'a, T> NodeType<'a> for RootElement<'a, T> {
    fn html(&self) -> Tag {

        Tag {
            open: "".to_owned(),
            close: "".to_owned()
        }
    }

    fn node_tag(&self) -> String {
        "[Root]".to_owned()
    }

    fn text(&self) -> &str {
        ""
    }

    fn set_text(&mut self, _text: &str) -> Result<String, String> {
        Err("Cant set text of root element".to_owned())
    }
}

impl<'a, T> RootElement<'a, T> {
    pub fn new() -> Self {
        RootElement {
            phantom: PhantomData
        }
    }
}
