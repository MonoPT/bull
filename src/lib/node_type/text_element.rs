use std::marker::PhantomData;

use super::super::node::{NodeType, Tag};

pub struct TextElement<'a, T> {
    text: String,
    phantom: PhantomData<&'a T>
}

impl<'a, T> NodeType<'a> for TextElement<'a, T> {
    fn html(&self) -> Tag {

        Tag {
            open: self.text.to_string(),
            close: "".to_owned()
        }
    }

    fn node_tag(&self) -> String {
        "Text node".to_owned()
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: &str) -> Result<String, String> {
        self.text = text.to_owned();

        Ok(self.text.to_string())
    }
}

impl<'a, T> TextElement<'a, T> {
    pub fn new(text: &str) -> Self {
        TextElement {
            text: text.to_string(),
            phantom: PhantomData
        }
    }
}
