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

        let id = self.generate_id_tag();
        let classes = self.generate_classes();

        Tag {
            open: format!("<{}{id}{classes}>", self.tag),
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
    pub fn new(tag: &str, id: &str, classes: Vec<String>) -> Self {
        HtmlElement {
            id: id.to_owned(),
            classes: classes,
            tag: tag.to_string(),
            text: String::new(),
            phantom: PhantomData
        }
    }

    fn generate_id_tag(&self) -> String {
        if self.id.len() > 0 {
            return format!(" id=\"{}\"", self.id);
        }

        String::new()
    }

    fn generate_classes(&self) -> String { //Classe deve ser atualizada uma vez que os attrs sejam implementados
        
        if self.classes.len() > 0 {
            return format!(" class=\"{}\"", self.classes.join(" "));
        }
        
        

        String::new()
    }
}

