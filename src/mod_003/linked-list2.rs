use std::rc::Rc;

struct Node {
    data: i32,
    next: Option<Rc<Node>>
}

struct LinkedList {
    head: Option<Rc<Node>>,
    tail: Option<Rc<Node>> 
}

impl LinkedList {
    
    fn push(&self, data: i32) {
        match &self.tail {
            None => {  }
            Some(last_node) => {
                let new = Rc::new(Node{ data: data, next: None });
                let mut temp = last_node;
                temp.next = Some(Rc::clone(&new)); // error
                // TODO: Dude read the entire book
            }
        }
    }

}

fn main () {
    
    
    
}
