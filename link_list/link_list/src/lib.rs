mod link_list{
    type Link<T> = Option<Box<Node<T>>>;
    struct Rawlink<T> {
        p: *mut T,
    }
    struct Node<T>{ value: T, next: Link<T>}
    impl <T> Node<T>{
        fn new(value: T) -> Node<T>{
            Node{ value: value, next: None}
        }
    }

    pub struct List<T>{ head: Link<T>, tail: Link<T>, length: usize }
    impl <T> List<T>{
        fn new(&self) -> List<T>{
            List{ head: None, tail: None, length: 0}
        }
        //Add another value
        fn push(mut self, value: T){
            match self.tail {
                Some(node) => {
                    let mut old_tail = node;
                    let mut node = Box::new(Node::new(value));
                    self.tail = (node);
                    old_tail.next = Some(node);
                },
                None => {
                    self.head = Some(Box::new(Node::new(value)));
                    self.tail = self.head;
                }
            }
        }
    }
}
#[test]
fn it_works() {
    assert!(false);
}

