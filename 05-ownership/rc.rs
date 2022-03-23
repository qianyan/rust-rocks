use std::rc::Rc;

// Rust 如何创建不收栈内存控制的堆内存？
// Box::leak() -> 从堆内存泄漏出去，不受栈内存控制。
// 静态检查：靠编译器保证代码符合所有权规则；
// 动态检查，通过Box::leak()
// 让堆内存拥有不受限的生命周期，然后再运行过程中，通过对引用计数的检查，保证这样的的堆内存最终会得到释放。
fn main() {
//    let a = Rc::new(1);
//    let b = a.clone();
//    let c = a.clone();
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(node4));

    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());
    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    node3.update_downstream(Rc::new(node5));
    println!("node1: {:?}, node2: {:?}", node1, node2);
}

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}
