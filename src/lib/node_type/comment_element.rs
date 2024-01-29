use std::marker::PhantomData;

use super::super::node::{NodeType, Tag};

pub struct CommetElement<'a, T> {
    text: String,
    phantom: PhantomData<&'a T>
}

impl<'a, T> NodeType<'a> for CommetElement<'a, T> {
    fn html(&self) -> Tag {

        Tag {
            open: "".to_owned(),
            close: "".to_owned()
        }
    }

    fn node_tag(&self) -> String {
        "comment node".to_owned()
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: &str) -> Result<String, String> {
        self.text = text.to_owned();

        Ok(self.text.to_string())
    }
}

impl<'a, T> CommetElement<'a, T> {
    pub fn new(text: &str) -> Self {
        CommetElement {
            text: text.to_string(),
            phantom: PhantomData
        }
    }
}
