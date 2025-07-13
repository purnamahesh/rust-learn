#[derive(Debug)]
struct Node<'a> {
    next: Option<&'a Node<'a>>,
    data: i32
}

impl<'a> Node<'a> {
    pub fn new(data: i32) -> Self {
        Node { next: None, data: data }
    }
}

struct LinkedList<'a> {
    head:Option<&'a Node<'a>>,
    tail:Option<&'a Node<'a>>,
}


impl<'a> LinkedList<'a> {

    fn display(&self) {
        let mut temp = self.head;
        while temp.is_some() {
            match temp {
                Some(val) => {
                    print!("{} -> ",val.data);
                    temp = val.next;
                },
                None => {
                    println!("NULL");
                    break;
                }
            }
        }
    }

    fn push(&mut self, data: i32) {
        let new_node:Node<'a> = Node::new(data);
        match self.tail {
            Some(mut last_node) => {
                last_node.next = Some(&new_node);
            },
            None => {
                self.head = Some(&new_node);
                self.tail = Some(&new_node);
            }
        }
    }
}


fn main ( ) {
    let head:Option<&Node> = None;
    let tail:Option<&Node> = None;
    
    let mut v = vec![1,2,3];

    v.push(10);

    let mut node1 = Node ::new(10);
    let mut node2 = Node::new(20);
    node1.next = Some(&node2);

}