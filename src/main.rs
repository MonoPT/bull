mod lib;

use lib::node::Node;

//
// Make functions to 
// //  - make a children iterator
//  - Make a function to find if there is infinite recursion and if there is panic!
//  - Passar funções exclusivamente internas para ficheiro à parte de forma a isolar esses métodos (set_parent)
//  - parse the .bull file
// - Add support for comment elements when building outputs
// - Na hora de gerar ficheiro final verificar se está na lista de self closing nativo e fechar automáticamente

fn main() {
    let html = r"
        a.first.element#container-1#secondIdMakeIntoAClass @click='1 +1' x='3'
            b class='red blue'
            .class.ok
        #box
            a
                cd
                    df
                d
            br
        // isto é um comentário
    ".to_string();

    //Reads code from string and outputs string with result
    let root_from_string = Node::parse_string(html);


    //Reads code from file and outputs string with result
    let root = Node::parse_file("./example.bull");


    println!("{}", root.borrow().html());
}

/*
fn main() {
    let root = Node::new_html_element( "root", Option::None);
    
    let child1 = Node::new_html_element( "nod1", Some(&root));
    let child3 = Node::new_html_element( "nod3", Some(&child1));
    let child5 = Node::new_html_element( "nod5", Some(&child1));
    let child4 = Node::new_html_element( "nod4", Some(&root));
    let mut child2 = Node::new_html_element( "node2", Some(&child1));
    let child6 = Node::new_html_element( "nod6", Some(&child4));

    let mut current = Rc::clone(&child2);

    loop {
        let mut parent_node: Option<Rc<RefCell<Node>>> = None;

        match current.borrow().get_parent() {
            Some(parent) => {
                parent_node = Some(parent);
            }
            None => ()
        }

        match parent_node {
            None => break,
            Some(node) => {
                current = node;
            }
        }
    }

    let child = Node::new_html_element( "nova", None);

    Node::reparent_node(&child4, &child, true);
    
    let index = child.borrow().index_as_child().unwrap();

    Node::insert_child_at_index(index, &child, &child4);
    Node::reparent_node(&child4, &child3, true);

    let child2 = Node::new_html_element( "nova2", Some(&child));

    let mut next = Some(child1);
    for _ in 0..4 {
        
        match next {
            None => println!("nothing"),
            Some(node) => {
                
                //println!("\n current node: {}", node.borrow().id);

                match node.borrow().index_as_child() {
                    None => println!("is root"),
                    Some(pos) => {
                        println!("is index {} ", pos);
                    }
                }

                match node.borrow().previous_node() {
                    None => (),
                    Some(prv) => {
                        //println!("previous {}", prv.borrow().id)
                    }
                }
    
                next = node.borrow().next_node();
            }
        }
    }

    //root.borrow().print_tree();

    let text_node = Node::new_text_element("This is a string", Some(&child5));
    let child8 = Node::new_html_element( "nod8", Some(&text_node));

    let mut text = text_node.borrow_mut().set_text("This is a modified string");

    println!("{}", text_node.borrow().html(0));
    print!("{}", root.borrow().html(0));
    
}

*/