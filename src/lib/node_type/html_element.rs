use std::marker::PhantomData;

use super::super::node::{NodeType, Tag};

pub struct HtmlElement<'a, T> {
    id: String,
    classes: Vec<String>,
    tag: String,
    phantom: PhantomData<&'a T>,
    text: String,
    self_closed: bool,
    attributes: Vec<(String, String)>
}

impl<'a, T> NodeType<'a> for HtmlElement<'a, T> {
    fn html(&self) -> Tag {

        let id = self.generate_id_tag();
        let classes = self.generate_classes();
        
        let attributes = self.handle_attributes();
        let inline_text = &self.text;

        let mut tag_attrs = [id, classes, attributes].join(" ").trim().to_owned();

        if tag_attrs.len() > 0 {
            tag_attrs = format!(" {tag_attrs}");
        }

        //Generate regular tag
        if !self.self_closed {
            return Tag {
                open: format!("<{}{tag_attrs}>{inline_text}", self.tag),
                close: format!("</{}>", self.tag)
            }
        }
        
        //Generate tag self closed
        return Tag {
            open: format!("<{}{tag_attrs}/>{inline_text}", self.tag),
            close: "".to_owned()
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
    pub fn new(tag: &str, id: &str, classes: Vec<String>, self_closed: bool, attributes: Vec<(String, String)>, inline_text: String) -> Self {
        HtmlElement {
            id: id.to_owned(),
            classes: classes,
            tag: tag.to_string(),
            text: inline_text,
            phantom: PhantomData,
            self_closed,
            attributes
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

    fn handle_attributes(&self) -> String { //Expand later for reactivity
        let mut output = String::new();

        for attr in self.attributes.iter(){
            if attr.0.starts_with("@") {
                continue;
            }

            let mut att1 = format!("=\"{}\"", attr.1);
        
            if attr.1.len() < 1 {
                att1 = String::new();
            }


            output += &format!("{}{} ", attr.0, att1);
        }

        output
    }
}

